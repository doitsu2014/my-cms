//! In-memory doubles and fixtures for the WU-05 worker tests. Test-only. `FakeClock` is
//! defined here because `domain::clock` ships only `SystemClock` (IFC section 9:
//! fault-injecting wrappers and `FakeClock` live behind `#[cfg(test)]`).

use async_trait::async_trait;
use chrono::{DateTime, Duration as ChronoDuration, TimeZone, Utc};
use sea_orm::DatabaseTransaction;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Notify;
use uuid::Uuid;

use crate::domain::clock::Clock;
use crate::domain::state_machine::is_allowed;
use crate::domain::status::BackupJobStatus;
use crate::ports::allocation_store::{
    Allocation, AllocationError, AllocationState, AllocationStore, AllocationTotals, BudgetKind,
    CleanupFailureOutcome,
};
use crate::ports::capture::{CaptureAdapter, CaptureContext, CaptureError, CaptureOutcome};
use crate::ports::fixture_storage::{
    DeleteOutcome, FixtureStorage, InventoryPage, StorageError, StorageKey,
};
use crate::ports::job_store::{
    AcceptResult, ComponentName, ComponentOutcome, ComponentState, Components, IdemRecord, JobRow,
    JobStore, NewJob, ReasonCode, StoreError, TransitionResult,
};
use crate::ports::policy::{ClaimPolicy, PolicyDecision, PolicyError};

pub(crate) const INSTANCE: &str = "local-1";

pub(crate) fn t0() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 22, 12, 0, 0).unwrap()
}

/// Shared ordered event log: each double appends one line per call.
pub(crate) type Log = Arc<Mutex<Vec<String>>>;

pub(crate) fn new_log() -> Log {
    Arc::default()
}

pub(crate) fn log_lines(log: &Log) -> Vec<String> {
    log.lock().unwrap().clone()
}

fn push(log: &Log, line: impl Into<String>) {
    log.lock().unwrap().push(line.into());
}

#[derive(Debug)]
pub(crate) struct FakeClock(Mutex<DateTime<Utc>>);

impl FakeClock {
    pub fn new(at: DateTime<Utc>) -> Arc<Self> {
        Arc::new(Self(Mutex::new(at)))
    }
    pub fn advance(&self, by: Duration) {
        let mut g = self.0.lock().unwrap();
        *g += ChronoDuration::from_std(by).unwrap();
    }
}

impl Clock for FakeClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

/// Advance the fake wall clock and tokio's paused time together (`advance_both`, TS-68).
pub(crate) async fn advance_both(clock: &FakeClock, by: Duration) {
    clock.advance(by);
    tokio::time::advance(by).await;
}

/// The exact components array of the contract (IFC section 2.3, TS-47).
pub(crate) fn contract_components() -> Components {
    Components(vec![
        ComponentState {
            name: ComponentName::Database,
            state: ComponentOutcome::SyntheticCaptured,
            reason_code: Some("DB_SYNTHETIC_ONLY".to_owned()),
        },
        ComponentState {
            name: ComponentName::Auth,
            state: ComponentOutcome::Unavailable,
            reason_code: Some("AUTH_NOT_CAPTURED".to_owned()),
        },
        ComponentState {
            name: ComponentName::Storage,
            state: ComponentOutcome::Unavailable,
            reason_code: Some("STORAGE_NOT_CAPTURED".to_owned()),
        },
    ])
}

pub(crate) fn ctx() -> CaptureContext {
    CaptureContext {
        job_id: Uuid::new_v4(),
        instance_id: INSTANCE.to_owned(),
        correlation_id: "corr-test".to_owned(),
        storage_key: StorageKey::from_allocation_id(Uuid::new_v4()),
    }
}

// ---------------------------------------------------------------- job store double

/// One `transition` call as the double saw it.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TransitionCall {
    pub id: Uuid,
    pub to: BackupJobStatus,
    pub reason: Option<ReasonCode>,
    /// `Ok(true)` applied, `Ok(false)` lost, `Err` an injected or guard failure.
    pub result: Result<bool, StoreError>,
}

