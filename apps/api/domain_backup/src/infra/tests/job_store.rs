//! `DbJobStore` red tests. TS-14a (store-level concurrency conflict), CAS and
//! claim race tests (unit-own, no TS id), and the audit coupling that TS-69
//! requires of transitions and claims: the store hands its own transaction to the
//! sink and an audit failure refuses the change (AC-005 S2, T-08). AC-002 S1..S3,
//! T-06, T-07, T-12. FN-15 (transition), FN-16 (claim), FN-34 (audit coupling).
//!
//! Design choices recorded for the conductor:
//! - `claim_next` is the only path to `running` that the tests pin. `transition`
//!   from `queued` to `running` is neither required nor forbidden here, because it
//!   carries no deadline or epoch (FD 7.1: the claim sets `started_at`, `deadline_at`).
//! - Audit event kind per transition is pinned only for `running`, `incomplete` and
//!   `failed`; for `queued -> failed` either `job.failed` or `dispatch.failed` is
//!   accepted, because the contract does not say which the store writes.
//! - Tests never depend on more than one job running at once, so both a per-instance
//!   and a global reading of "concurrency 1" (FN-16, CFG-11) pass.

use super::support::*;
use crate::domain::status::BackupJobStatus::{self, *};
use crate::infra::db_job_store::DbJobStore;
use crate::ports::audit::AuditSink;
use crate::ports::job_store::{
    AcceptResult, ComponentName, ComponentOutcome, ComponentState, Components, IdemRecord,
    JobStore, ReasonCode, StoreError, TransitionResult,
};
use chrono::Duration as ChronoDuration;
use sea_orm::{DatabaseConnection, TransactionTrait};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Barrier;
use uuid::Uuid;

// ---------- helpers ----------

/// Accept on the caller's own transaction, committing on success and rolling
/// back on error. With a barrier the transaction is opened first, so the racers
/// reach `accept` together.
async fn accept_in_tx(
    store: &DbJobStore,
    conn: &DatabaseConnection,
    barrier: Option<Arc<Barrier>>,
    new: crate::ports::job_store::NewJob,
) -> Result<AcceptResult, StoreError> {
    let idem = idem_for(&new);
    accept_with_idem_in_tx(store, conn, barrier, new, idem).await
}

async fn accept_with_idem_in_tx(
    store: &DbJobStore,
    conn: &DatabaseConnection,
    barrier: Option<Arc<Barrier>>,
    new: crate::ports::job_store::NewJob,
    idem: IdemRecord,
) -> Result<AcceptResult, StoreError> {
    let tx = conn.begin().await.expect("begin");
    if let Some(b) = barrier {
        b.wait().await;
    }
    let result = store.accept(&tx, new, idem).await;
    match &result {
        Ok(_) => tx.commit().await.expect("commit"),
        Err(_) => tx.rollback().await.expect("rollback"),
    }
    result
}

fn reason(code: ReasonCode) -> Option<ReasonCode> {
    Some(code)
}

/// Audit events a transition to `to` may write. `queued -> failed` may be either the
/// job event or the dispatch event (not fixed by the contract).
fn allowed_events(from: BackupJobStatus, to: BackupJobStatus) -> &'static [&'static str] {
    match (from, to) {
        (_, Running) => &["job.running"],
        (_, Incomplete) => &["job.incomplete"],
        (Queued, Failed) => &["job.failed", "dispatch.failed"],
        (_, Failed) => &["job.failed"],
        other => panic!("no job event for {other:?}"),
    }
}

/// The four U1 edges, spelled independently of `is_allowed` (FD 7.1).
const U1_EDGES: [(BackupJobStatus, BackupJobStatus); 4] = [
    (Queued, Running),
    (Queued, Failed),
    (Running, Incomplete),
    (Running, Failed),
];

/// FD-consistent reason for each terminal edge (IFC 10.1).
fn edge_reason(from: BackupJobStatus, to: BackupJobStatus) -> ReasonCode {
    match (from, to) {
        (Queued, Failed) => ReasonCode::DispatchFailed,
        (Running, Incomplete) => ReasonCode::RequiredComponentsUnavailable,
        (Running, Failed) => ReasonCode::AdapterError,
        other => panic!("no terminal reason for {other:?}"),
    }
}

/// Reason spellings from IFC 10.1, independent of `ReasonCode::as_str`.
fn expected_reason_text(code: ReasonCode) -> &'static str {
    match code {
        ReasonCode::RequiredComponentsUnavailable => "REQUIRED_COMPONENTS_UNAVAILABLE",
        ReasonCode::AdapterError => "ADAPTER_ERROR",
        ReasonCode::DeadlineExceeded => "DEADLINE_EXCEEDED",
        ReasonCode::AllocationLimit => "ALLOCATION_LIMIT",
        ReasonCode::PersistenceError => "PERSISTENCE_ERROR",
        ReasonCode::InterruptedRestart => "INTERRUPTED_RESTART",
        ReasonCode::DispatchFailed => "DISPATCH_FAILED",
        ReasonCode::AbandonedQueued => "ABANDONED_QUEUED",
        ReasonCode::InstanceNotAllowed => "INSTANCE_NOT_ALLOWED",
    }
}

/// A contract-valid components array (IFC 2.3) as raw JSON, for seeding.
const COMPS_JSON: &str =
    r#"[{"name": "database", "state": "synthetic_captured", "reason_code": "DB_SYNTHETIC_ONLY"}]"#;

fn comps_value() -> Components {
    Components(vec![ComponentState {
        name: ComponentName::Database,
        state: ComponentOutcome::SyntheticCaptured,
        reason_code: Some("DB_SYNTHETIC_ONLY".to_owned()),
    }])
}

#[test]
fn reason_codes_map_to_the_ifc_10_1_spellings() {
    // IFC 10.1 job-level reason codes: a closed set of nine stable spellings. No Docker.
    let stored: Vec<&str> = ReasonCode::ALL.iter().map(|c| c.as_str()).collect();
    let expected: Vec<&str> = ReasonCode::ALL
        .iter()
        .map(|c| expected_reason_text(*c))
        .collect();
    assert_eq!(stored, expected);
    assert_eq!(ReasonCode::ALL.len(), 9);
}

