//! WU-05 red tests, PostgreSQL-backed: the worker over `DbJobStore`, `DbAllocationStore`,
//! `DbAuditSink` and `FsFixtureStorage` (TS-47, TS-48, TS-67, TS-05a and the job half of
//! TS-44b). The in-memory and paused-time tests (TS-68 and the interleavings that need
//! no database) live in `lifecycle::worker::tests`. Ground truth is read with raw SQL.

use super::alloc_support::{raw_alloc, seed_alloc, SeedAlloc, INSTANCE};
use super::fs_support::{names_in, TempDir};
use super::support::{
    audit_rows_for_job, fx, pg, raw_job, require_job, seed, store_with, t0, AuditRow,
    FailingAuditSink, FixedClock, Fx, OutageAuditSink, Seed,
};
use crate::domain::status::BackupJobStatus;
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::lifecycle::fixture_capture::{FixtureCaptureAdapter, FIXTURE_BYTES};
use crate::lifecycle::test_support::{
    contract_components, wait_entered, PolicyDouble, PolicyMode, Script, ScriptedAdapter,
};
use crate::lifecycle::worker::{RunOutcome, Worker, WorkerConfig};
use crate::ports::allocation_store::{AllocationLimits, AllocationState, AllocationStore};
use crate::ports::audit::{AuditError, AuditEvent, AuditEventKind, AuditSink};
use crate::ports::capture::CaptureAdapter;
use crate::ports::fixture_storage::FixtureStorage;
use crate::ports::job_store::{JobStore, ReasonCode, StoreError, TransitionResult};
use async_trait::async_trait;
use chrono::Duration as ChronoDuration;
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// Fails only one audit event kind (everything else is written by the real sink).
#[derive(Debug)]
struct SelectiveFailSink {
    inner: DbAuditSink,
    fail: AuditEventKind,
}

#[async_trait]
impl AuditSink for SelectiveFailSink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        if ev.event == self.fail {
            return Err(AuditError::Rejected);
        }
        self.inner.record(tx, ev).await
    }
}

struct Wiring {
    job_audit: Arc<dyn AuditSink>,
    alloc_audit: Arc<dyn AuditSink>,
    limits: AllocationLimits,
    policy: PolicyMode,
    config: WorkerConfig,
}

impl Wiring {
    fn new(fx: &Fx) -> Self {
        Self {
            job_audit: fx.sink.clone(),
            alloc_audit: fx.sink.clone(),
            limits: AllocationLimits::default(),
            policy: PolicyMode::Allow,
            config: WorkerConfig::new(1),
        }
    }
}

struct Rig {
    worker: Arc<Worker>,
    policy: Arc<PolicyDouble>,
}

/// Real stores over `fx`, a real `FsFixtureStorage` under `tmp`, and the given adapter
/// factory (it receives the storage, so the real fixture adapter can use it).
fn rig(
    fx: &Fx,
    tmp: &TempDir,
    w: Wiring,
    adapter: impl FnOnce(Arc<dyn FixtureStorage>) -> Arc<dyn CaptureAdapter>,
) -> Rig {
    let allocs: Arc<dyn AllocationStore> = Arc::new(DbAllocationStore::new(
        fx.conn.clone(),
        w.alloc_audit,
        Arc::new(FixedClock(t0())),
        INSTANCE,
        w.limits,
    ));
    let storage: Arc<dyn FixtureStorage> =
        Arc::new(FsFixtureStorage::open(&tmp.root(), allocs.clone()).expect("open storage"));
    let policy = PolicyDouble::new(w.policy);
    let worker = Arc::new(Worker::new(
        store_with(&fx.conn, w.job_audit),
        allocs,
        adapter(storage),
        policy.clone(),
        Arc::new(FixedClock(t0())),
        w.config,
    ));
    Rig { worker, policy }
}

fn real_adapter(storage: Arc<dyn FixtureStorage>) -> Arc<dyn CaptureAdapter> {
    Arc::new(FixtureCaptureAdapter::new(storage))
}

async fn queued(fx: &Fx) -> Uuid {
    seed(&fx.conn, Seed::new(INSTANCE, BackupJobStatus::Queued)).await
}

