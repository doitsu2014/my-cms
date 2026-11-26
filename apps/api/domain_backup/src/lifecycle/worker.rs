//! Job worker (FN-16 claim, FN-18 `complete_incomplete`, FN-19 `fail_job`, FN-20
//! `enforce_deadline`).
//!
//! A queued job is claimed (CAS `queued -> running`), re-checked through the claim policy
//! port (FN-04, work-units discrepancy 1), given a registered allocation (register before
//! write), captured through the `CaptureAdapter` under the job deadline, and ends
//! `incomplete` (`REQUIRED_COMPONENTS_UNAVAILABLE`) or `failed` with a safe reason. It
//! never ends `succeeded` (ADR-002, D-10). A lost compare-and-set changes nothing.
//!
//! WU-05 red-tests compile shell: no behaviour yet.

use chrono::Duration as ChronoDuration;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use crate::domain::status::BackupJobStatus;
use crate::ports::allocation_store::{Allocation, AllocationError, BudgetKind};
use crate::ports::capture::{CaptureContext, CaptureError};
use crate::ports::job_store::{Components, TransitionResult};
use crate::ports::policy::PolicyDecision;
use uuid::Uuid;

use crate::domain::clock::Clock;
use crate::lifecycle::fixture_capture::FIXTURE_BYTES;
use crate::ports::allocation_store::AllocationStore;
use crate::ports::capture::CaptureAdapter;
use crate::ports::job_store::{JobRow, JobStore, ReasonCode, StoreError};
use crate::ports::policy::ClaimPolicy;

/// Terminal-write retries after a refused (audit-blocked) terminal transition
/// (FD section 3.2, FN-16..FN-20: "retries up to 3 times at 1 s intervals", code constants [D]).
pub const TERMINAL_RETRIES: u32 = 3;
pub const TERMINAL_RETRY_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    /// Current runtime epoch, stored on the claimed job.
    pub epoch: i64,
    /// CFG-10: `deadline_at = now + deadline` (default 600 s).
    pub deadline: Duration,
    /// Bytes reserved for, and written by, the synthetic fixture.
    pub fixture_bytes: u64,
    /// CFG-05: allocation `expires_at = now + allocation_ttl` (default 24 h).
    pub allocation_ttl: Duration,
    pub terminal_retries: u32,
    pub terminal_retry_interval: Duration,
}

impl WorkerConfig {
    pub fn new(epoch: i64) -> Self {
        Self {
            epoch,
            deadline: Duration::from_secs(600),
            fixture_bytes: FIXTURE_BYTES,
            allocation_ttl: Duration::from_secs(24 * 3600),
            terminal_retries: TERMINAL_RETRIES,
            terminal_retry_interval: TERMINAL_RETRY_INTERVAL,
        }
    }
}

/// What one worker run did to one job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    /// Nothing was queued (or this worker lost the claim race).
    Idle,
    /// `running -> incomplete`, `REQUIRED_COMPONENTS_UNAVAILABLE`.
    Incomplete { job_id: Uuid },
    /// `running|queued -> failed` with a safe reason.
    Failed { job_id: Uuid, reason: ReasonCode },
    /// Another actor won a compare-and-set (a terminal job is never overwritten); this
    /// worker changed nothing about the job.
    Lost { job_id: Uuid },
    /// The terminal write was refused (audit-blocked) after every retry: the job stays
    /// `running` and the sweeper resolves it after the deadline (FD 6.7).
    Stalled { job_id: Uuid },
}

pub struct Worker {
    jobs: Arc<dyn JobStore>,
    allocations: Arc<dyn AllocationStore>,
    adapter: Arc<dyn CaptureAdapter>,
    policy: Arc<dyn ClaimPolicy>,
    clock: Arc<dyn Clock>,
    config: WorkerConfig,
}

impl fmt::Debug for Worker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Worker").finish_non_exhaustive()
    }
}

impl Worker {
    pub fn new(
        jobs: Arc<dyn JobStore>,
        allocations: Arc<dyn AllocationStore>,
        adapter: Arc<dyn CaptureAdapter>,
        policy: Arc<dyn ClaimPolicy>,
        clock: Arc<dyn Clock>,
        config: WorkerConfig,
    ) -> Self {
        Self {
            jobs,
            allocations,
            adapter,
            policy,
            clock,
            config,
        }
    }

    /// Claim the oldest queued job (CAS) and run it. A refused claim (audit or store
    /// failure) is returned as the error: nothing ran and the job stays queued.
    pub async fn run_next(&self) -> Result<RunOutcome, StoreError> {
        let Some(claimed) = self
            .jobs
            .claim_next(self.config.epoch, self.config.deadline)
            .await?
        else {
            return Ok(RunOutcome::Idle);
        };
        Ok(self.execute(claimed).await)
    }