/// In-memory `JobStore` with the same compare-and-set and guard semantics as
/// `DbJobStore`. Injected failures apply to transitions other than `-> running`.
pub(crate) struct MemJobStore {
    clock: Arc<dyn Clock>,
    log: Log,
    state: Mutex<MemJobs>,
    /// When set, every `claim_next` waits here before it looks at the queue, so that
    /// concurrent claimers are all in flight together (explicit contention, no sleeps).
    claim_gate: Mutex<Option<Arc<tokio::sync::Barrier>>>,
}

#[derive(Default)]
struct MemJobs {
    rows: HashMap<Uuid, JobRow>,
    calls: Vec<TransitionCall>,
    terminal_failures: VecDeque<StoreError>,
    terminal_attempt_times: Vec<tokio::time::Instant>,
}

impl std::fmt::Debug for MemJobStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemJobStore").finish_non_exhaustive()
    }
}

impl MemJobStore {
    pub fn new(clock: Arc<dyn Clock>, log: Log) -> Arc<Self> {
        Arc::new(Self {
            clock,
            log,
            state: Mutex::default(),
            claim_gate: Mutex::default(),
        })
    }

    /// Make `parties` concurrent `claim_next` calls meet before any of them claims.
    pub fn gate_claims(&self, parties: usize) {
        *self.claim_gate.lock().unwrap() = Some(Arc::new(tokio::sync::Barrier::new(parties)));
    }

    pub fn seed_queued(&self, instance: &str) -> Uuid {
        let id = Uuid::new_v4();
        let now = self.clock.now();
        let row = JobRow {
            id,
            actor_sub: "admin-sub-1".to_owned(),
            instance_id: instance.to_owned(),
            operation: "backup_db_slice".to_owned(),
            synthetic: true,
            status: BackupJobStatus::Queued,
            state_version: 0,
            reason_code: None,
            components: Components::default(),
            epoch: 1,
            correlation_id: format!("corr-{id}"),
            created_at: now,
            updated_at: now,
            started_at: None,
            finished_at: None,
            deadline_at: None,
            retain_until: now + ChronoDuration::days(30),
        };
        self.state.lock().unwrap().rows.insert(id, row);
        id
    }

    pub fn row(&self, id: Uuid) -> JobRow {
        self.state.lock().unwrap().rows[&id].clone()
    }

    pub fn calls(&self) -> Vec<TransitionCall> {
        self.state.lock().unwrap().calls.clone()
    }

    /// The next terminal transitions fail with these errors, in order.
    pub fn inject_terminal_failures(&self, errs: Vec<StoreError>) {
        self.state.lock().unwrap().terminal_failures = errs.into();
    }

    /// Paused-clock instants of every attempted terminal transition.
    pub fn terminal_attempt_times(&self) -> Vec<tokio::time::Instant> {
        self.state.lock().unwrap().terminal_attempt_times.clone()
    }

    /// What the sweeper does to a job past its deadline (FD 6.4, 6.7 step 1): it reads only
    /// the Clock, and refuses (test failure) to fail a job that is not yet due.
    pub async fn sweep_deadline(&self, id: Uuid) -> TransitionResult {
        let snapshot = self.row(id);
        self.sweep_from(&snapshot).await
    }

    /// The same sweep, acting on a snapshot taken earlier (a sweeper that read the job
    /// before another actor finished it), so a lost compare-and-set can be exercised.
    pub async fn sweep_from(&self, snapshot: &JobRow) -> TransitionResult {
        let due = snapshot
            .deadline_at
            .expect("the sweeper only sees claimed jobs");
        assert!(
            self.clock.now() >= due,
            "the sweeper must not fail a job before its deadline"
        );
        self.transition(
            snapshot.id,
            (snapshot.status, snapshot.state_version),
            BackupJobStatus::Failed,
            Some(ReasonCode::DeadlineExceeded),
            None,
        )
        .await
        .expect("sweeper transition")
    }
}

#[async_trait]
impl JobStore for MemJobStore {
    async fn accept(
        &self,
        _tx: &DatabaseTransaction,
        _new: NewJob,
        _idem: IdemRecord,
    ) -> Result<AcceptResult, StoreError> {
        Err(StoreError::Persistence)
    }