async fn alloc_ids_for_job(c: &DatabaseConnection, job: Uuid) -> Vec<Uuid> {
    c.query_all(pg(
        "SELECT id FROM backup_allocations WHERE job_id = $1 ORDER BY created_at, id",
        vec![job.into()],
    ))
    .await
    .expect("allocation query")
    .iter()
    .map(|r| r.try_get::<Uuid>("", "id").unwrap())
    .collect()
}

async fn alloc_rows(c: &DatabaseConnection) -> i64 {
    super::alloc_support::alloc_rows(c).await
}

/// Audit rows of the job's own events (`job.*` and `dispatch.failed`). Allocation events
/// (`allocation.registered`) also carry the job id (WU-04) and are WU-06's half of TS-44b;
/// they are asserted separately where a test needs them.
async fn job_event_rows(c: &DatabaseConnection, job: Uuid) -> Vec<AuditRow> {
    audit_rows_for_job(c, job)
        .await
        .into_iter()
        .filter(|r| r.event.starts_with("job.") || r.event == "dispatch.failed")
        .collect()
}

async fn events(c: &DatabaseConnection, job: Uuid) -> Vec<String> {
    let mut e: Vec<String> = job_event_rows(c, job)
        .await
        .into_iter()
        .map(|r| r.event)
        .collect();
    e.sort();
    e
}

fn alloc_dir(tmp: &TempDir) -> std::path::PathBuf {
    tmp.root().join("alloc")
}

// -------------------------------------------------------------------- TS-47

/// TS-47 (AC-002 S2/S3; T-05, T-06; FN-16..FN-18): a queued job runs through the real
/// fixture adapter over real storage and ends `incomplete` with the exact components.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts47_queued_job_runs_through_fixture_adapter_and_ends_incomplete() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-happy");
    let rig = rig(&fx, &tmp, Wiring::new(&fx), real_adapter);
    let id = queued(&fx).await;

    let outcome = rig.worker.run_next().await;

    assert_eq!(outcome, Ok(RunOutcome::Incomplete { job_id: id }));
    let job = require_job(&fx.conn, id).await;
    assert_eq!(job.status, "incomplete");
    assert_ne!(job.status, "succeeded");
    assert_eq!(
        job.reason_code.as_deref(),
        Some("REQUIRED_COMPONENTS_UNAVAILABLE")
    );
    assert_eq!(job.state_version, 2, "claim, then the terminal transition");
    assert_eq!(job.deadline_at, Some(t0() + ChronoDuration::seconds(600)));
    assert_eq!(job.finished_at, Some(t0()));
    let comps: serde_json::Value = serde_json::from_str(&job.components).unwrap();
    assert_eq!(comps, serde_json::to_value(contract_components()).unwrap());

    let allocs = alloc_ids_for_job(&fx.conn, id).await;
    assert_eq!(allocs.len(), 1, "one allocation, registered for this job");
    let row = raw_alloc(&fx.conn, allocs[0]).await.expect("row");
    assert_eq!(
        row.state, "written",
        "an incomplete job keeps its written allocation"
    );
    assert_eq!(row.bytes_reserved as u64, FIXTURE_BYTES);
    assert_eq!(row.expires_at, t0() + ChronoDuration::hours(24));
    assert_eq!(names_in(&alloc_dir(&tmp)), [format!("{}.bin", allocs[0])]);
    let written = std::fs::read(alloc_dir(&tmp).join(format!("{}.bin", allocs[0]))).unwrap();
    assert_eq!(written, FixtureCaptureAdapter::fixture_content());
    assert_eq!(
        super::alloc_support::audit_events_for_allocation(&fx.conn, allocs[0]).await,
        [("allocation.registered".to_owned(), "success".to_owned())]
    );
    assert_eq!(
        events(&fx.conn, id).await,
        ["job.incomplete", "job.running"]
    );
}

// -------------------------------------------------------------------- TS-44b (job events)