    /// Run a job this worker has claimed (`running`). The row may be stale: a terminal
    /// transition that loses the compare-and-set changes nothing.
    pub async fn execute(&self, claimed: JobRow) -> RunOutcome {
        let id = claimed.id;
        let current = match self.jobs.get(id).await {
            Ok(Some(row))
                if row.status == BackupJobStatus::Running
                    && row.state_version == claimed.state_version =>
            {
                row
            }
            _ => return RunOutcome::Lost { job_id: id },
        };
        if !matches!(
            self.policy
                .authorize_resource(&current.instance_id, id)
                .await,
            Ok(PolicyDecision::Allowed)
        ) {
            return self
                .fail(current, ReasonCode::InstanceNotAllowed, None)
                .await;
        }
        let now = self.clock.now();
        let Some(deadline_at) = current.deadline_at else {
            return self.fail(current, ReasonCode::PersistenceError, None).await;
        };
        let remaining = (deadline_at - now).to_std().unwrap_or(Duration::ZERO);
        if remaining.is_zero() {
            return self.fail(current, ReasonCode::DeadlineExceeded, None).await;
        }
        let expires = now.checked_add_signed(
            ChronoDuration::from_std(self.config.allocation_ttl).unwrap_or(ChronoDuration::MAX),
        );
        let Some(expires) = expires else {
            return self.fail(current, ReasonCode::PersistenceError, None).await;
        };
        let mut allocation = match self
            .allocations
            .register(Some(id), self.config.fixture_bytes, current.epoch, expires)
            .await
        {
            Ok(allocation) => allocation,
            Err(AllocationError::BudgetExceeded(BudgetKind::PerAllocation | BudgetKind::Total)) => {
                return self.fail(current, ReasonCode::AllocationLimit, None).await;
            }
            Err(_) => return self.fail(current, ReasonCode::PersistenceError, None).await,
        };
        // Registration may await long enough to consume part or all of the job's
        // deadline. Budget capture from the actual remaining time after it returns.
        let remaining = (deadline_at - self.clock.now())
            .to_std()
            .unwrap_or(Duration::ZERO);
        if remaining.is_zero() {
            return self
                .fail_with_allocation(current, ReasonCode::DeadlineExceeded, allocation)
                .await;
        }
        let ctx = CaptureContext {
            job_id: id,
            instance_id: current.instance_id.clone(),
            correlation_id: current.correlation_id.clone(),
            storage_key: allocation.storage_key.clone(),
        };
        let capture = tokio::time::timeout(remaining, self.adapter.capture(&ctx)).await;
        let outcome = match capture {
            Err(_) => {
                return self
                    .fail_with_allocation(current, ReasonCode::DeadlineExceeded, allocation)
                    .await
            }
            Ok(Err(CaptureError::Failed)) => {
                return self
                    .fail_with_allocation(current, ReasonCode::AdapterError, allocation)
                    .await
            }
            Ok(Ok(outcome)) => outcome,
        };
        if self
            .allocations
            .mark_written(allocation.id, allocation.state_version)
            .await
            .is_err()
        {
            return self
                .fail_with_allocation(current, ReasonCode::PersistenceError, allocation)
                .await;
        }
        let components = outcome.components;
        allocation.state_version += 1;
        match self
            .transition_retry(
                &current,
                BackupJobStatus::Incomplete,
                Some(ReasonCode::RequiredComponentsUnavailable),
                Some(components),
            )
            .await
        {
            Ok(TransitionResult::Applied) => RunOutcome::Incomplete { job_id: id },
            Ok(TransitionResult::Lost) => {
                self.release(&allocation).await;
                RunOutcome::Lost { job_id: id }
            }
            Err(_) => RunOutcome::Stalled { job_id: id },
        }
    }

    /// FN-19: a queued job that cannot be dispatched to a worker goes
    /// `queued -> failed` with `DISPATCH_FAILED`.
    pub async fn fail_dispatch(&self, job_id: Uuid) -> Result<RunOutcome, StoreError> {
        let Some(row) = self.jobs.get(job_id).await? else {
            return Ok(RunOutcome::Lost { job_id });
        };
        if row.status != BackupJobStatus::Queued {
            return Ok(RunOutcome::Lost { job_id });
        }
        match self
            .transition_retry(
                &row,
                BackupJobStatus::Failed,
                Some(ReasonCode::DispatchFailed),
                None,
            )
            .await
        {
            Ok(TransitionResult::Applied) => Ok(RunOutcome::Failed {
                job_id,
                reason: ReasonCode::DispatchFailed,
            }),
            Ok(TransitionResult::Lost) => Ok(RunOutcome::Lost { job_id }),
            Err(e) => Err(e),
        }
    }

    async fn release(&self, allocation: &Allocation) {
        let _ = self
            .allocations
            .release(allocation.id, allocation.state_version)
            .await;
    }

    async fn fail_with_allocation(
        &self,
        job: JobRow,
        reason: ReasonCode,
        allocation: Allocation,
    ) -> RunOutcome {
        let result = self.fail(job, reason, None).await;
        if !matches!(result, RunOutcome::Stalled { .. }) {
            self.release(&allocation).await;
        }
        result
    }

    async fn fail(
        &self,
        job: JobRow,
        reason: ReasonCode,
        components: Option<Components>,
    ) -> RunOutcome {
        match self
            .transition_retry(&job, BackupJobStatus::Failed, Some(reason), components)
            .await
        {
            Ok(TransitionResult::Applied) => RunOutcome::Failed {
                job_id: job.id,
                reason,
            },
            Ok(TransitionResult::Lost) => RunOutcome::Lost { job_id: job.id },
            Err(_) => RunOutcome::Stalled { job_id: job.id },
        }
    }