    async fn get(&self, id: Uuid) -> Result<Option<JobRow>, StoreError> {
        Ok(self.state.lock().unwrap().rows.get(&id).cloned())
    }

    async fn transition(
        &self,
        id: Uuid,
        expect: (BackupJobStatus, i64),
        to: BackupJobStatus,
        reason: Option<ReasonCode>,
        comps: Option<Components>,
    ) -> Result<TransitionResult, StoreError> {
        push(&self.log, format!("transition:{}", wire(to)));
        let mut s = self.state.lock().unwrap();
        let record = |s: &mut MemJobs, result: Result<bool, StoreError>| {
            s.calls.push(TransitionCall {
                id,
                to,
                reason,
                result,
            });
        };
        let terminal = to != BackupJobStatus::Running;
        if terminal {
            s.terminal_attempt_times.push(tokio::time::Instant::now());
            if let Some(e) = s.terminal_failures.pop_front() {
                record(&mut s, Err(clone_err(&e)));
                return Err(e);
            }
        }
        let (from, version) = expect;
        if !is_allowed(from, to) {
            record(&mut s, Err(StoreError::IllegalTransition { from, to }));
            return Err(StoreError::IllegalTransition { from, to });
        }
        let now = self.clock.now();
        let Some(row) = s.rows.get_mut(&id) else {
            record(&mut s, Err(StoreError::NotFound));
            return Err(StoreError::NotFound);
        };
        if row.status != from || row.state_version != version {
            record(&mut s, Ok(false));
            return Ok(TransitionResult::Lost);
        }
        row.status = to;
        row.state_version += 1;
        row.updated_at = now;
        if reason.is_some() {
            row.reason_code = reason.map(|r| r.as_str().to_owned());
        }
        if let Some(c) = comps {
            row.components = c;
        }
        if matches!(to, BackupJobStatus::Failed | BackupJobStatus::Incomplete) {
            row.finished_at = Some(now);
        }
        record(&mut s, Ok(true));
        Ok(TransitionResult::Applied)
    }

    async fn claim_next(
        &self,
        epoch: i64,
        deadline: Duration,
    ) -> Result<Option<JobRow>, StoreError> {
        let gate = self.claim_gate.lock().unwrap().clone();
        if let Some(gate) = gate {
            gate.wait().await;
        }
        let now = self.clock.now();
        let mut s = self.state.lock().unwrap();
        let next = s
            .rows
            .values()
            .filter(|r| r.status == BackupJobStatus::Queued)
            .min_by_key(|r| (r.created_at, r.id))
            .map(|r| r.id);
        let Some(id) = next else {
            return Ok(None);
        };
        push(&self.log, "claim");
        let row = s.rows.get_mut(&id).expect("row");
        row.status = BackupJobStatus::Running;
        row.state_version += 1;
        row.epoch = epoch;
        row.started_at = Some(now);
        row.deadline_at = Some(now + ChronoDuration::from_std(deadline).unwrap());
        row.updated_at = now;
        Ok(Some(row.clone()))
    }
}

impl Clone for TransitionCall {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            to: self.to,
            reason: self.reason,
            result: match &self.result {
                Ok(b) => Ok(*b),
                Err(e) => Err(clone_err(e)),
            },
        }
    }
}

fn clone_err(e: &StoreError) -> StoreError {
    match e {
        StoreError::ConcurrencyConflict => StoreError::ConcurrencyConflict,
        StoreError::NotFound => StoreError::NotFound,
        StoreError::IllegalTransition { from, to } => StoreError::IllegalTransition {
            from: *from,
            to: *to,
        },
        StoreError::AuditUnavailable => StoreError::AuditUnavailable,
        StoreError::StoreUnavailable => StoreError::StoreUnavailable,
        StoreError::Persistence => StoreError::Persistence,
    }
}

fn wire(s: BackupJobStatus) -> &'static str {
    match s {
        BackupJobStatus::Queued => "queued",
        BackupJobStatus::Running => "running",
        BackupJobStatus::Succeeded => "succeeded",
        BackupJobStatus::Failed => "failed",
        BackupJobStatus::RecoveryRequired => "recovery-required",
        BackupJobStatus::Incomplete => "incomplete",
    }
}

// ---------------------------------------------------------------- allocation double