/// TS-44b (AC-005 S2; T-09; FN-34): `job.running`, `job.incomplete` and `job.failed` rows
/// carry instance, job, time, operation and correlation id; the outcome is populated; no
/// excluded field reaches any column. The actor column and the `job.incomplete` outcome
/// value are pinned by the two tests below (conductor decisions C-1 and C-2, WU-05 red
/// evidence).
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts44b_job_events_are_complete_and_free_of_excluded_fields() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-audit");
    let ok = rig(&fx, &tmp, Wiring::new(&fx), real_adapter);
    let failing = rig(&fx, &tmp, Wiring::new(&fx), |_| {
        ScriptedAdapter::new(Script::Fail)
    });

    let incomplete_job = queued(&fx).await;
    assert_eq!(
        ok.worker.run_next().await,
        Ok(RunOutcome::Incomplete {
            job_id: incomplete_job
        })
    );
    let failed_job = queued(&fx).await;
    assert_eq!(
        failing.worker.run_next().await,
        Ok(RunOutcome::Failed {
            job_id: failed_job,
            reason: ReasonCode::AdapterError
        })
    );

    let columns = [
        "actor_sub",
        "allocation_id",
        "correlation_id",
        "digest",
        "event",
        "id",
        "instance_id",
        "job_id",
        "occurred_at",
        "operation",
        "outcome",
        "reason_code",
    ];
    let root_text = tmp.path().to_string_lossy().into_owned();
    for (job, terminal, terminal_reason) in [
        (
            incomplete_job,
            "job.incomplete",
            "REQUIRED_COMPONENTS_UNAVAILABLE",
        ),
        (failed_job, "job.failed", "ADAPTER_ERROR"),
    ] {
        let correlation = fx.store.get(job).await.unwrap().unwrap().correlation_id;
        let rows = job_event_rows(&fx.conn, job).await;
        let mut kinds: Vec<_> = rows.iter().map(|r| r.event.as_str()).collect();
        kinds.sort();
        assert_eq!(kinds, [terminal, "job.running"], "job {job}");
        for r in &rows {
            assert_eq!(r.instance_id, INSTANCE);
            assert_eq!(r.job_id, Some(job));
            assert_eq!(r.occurred_at, t0());
            assert_eq!(r.operation, "backup_db_slice");
            assert_eq!(r.correlation_id, correlation);
            assert_eq!(r.allocation_id, None);
            assert_eq!(r.digest, None);
            assert!(!r.outcome.is_empty(), "outcome is populated");
            if r.event == "job.running" {
                assert_eq!(r.outcome, "success");
                assert_eq!(r.reason_code, None);
            } else {
                assert_eq!(r.reason_code.as_deref(), Some(terminal_reason));
                // Both terminal events record `failure` (conductor decision C-2).
                assert_eq!(r.outcome, "failure");
            }
        }
        for json in super::support::audit_json_for_job(&fx.conn, job).await {
            let v: serde_json::Value = serde_json::from_str(&json).unwrap();
            let mut keys: Vec<_> = v.as_object().unwrap().keys().cloned().collect();
            keys.sort();
            assert_eq!(keys, columns, "exactly the audit columns, no payload");
            for banned in [root_text.as_str(), "Bearer", "postgres://", "alloc/", "@"] {
                assert!(!json.contains(banned), "audit row leaks {banned}: {json}");
            }
        }
    }
}

/// TS-44b, conductor decision C-1: `job.running`, `job.incomplete` and `job.failed` are
/// written by the worker, a system actor, so `actor_sub` is NULL (IFC section 7: "NULL
/// (system events null)"). The requesting administrator is attributed on `job.accepted`
/// (TS-44a) and on the job row, both reachable through `job_id`. The TS-44b row's "actor
/// populated" wording is read as "the actor column follows IFC section 7".
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts44b_job_events_use_the_system_actor() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-actor");
    let rig = rig(&fx, &tmp, Wiring::new(&fx), real_adapter);
    let id = queued(&fx).await;
    assert_eq!(
        rig.worker.run_next().await,
        Ok(RunOutcome::Incomplete { job_id: id })
    );

    let rows = job_event_rows(&fx.conn, id).await;

    assert_eq!(rows.len(), 2, "job.running and job.incomplete");
    for r in rows {
        assert_eq!(
            r.actor_sub, None,
            "{}: system actor (IFC section 7)",
            r.event
        );
    }
}