    async fn transition_retry(
        &self,
        job: &JobRow,
        to: BackupJobStatus,
        reason: Option<ReasonCode>,
        components: Option<Components>,
    ) -> Result<TransitionResult, StoreError> {
        for attempt in 0..=self.config.terminal_retries {
            match self
                .jobs
                .transition(
                    job.id,
                    (job.status, job.state_version),
                    to,
                    reason,
                    components.clone(),
                )
                .await
            {
                Err(StoreError::AuditUnavailable | StoreError::StoreUnavailable)
                    if attempt < self.config.terminal_retries =>
                {
                    tokio::time::sleep(self.config.terminal_retry_interval).await
                }
                other => return other,
            }
        }
        Err(StoreError::AuditUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::status::BackupJobStatus;
    use crate::lifecycle::fixture_capture::FixtureCaptureAdapter;
    use crate::lifecycle::test_support::{
        advance_both, contract_components, join_within, log_lines, new_log, t0, wait_entered,
        FakeClock, Log, MemAllocStore, MemJobStore, MemStorage, PolicyDouble, PolicyMode, Script,
        ScriptedAdapter, INSTANCE,
    };
    use crate::ports::allocation_store::{AllocationError, AllocationState, BudgetKind};
    use crate::ports::job_store::TransitionResult;
    use chrono::Duration as ChronoDuration;

    struct H {
        clock: Arc<FakeClock>,
        log: Log,
        jobs: Arc<MemJobStore>,
        allocs: Arc<MemAllocStore>,
        policy: Arc<PolicyDouble>,
        worker: Arc<Worker>,
    }

    fn harness_with(
        log: Log,
        allocs: Arc<MemAllocStore>,
        adapter: Arc<dyn CaptureAdapter>,
        mode: PolicyMode,
    ) -> H {
        let clock = FakeClock::new(t0());
        let jobs = MemJobStore::new(clock.clone(), log.clone());
        let policy = PolicyDouble::logged(mode, log.clone());
        let worker = Arc::new(Worker::new(
            jobs.clone(),
            allocs.clone(),
            adapter,
            policy.clone(),
            clock.clone(),
            WorkerConfig::new(1),
        ));
        H {
            clock,
            log,
            jobs,
            allocs,
            policy,
            worker,
        }
    }

    fn harness(adapter: Arc<ScriptedAdapter>, mode: PolicyMode, log: Log) -> H {
        let allocs = MemAllocStore::new(log.clone());
        harness_with(log, allocs, adapter, mode)
    }

    fn assert_never_succeeded(h: &H) {
        for call in h.jobs.calls() {
            assert!(
                !matches!(
                    call.to,
                    BackupJobStatus::Succeeded | BackupJobStatus::RecoveryRequired
                ),
                "a U1 job must never be moved to {:?}",
                call.to
            );
        }
    }

    /// A failed job carries one closed reason code, a finish time and nothing that could
    /// leak internals (IFC 10.1 closed `reason_code` set; no paths, SQL or credentials in
    /// any stored field of the row).
    fn assert_failed(h: &H, id: Uuid, reason: ReasonCode) {
        let row = h.jobs.row(id);
        assert_eq!(row.status, BackupJobStatus::Failed, "job must be failed");
        assert_eq!(row.reason_code.as_deref(), Some(reason.as_str()));
        assert!(row.finished_at.is_some());
        assert_safe_row(&row);
        assert_never_succeeded(h);
    }

    /// TS-48 "no leaked internals": inspect the stored row, which is what the status API
    /// projects (the `RunOutcome` type itself cannot carry text).
    fn assert_safe_row(row: &JobRow) {
        let code = row
            .reason_code
            .as_deref()
            .expect("a failed job has a reason");
        assert!(
            ReasonCode::ALL.iter().any(|r| r.as_str() == code),
            "reason code {code} is outside the closed set"
        );
        let text = format!("{row:?}");
        for banned in ["/", "\\", "sql", "postgres", "Bearer"] {
            assert!(!text.contains(banned), "job row leaks {banned}: {text}");
        }
    }

    /// No component outcome was recorded (the capture never produced one). IFC 2.3:
    /// `components` is `[]` until the worker records outcomes.
    fn assert_no_components(h: &H, id: Uuid) {
        assert!(
            h.jobs.row(id).components.0.is_empty(),
            "no component outcome exists yet"
        );
    }

    /// Whatever mechanism the worker uses, no allocation of the job may be left live.
    fn assert_no_live_allocation(h: &H, id: Uuid) {
        assert!(
            h.allocs.live_for(id).is_empty(),
            "a terminal job must not keep a registered or written allocation"
        );
    }

    // ------------------------------------------------------------------ TS-47

    /// TS-47 (AC-002 S2/S3; T-05, T-06; FN-16..FN-18): a queued job is claimed, runs through
    /// the fixture adapter and ends `incomplete` with the exact components array.
    #[tokio::test]
    async fn happy_path_ends_incomplete_with_exact_components() {
        let log = new_log();
        let storage = MemStorage::logged(log.clone());
        let adapter = Arc::new(FixtureCaptureAdapter::new(storage.clone()));
        let allocs = MemAllocStore::new(log.clone());
        let h = harness_with(log, allocs, adapter, PolicyMode::Allow);
        let id = h.jobs.seed_queued(INSTANCE);

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(outcome, RunOutcome::Incomplete { job_id: id });
        let row = h.jobs.row(id);
        assert_eq!(row.status, BackupJobStatus::Incomplete);
        assert_eq!(
            row.reason_code.as_deref(),
            Some("REQUIRED_COMPONENTS_UNAVAILABLE")
        );
        assert_eq!(row.components, contract_components());
        assert_eq!(
            row.deadline_at,
            Some(t0() + ChronoDuration::seconds(600)),
            "claim sets deadline_at = now + CFG-10"
        );
        assert_eq!(row.epoch, 1);
        assert_eq!(row.finished_at, Some(t0()));
        assert_never_succeeded(&h);

        // Register before write; the written allocation of an incomplete job is kept.
        let allocs = h.allocs.all();
        assert_eq!(allocs.len(), 1);
        assert_eq!(allocs[0].job_id, Some(id));
        assert_eq!(allocs[0].state, AllocationState::Written);
        assert_eq!(allocs[0].bytes_reserved, FIXTURE_BYTES);
        assert_eq!(allocs[0].expires_at, t0() + ChronoDuration::hours(24));
        let writes = storage.writes();
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0].0, allocs[0].storage_key);
        assert_eq!(writes[0].1.len() as u64, FIXTURE_BYTES);
        assert_eq!(
            log_lines(&h.log),
            [
                "claim",
                "policy",
                "register",
                "write",
                "mark_written",
                "transition:incomplete"
            ]
        );
    }