/// In-memory `AllocationStore` with budget checks and version compare-and-set.
pub(crate) struct MemAllocStore {
    log: Log,
    alloc_max: u64,
    total_max: u64,
    rows: Mutex<HashMap<Uuid, Allocation>>,
    register_error: Mutex<Option<AllocationError>>,
    mark_written_error: Mutex<Option<AllocationError>>,
}

impl std::fmt::Debug for MemAllocStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemAllocStore").finish_non_exhaustive()
    }
}

impl MemAllocStore {
    pub fn new(log: Log) -> Arc<Self> {
        Self::with_alloc_max(log, 256 * 1024 * 1024)
    }

    pub fn with_alloc_max(log: Log, alloc_max: u64) -> Arc<Self> {
        Self::with_limits(log, alloc_max, 1024 * 1024 * 1024)
    }

    /// CFG-12 / CFG-13 style limits (defaults 256 MiB per allocation, 1 GiB total).
    pub fn with_limits(log: Log, alloc_max: u64, total_max: u64) -> Arc<Self> {
        Arc::new(Self {
            log,
            alloc_max,
            total_max,
            rows: Mutex::default(),
            register_error: Mutex::default(),
            mark_written_error: Mutex::default(),
        })
    }

    pub fn fail_register(&self, e: AllocationError) {
        *self.register_error.lock().unwrap() = Some(e);
    }

    pub fn fail_mark_written(&self, e: AllocationError) {
        *self.mark_written_error.lock().unwrap() = Some(e);
    }

    pub fn all(&self) -> Vec<Allocation> {
        self.rows.lock().unwrap().values().cloned().collect()
    }

    /// Rows of the job that are still live (`registered` or `written`).
    pub fn live_for(&self, job: Uuid) -> Vec<Allocation> {
        self.all()
            .into_iter()
            .filter(|a| {
                a.job_id == Some(job)
                    && matches!(
                        a.state,
                        AllocationState::Registered | AllocationState::Written
                    )
            })
            .collect()
    }

    /// A live allocation of another owner that already holds `bytes` of the total budget.
    pub fn seed_live(&self, bytes: u64) {
        let id = Uuid::new_v4();
        self.rows.lock().unwrap().insert(
            id,
            Allocation {
                id,
                job_id: None,
                storage_key: StorageKey::from_allocation_id(id),
                state: AllocationState::Written,
                state_version: 1,
                epoch: 1,
                bytes_reserved: bytes,
                expires_at: t0() + ChronoDuration::hours(24),
                orphan_grace_until: None,
                cleanup_attempts: 0,
                next_attempt_at: None,
                last_reason_code: None,
                created_at: t0(),
                updated_at: t0(),
                deleted_at: None,
            },
        );
    }
}

#[async_trait]
impl AllocationStore for MemAllocStore {
    async fn register(
        &self,
        job: Option<Uuid>,
        bytes: u64,
        epoch: i64,
        expires: DateTime<Utc>,
    ) -> Result<Allocation, AllocationError> {
        push(&self.log, "register");
        if let Some(e) = self.register_error.lock().unwrap().take() {
            return Err(e);
        }
        if bytes > self.alloc_max {
            return Err(AllocationError::BudgetExceeded(BudgetKind::PerAllocation));
        }
        let live: u64 = self
            .rows
            .lock()
            .unwrap()
            .values()
            .map(|a| a.bytes_reserved)
            .sum();
        if live.saturating_add(bytes) > self.total_max {
            return Err(AllocationError::BudgetExceeded(BudgetKind::Total));
        }
        let id = Uuid::new_v4();
        let row = Allocation {
            id,
            job_id: job,
            storage_key: StorageKey::from_allocation_id(id),
            state: AllocationState::Registered,
            state_version: 0,
            epoch,
            bytes_reserved: bytes,
            expires_at: expires,
            orphan_grace_until: None,
            cleanup_attempts: 0,
            next_attempt_at: None,
            last_reason_code: None,
            created_at: t0(),
            updated_at: t0(),
            deleted_at: None,
        };
        self.rows.lock().unwrap().insert(id, row.clone());
        Ok(row)
    }