/// TS-44b, conductor decision C-2: the audit `outcome` of `job.incomplete` is `failure`.
/// The job produced no usable backup (required components unavailable), so the audit trail
/// never records success for it. This keeps the WU-03 mapping and the ADR-002 rule that a U1
/// job never reads as a success.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts44b_incomplete_event_outcome_is_failure() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-outcome");
    let rig = rig(&fx, &tmp, Wiring::new(&fx), real_adapter);
    let id = queued(&fx).await;
    assert_eq!(
        rig.worker.run_next().await,
        Ok(RunOutcome::Incomplete { job_id: id })
    );

    let incomplete = job_event_rows(&fx.conn, id)
        .await
        .into_iter()
        .find(|r| r.event == "job.incomplete")
        .expect("job.incomplete row");

    assert_eq!(incomplete.outcome, "failure");
}

/// TS-44b + FD 6.6 (IFC 6.3): audit is audit-gated and fails closed. A claim whose
/// `job.running` audit is refused leaves the job queued; nothing runs.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts44b_refused_running_audit_leaves_job_queued_and_nothing_runs() {
    for (sink, expected) in [
        (
            Arc::new(FailingAuditSink) as Arc<dyn AuditSink>,
            StoreError::AuditUnavailable,
        ),
        (Arc::new(OutageAuditSink), StoreError::StoreUnavailable),
    ] {
        let fx = fx().await;
        let tmp = TempDir::new("wu05-claim-audit");
        let mut w = Wiring::new(&fx);
        w.job_audit = sink;
        let adapter = ScriptedAdapter::new(Script::Ok);
        let probe = adapter.clone();
        let rig = rig(&fx, &tmp, w, move |_| probe);
        let id = queued(&fx).await;

        let outcome = rig.worker.run_next().await;

        assert_eq!(outcome, Err(expected));
        let job = require_job(&fx.conn, id).await;
        assert_eq!(job.status, "queued");
        assert_eq!(job.state_version, 0);
        assert_eq!(adapter.calls(), 0);
        assert_eq!(alloc_rows(&fx.conn).await, 0);
        assert!(events(&fx.conn, id).await.is_empty());
    }
}

/// TS-44b + FD 3.2 (FN-16..FN-20): when the terminal audit is refused on every retry the
/// transition is refused (no unaudited effect): the job stays `running`, no
/// `job.incomplete` row exists and the written allocation is kept.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts44b_refused_terminal_audit_leaves_job_running_without_unaudited_effect() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-terminal-audit");
    let mut w = Wiring::new(&fx);
    w.job_audit = Arc::new(SelectiveFailSink {
        inner: DbAuditSink::new(fx.conn.clone()),
        fail: AuditEventKind::JobIncomplete,
    });
    w.config.terminal_retry_interval = Duration::ZERO;
    let rig = rig(&fx, &tmp, w, real_adapter);
    let id = queued(&fx).await;

    let outcome = rig.worker.run_next().await;

    assert_eq!(outcome, Ok(RunOutcome::Stalled { job_id: id }));
    let job = require_job(&fx.conn, id).await;
    assert_eq!(job.status, "running");
    assert_eq!(job.state_version, 1);
    assert_eq!(job.components, "[]");
    assert_eq!(events(&fx.conn, id).await, ["job.running"]);
    let allocs = alloc_ids_for_job(&fx.conn, id).await;
    assert_eq!(allocs.len(), 1);
    assert_eq!(
        raw_alloc(&fx.conn, allocs[0]).await.unwrap().state,
        "written"
    );
}

// -------------------------------------------------------------------- TS-48

/// TS-48 (T-13): a per-allocation budget refusal fails the job `ALLOCATION_LIMIT`;
/// nothing is registered, written or captured; `job.failed` carries the reason. The limit
/// is lowered below the fixture size only to reach the mapping: CFG-12 itself is fixed at
/// 256 MiB (IFC section 8), so the reachable production case is the total budget below.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts48_allocation_limit_fails_job_without_a_row_or_a_file() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-limit");
    let mut w = Wiring::new(&fx);
    w.limits.alloc_max_bytes = FIXTURE_BYTES - 1;
    let adapter = ScriptedAdapter::new(Script::Ok);
    let probe = adapter.clone();
    let rig = rig(&fx, &tmp, w, move |_| probe);
    let id = queued(&fx).await;

    let outcome = rig.worker.run_next().await;

    assert_eq!(
        outcome,
        Ok(RunOutcome::Failed {
            job_id: id,
            reason: ReasonCode::AllocationLimit
        })
    );
    let job = require_job(&fx.conn, id).await;
    assert_eq!(job.status, "failed");
    assert_eq!(job.reason_code.as_deref(), Some("ALLOCATION_LIMIT"));
    assert_eq!(job.components, "[]");
    assert_eq!(adapter.calls(), 0);
    assert_eq!(alloc_rows(&fx.conn).await, 0);
    assert!(names_in(&alloc_dir(&tmp)).is_empty());
    assert_eq!(events(&fx.conn, id).await, ["job.failed", "job.running"]);
}