    /// TS-47: never `succeeded` or `recovery-required`, asserted over every path.
    #[tokio::test]
    async fn no_path_ends_succeeded_or_recovery_required() {
        #[derive(Clone, Copy, Debug)]
        enum Path {
            Happy,
            AdapterFails,
            PolicyDenies,
            PolicyUnavailable,
            BudgetExceeded,
            RegistrationRefused,
            MarkWrittenLost,
        }
        for path in [
            Path::Happy,
            Path::AdapterFails,
            Path::PolicyDenies,
            Path::PolicyUnavailable,
            Path::BudgetExceeded,
            Path::RegistrationRefused,
            Path::MarkWrittenLost,
        ] {
            let log = new_log();
            let script = if matches!(path, Path::AdapterFails) {
                Script::Fail
            } else {
                Script::Ok
            };
            let adapter = ScriptedAdapter::logged(script, log.clone());
            let mode = match path {
                Path::PolicyDenies => PolicyMode::Deny,
                Path::PolicyUnavailable => PolicyMode::Error,
                _ => PolicyMode::Allow,
            };
            let allocs = if matches!(path, Path::BudgetExceeded) {
                MemAllocStore::with_alloc_max(log.clone(), 1)
            } else {
                MemAllocStore::new(log.clone())
            };
            match path {
                Path::RegistrationRefused => {
                    allocs.fail_register(AllocationError::AuditUnavailable)
                }
                Path::MarkWrittenLost => allocs.fail_mark_written(AllocationError::Lost),
                _ => {}
            }
            let h = harness_with(log, allocs, adapter, mode);
            let id = h.jobs.seed_queued(INSTANCE);

            let outcome = h.worker.run_next().await;

            assert!(outcome.is_ok(), "{path:?}: claim must work: {outcome:?}");
            let status = h.jobs.row(id).status;
            assert!(
                matches!(
                    status,
                    BackupJobStatus::Incomplete | BackupJobStatus::Failed
                ),
                "{path:?}: job must end incomplete or failed, got {status:?}"
            );
            assert_never_succeeded(&h);
        }
    }

    // ------------------------------------------------------------------ TS-48