// ---------- accept: queued-only insert, atomicity, TS-14a ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn accept_inserts_a_queued_job_and_its_idempotency_row() {
    // The store inserts only `queued` (WU-02 limitation); NewJob has no status field.
    let f = fx().await;
    let new = new_job("inst-a");
    let idem = idem_for(&new);
    let tx = f.conn.begin().await.unwrap();
    let result = f
        .store
        .accept(&tx, new.clone(), idem.clone())
        .await
        .expect("accept");
    tx.commit().await.unwrap();

    let job = require_job(&f.conn, new.id).await;
    assert_eq!(job.status, "queued");
    assert_eq!(job.state_version, 0);
    assert_eq!(job.epoch, 3);
    assert_eq!(job.reason_code, None);
    assert_eq!(job.started_at, None);
    assert_eq!(job.finished_at, None);
    assert_eq!(job.deadline_at, None);
    assert_eq!(job.updated_at, new.created_at);
    assert_eq!(job.components, "[]");
    let stored = count(
        &f.conn,
        "SELECT COUNT(*)::bigint AS n FROM backup_jobs WHERE id = $1 AND actor_sub = $2 \
         AND instance_id = $3 AND correlation_id = $4 AND created_at = $5 AND retain_until = $6",
        vec![
            new.id.into(),
            new.actor_sub.clone().into(),
            new.instance_id.clone().into(),
            new.correlation_id.clone().into(),
            new.created_at.into(),
            new.retain_until.into(),
        ],
    )
    .await;
    assert_eq!(stored, 1, "job columns must carry the NewJob values");
    let idem_rows = count(
        &f.conn,
        "SELECT COUNT(*)::bigint AS n FROM backup_idempotency WHERE id = $1 AND job_id = $2 \
         AND actor_sub = $3 AND operation = $4 AND key_digest = $5 AND request_fingerprint = $6",
        vec![
            idem.id.into(),
            new.id.into(),
            idem.actor_sub.clone().into(),
            idem.operation.clone().into(),
            idem.key_digest.clone().into(),
            idem.request_fingerprint.clone().into(),
        ],
    )
    .await;
    assert_eq!(idem_rows, 1, "idempotency row links the job by digest only");
    // [D] job.accepted is written by the WU-07 handler, not by the store.
    assert_eq!(audit_count_for_job(&f.conn, new.id).await, 0);
    match result {
        AcceptResult::Created(row) => {
            assert_eq!(row.id, new.id);
            assert_eq!(row.status, Queued);
            assert_eq!(row.state_version, 0);
        }
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn accept_is_part_of_the_callers_transaction() {
    let f = fx().await;
    let new = new_job("inst-a");
    let idem = idem_for(&new);
    let tx = f.conn.begin().await.unwrap();
    f.store
        .accept(&tx, new.clone(), idem)
        .await
        .expect("accept");
    assert_eq!(
        jobs_for_instance(&tx, "inst-a").await,
        1,
        "the job row must be written on the caller's transaction"
    );
    assert_eq!(
        jobs_for_instance(&f.conn, "inst-a").await,
        0,
        "not visible to other connections before commit"
    );
    tx.rollback().await.unwrap();
    assert_eq!(jobs_for_instance(&f.conn, "inst-a").await, 0);
    assert_eq!(idempotency_rows(&f.conn).await, 0);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn second_accept_conflicts_while_the_first_job_is_queued() {
    // TS-14a
    let f = fx().await;
    let first = new_job("inst-a");
    accept_in_tx(&f.store, &f.conn, None, first.clone())
        .await
        .expect("first accept");
    assert_eq!(active_jobs_for_instance(&f.conn, "inst-a").await, 1);

    let second = new_job("inst-a");
    let result = accept_in_tx(&f.store, &f.conn, None, second.clone()).await;
    assert_eq!(result, Err(StoreError::ConcurrencyConflict));
    assert!(raw_job(&f.conn, second.id).await.is_none());
    assert_eq!(jobs_for_instance(&f.conn, "inst-a").await, 1);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn second_accept_conflicts_while_a_job_is_running() {
    // TS-14a
    let f = fx().await;
    seed(&f.conn, Seed::new("inst-a", Running)).await;
    let result = accept_in_tx(&f.store, &f.conn, None, new_job("inst-a")).await;
    assert_eq!(result, Err(StoreError::ConcurrencyConflict));
    assert_eq!(jobs_for_instance(&f.conn, "inst-a").await, 1);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn accept_succeeds_after_the_active_job_is_terminal_and_for_other_instances() {
    // TS-14a: the index covers queued and running only, per instance.
    let f = fx().await;
    seed(&f.conn, Seed::new("inst-a", Failed)).await;
    seed(&f.conn, Seed::new("inst-a", Incomplete)).await;
    seed(&f.conn, Seed::new("inst-b", Running)).await;

    let after_terminal = new_job("inst-a");
    accept_in_tx(&f.store, &f.conn, None, after_terminal.clone())
        .await
        .expect("terminal jobs do not block a new job");
    assert_eq!(
        require_job(&f.conn, after_terminal.id).await.status,
        "queued"
    );

    let other_instance = new_job("inst-c");
    accept_in_tx(&f.store, &f.conn, None, other_instance.clone())
        .await
        .expect("a busy instance does not block another instance");
    assert_eq!(
        require_job(&f.conn, other_instance.id).await.status,
        "queued"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn conflicting_accept_leaves_no_job_or_idempotency_row() {
    // TS-14a: the refusal is durable and leaves nothing behind.
    let f = fx().await;
    seed(&f.conn, Seed::new("inst-a", Queued)).await;
    assert_eq!(idempotency_rows(&f.conn).await, 0);

    let loser = new_job("inst-a");
    let result = accept_in_tx(&f.store, &f.conn, None, loser.clone()).await;
    assert_eq!(result, Err(StoreError::ConcurrencyConflict));
    assert!(raw_job(&f.conn, loser.id).await.is_none());
    assert_eq!(idempotency_rows(&f.conn).await, 0);
    assert_eq!(audit_count_for_instance(&f.conn, "inst-a").await, 0);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn concurrent_accepts_for_one_instance_create_exactly_one_job() {
    // TS-14a race: N creates for the same instance, one winner, N-1 conflicts.
    const RACERS: usize = 6;
    const ROUNDS: usize = 5;
    let f = fx().await;
    for round in 0..ROUNDS {
        let instance = format!("race-inst-{round}");
        let barrier = Arc::new(Barrier::new(RACERS));
        let mut handles = Vec::new();
        for _ in 0..RACERS {
            let (store, conn, b, new) = (
                f.store.clone(),
                f.conn.clone(),
                barrier.clone(),
                new_job(&instance),
            );
            handles.push(tokio::spawn(async move {
                accept_in_tx(&store, &conn, Some(b), new).await
            }));
        }
        let mut created = 0;
        let mut conflicts = 0;
        let mut other = Vec::new();
        for h in handles {
            match h.await.expect("task") {
                Ok(_) => created += 1,
                Err(StoreError::ConcurrencyConflict) => conflicts += 1,
                Err(e) => other.push(e),
            }
        }
        assert!(
            other.is_empty(),
            "round {round}: unexpected errors {other:?}"
        );
        assert_eq!(created, 1, "round {round}: exactly one create wins");
        assert_eq!(conflicts, RACERS - 1, "round {round}: the rest conflict");
        assert_eq!(
            jobs_for_instance(&f.conn, &instance).await,
            1,
            "round {round}: exactly one durable job"
        );
        assert_eq!(active_jobs_for_instance(&f.conn, &instance).await, 1);
    }
    assert_eq!(
        idempotency_rows(&f.conn).await,
        ROUNDS as i64,
        "only winners leave idempotency rows"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn duplicate_job_id_is_a_persistence_error_not_a_concurrency_conflict() {
    // Only unique violations on the active-job index and the idempotency key map to
    // ConcurrencyConflict; a primary-key clash is a persistence error.
    let f = fx().await;
    let first = new_job("inst-a");
    accept_in_tx(&f.store, &f.conn, None, first.clone())
        .await
        .expect("first accept");
    let mut clash = new_job("inst-b");
    clash.id = first.id;
    let result = accept_in_tx(&f.store, &f.conn, None, clash).await;
    assert!(
        matches!(result, Err(StoreError::Persistence)),
        "a primary key clash is a persistence error, got {result:?}"
    );
    assert_eq!(jobs_for_instance(&f.conn, "inst-b").await, 0);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn accept_refuses_digests_that_are_not_sha256_hex_without_writing() {
    // Test-after (build-and-verify WU-03): `char(64)` accepts any 64 characters, so the
    // store must check both digests before any SQL. A raw Idempotency-Key must never be
    // stored in place of its digest.
    let f = fx().await;
    let job = new_job("inst-a");
    let good = idem_for(&job);
    for (label, bad) in [
        ("key_digest", {
            let mut i = good.clone();
            i.key_digest = "k".repeat(64);
            i
        }),
        ("request_fingerprint", {
            let mut i = good.clone();
            i.request_fingerprint = "raw-idempotency-key".to_owned();
            i
        }),
    ] {
        let result = accept_with_idem_in_tx(&f.store, &f.conn, None, job.clone(), bad).await;
        assert!(
            matches!(result, Err(StoreError::Persistence)),
            "{label}: a non-digest value must be refused, got {result:?}"
        );
        assert_eq!(jobs_for_instance(&f.conn, "inst-a").await, 0, "{label}");
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn an_idempotency_key_collision_is_a_unique_violation_not_a_persistence_error() {
    // IFC 4: on ANY unique violation (active-job index or idempotency UNIQUE) the caller
    // rolls back and re-reads by key_digest to choose replay, IDEMPOTENCY_KEY_REUSED or
    // CONCURRENCY_LIMIT. Decision: the store reports both as ConcurrencyConflict, so a
    // same-key accept after the first job is terminal is not projected as a 503.
    let f = fx().await;
    let first = new_job("inst-a");
    let first_idem = idem_for(&first);
    accept_with_idem_in_tx(&f.store, &f.conn, None, first.clone(), first_idem.clone())
        .await
        .expect("first accept");
    // Terminal, so the active-job index no longer blocks inst-a (queued -> failed).
    exec(
        &f.conn,
        "UPDATE backup_jobs SET status = 'failed', state_version = 1, finished_at = $2, \
         reason_code = 'DISPATCH_FAILED', updated_at = $2 WHERE id = $1",
        vec![first.id.into(), t0().into()],
    )
    .await;

    let second = new_job("inst-a");
    let mut second_idem = idem_for(&second);
    second_idem.key_digest = first_idem.key_digest.clone();
    let result = accept_with_idem_in_tx(&f.store, &f.conn, None, second.clone(), second_idem).await;
    assert_eq!(result, Err(StoreError::ConcurrencyConflict));
    assert!(raw_job(&f.conn, second.id).await.is_none());
    assert_eq!(jobs_for_instance(&f.conn, "inst-a").await, 1);
    assert_eq!(idempotency_rows(&f.conn).await, 1);
}

// ---------- get ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn get_returns_the_stored_row_and_none_for_an_unknown_id() {
    let f = fx().await;
    for (i, status) in SEEDABLE.into_iter().enumerate() {
        let id = seed(
            &f.conn,
            Seed::new(&format!("inst-{i}"), status)
                .version(i as i64 + 2)
                .reason("ADAPTER_ERROR")
                .components(COMPS_JSON),
        )
        .await;
        let row = f
            .store
            .get(id)
            .await
            .expect("get")
            .unwrap_or_else(|| panic!("row for {status:?} must be found"));
        assert_eq!(row.id, id);
        assert_eq!(row.status, status);
        assert_eq!(row.state_version, i as i64 + 2);
        assert_eq!(row.instance_id, format!("inst-{i}"));
        assert_eq!(row.actor_sub, "seed-actor");
        assert_eq!(row.reason_code.as_deref(), Some("ADAPTER_ERROR"));
        assert_eq!(row.epoch, 1);
        assert_eq!(row.components, comps_value());
        assert_eq!(row.operation, "backup_db_slice");
        assert!(row.synthetic);
        assert_eq!(row.started_at.is_some(), status != Queued);
        assert_eq!(row.finished_at.is_some(), is_terminal(status));
    }
    assert_eq!(f.store.get(Uuid::new_v4()).await.expect("get"), None);
}

// ---------- transition: compare-and-set ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn each_legal_edge_applies_bumps_the_version_and_stamps_times() {
    // FN-15: the terminal edges queued->failed, running->incomplete, running->failed.
    // queued->running is the claim (claim_next tests); transition is not pinned for it.
    let f = fx().await;
    for (i, (from, to)) in U1_EDGES.into_iter().skip(1).enumerate() {
        let id = seed(&f.conn, Seed::new(&format!("inst-{i}"), from).version(3)).await;
        let code = edge_reason(from, to);
        let result = f
            .store
            .transition(id, (from, 3), to, Some(code), Some(comps_value()))
            .await
            .unwrap_or_else(|e| panic!("{from:?} -> {to:?}: {e:?}"));
        assert_eq!(result, TransitionResult::Applied, "{from:?} -> {to:?}");

        let row = require_job(&f.conn, id).await;
        assert_eq!(row.status, wire(to), "{from:?} -> {to:?}");
        assert_eq!(row.state_version, 4, "state_version bumps by one");
        assert_eq!(row.updated_at, t0());
        assert_eq!(
            row.finished_at.is_some(),
            is_terminal(to),
            "finished_at is set exactly for terminal states"
        );
        assert_eq!(row.finished_at, Some(t0()));
        assert_eq!(row.reason_code.as_deref(), Some(expected_reason_text(code)));
        let comps_json: serde_json::Value = serde_json::from_str(&row.components).unwrap();
        assert_eq!(
            comps_json,
            serde_json::from_str::<serde_json::Value>(COMPS_JSON).unwrap(),
            "components are stored when supplied, in the IFC 2.3 shape"
        );
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_stale_version_or_status_loses_and_changes_nothing() {
    // CAS: Lost, not Err; no row change, no audit row.
    let f = fx().await;
    let id = seed(&f.conn, Seed::new("inst-a", Queued).version(2)).await;
    let before = require_job(&f.conn, id).await;

    let stale_version = f
        .store
        .transition(
            id,
            (Queued, 1),
            Failed,
            reason(ReasonCode::DispatchFailed),
            None,
        )
        .await
        .expect("stale version is Lost, not an error");
    assert_eq!(stale_version, TransitionResult::Lost);
    let stale_status = f
        .store
        .transition(
            id,
            (Running, 2),
            Incomplete,
            reason(ReasonCode::RequiredComponentsUnavailable),
            None,
        )
        .await
        .expect("stale status is Lost, not an error");
    assert_eq!(stale_status, TransitionResult::Lost);

    assert_eq!(require_job(&f.conn, id).await, before);
    assert_eq!(audit_count_for_job(&f.conn, id).await, 0);

    let fresh = f
        .store
        .transition(
            id,
            (Queued, 2),
            Failed,
            reason(ReasonCode::DispatchFailed),
            None,
        )
        .await
        .expect("fresh expectation");
    assert_eq!(fresh, TransitionResult::Applied);
    assert_eq!(require_job(&f.conn, id).await.state_version, 3);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn edges_outside_the_u1_graph_are_refused_without_changes() {
    // FN-15 (status enum retained, unreachable values): every non-edge from a live state, including succeeded and recovery-required.
    let f = fx().await;
    for from in [Queued, Running] {
        let id = seed(
            &f.conn,
            Seed::new(&format!("inst-{from:?}"), from).version(1),
        )
        .await;
        let before = require_job(&f.conn, id).await;
        for to in BackupJobStatus::ALL {
            if U1_EDGES.contains(&(from, to)) {
                continue;
            }
            let result = f.store.transition(id, (from, 1), to, None, None).await;
            assert_eq!(
                result,
                Err(StoreError::IllegalTransition { from, to }),
                "{from:?} -> {to:?} must be refused"
            );
            assert_eq!(require_job(&f.conn, id).await, before, "{from:?} -> {to:?}");
            assert_eq!(audit_count_for_job(&f.conn, id).await, 0);
        }
    }
    // An expectation of an unreachable status is refused before touching the row.
    let id = seed(&f.conn, Seed::new("inst-x", Queued)).await;
    for expect in [Succeeded, RecoveryRequired] {
        let result = f
            .store
            .transition(id, (expect, 0), Failed, None, None)
            .await;
        assert_eq!(
            result,
            Err(StoreError::IllegalTransition {
                from: expect,
                to: Failed
            })
        );
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn terminal_rows_are_immutable_through_the_store() {
    // FN-15: failed and incomplete have no outgoing transition and keep every field.
    let f = fx().await;
    for terminal in [Failed, Incomplete] {
        let id = seed(
            &f.conn,
            Seed::new(&format!("inst-{terminal:?}"), terminal)
                .version(2)
                .reason("ADAPTER_ERROR")
                .components(COMPS_JSON),
        )
        .await;
        let before = require_job(&f.conn, id).await;
        for to in BackupJobStatus::ALL {
            let result = f
                .store
                .transition(
                    id,
                    (terminal, 2),
                    to,
                    reason(ReasonCode::DeadlineExceeded),
                    Some(Components::default()),
                )
                .await;
            assert_eq!(
                result,
                Err(StoreError::IllegalTransition { from: terminal, to }),
                "{terminal:?} -> {to:?}"
            );
            assert_eq!(
                require_job(&f.conn, id).await,
                before,
                "{terminal:?} -> {to:?} must not change the row"
            );
        }
        assert_eq!(audit_count_for_job(&f.conn, id).await, 0);
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn transition_of_an_unknown_job_is_not_found() {
    let f = fx().await;
    let result = f
        .store
        .transition(Uuid::new_v4(), (Queued, 0), Failed, None, None)
        .await;
    assert_eq!(result, Err(StoreError::NotFound));
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_late_worker_cannot_overwrite_a_terminal_result() {
    // The loser of a race never overwrites a terminal result.
    let f = fx().await;
    let id = seed(&f.conn, Seed::new("inst-a", Running)).await;
    let applied = f
        .store
        .transition(
            id,
            (Running, 0),
            Failed,
            reason(ReasonCode::DeadlineExceeded),
            None,
        )
        .await
        .expect("sweeper wins");
    assert_eq!(applied, TransitionResult::Applied);
    let terminal = require_job(&f.conn, id).await;
    assert_eq!(terminal.status, "failed");
    assert_eq!(terminal.state_version, 1);

    // The worker still holds its old expectation.
    for to in [Incomplete, Failed] {
        let late = f
            .store
            .transition(id, (Running, 0), to, reason(ReasonCode::AdapterError), None)
            .await
            .expect("a late writer is Lost, not an error");
        assert_eq!(late, TransitionResult::Lost, "late {to:?}");
    }
    assert_eq!(require_job(&f.conn, id).await, terminal);
    assert_eq!(
        audit_rows_for_job(&f.conn, id).await.len(),
        1,
        "only the winner is audited"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn racing_terminal_transitions_have_exactly_one_winner() {
    // CAS race: running -> incomplete against running -> failed on the same version.
    const ROUNDS: usize = 10;
    let f = fx().await;
    for round in 0..ROUNDS {
        let id = seed(&f.conn, Seed::new(&format!("race-{round}"), Running)).await;
        let barrier = Arc::new(Barrier::new(2));
        let mut handles = Vec::new();
        for to in [Incomplete, Failed] {
            let (store, b) = (f.store.clone(), barrier.clone());
            handles.push(tokio::spawn(async move {
                b.wait().await;
                (
                    to,
                    store
                        .transition(id, (Running, 0), to, reason(ReasonCode::AdapterError), None)
                        .await,
                )
            }));
        }
        let mut winners = Vec::new();
        let mut losers = 0;
        for h in handles {
            match h.await.expect("task") {
                (to, Ok(TransitionResult::Applied)) => winners.push(to),
                (_, Ok(TransitionResult::Lost)) => losers += 1,
                (to, Err(e)) => panic!("round {round}: {to:?} errored: {e:?}"),
            }
        }
        assert_eq!(winners.len(), 1, "round {round}: exactly one winner");
        assert_eq!(losers, 1, "round {round}: the other reports Lost");
        let row = require_job(&f.conn, id).await;
        assert_eq!(row.status, wire(winners[0]), "round {round}");
        assert_eq!(row.state_version, 1, "round {round}: bumped once");
        let events = audit_rows_for_job(&f.conn, id).await;
        assert_eq!(events.len(), 1, "round {round}: one audit row");
        assert!(allowed_events(Running, winners[0]).contains(&events[0].event.as_str()));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_claim_and_a_queued_failure_race_to_exactly_one_winner() {
    // FN-16 vs queued->failed (dispatch failure) on the same version.
    const ROUNDS: usize = 10;
    let f = fx().await;
    for round in 0..ROUNDS {
        let instance = format!("race-{round}");
        let id = seed(&f.conn, Seed::new(&instance, Queued)).await;
        let barrier = Arc::new(Barrier::new(2));
        let claimer = {
            let (store, b) = (f.store.clone(), barrier.clone());
            tokio::spawn(async move {
                b.wait().await;
                store.claim_next(5, Duration::from_secs(600)).await
            })
        };
        let failer = {
            let (store, b) = (f.store.clone(), barrier.clone());
            tokio::spawn(async move {
                b.wait().await;
                store
                    .transition(
                        id,
                        (Queued, 0),
                        Failed,
                        reason(ReasonCode::DispatchFailed),
                        None,
                    )
                    .await
            })
        };
        let claimed = claimer.await.expect("task").expect("claim");
        let failed = failer.await.expect("task").expect("transition");
        let row = require_job(&f.conn, id).await;
        let events = audit_rows_for_job(&f.conn, id).await;
        assert_eq!(events.len(), 1, "round {round}: exactly one audit row");
        assert_eq!(row.state_version, 1, "round {round}: bumped once");
        match (claimed, failed) {
            (Some(job), TransitionResult::Lost) => {
                assert_eq!(job.id, id);
                assert_eq!(row.status, "running", "round {round}");
                assert_eq!(events[0].event, "job.running");
            }
            (None, TransitionResult::Applied) => {
                assert_eq!(row.status, "failed", "round {round}");
                assert!(allowed_events(Queued, Failed).contains(&events[0].event.as_str()));
            }
            (claimed, failed) => {
                panic!("round {round}: not exactly one winner: {claimed:?} / {failed:?}")
            }
        }
    }
}

// ---------- transition: audit coupling (TS-69 at the store boundary) ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_transition_writes_one_job_audit_row_with_its_event_and_ids() {
    // Unit-own, forward cover for TS-44b (WU-05): which event and which columns. System
    // events carry a null actor. The outcome vocabulary is not fixed by the contract, so
    // only membership of the closed AuditOutcome spellings is asserted.
    let f = fx().await;
    for (i, (from, to)) in U1_EDGES.into_iter().skip(1).enumerate() {
        let instance = format!("inst-{i}");
        let id = seed(&f.conn, Seed::new(&instance, from)).await;
        f.store
            .transition(id, (from, 0), to, Some(edge_reason(from, to)), None)
            .await
            .expect("transition");
        let rows = audit_rows_for_job(&f.conn, id).await;
        assert_eq!(rows.len(), 1, "{from:?} -> {to:?}: exactly one audit row");
        let row = &rows[0];
        let job = require_job(&f.conn, id).await;
        assert!(
            allowed_events(from, to).contains(&row.event.as_str()),
            "{from:?} -> {to:?}: event {}",
            row.event
        );
        assert!(["success", "failure", "denied"].contains(&row.outcome.as_str()));
        assert_eq!(row.actor_sub, None, "system event");
        assert_eq!(row.instance_id, instance);
        assert_eq!(row.job_id, Some(id));
        assert_eq!(row.allocation_id, None);
        assert_eq!(row.operation, "backup_db_slice");
        assert_eq!(row.occurred_at, t0());
        assert_eq!(row.digest, None);
        assert_eq!(row.correlation_id, format!("seed-corr-{id}"));
        assert_eq!(row.reason_code, job.reason_code);
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_transition_hands_its_own_uncommitted_transaction_to_the_audit_sink() {
    // TS-69 (AC-005 S2, T-08): the sink must be called with Some(tx), and while it runs the
    // job change must not yet be visible to another connection. An implementation that
    // audits with `None`, or commits before auditing, is caught.
    let f = fx().await;
    let probe = Arc::new(TxProbeSink::new(&f.conn));
    let store = store_with(&f.conn, probe.clone());
    for (i, (from, to)) in U1_EDGES.into_iter().skip(1).enumerate() {
        let id = seed(&f.conn, Seed::new(&format!("inst-{i}"), from).version(1)).await;
        probe.seen.lock().unwrap().clear();
        let result = store
            .transition(id, (from, 1), to, Some(edge_reason(from, to)), None)
            .await
            .expect("transition");
        assert_eq!(result, TransitionResult::Applied);
        let seen = probe.seen.lock().unwrap().clone();
        assert_eq!(seen.len(), 1, "{from:?} -> {to:?}: one audit call");
        assert!(
            seen[0].saw_tx,
            "{from:?} -> {to:?}: the sink got no transaction"
        );
        assert_eq!(
            seen[0].outside_view,
            Some((wire(from).to_owned(), 1)),
            "{from:?} -> {to:?}: the change was already committed when the sink ran"
        );
        assert_eq!(audit_count_for_job(&f.conn, id).await, 1);
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn an_audit_failure_refuses_the_transition() {
    // TS-69: sensitive operations require audit success (IFC 6.3, T-08). The cause of the
    // failure is not distinguished at this unit (see the report); the mapping is
    // StoreError::AuditUnavailable.
    let f = fx().await;
    let failing = store_with(&f.conn, Arc::new(FailingAuditSink));
    for (i, (from, to)) in U1_EDGES.into_iter().skip(1).enumerate() {
        let id = seed(&f.conn, Seed::new(&format!("inst-{i}"), from).version(1)).await;
        let before = require_job(&f.conn, id).await;
        let result = failing
            .transition(id, (from, 1), to, Some(edge_reason(from, to)), None)
            .await;
        assert_eq!(
            result,
            Err(StoreError::AuditUnavailable),
            "{from:?} -> {to:?}"
        );
        assert_eq!(
            require_job(&f.conn, id).await,
            before,
            "{from:?} -> {to:?}: the state change must roll back with the audit"
        );
        assert_eq!(audit_count_for_job(&f.conn, id).await, 0);
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn an_audit_row_written_before_the_sink_fails_rolls_back_with_the_transition() {
    // TS-69: job update and audit row are one unit. The sink writes its row on the store's
    // transaction and then fails; neither the row nor the job change may survive.
    let f = fx().await;
    let store = store_with(&f.conn, Arc::new(WriteThenFailSink::new(&f.conn)));
    for (i, (from, to)) in U1_EDGES.into_iter().skip(1).enumerate() {
        let id = seed(&f.conn, Seed::new(&format!("inst-{i}"), from).version(1)).await;
        let before = require_job(&f.conn, id).await;
        let result = store
            .transition(id, (from, 1), to, Some(edge_reason(from, to)), None)
            .await;
        assert_eq!(
            result,
            Err(StoreError::AuditUnavailable),
            "{from:?} -> {to:?}"
        );
        assert_eq!(require_job(&f.conn, id).await, before, "{from:?} -> {to:?}");
        assert_eq!(
            audit_count_for_job(&f.conn, id).await,
            0,
            "{from:?} -> {to:?}: an orphan audit row survived the refusal"
        );
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn the_job_audit_row_excludes_free_text_stored_on_the_job() {
    // Unit-own, forward cover for TS-17 (WU-08, WU-15). T-08 / T-09: the audit schema has no
    // payload column, so job content never leaks into it.
    let f = fx().await;
    let id = seed(&f.conn, Seed::new("inst-a", Running)).await;
    let comps = Components(vec![ComponentState {
        name: ComponentName::Storage,
        state: ComponentOutcome::Unavailable,
        reason_code: Some("CANARY-SECRET-xyz bearer CANARY-TOKEN-abc".to_owned()),
    }]);
    let result = f
        .store
        .transition(
            id,
            (Running, 0),
            Failed,
            reason(ReasonCode::AdapterError),
            Some(comps),
        )
        .await
        .expect("transition");
    assert_eq!(result, TransitionResult::Applied);
    assert!(
        require_job(&f.conn, id)
            .await
            .components
            .contains("CANARY-SECRET-xyz"),
        "the canary reaches the job row, so its absence from audit is meaningful"
    );
    let audit = audit_json_for_job(&f.conn, id).await;
    assert_eq!(audit.len(), 1, "the transition must be audited");
    for needle in ["CANARY-SECRET", "CANARY-TOKEN", "seed-actor"] {
        assert!(
            !audit[0].contains(needle),
            "audit row must not contain {needle}: {}",
            audit[0]
        );
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn omitting_components_keeps_the_stored_components() {
    // `comps: None` means "leave unchanged" (IFC 9: Option<Components>).
    let f = fx().await;
    let id = seed(&f.conn, Seed::new("inst-a", Running).components(COMPS_JSON)).await;
    let result = f
        .store
        .transition(
            id,
            (Running, 0),
            Incomplete,
            reason(ReasonCode::RequiredComponentsUnavailable),
            None,
        )
        .await
        .expect("transition");
    assert_eq!(result, TransitionResult::Applied);
    let row = require_job(&f.conn, id).await;
    assert_eq!(row.status, "incomplete");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&row.components).unwrap(),
        serde_json::from_str::<serde_json::Value>(COMPS_JSON).unwrap()
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_transition_blocked_on_the_row_lock_loses_after_the_winner_commits() {
    // Deterministic interleaving of the CAS (T-12): the winner updates the row on its open
    // transaction, the loser's transition runs and blocks on the row lock, then the winner
    // commits. The loser must report Lost and neither overwrite nor audit.
    let f = fx().await;
    let id = seed(&f.conn, Seed::new("inst-a", Queued)).await;
    let winner = f.conn.begin().await.unwrap();
    exec(
        &winner,
        "UPDATE backup_jobs SET status = 'running', state_version = 1, started_at = $2, \
         updated_at = $2 WHERE id = $1",
        vec![id.into(), t0().into()],
    )
    .await;
    let loser = {
        let store = f.store.clone();
        tokio::spawn(async move {
            store
                .transition(
                    id,
                    (Queued, 0),
                    Failed,
                    reason(ReasonCode::DispatchFailed),
                    None,
                )
                .await
        })
    };
    wait_until_blocked_or_done(&f.conn, &loser).await;
    winner.commit().await.unwrap();
    let result = loser.await.expect("task");
    assert_eq!(result, Ok(TransitionResult::Lost));
    let row = require_job(&f.conn, id).await;
    assert_eq!(
        row.status, "running",
        "the winner's state is not overwritten"
    );
    assert_eq!(row.state_version, 1);
    assert_eq!(row.finished_at, None);
    assert_eq!(
        audit_count_for_job(&f.conn, id).await,
        0,
        "the loser is not audited"
    );
}

// ---------- claim (FN-16) ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn claim_next_claims_the_oldest_queued_job() {
    let f = fx().await;
    let newer = seed(
        &f.conn,
        Seed::new("inst-b", Queued).created_at(t0() - ChronoDuration::hours(1)),
    )
    .await;
    let older = seed(
        &f.conn,
        Seed::new("inst-a", Queued).created_at(t0() - ChronoDuration::hours(2)),
    )
    .await;

    let claimed = f
        .store
        .claim_next(7, Duration::from_secs(600))
        .await
        .expect("claim")
        .expect("a queued job exists");
    assert_eq!(claimed.id, older, "oldest created_at first");
    assert_eq!(claimed.status, Running);
    assert_eq!(claimed.state_version, 1);
    assert_eq!(claimed.epoch, 7);
    assert_eq!(claimed.started_at, Some(t0()));
    assert_eq!(
        claimed.deadline_at,
        Some(t0() + ChronoDuration::seconds(600))
    );

    let row = require_job(&f.conn, older).await;
    assert_eq!(row.status, "running");
    assert_eq!(row.state_version, 1);
    assert_eq!(row.epoch, 7);
    assert_eq!(row.started_at, Some(t0()));
    assert_eq!(row.deadline_at, Some(t0() + ChronoDuration::seconds(600)));
    assert_eq!(row.finished_at, None);
    assert_eq!(require_job(&f.conn, newer).await.status, "queued");

    let rows = audit_rows_for_job(&f.conn, older).await;
    assert_eq!(
        rows.len(),
        1,
        "the claim writes job.running in its transaction"
    );
    assert_eq!(rows[0].event, "job.running");
    assert_eq!(rows[0].actor_sub, None);
    assert_eq!(audit_count_for_job(&f.conn, newer).await, 0);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn claim_next_never_reclaims_a_running_or_terminal_job() {
    let f = fx().await;
    // Running and terminal rows are never claimed. Only one job is ever running at a time
    // here, so a global or per-instance concurrency-1 rule both pass.
    seed(&f.conn, Seed::new("inst-f", Failed)).await;
    seed(&f.conn, Seed::new("inst-i", Incomplete)).await;
    let only = seed(&f.conn, Seed::new("inst-q", Queued)).await;

    let first = f
        .store
        .claim_next(1, Duration::from_secs(60))
        .await
        .expect("claim")
        .expect("the one queued job");
    assert_eq!(first.id, only);
    let second = f
        .store
        .claim_next(1, Duration::from_secs(60))
        .await
        .expect("claim");
    assert_eq!(
        second, None,
        "nothing queued remains; the running job is not reclaimed"
    );
    let row = require_job(&f.conn, only).await;
    assert_eq!(row.state_version, 1, "claimed once");
    assert_eq!(audit_count_for_job(&f.conn, only).await, 1);
    assert_eq!(
        count(
            &f.conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_audit",
            vec![]
        )
        .await,
        1,
        "other rows are untouched and unaudited"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn exactly_one_of_n_concurrent_claims_wins() {
    // FN-16: conditional claim, one winner per queued job (T-07, T-12).
    const CLAIMERS: usize = 6;
    const ROUNDS: usize = 5;
    let f = fx().await;
    for round in 0..ROUNDS {
        let id = seed(&f.conn, Seed::new(&format!("race-{round}"), Queued)).await;
        let barrier = Arc::new(Barrier::new(CLAIMERS));
        let mut handles = Vec::new();
        for _ in 0..CLAIMERS {
            let (store, b) = (f.store.clone(), barrier.clone());
            handles.push(tokio::spawn(async move {
                b.wait().await;
                store.claim_next(9, Duration::from_secs(600)).await
            }));
        }
        let mut winners = Vec::new();
        for h in handles {
            if let Some(job) = h.await.expect("task").expect("claim") {
                winners.push(job.id);
            }
        }
        assert_eq!(winners, vec![id], "round {round}: exactly one claim wins");
        let row = require_job(&f.conn, id).await;
        assert_eq!(row.status, "running");
        assert_eq!(row.state_version, 1, "round {round}: claimed once");
        assert_eq!(
            audit_rows_for_job(&f.conn, id).await.len(),
            1,
            "round {round}: one job.running row"
        );
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn concurrent_claims_never_claim_a_job_twice() {
    // FN-16: many queued jobs, many claimers: whatever is claimed is claimed exactly once,
    // and at least one claim succeeds. The count of winners is not pinned, so a global or
    // per-instance concurrency-1 rule both pass.
    const JOBS: usize = 5;
    let f = fx().await;
    let mut ids = Vec::new();
    for i in 0..JOBS {
        ids.push(seed(&f.conn, Seed::new(&format!("inst-{i}"), Queued)).await);
    }
    let barrier = Arc::new(Barrier::new(JOBS));
    let mut handles = Vec::new();
    for _ in 0..JOBS {
        let (store, b) = (f.store.clone(), barrier.clone());
        handles.push(tokio::spawn(async move {
            b.wait().await;
            store.claim_next(2, Duration::from_secs(600)).await
        }));
    }
    let mut claimed = Vec::new();
    for h in handles {
        if let Some(job) = h.await.expect("task").expect("claim") {
            claimed.push(job.id);
        }
    }
    assert!(!claimed.is_empty(), "at least one claimer wins");
    let mut sorted = claimed.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), claimed.len(), "no job is claimed twice");
    for id in &ids {
        let row = require_job(&f.conn, *id).await;
        if claimed.contains(id) {
            assert_eq!(row.status, "running");
            assert_eq!(row.state_version, 1, "claimed once");
            assert_eq!(audit_rows_for_job(&f.conn, *id).await.len(), 1);
        } else {
            assert_eq!(row.status, "queued");
            assert_eq!(row.state_version, 0);
            assert_eq!(audit_count_for_job(&f.conn, *id).await, 0);
        }
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_claim_blocked_on_a_row_taken_by_another_transaction_does_not_claim_it_again() {
    // FN-16 deterministic interleaving: another actor moves the oldest queued job to running
    // on an open transaction; the claim runs and blocks (or skips the locked row); the other
    // actor commits. A naive read-then-update claim would re-claim the running job and
    // bump its version again. Once the job is terminal, a fresh job is claimed normally.
    let f = fx().await;
    let taken = seed(&f.conn, Seed::new("inst-a", Queued)).await;
    let other = f.conn.begin().await.unwrap();
    exec(
        &other,
        "UPDATE backup_jobs SET status = 'running', state_version = 1, started_at = $2, \
         updated_at = $2 WHERE id = $1",
        vec![taken.into(), t0().into()],
    )
    .await;
    let claim = {
        let store = f.store.clone();
        tokio::spawn(async move { store.claim_next(4, Duration::from_secs(600)).await })
    };
    wait_until_blocked_or_done(&f.conn, &claim).await;
    other.commit().await.unwrap();
    let result = claim.await.expect("task");
    assert_eq!(result, Ok(None), "the job was already taken");
    let row = require_job(&f.conn, taken).await;
    assert_eq!(row.status, "running");
    assert_eq!(row.state_version, 1, "not claimed a second time");
    assert_eq!(row.epoch, 1, "the stale claim must not re-stamp the epoch");
    assert_eq!(audit_count_for_job(&f.conn, taken).await, 0);

    // Free the instance (running -> failed is a legal edge), then claim a fresh job.
    exec(
        &f.conn,
        "UPDATE backup_jobs SET status = 'failed', state_version = 2, finished_at = $2, \
         reason_code = 'ADAPTER_ERROR', updated_at = $2 WHERE id = $1",
        vec![taken.into(), t0().into()],
    )
    .await;
    let fresh = seed(&f.conn, Seed::new("inst-a", Queued)).await;
    let claimed = f
        .store
        .claim_next(4, Duration::from_secs(600))
        .await
        .expect("claim")
        .expect("the fresh queued job");
    assert_eq!(claimed.id, fresh);
    let row = require_job(&f.conn, fresh).await;
    assert_eq!(row.status, "running");
    assert_eq!(row.state_version, 1);
    assert_eq!(audit_rows_for_job(&f.conn, fresh).await.len(), 1);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn the_claim_hands_its_own_uncommitted_transaction_to_the_audit_sink() {
    // TS-69: the claim is a job transition, so the sink gets Some(tx) and the claim is not
    // yet visible to another connection while the sink runs.
    let f = fx().await;
    let id = seed(&f.conn, Seed::new("inst-a", Queued)).await;
    let probe = Arc::new(TxProbeSink::new(&f.conn));
    let store = store_with(&f.conn, probe.clone());
    let claimed = store
        .claim_next(1, Duration::from_secs(60))
        .await
        .expect("claim")
        .expect("queued job");
    assert_eq!(claimed.id, id);
    let seen = probe.seen.lock().unwrap().clone();
    assert_eq!(seen.len(), 1, "one audit call");
    assert!(seen[0].saw_tx, "the sink got no transaction");
    assert_eq!(
        seen[0].outside_view,
        Some(("queued".to_owned(), 0)),
        "the claim was already committed when the sink ran"
    );
    assert_eq!(audit_count_for_job(&f.conn, id).await, 1);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn an_audit_failure_refuses_the_claim() {
    // TS-69: the claim is a job transition, so a failed audit leaves the job queued.
    let f = fx().await;
    let id = seed(&f.conn, Seed::new("inst-a", Queued)).await;
    let before = require_job(&f.conn, id).await;
    let failing = store_with(&f.conn, Arc::new(FailingAuditSink) as Arc<dyn AuditSink>);
    let result = failing.claim_next(1, Duration::from_secs(60)).await;
    assert_eq!(result, Err(StoreError::AuditUnavailable));
    assert_eq!(require_job(&f.conn, id).await, before, "still queued");
    assert_eq!(audit_count_for_job(&f.conn, id).await, 0);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn an_audit_outage_is_a_store_outage_for_transitions_and_claims() {
    // FD 6.6: when the audit sink reports an outage (not a healthy-connection rejection),
    // the operation fails as STORE_UNAVAILABLE and nothing changes.
    let f = fx().await;
    let outage = store_with(&f.conn, Arc::new(OutageAuditSink) as Arc<dyn AuditSink>);
    for (i, (from, to)) in U1_EDGES.into_iter().skip(1).enumerate() {
        let id = seed(&f.conn, Seed::new(&format!("inst-{i}"), from).version(1)).await;
        let before = require_job(&f.conn, id).await;
        let result = outage
            .transition(id, (from, 1), to, Some(edge_reason(from, to)), None)
            .await;
        assert_eq!(
            result,
            Err(StoreError::StoreUnavailable),
            "{from:?} -> {to:?}"
        );
        assert_eq!(require_job(&f.conn, id).await, before, "{from:?} -> {to:?}");
        assert_eq!(audit_count_for_job(&f.conn, id).await, 0);
    }
    let queued = seed(&f.conn, Seed::new("inst-claim", Queued)).await;
    let before = require_job(&f.conn, queued).await;
    assert_eq!(
        outage.claim_next(1, Duration::from_secs(60)).await,
        Err(StoreError::StoreUnavailable)
    );
    assert_eq!(require_job(&f.conn, queued).await, before, "still queued");
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn an_audit_row_written_before_the_sink_fails_rolls_back_with_the_claim() {
    // TS-69: as for transition, the claim and its audit row are one unit.
    let f = fx().await;
    let id = seed(&f.conn, Seed::new("inst-a", Queued)).await;
    let before = require_job(&f.conn, id).await;
    let store = store_with(&f.conn, Arc::new(WriteThenFailSink::new(&f.conn)));
    let result = store.claim_next(1, Duration::from_secs(60)).await;
    assert_eq!(result, Err(StoreError::AuditUnavailable));
    assert_eq!(require_job(&f.conn, id).await, before, "still queued");
    assert_eq!(
        audit_count_for_job(&f.conn, id).await,
        0,
        "an orphan audit row survived the refusal"
    );
}