/// TS-48 (T-13; IFC 6.1 step 1, FN-21): under the default limits the reachable refusal is
/// the 1 GiB total budget (CFG-13). Four live 256 MiB allocations fill it; the job fails
/// `ALLOCATION_LIMIT`, no row is added, no file is written and nothing is captured.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts48_total_budget_exhausted_fails_job_allocation_limit() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-total");
    let adapter = ScriptedAdapter::new(Script::Ok);
    let probe = adapter.clone();
    let rig = rig(&fx, &tmp, Wiring::new(&fx), move |_| probe);
    for _ in 0..4 {
        seed_alloc(
            &fx.conn,
            SeedAlloc::new(AllocationState::Written, 256 * 1024 * 1024),
        )
        .await;
    }
    let id = queued(&fx).await;

    let outcome = rig.worker.run_next().await;

    assert_eq!(
        outcome,
        Ok(RunOutcome::Failed {
            job_id: id,
            reason: ReasonCode::AllocationLimit
        })
    );
    let job = require_job(&fx.conn, id).await;
    assert_eq!(job.status, "failed");
    assert_eq!(job.reason_code.as_deref(), Some("ALLOCATION_LIMIT"));
    assert_eq!(adapter.calls(), 0);
    assert_eq!(alloc_rows(&fx.conn).await, 4, "only the seeded rows");
    assert!(alloc_ids_for_job(&fx.conn, id).await.is_empty());
    assert!(names_in(&alloc_dir(&tmp)).is_empty());
    assert_eq!(events(&fx.conn, id).await, ["job.failed", "job.running"]);
}

/// TS-48 (FN-21): an audit-refused registration is `PERSISTENCE_ERROR`, never
/// `ALLOCATION_LIMIT`; nothing is written and nothing leaks into the job or the audit.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts48_registration_refused_fails_job_persistence_error() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-persist");
    let mut w = Wiring::new(&fx);
    w.alloc_audit = Arc::new(FailingAuditSink);
    let adapter = ScriptedAdapter::new(Script::Ok);
    let probe = adapter.clone();
    let rig = rig(&fx, &tmp, w, move |_| probe);
    let id = queued(&fx).await;

    let outcome = rig.worker.run_next().await;

    assert_eq!(
        outcome,
        Ok(RunOutcome::Failed {
            job_id: id,
            reason: ReasonCode::PersistenceError
        })
    );
    let job = require_job(&fx.conn, id).await;
    assert_eq!(job.status, "failed");
    assert_eq!(job.reason_code.as_deref(), Some("PERSISTENCE_ERROR"));
    assert_eq!(adapter.calls(), 0);
    assert_eq!(alloc_rows(&fx.conn).await, 0);
    assert!(names_in(&alloc_dir(&tmp)).is_empty());
    let root_text = tmp.path().to_string_lossy().into_owned();
    for json in super::support::audit_json_for_job(&fx.conn, id).await {
        assert!(!json.contains(root_text.as_str()));
    }
}

/// TS-48 (T-12): an adapter error fails the job `ADAPTER_ERROR` and the allocation is
/// released at once (`release_pending`).
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts48_adapter_error_fails_job_and_releases_allocation() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-adapter");
    let rig = rig(&fx, &tmp, Wiring::new(&fx), |_| {
        ScriptedAdapter::new(Script::Fail)
    });
    let id = queued(&fx).await;

    let outcome = rig.worker.run_next().await;

    assert_eq!(
        outcome,
        Ok(RunOutcome::Failed {
            job_id: id,
            reason: ReasonCode::AdapterError
        })
    );
    let job = require_job(&fx.conn, id).await;
    assert_eq!(job.status, "failed");
    assert_eq!(job.reason_code.as_deref(), Some("ADAPTER_ERROR"));
    assert_eq!(job.components, "[]");
    let allocs = alloc_ids_for_job(&fx.conn, id).await;
    assert_eq!(allocs.len(), 1);
    assert_eq!(
        raw_alloc(&fx.conn, allocs[0]).await.unwrap().state,
        "release_pending"
    );
}