    async fn mark_written(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        push(&self.log, "mark_written");
        if let Some(e) = self.mark_written_error.lock().unwrap().take() {
            return Err(e);
        }
        let mut rows = self.rows.lock().unwrap();
        let row = rows.get_mut(&id).ok_or(AllocationError::Lost)?;
        if row.state != AllocationState::Registered || row.state_version != ver {
            return Err(AllocationError::Lost);
        }
        row.state = AllocationState::Written;
        row.state_version += 1;
        Ok(())
    }

    async fn release(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        push(&self.log, "release");
        let mut rows = self.rows.lock().unwrap();
        let row = rows.get_mut(&id).ok_or(AllocationError::Lost)?;
        if !matches!(
            row.state,
            AllocationState::Registered | AllocationState::Written
        ) || row.state_version != ver
        {
            return Err(AllocationError::Lost);
        }
        row.state = AllocationState::ReleasePending;
        row.state_version += 1;
        Ok(())
    }

    async fn claim_for_delete(
        &self,
        _id: Uuid,
        _ver: i64,
    ) -> Result<Option<Allocation>, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn mark_deleted(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn record_cleanup_failure(
        &self,
        _id: Uuid,
        _ver: i64,
        _reason: ReasonCode,
        _now: DateTime<Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn insert_orphan(
        &self,
        _id: Uuid,
        _key: &StorageKey,
        _bytes: u64,
        _now: DateTime<Utc>,
    ) -> Result<(), AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn due_for_cleanup(
        &self,
        _now: DateTime<Utc>,
    ) -> Result<Vec<Allocation>, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn totals(&self) -> Result<AllocationTotals, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn get(&self, id: Uuid) -> Result<Option<Allocation>, AllocationError> {
        Ok(self.rows.lock().unwrap().get(&id).cloned())
    }
}

// ---------------------------------------------------------------- storage double

pub(crate) struct MemStorage {
    log: Option<Log>,
    writes: Mutex<Vec<(StorageKey, Vec<u8>)>>,
    error: Option<StorageError>,
}

impl std::fmt::Debug for MemStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemStorage").finish_non_exhaustive()
    }
}

impl MemStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            log: None,
            writes: Mutex::default(),
            error: None,
        })
    }
    pub fn logged(log: Log) -> Arc<Self> {
        Arc::new(Self {
            log: Some(log),
            writes: Mutex::default(),
            error: None,
        })
    }
    pub fn failing(e: StorageError) -> Arc<Self> {
        Arc::new(Self {
            log: None,
            writes: Mutex::default(),
            error: Some(e),
        })
    }
    pub fn writes(&self) -> Vec<(StorageKey, Vec<u8>)> {
        self.writes.lock().unwrap().clone()
    }
}

#[async_trait]
impl FixtureStorage for MemStorage {
    async fn write(&self, key: &StorageKey, bytes: &[u8]) -> Result<(), StorageError> {
        if let Some(log) = &self.log {
            push(log, "write");
        }
        if let Some(e) = self.error {
            return Err(e);
        }
        self.writes
            .lock()
            .unwrap()
            .push((key.clone(), bytes.to_vec()));
        Ok(())
    }
    async fn delete(&self, _key: &StorageKey) -> Result<DeleteOutcome, StorageError> {
        Ok(DeleteOutcome::AlreadyAbsent)
    }
    async fn inventory(&self, _max_entries: usize) -> Result<InventoryPage, StorageError> {
        Ok(InventoryPage {
            entries: Vec::new(),
            truncated: false,
        })
    }
}

// ---------------------------------------------------------------- capture adapter double

#[derive(Debug, Clone)]
pub(crate) enum Script {
    Ok,
    Fail,
    /// Signals `entered`, waits for `release`, then follows the inner script.
    Block(Box<Script>),
}

/// Scripted `CaptureAdapter`: counts calls, can fail, can block until released.
#[derive(Debug)]
pub(crate) struct ScriptedAdapter {
    script: Script,
    log: Option<Log>,
    calls: AtomicUsize,
    contexts: Mutex<Vec<CaptureContext>>,
    pub entered: Notify,
    pub release: Notify,
}