    /// TS-48 (AC-002 S2; T-12): an adapter error fails the job `ADAPTER_ERROR` and releases
    /// the allocation immediately.
    #[tokio::test]
    async fn adapter_error_fails_job_and_releases_allocation() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Fail, log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::AdapterError
            }
        );
        assert_failed(&h, id, ReasonCode::AdapterError);
        assert_no_components(&h, id);
        let allocs = h.allocs.all();
        assert_eq!(allocs.len(), 1, "registered before the capture");
        assert_eq!(allocs[0].state, AllocationState::ReleasePending);
    }

    /// TS-48: `mark_written` CAS failure is `PERSISTENCE_ERROR` and releases the allocation.
    #[tokio::test]
    async fn mark_written_failure_fails_job_persistence_error() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let allocs = MemAllocStore::new(log.clone());
        allocs.fail_mark_written(AllocationError::Lost);
        let h = harness_with(log, allocs, adapter, PolicyMode::Allow);
        let id = h.jobs.seed_queued(INSTANCE);

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::PersistenceError
            }
        );
        assert_failed(&h, id, ReasonCode::PersistenceError);
        let allocs = h.allocs.all();
        assert_eq!(allocs.len(), 1);
        assert_eq!(allocs[0].state, AllocationState::ReleasePending);
    }

    /// TS-48, FN-21: ledger or audit unavailable at registration is `PERSISTENCE_ERROR`,
    /// not `ALLOCATION_LIMIT`, and nothing is captured or written.
    #[tokio::test]
    async fn registration_refused_fails_job_persistence_error() {
        for refusal in [
            AllocationError::AuditUnavailable,
            AllocationError::StoreUnavailable,
            AllocationError::Persistence,
        ] {
            let log = new_log();
            let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
            let allocs = MemAllocStore::new(log.clone());
            allocs.fail_register(refusal);
            let h = harness_with(log, allocs, adapter.clone(), PolicyMode::Allow);
            let id = h.jobs.seed_queued(INSTANCE);

            let outcome = h.worker.run_next().await.expect("claim succeeds");

            assert_eq!(
                outcome,
                RunOutcome::Failed {
                    job_id: id,
                    reason: ReasonCode::PersistenceError
                }
            );
            assert_failed(&h, id, ReasonCode::PersistenceError);
            assert_no_components(&h, id);
            assert_eq!(
                adapter.calls(),
                0,
                "nothing is captured without an allocation"
            );
            assert!(h.allocs.all().is_empty());
        }
    }

    /// TS-48 (AC-002 S2; T-13): a budget refusal fails the job `ALLOCATION_LIMIT`; nothing
    /// is captured.
    #[tokio::test]
    async fn budget_exceeded_fails_job_allocation_limit() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let allocs = MemAllocStore::with_alloc_max(log.clone(), FIXTURE_BYTES - 1);
        let h = harness_with(log, allocs, adapter.clone(), PolicyMode::Allow);
        let id = h.jobs.seed_queued(INSTANCE);

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::AllocationLimit
            }
        );
        assert_failed(&h, id, ReasonCode::AllocationLimit);
        assert_no_components(&h, id);
        assert_eq!(adapter.calls(), 0);
        assert!(
            h.allocs.all().is_empty(),
            "a refused registration leaves no row"
        );
    }

    /// TS-48 (T-13; IFC 6.1 step 1, FN-21): the 1 GiB total budget (CFG-13) is the
    /// reachable `ALLOCATION_LIMIT` under the fixed CFG-12; it maps the same way, and no
    /// capture or file write happens. Both `BudgetKind`s are covered.
    #[tokio::test]
    async fn total_budget_exhausted_fails_job_allocation_limit() {
        let gib = 1024 * 1024 * 1024;
        for kind in [BudgetKind::PerAllocation, BudgetKind::Total] {
            let log = new_log();
            let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
            let allocs = match kind {
                BudgetKind::PerAllocation => {
                    MemAllocStore::with_limits(log.clone(), FIXTURE_BYTES - 1, gib)
                }
                BudgetKind::Total => {
                    let a = MemAllocStore::new(log.clone());
                    a.seed_live(gib - FIXTURE_BYTES + 1);
                    a
                }
            };
            let seeded = allocs.all().len();
            let h = harness_with(log, allocs, adapter.clone(), PolicyMode::Allow);
            let id = h.jobs.seed_queued(INSTANCE);

            let outcome = h.worker.run_next().await.expect("claim succeeds");

            assert_eq!(
                outcome,
                RunOutcome::Failed {
                    job_id: id,
                    reason: ReasonCode::AllocationLimit
                },
                "{kind:?}"
            );
            assert_failed(&h, id, ReasonCode::AllocationLimit);
            assert_eq!(adapter.calls(), 0, "{kind:?}: nothing is captured");
            assert_eq!(h.allocs.all().len(), seeded, "{kind:?}: no new row");
        }
    }

    /// TS-48 (FN-19): a queued job that cannot be dispatched fails `DISPATCH_FAILED`
    /// (`queued -> failed`), without claiming, registering or capturing.
    #[tokio::test]
    async fn undispatchable_queued_job_fails_dispatch_failed() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);

        let outcome = h
            .worker
            .fail_dispatch(id)
            .await
            .expect("dispatch failure recorded");

        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::DispatchFailed
            }
        );
        assert_failed(&h, id, ReasonCode::DispatchFailed);
        assert_no_components(&h, id);
        assert_eq!(h.jobs.row(id).started_at, None, "never ran");
        assert_eq!(log_lines(&h.log), ["transition:failed"]);
        assert_eq!(adapter.calls(), 0);
        assert!(h.allocs.all().is_empty());
    }

    /// TS-48 / TS-67: dispatch failure of a job that is no longer queued changes nothing.
    #[tokio::test]
    async fn dispatch_failure_does_not_touch_a_running_job() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let running = h.worker.clone();
        let mut task = tokio::spawn(async move { running.run_next().await });
        wait_entered(&adapter, &mut task).await;

        let outcome = h.worker.fail_dispatch(id).await.expect("no error");

        assert_eq!(outcome, RunOutcome::Lost { job_id: id });
        assert_eq!(h.jobs.row(id).status, BackupJobStatus::Running);
        adapter.release.notify_one();
        let done = task.await.expect("join").expect("claim");
        assert_eq!(done, RunOutcome::Incomplete { job_id: id });
    }

    // ------------------------------------------------------------------ TS-67

    /// TS-67 (AC-002 S1, S2; T-07, T-12): duplicate dispatch of one queued job has exactly
    /// one winner. The winner is held inside the adapter while the duplicate runs, so the
    /// interleaving is explicit and no sleep is used.
    #[tokio::test]
    async fn duplicate_dispatch_has_exactly_one_winner() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let winner = h.worker.clone();
        let mut winner_task = tokio::spawn(async move { winner.run_next().await });
        wait_entered(&adapter, &mut winner_task).await;

        let duplicate = h
            .worker
            .run_next()
            .await
            .expect("duplicate dispatch is not an error");

        assert_eq!(duplicate, RunOutcome::Idle, "the duplicate lost the claim");
        adapter.release.notify_one();
        let won = winner_task.await.expect("join").expect("claim");
        assert_eq!(won, RunOutcome::Incomplete { job_id: id });
        assert_eq!(adapter.calls(), 1, "exactly one capture");
        assert_eq!(h.allocs.all().len(), 1, "exactly one allocation");
        let claims = log_lines(&h.log).iter().filter(|l| *l == "claim").count();
        assert_eq!(claims, 1, "exactly one claim");
        let applied: Vec<_> = h
            .jobs
            .calls()
            .into_iter()
            .filter(|c| c.result == Ok(true))
            .collect();
        assert_eq!(applied.len(), 1, "exactly one terminal transition applied");
        assert_eq!(h.jobs.row(id).status, BackupJobStatus::Incomplete);
    }

    /// TS-67 (T-07): two claimers are both in flight at the claim (a barrier inside the
    /// store holds each until the other has arrived); exactly one wins, the other is idle.
    #[tokio::test]
    async fn simultaneous_claims_have_exactly_one_winner() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        h.jobs.gate_claims(2);
        let (a, b) = (h.worker.clone(), h.worker.clone());
        let ta = tokio::spawn(async move { a.run_next().await });
        let tb = tokio::spawn(async move { b.run_next().await });

        let joined = tokio::time::timeout(Duration::from_secs(30), async {
            (ta.await.expect("join a"), tb.await.expect("join b"))
        })
        .await
        .expect("both claimers finish");

        let mut results = vec![joined.0.expect("claim a"), joined.1.expect("claim b")];
        results.sort_by_key(|r| format!("{r:?}"));
        assert_eq!(
            results,
            [RunOutcome::Idle, RunOutcome::Incomplete { job_id: id }]
        );
        assert_eq!(adapter.calls(), 1, "exactly one capture");
        assert_eq!(h.allocs.all().len(), 1, "exactly one allocation");
        let claims = log_lines(&h.log).iter().filter(|l| *l == "claim").count();
        assert_eq!(claims, 1, "exactly one claim");
    }

    /// TS-67 (T-12; FD 7.1: "the loser neither replays effects nor overwrites a terminal
    /// result"): a late worker holding a stale claim cannot overwrite a terminal job. The
    /// property is asserted, not the mechanism: the late worker may pre-read the job or may
    /// lose the compare-and-set, but the row, the applied transitions and the allocation
    /// ledger must end the same way either way.
    #[tokio::test]
    async fn late_worker_cannot_overwrite_a_terminal_job() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter, PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let claimed = h
            .jobs
            .claim_next(1, Duration::from_secs(600))
            .await
            .expect("claim")
            .expect("a queued job");
        // Someone else (the sweeper, past the deadline) finishes the job first.
        h.clock.advance(Duration::from_secs(600));
        let swept = h.jobs.sweep_deadline(id).await;
        assert_eq!(swept, TransitionResult::Applied);
        let before = h.jobs.row(id);

        let outcome = h.worker.execute(claimed).await;

        assert_eq!(outcome, RunOutcome::Lost { job_id: id });
        let after = h.jobs.row(id);
        assert_eq!(after, before, "the terminal job is untouched");
        assert_eq!(after.status, BackupJobStatus::Failed);
        assert_eq!(after.reason_code.as_deref(), Some("DEADLINE_EXCEEDED"));
        let applied: Vec<_> = h
            .jobs
            .calls()
            .into_iter()
            .filter(|c| c.result == Ok(true))
            .map(|c| c.to)
            .collect();
        assert_eq!(
            applied,
            [BackupJobStatus::Failed],
            "only the sweeper's terminal transition was applied"
        );
        assert_no_live_allocation(&h, id);
    }

    // ------------------------------------------------------------------ TS-68

    /// TS-68 (AC-002 S2; T-12, T-13; FN-20): a capture that never finishes hits the
    /// 10-minute deadline: `running -> failed DEADLINE_EXCEEDED`, allocation released.
    #[tokio::test(start_paused = true)]
    async fn deadline_exceeded_fails_job() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let worker = h.worker.clone();
        let mut task = tokio::spawn(async move { worker.run_next().await });
        wait_entered(&adapter, &mut task).await;

        advance_both(&h.clock, Duration::from_secs(600)).await;

        let outcome = join_within(task).await.expect("claim");
        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::DeadlineExceeded
            }
        );
        assert_failed(&h, id, ReasonCode::DeadlineExceeded);
        assert_eq!(
            h.jobs.row(id).finished_at,
            Some(t0() + ChronoDuration::seconds(600))
        );
        let allocs = h.allocs.all();
        assert_eq!(allocs.len(), 1);
        assert_eq!(allocs[0].state, AllocationState::ReleasePending);
    }

    /// TS-68: registration consumes five minutes of both wall and paused Tokio time. Capture
    /// only gets the five minutes actually left before `deadline_at`, not the stale ten-minute
    /// duration computed before the awaited registration.
    #[tokio::test(start_paused = true)]
    async fn deadline_budget_excludes_time_spent_registering_allocation() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let allocs = MemAllocStore::new(log.clone());
        let h = harness_with(log, allocs, adapter.clone(), PolicyMode::Allow);
        h.allocs
            .delay_next_register(h.clock.clone(), Duration::from_secs(300));
        let id = h.jobs.seed_queued(INSTANCE);
        let worker = h.worker.clone();
        let mut task = tokio::spawn(async move { worker.run_next().await });
        tokio::select! {
            _ = adapter.entered.notified() => {}
            finished = &mut task => panic!("worker finished before capture: {finished:?}"),
        }

        advance_both(&h.clock, Duration::from_secs(299)).await;
        assert!(
            !task.is_finished(),
            "599 s after claim is still inside deadline"
        );
        assert_eq!(h.jobs.row(id).status, BackupJobStatus::Running);
        advance_both(&h.clock, Duration::from_secs(1)).await;

        let outcome = join_within(task).await.expect("claim");
        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::DeadlineExceeded
            }
        );
        assert_failed(&h, id, ReasonCode::DeadlineExceeded);
        assert_no_components(&h, id);
        assert_eq!(adapter.calls(), 1, "capture entered but did not complete");
        let allocations = h.allocs.all();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].state, AllocationState::ReleasePending);
        assert_eq!(
            h.jobs.row(id).finished_at,
            Some(t0() + ChronoDuration::seconds(600))
        );
    }

    /// TS-68 (CFG-10): the deadline is exactly 600 s, not earlier.
    #[tokio::test(start_paused = true)]
    async fn deadline_is_not_enforced_before_it_is_due() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let worker = h.worker.clone();
        let mut task = tokio::spawn(async move { worker.run_next().await });
        wait_entered(&adapter, &mut task).await;

        advance_both(&h.clock, Duration::from_secs(599)).await;

        assert!(!task.is_finished(), "599 s is inside the deadline");
        assert_eq!(h.jobs.row(id).status, BackupJobStatus::Running);
        advance_both(&h.clock, Duration::from_secs(1)).await;
        let outcome = join_within(task).await.expect("claim");
        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::DeadlineExceeded
            }
        );
    }

    /// TS-68 (test-strategy: the in-flight timeout is `deadline_at - clock.now()`; FD
    /// FN-20 "timeout for remaining deadline"): a worker that starts 300 s after the claim
    /// has 300 s left, not a fresh 600 s.
    #[tokio::test(start_paused = true)]
    async fn deadline_is_the_time_remaining_until_deadline_at() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let claimed = h
            .jobs
            .claim_next(1, Duration::from_secs(600))
            .await
            .expect("claim")
            .expect("queued job");
        advance_both(&h.clock, Duration::from_secs(300)).await;
        let worker = h.worker.clone();
        let mut task = tokio::spawn(async move { worker.execute(claimed).await });
        wait_entered(&adapter, &mut task).await;

        advance_both(&h.clock, Duration::from_secs(299)).await;

        assert!(
            !task.is_finished(),
            "599 s after the claim is inside the deadline"
        );
        assert_eq!(h.jobs.row(id).status, BackupJobStatus::Running);
        advance_both(&h.clock, Duration::from_secs(1)).await;
        let outcome = join_within(task).await;
        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::DeadlineExceeded
            }
        );
        assert_eq!(
            h.jobs.row(id).finished_at,
            Some(t0() + ChronoDuration::seconds(600))
        );
    }

    /// TS-68: a claim whose deadline has already passed fails with `DEADLINE_EXCEEDED`.
    #[tokio::test]
    async fn deadline_already_elapsed_fails_without_capture() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let claimed = h
            .jobs
            .claim_next(1, Duration::from_secs(600))
            .await
            .expect("claim")
            .expect("queued job");
        h.clock.advance(Duration::from_secs(601));

        let outcome = h.worker.execute(claimed).await;

        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::DeadlineExceeded
            }
        );
        assert_failed(&h, id, ReasonCode::DeadlineExceeded);
        assert_no_live_allocation(&h, id);
    }

    /// TS-68 (T-12): the sweeper wins first; the late worker's own result does not
    /// overwrite `failed DEADLINE_EXCEEDED`.
    #[tokio::test]
    async fn late_worker_does_not_overwrite_sweeper_result() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let worker = h.worker.clone();
        let mut task = tokio::spawn(async move { worker.run_next().await });
        wait_entered(&adapter, &mut task).await;
        h.clock.advance(Duration::from_secs(600));
        let swept = h.jobs.sweep_deadline(id).await;
        assert_eq!(swept, TransitionResult::Applied);

        adapter.release.notify_one();
        let outcome = task.await.expect("join").expect("claim");

        assert_eq!(outcome, RunOutcome::Lost { job_id: id });
        assert_failed(&h, id, ReasonCode::DeadlineExceeded);
        assert_eq!(
            h.jobs.row(id).state_version,
            2,
            "claim, then the sweeper only"
        );
        assert!(
            h.allocs
                .all()
                .iter()
                .all(|a| a.state == AllocationState::ReleasePending),
            "the failed job's allocation is released"
        );
    }

    /// TS-68: at the deadline the sweeper acts first (it reads the Clock), then the
    /// worker's own timer fires: exactly one terminal result, no allocation left live.
    #[tokio::test(start_paused = true)]
    async fn sweeper_first_then_worker_timer_has_one_terminal_result() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let worker = h.worker.clone();
        let mut task = tokio::spawn(async move { worker.run_next().await });
        wait_entered(&adapter, &mut task).await;
        h.clock.advance(Duration::from_secs(600));
        let swept = h.jobs.sweep_deadline(id).await;
        assert_eq!(swept, TransitionResult::Applied);

        tokio::time::advance(Duration::from_secs(600)).await;

        let outcome = join_within(task).await.expect("claim");
        assert_eq!(outcome, RunOutcome::Lost { job_id: id });
        assert_failed(&h, id, ReasonCode::DeadlineExceeded);
        let applied_terminal = h
            .jobs
            .calls()
            .into_iter()
            .filter(|c| c.to != BackupJobStatus::Running && c.result == Ok(true))
            .count();
        assert_eq!(applied_terminal, 1);
        assert_no_live_allocation(&h, id);
    }

    /// TS-68 (reverse order): the worker's timer fires first and commits
    /// `failed DEADLINE_EXCEEDED`; a sweeper that read the job earlier then loses its
    /// compare-and-set and changes nothing.
    #[tokio::test(start_paused = true)]
    async fn worker_timer_first_then_late_sweeper_loses() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Block(Box::new(Script::Ok)), log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        let worker = h.worker.clone();
        let mut task = tokio::spawn(async move { worker.run_next().await });
        wait_entered(&adapter, &mut task).await;
        let sweeper_view = h.jobs.row(id);

        advance_both(&h.clock, Duration::from_secs(600)).await;

        let outcome = join_within(task).await.expect("claim");
        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::DeadlineExceeded
            }
        );
        let committed = h.jobs.row(id);
        let swept = h.jobs.sweep_from(&sweeper_view).await;
        assert_eq!(swept, TransitionResult::Lost);
        assert_eq!(
            h.jobs.row(id),
            committed,
            "the late sweeper changed nothing"
        );
        assert_failed(&h, id, ReasonCode::DeadlineExceeded);
        let applied_terminal = h
            .jobs
            .calls()
            .into_iter()
            .filter(|c| c.to != BackupJobStatus::Running && c.result == Ok(true))
            .count();
        assert_eq!(
            applied_terminal, 1,
            "exactly one terminal transition applied"
        );
        assert_no_live_allocation(&h, id);
    }

    // ------------------------------------------------------------------ TS-05a

    /// TS-05a (AC-005 S1; T-02): a denying policy at claim fails the job
    /// `INSTANCE_NOT_ALLOWED` (`running -> failed`); no allocation, no capture.
    #[tokio::test]
    async fn policy_denial_at_claim_fails_job_instance_not_allowed() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter.clone(), PolicyMode::Deny, log);
        let id = h.jobs.seed_queued("revoked-instance");

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::InstanceNotAllowed
            }
        );
        assert_failed(&h, id, ReasonCode::InstanceNotAllowed);
        assert_no_components(&h, id);
        assert_eq!(
            log_lines(&h.log),
            ["claim", "policy", "transition:failed"],
            "the re-check runs after the claim and before any allocation"
        );
        assert_eq!(adapter.calls(), 0, "the job does not run");
        assert!(h.allocs.all().is_empty());
        let call = &h.jobs.calls()[0];
        assert_eq!(call.to, BackupJobStatus::Failed);
        assert!(h.jobs.row(id).started_at.is_some(), "it was claimed first");
    }

    /// TS-05a: an unavailable policy fails closed, the same way.
    #[tokio::test]
    async fn policy_unavailable_at_claim_fails_closed() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter.clone(), PolicyMode::Error, log);
        let id = h.jobs.seed_queued(INSTANCE);

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(
            outcome,
            RunOutcome::Failed {
                job_id: id,
                reason: ReasonCode::InstanceNotAllowed
            }
        );
        assert_failed(&h, id, ReasonCode::InstanceNotAllowed);
        assert_eq!(adapter.calls(), 0);
        assert!(h.allocs.all().is_empty());
    }

    /// TS-05a / TS-67: the claim re-check is made once, for the claimed job's own instance.
    #[tokio::test]
    async fn claim_recheck_is_made_once_for_the_jobs_instance() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter.clone(), PolicyMode::Allow, log);
        let id = h.jobs.seed_queued("instance-under-test");

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(outcome, RunOutcome::Incomplete { job_id: id });
        assert_eq!(
            h.policy.calls(),
            vec![("instance-under-test".to_owned(), id)]
        );
        let contexts = adapter.contexts();
        assert_eq!(contexts.len(), 1);
        assert_eq!(contexts[0].job_id, id);
        assert_eq!(contexts[0].instance_id, "instance-under-test");
        assert_eq!(contexts[0].correlation_id, h.jobs.row(id).correlation_id);
        assert_eq!(
            contexts[0].storage_key,
            h.allocs.all()[0].storage_key,
            "the adapter writes under the registered allocation key"
        );
    }

    // ------------------------------------------------------------------ terminal audit failure (FD 3.2 FN-16..FN-20, 6.6, 6.7)

    /// FD 3.2 (FN-16..FN-20): a refused (audit-blocked) terminal write is retried 3 times at 1 s
    /// intervals, then the job stays `running` (fail closed, no unaudited effect). The
    /// allocation of a running job is kept.
    #[tokio::test(start_paused = true)]
    async fn refused_terminal_write_retries_three_times_then_leaves_job_running() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter, PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        h.jobs
            .inject_terminal_failures((0..8).map(|_| StoreError::AuditUnavailable).collect());

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(outcome, RunOutcome::Stalled { job_id: id });
        let row = h.jobs.row(id);
        assert_eq!(row.status, BackupJobStatus::Running);
        assert_eq!(row.state_version, 1, "no terminal write was applied");
        let times = h.jobs.terminal_attempt_times();
        assert_eq!(times.len(), 4, "one attempt and three retries");
        for pair in times.windows(2) {
            assert_eq!(pair[1] - pair[0], Duration::from_secs(1));
        }
        let allocs = h.allocs.all();
        assert_eq!(allocs.len(), 1);
        assert_eq!(
            allocs[0].state,
            AllocationState::Written,
            "expiry alone never deletes the work of a running job"
        );
        assert_never_succeeded(&h);
    }

    /// FD 3.2 (FN-16..FN-20): a transient audit refusal within the retries still completes
    /// the job.
    #[tokio::test(start_paused = true)]
    async fn transient_audit_refusal_recovers_within_the_retries() {
        let log = new_log();
        let adapter = ScriptedAdapter::logged(Script::Ok, log.clone());
        let h = harness(adapter, PolicyMode::Allow, log);
        let id = h.jobs.seed_queued(INSTANCE);
        h.jobs.inject_terminal_failures(vec![
            StoreError::AuditUnavailable,
            StoreError::AuditUnavailable,
        ]);

        let outcome = h.worker.run_next().await.expect("claim succeeds");

        assert_eq!(outcome, RunOutcome::Incomplete { job_id: id });
        assert_eq!(h.jobs.row(id).status, BackupJobStatus::Incomplete);
        assert_eq!(h.jobs.terminal_attempt_times().len(), 3);
    }

    /// FD 6.6 / FD 3.2: every terminal path (completion, adapter failure, persistence
    /// failure, elapsed deadline) retries a refused terminal write 3 times at 1 s, whether
    /// the refusal is `AuditUnavailable` or `StoreUnavailable`, then leaves the job
    /// `running` with `Stalled`. While the job is still running its allocation is not
    /// released (IFC 6.1 step 4, 6.1a: "registered | running, current epoch | unchanged").
    /// The `degraded` hand-off of FD 6.7 is WU-06 (TS-29), not asserted here.
    #[tokio::test(start_paused = true)]
    async fn refused_terminal_write_stalls_on_every_path_and_keeps_the_allocation() {
        #[derive(Clone, Copy, Debug)]
        enum Path {
            Completion,
            AdapterError,
            PersistenceError,
            ElapsedDeadline,
        }
        for path in [
            Path::Completion,
            Path::AdapterError,
            Path::PersistenceError,
            Path::ElapsedDeadline,
        ] {
            for audit_refusal in [true, false] {
                let make = || {
                    if audit_refusal {
                        StoreError::AuditUnavailable
                    } else {
                        StoreError::StoreUnavailable
                    }
                };
                let log = new_log();
                let script = if matches!(path, Path::AdapterError) {
                    Script::Fail
                } else {
                    Script::Ok
                };
                let adapter = ScriptedAdapter::logged(script, log.clone());
                let allocs = MemAllocStore::new(log.clone());
                if matches!(path, Path::PersistenceError) {
                    allocs.fail_mark_written(AllocationError::Lost);
                }
                let h = harness_with(log, allocs, adapter, PolicyMode::Allow);
                let id = h.jobs.seed_queued(INSTANCE);
                let label = format!("{path:?} / audit_refusal={audit_refusal}");
                h.jobs
                    .inject_terminal_failures((0..8).map(|_| make()).collect());

                let outcome = if matches!(path, Path::ElapsedDeadline) {
                    let claimed = h
                        .jobs
                        .claim_next(1, Duration::from_secs(600))
                        .await
                        .expect("claim")
                        .expect("queued job");
                    h.clock.advance(Duration::from_secs(601));
                    h.worker.execute(claimed).await
                } else {
                    h.worker.run_next().await.expect("claim succeeds")
                };

                assert_eq!(outcome, RunOutcome::Stalled { job_id: id }, "{label}");
                let row = h.jobs.row(id);
                assert_eq!(row.status, BackupJobStatus::Running, "{label}");
                assert_eq!(row.state_version, 1, "{label}: no terminal write applied");
                let times = h.jobs.terminal_attempt_times();
                assert_eq!(times.len(), 4, "{label}: one attempt and three retries");
                for pair in times.windows(2) {
                    assert_eq!(pair[1] - pair[0], Duration::from_secs(1), "{label}");
                }
                for a in h.allocs.all() {
                    assert!(
                        matches!(
                            a.state,
                            AllocationState::Registered | AllocationState::Written
                        ),
                        "{label}: a running job's allocation must not be released: {:?}",
                        a.state
                    );
                }
                assert_never_succeeded(&h);
            }
        }
    }

    // ------------------------------------------------------------------ configuration

    /// FN-01, IFC section 8: the worker's deadline and allocation TTL are the CFG-10 and
    /// CFG-05 values. Structural drift guard (not red): `config::load` accepts only the
    /// owner-approved defaults, so a bounds-derived constructor could not differ from
    /// these constants; this pins the constants to the validated bounds.
    #[test]
    fn worker_config_defaults_equal_the_validated_bounds() {
        use crate::domain::config::BackupBounds;
        let bounds = BackupBounds::default();
        let cfg = WorkerConfig::new(1);
        assert_eq!(cfg.deadline.as_secs(), bounds.job_deadline_secs());
        assert_eq!(cfg.allocation_ttl.as_secs(), bounds.staging_expiry_secs());
    }
}