/// TS-48 (FN-19): a queued job that cannot be dispatched fails `DISPATCH_FAILED`
/// (`queued -> failed`) and the audit event is `dispatch.failed`.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts48_dispatch_failed_fails_queued_job() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-dispatch");
    let adapter = ScriptedAdapter::new(Script::Ok);
    let probe = adapter.clone();
    let rig = rig(&fx, &tmp, Wiring::new(&fx), move |_| probe);
    let id = queued(&fx).await;

    let outcome = rig.worker.fail_dispatch(id).await;

    assert_eq!(
        outcome,
        Ok(RunOutcome::Failed {
            job_id: id,
            reason: ReasonCode::DispatchFailed
        })
    );
    let job = require_job(&fx.conn, id).await;
    assert_eq!(job.status, "failed");
    assert_eq!(job.reason_code.as_deref(), Some("DISPATCH_FAILED"));
    assert_eq!(job.started_at, None, "it never ran");
    let rows = audit_rows_for_job(&fx.conn, id).await;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].event, "dispatch.failed");
    assert_eq!(rows[0].outcome, "failure");
    assert_eq!(rows[0].reason_code.as_deref(), Some("DISPATCH_FAILED"));
    assert_eq!(adapter.calls(), 0);
    assert_eq!(alloc_rows(&fx.conn).await, 0);
}

// -------------------------------------------------------------------- TS-67

/// TS-67 (AC-002 S1; T-07): duplicate dispatch of one queued job has exactly one winner.
/// The winner is held inside the adapter after its claim committed, so the duplicate
/// runs against a `running` job: explicit interleaving, no sleep.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts67_duplicate_dispatch_has_exactly_one_winner() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-dup");
    let adapter = ScriptedAdapter::new(Script::Block(Box::new(Script::Ok)));
    let probe = adapter.clone();
    let rig = rig(&fx, &tmp, Wiring::new(&fx), move |_| probe);
    let id = queued(&fx).await;
    let winner = rig.worker.clone();
    let mut task = tokio::spawn(async move { winner.run_next().await });
    wait_entered(&adapter, &mut task).await;

    let duplicate = rig.worker.run_next().await;

    assert_eq!(duplicate, Ok(RunOutcome::Idle));
    adapter.release.notify_one();
    assert_eq!(
        task.await.expect("join"),
        Ok(RunOutcome::Incomplete { job_id: id })
    );
    assert_eq!(adapter.calls(), 1);
    assert_eq!(alloc_ids_for_job(&fx.conn, id).await.len(), 1);
    assert_eq!(
        events(&fx.conn, id).await,
        ["job.incomplete", "job.running"]
    );
    assert_eq!(require_job(&fx.conn, id).await.state_version, 2);
}

/// TS-67: two workers dispatched at the same instant over one queued job: one result is
/// `incomplete`, the other `Idle`, whatever the interleaving.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts67_simultaneous_dispatch_has_exactly_one_winner() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-simul");
    let adapter = ScriptedAdapter::new(Script::Ok);
    let probe = adapter.clone();
    let rig = rig(&fx, &tmp, Wiring::new(&fx), move |_| probe);
    let id = queued(&fx).await;
    let (a, b) = (rig.worker.clone(), rig.worker.clone());
    let ta = tokio::spawn(async move { a.run_next().await });
    let tb = tokio::spawn(async move { b.run_next().await });

    let mut results = vec![ta.await.expect("join a"), tb.await.expect("join b")];
    results.sort_by_key(|r| format!("{r:?}"));

    assert_eq!(
        results,
        [
            Ok(RunOutcome::Idle),
            Ok(RunOutcome::Incomplete { job_id: id })
        ]
    );
    assert_eq!(adapter.calls(), 1);
    let running = events(&fx.conn, id)
        .await
        .into_iter()
        .filter(|e| e == "job.running")
        .count();
    assert_eq!(running, 1, "one claim, one job.running row");
    assert_eq!(alloc_ids_for_job(&fx.conn, id).await.len(), 1);
}