impl ScriptedAdapter {
    pub fn new(script: Script) -> Arc<Self> {
        Arc::new(Self {
            script,
            log: None,
            calls: AtomicUsize::new(0),
            contexts: Mutex::default(),
            entered: Notify::new(),
            release: Notify::new(),
        })
    }
    pub fn logged(script: Script, log: Log) -> Arc<Self> {
        Arc::new(Self {
            script,
            log: Some(log),
            calls: AtomicUsize::new(0),
            contexts: Mutex::default(),
            entered: Notify::new(),
            release: Notify::new(),
        })
    }
    pub fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
    pub fn contexts(&self) -> Vec<CaptureContext> {
        self.contexts.lock().unwrap().clone()
    }
}

#[async_trait]
impl CaptureAdapter for ScriptedAdapter {
    async fn capture(&self, ctx: &CaptureContext) -> Result<CaptureOutcome, CaptureError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.contexts.lock().unwrap().push(ctx.clone());
        if let Some(log) = &self.log {
            push(log, "capture");
        }
        let mut script = self.script.clone();
        if let Script::Block(next) = script {
            self.entered.notify_one();
            self.release.notified().await;
            script = *next;
        }
        match script {
            Script::Ok => Ok(CaptureOutcome {
                components: contract_components(),
            }),
            _ => Err(CaptureError::Failed),
        }
    }
}

// ---------------------------------------------------------------- policy double

#[derive(Debug, Clone, Copy)]
pub(crate) enum PolicyMode {
    Allow,
    Deny,
    Error,
}

#[derive(Debug)]
pub(crate) struct PolicyDouble {
    mode: PolicyMode,
    log: Option<Log>,
    calls: Mutex<Vec<(String, Uuid)>>,
}

impl PolicyDouble {
    pub fn new(mode: PolicyMode) -> Arc<Self> {
        Arc::new(Self {
            mode,
            log: None,
            calls: Mutex::default(),
        })
    }
    pub fn logged(mode: PolicyMode, log: Log) -> Arc<Self> {
        Arc::new(Self {
            mode,
            log: Some(log),
            calls: Mutex::default(),
        })
    }
    pub fn calls(&self) -> Vec<(String, Uuid)> {
        self.calls.lock().unwrap().clone()
    }
}

#[async_trait]
impl ClaimPolicy for PolicyDouble {
    async fn authorize_resource(
        &self,
        instance_id: &str,
        job_id: Uuid,
    ) -> Result<PolicyDecision, PolicyError> {
        if let Some(log) = &self.log {
            push(log, "policy");
        }
        self.calls
            .lock()
            .unwrap()
            .push((instance_id.to_owned(), job_id));
        match self.mode {
            PolicyMode::Allow => Ok(PolicyDecision::Allowed),
            PolicyMode::Deny => Ok(PolicyDecision::Denied),
            PolicyMode::Error => Err(PolicyError),
        }
    }
}

/// Wait until the adapter has been entered, or fail at once if the spawned worker task
/// finished without getting there (so a worker that does nothing is an assertion on its
/// result, not a timeout). The 30 s bound is a last-resort guard against a hang, never the
/// success path and never the expected red reason.
pub(crate) async fn wait_entered<T: std::fmt::Debug>(
    adapter: &ScriptedAdapter,
    task: &mut tokio::task::JoinHandle<T>,
) {
    tokio::select! {
        _ = adapter.entered.notified() => {}
        finished = &mut *task => {
            panic!("worker finished before reaching the capture adapter: {finished:?}");
        }
        _ = tokio::time::sleep(Duration::from_secs(30)) => {
            panic!("the worker neither reached the capture adapter nor finished");
        }
    }
}

/// Join a worker task under paused time with a 1 s bound that a pending tokio timer can
/// trip: a worker that never arms a tokio timeout for the deadline stalls forever on its
/// blocked adapter, and the paused clock then auto-advances this bound, so the missing
/// timer is an assertion failure and not a hang.
pub(crate) async fn join_within<T>(task: tokio::task::JoinHandle<T>) -> T {
    match tokio::time::timeout(Duration::from_secs(1), task).await {
        Ok(joined) => joined.expect("join"),
        Err(_) => panic!("the worker did not finish: the deadline is not enforced on tokio time"),
    }
}