/// TS-67 (T-12; FD 7.1: "the loser neither replays effects nor overwrites a terminal
/// result"): a late worker with a stale claim cannot overwrite a terminal job and writes no
/// job audit row of its own. The property is asserted, not the mechanism (pre-read or lost
/// compare-and-set): the job row and job events are unchanged, and no allocation of the job
/// is live and no file exists outside a `release_pending` ledger row.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts67_late_worker_cannot_overwrite_a_terminal_job() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-late");
    let rig = rig(&fx, &tmp, Wiring::new(&fx), real_adapter);
    let id = queued(&fx).await;
    let claimed = fx
        .store
        .claim_next(1, Duration::from_secs(600))
        .await
        .expect("claim")
        .expect("queued job");
    let swept = fx
        .store
        .transition(
            id,
            (BackupJobStatus::Running, claimed.state_version),
            BackupJobStatus::Failed,
            Some(ReasonCode::DeadlineExceeded),
            None,
        )
        .await;
    assert_eq!(swept, Ok(TransitionResult::Applied));
    let before = require_job(&fx.conn, id).await;
    let events_before = events(&fx.conn, id).await;
    assert_eq!(events_before, ["job.failed", "job.running"]);

    let outcome = rig.worker.execute(claimed).await;

    assert_eq!(outcome, RunOutcome::Lost { job_id: id });
    assert_eq!(require_job(&fx.conn, id).await, before);
    assert_eq!(before.status, "failed");
    assert_eq!(before.reason_code.as_deref(), Some("DEADLINE_EXCEEDED"));
    assert_eq!(
        events(&fx.conn, id).await,
        events_before,
        "the late worker wrote no job audit row"
    );
    for a in alloc_ids_for_job(&fx.conn, id).await {
        assert_eq!(
            raw_alloc(&fx.conn, a).await.unwrap().state,
            "release_pending",
            "no allocation of a terminal job stays live"
        );
    }
    for name in names_in(&alloc_dir(&tmp)) {
        let id_text = name.strip_suffix(".bin").expect("allocation file name");
        let alloc_id = Uuid::parse_str(id_text).expect("allocation id");
        let row = raw_alloc(&fx.conn, alloc_id)
            .await
            .expect("every file has a ledger row");
        assert_eq!(
            row.state, "release_pending",
            "a file may remain only awaiting release"
        );
    }
}

// -------------------------------------------------------------------- TS-05a

/// TS-05a (AC-005 S1; T-02): a denying policy at claim fails the job
/// `INSTANCE_NOT_ALLOWED` (`running -> failed`) with audit; the job does not run.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts05a_denied_policy_at_claim_fails_job_without_running_it() {
    let fx = fx().await;
    let tmp = TempDir::new("wu05-policy");
    let mut w = Wiring::new(&fx);
    w.policy = PolicyMode::Deny;
    let adapter = ScriptedAdapter::new(Script::Ok);
    let probe = adapter.clone();
    let rig = rig(&fx, &tmp, w, move |_| probe);
    let id = queued(&fx).await;

    let outcome = rig.worker.run_next().await;

    assert_eq!(
        outcome,
        Ok(RunOutcome::Failed {
            job_id: id,
            reason: ReasonCode::InstanceNotAllowed
        })
    );
    let job = raw_job(&fx.conn, id).await.expect("row");
    assert_eq!(job.status, "failed");
    assert_eq!(job.reason_code.as_deref(), Some("INSTANCE_NOT_ALLOWED"));
    assert_eq!(rig.policy.calls(), vec![(INSTANCE.to_owned(), id)]);
    assert_eq!(adapter.calls(), 0);
    assert_eq!(alloc_rows(&fx.conn).await, 0);
    assert_eq!(events(&fx.conn, id).await, ["job.failed", "job.running"]);
    let failed = audit_rows_for_job(&fx.conn, id)
        .await
        .into_iter()
        .find(|r| r.event == "job.failed")
        .expect("job.failed row");
    assert_eq!(failed.reason_code.as_deref(), Some("INSTANCE_NOT_ALLOWED"));
}
