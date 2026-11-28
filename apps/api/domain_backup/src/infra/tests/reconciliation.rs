//! WU-06 store-port red tests. These exercise the new reconciliation contracts
//! against PostgreSQL 15 and raw SQL pre/post state, not mocks.
//!
//! TS-20..TS-30, TS-32, TS-42, TS-44b, TS-56 and TS-65 require the lifecycle
//! wiring and RetentionStore adapter; those scenarios are intentionally not
//! represented as covered by these store-level cases.
use super::alloc_support::{
    audit_events_for_allocation, audit_rows_for_allocation, raw_alloc, seed_alloc, AllocProbeSink,
    SeedAlloc, INSTANCE,
};
use super::support::{audit_rows_for_job, exec, fx, raw_job, seed, t0, Seed, WriteThenFailSink};
use crate::domain::status::BackupJobStatus;
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::ports::allocation_store::{AllocationLimits, AllocationState, AllocationStore};
use crate::ports::audit::AuditSink;
use crate::ports::job_store::{JobStore, ReasonCode, TransitionResult};
use crate::test_db::backup_test_db;
use chrono::Duration;
use sea_orm::ConnectionTrait;
use std::sync::Arc;

/// JobStore store contract: reconciliation inventory and durable epoch advancement.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn job_store_reconciliation_inventory_and_epoch_are_durable() {
    let fx = fx().await;
    let queued = seed(&fx.conn, Seed::new(INSTANCE, BackupJobStatus::Queued)).await;
    let running = seed(
        &fx.conn,
        Seed::new("reconcile-running-instance", BackupJobStatus::Running).version(3),
    )
    .await;

    let first_epoch = fx.store.advance_epoch(t0()).await.expect("epoch advances");
    assert_eq!(first_epoch, 1, "a fresh database creates epoch 1");
    let first_row = fx
        .conn
        .query_one(super::support::pg(
            "SELECT epoch FROM backup_epoch WHERE id = 1",
            vec![],
        ))
        .await
        .expect("query backup_epoch after first advance")
        .expect("backup_epoch row after first advance");
    assert_eq!(
        first_row.try_get::<i64>("", "epoch").unwrap(),
        1,
        "first epoch is durable in the raw SQL row"
    );

    let second_epoch = fx.store.advance_epoch(t0()).await.expect("epoch advances");
    assert_eq!(second_epoch, 2, "the existing epoch increments");
    let second_row = fx
        .conn
        .query_one(super::support::pg(
            "SELECT epoch FROM backup_epoch WHERE id = 1",
            vec![],
        ))
        .await
        .expect("query backup_epoch after second advance")
        .expect("backup_epoch row after second advance");
    assert_eq!(
        second_row.try_get::<i64>("", "epoch").unwrap(),
        2,
        "incremented epoch is durable in the raw SQL row"
    );

    let jobs = fx
        .store
        .list_for_reconciliation()
        .await
        .expect("complete inventory");
    let mut ids = jobs.iter().map(|j| j.id).collect::<Vec<_>>();
    ids.sort();
    let mut expected = vec![queued, running];
    expected.sort();
    assert_eq!(ids, expected);
    assert_eq!(raw_job(&fx.conn, queued).await.unwrap().status, "queued");
    assert_eq!(raw_job(&fx.conn, running).await.unwrap().state_version, 3);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically() {
    let fx = fx().await;
    let id = seed(
        &fx.conn,
        Seed::new(INSTANCE, BackupJobStatus::Running).version(4),
    )
    .await;

    let stale = fx
        .store
        .fence_job(
            id,
            0,
            BackupJobStatus::Running,
            4,
            BackupJobStatus::Failed,
            ReasonCode::InterruptedRestart,
            t0(),
        )
        .await
        .expect("stale epoch loses");
    assert_eq!(stale, TransitionResult::Lost);
    assert_eq!(raw_job(&fx.conn, id).await.unwrap().status, "running");
    assert!(audit_rows_for_job(&fx.conn, id).await.is_empty());

    let fenced = fx
        .store
        .fence_job(
            id,
            1,
            BackupJobStatus::Running,
            4,
            BackupJobStatus::Failed,
            ReasonCode::InterruptedRestart,
            t0(),
        )
        .await
        .expect("matching fence applies");
    assert_eq!(fenced, TransitionResult::Applied);
    let after = raw_job(&fx.conn, id).await.unwrap();
    assert_eq!((after.status.as_str(), after.state_version), ("failed", 5));
    let audit = audit_rows_for_job(&fx.conn, id).await;
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].event, "reconcile.fenced");
    assert_eq!(audit[0].reason_code.as_deref(), Some("INTERRUPTED_RESTART"));
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts30_deadline_inventory_includes_only_due_jobs_at_epoch() {
    let fx = fx().await;
    let overdue = seed(&fx.conn, Seed::new(INSTANCE, BackupJobStatus::Running)).await;
    let at_deadline = seed(
        &fx.conn,
        Seed::new("deadline-at-instance", BackupJobStatus::Running),
    )
    .await;
    let future = seed(
        &fx.conn,
        Seed::new("deadline-future-instance", BackupJobStatus::Running),
    )
    .await;
    let other_epoch = seed(
        &fx.conn,
        Seed::new("deadline-other-epoch-instance", BackupJobStatus::Running),
    )
    .await;
    let now = t0() + Duration::seconds(600);
    for (id, deadline, epoch) in [
        (overdue, now - Duration::seconds(1), 7_i64),
        (at_deadline, now, 7_i64),
        (future, now + Duration::seconds(1), 7_i64),
        (other_epoch, now - Duration::seconds(1), 6_i64),
    ] {
        exec(
            &fx.conn,
            "UPDATE backup_jobs SET deadline_at = $1, epoch = $2 WHERE id = $3",
            vec![deadline.into(), epoch.into(), id.into()],
        )
        .await;
    }

    let mut ids = fx
        .store
        .list_deadline_expired(now, 7)
        .await
        .expect("deadline inventory")
        .into_iter()
        .map(|job| job.id)
        .collect::<Vec<_>>();
    ids.sort();
    let mut expected = vec![overdue, at_deadline];
    expected.sort();
    assert_eq!(
        ids, expected,
        "includes deadlines at/before now, only in epoch"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts56_allocation_inventory_preserves_nullable_orphan_owners() {
    let db = backup_test_db().await;
    let conn = db.conn.clone();
    let sink: Arc<dyn AuditSink> = Arc::new(DbAuditSink::new(conn.clone()));
    let store = DbAllocationStore::new(
        conn.clone(),
        sink,
        Arc::new(super::support::FixedClock(t0())),
        INSTANCE,
        AllocationLimits::default(),
    );
    let missing_owner_id = uuid::Uuid::new_v4();
    let missing_owner_allocation_id = uuid::Uuid::new_v4();
    let missing_owner_insert = conn
        .execute(super::support::pg(
            "INSERT INTO backup_allocations (id, job_id, storage_key, state, epoch, bytes_reserved, expires_at, created_at, updated_at) \
             VALUES ($1, $2, $3, 'orphan_discovered', 1, 1, $4, $4, $4)",
            vec![
                missing_owner_allocation_id.into(),
                missing_owner_id.into(),
                format!("alloc/{missing_owner_allocation_id}.bin").into(),
                t0().into(),
            ],
        ))
        .await;
    assert!(
        missing_owner_insert.is_err(),
        "the backup_allocations.job_id foreign key rejects a non-NULL missing owner"
    );

    let failed_owner = seed(
        &conn,
        Seed::new("reconciliation-failed-owner", BackupJobStatus::Failed),
    )
    .await;
    let incomplete_owner = seed(
        &conn,
        Seed::new(
            "reconciliation-incomplete-owner",
            BackupJobStatus::Incomplete,
        ),
    )
    .await;
    let failed_allocation = seed_alloc(
        &conn,
        SeedAlloc::new(AllocationState::Written, 17).job_id(failed_owner),
    )
    .await;
    let incomplete_allocation = seed_alloc(
        &conn,
        SeedAlloc::new(AllocationState::ReleasePending, 19).job_id(incomplete_owner),
    )
    .await;
    let orphan = seed_alloc(&conn, SeedAlloc::new(AllocationState::OrphanDiscovered, 23)).await;

    let rows = store
        .list_for_reconciliation()
        .await
        .expect("complete left-joined inventory");
    let mut actual_ids = rows.iter().map(|r| r.allocation.id).collect::<Vec<_>>();
    actual_ids.sort();
    let mut expected_ids = vec![failed_allocation, incomplete_allocation, orphan];
    expected_ids.sort();
    assert_eq!(
        actual_ids, expected_ids,
        "unfiltered inventory includes every owner-joined and NULL-owner allocation"
    );

    let failed_row = rows
        .iter()
        .find(|r| r.allocation.id == failed_allocation)
        .expect("failed-owner allocation retained");
    assert_eq!(failed_row.allocation.job_id, Some(failed_owner));
    assert_eq!(failed_row.owner_status, Some(BackupJobStatus::Failed));
    assert_eq!(failed_row.owner_epoch, Some(1));

    let incomplete_row = rows
        .iter()
        .find(|r| r.allocation.id == incomplete_allocation)
        .expect("incomplete-owner allocation retained");
    assert_eq!(incomplete_row.allocation.job_id, Some(incomplete_owner));
    assert_eq!(
        incomplete_row.owner_status,
        Some(BackupJobStatus::Incomplete)
    );
    assert_eq!(incomplete_row.owner_epoch, Some(1));

    // The schema's nullable FK permits a NULL owner, but prevents a non-NULL missing job id.
    // Therefore this store test exercises the representable NULL-owner/orphan case.
    let orphan_row = rows
        .iter()
        .find(|r| r.allocation.id == orphan)
        .expect("orphan retained");
    assert_eq!(orphan_row.allocation.job_id, None);
    assert_eq!(orphan_row.owner_status, None);
    assert_eq!(orphan_row.owner_epoch, None);
    assert_eq!(
        raw_alloc(&conn, orphan).await.unwrap().state,
        "orphan_discovered"
    );
}

/// IFC 6.1a: an applied allocation reconciliation CAS commits its audit event atomically.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn allocation_reconciliation_cas_updates_state_and_audits_atomically() {
    let db = backup_test_db().await;
    let conn = db.conn.clone();
    let probe = Arc::new(AllocProbeSink::new(&conn));
    let sink: Arc<dyn AuditSink> = probe.clone();
    let store = DbAllocationStore::new(
        conn.clone(),
        sink,
        Arc::new(super::support::FixedClock(t0())),
        INSTANCE,
        AllocationLimits::default(),
    );
    let id = seed_alloc(
        &conn,
        SeedAlloc::new(AllocationState::Written, 17).version(2),
    )
    .await;

    let applied = store
        .reconcile_state(
            id,
            (AllocationState::Written, 2, 1),
            AllocationState::ReleasePending,
            t0(),
        )
        .await
        .expect("reconcile CAS");

    if applied != TransitionResult::Applied {
        panic!(
            "TS-44b allocation.reconciled audit-field assertions below were not reached against the inert default: got {applied:?}; their execution is deferred to GREEN"
        );
    }
    let after = raw_alloc(&conn, id).await.unwrap();
    assert_eq!(
        (after.state.as_str(), after.state_version),
        ("release_pending", 3)
    );
    let audit = audit_rows_for_allocation(&conn, id).await;
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].event, "allocation.reconciled");
    assert_eq!(audit[0].outcome, "success");
    assert_eq!(audit[0].reason_code.as_deref(), Some("RECONCILED"));
    assert_eq!(audit[0].instance_id, INSTANCE);
    assert_eq!(audit[0].actor_sub, None);
    assert_eq!(audit[0].job_id, None);
    assert_eq!(audit[0].allocation_id, Some(id));
    assert_eq!(audit[0].operation, "reconciliation");
    assert_eq!(audit[0].digest, None);
    assert_eq!(audit[0].correlation_id, id.to_string());
    let seen = probe.seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert!(
        seen[0].saw_tx,
        "audit write must share the allocation transaction"
    );
    assert_eq!(seen[0].outside_state.as_deref(), Some("written"));
}

/// IFC 6.1a: a lost CAS must neither mutate the allocation nor append an audit row.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn lost_allocation_reconciliation_cas_has_no_audit_or_state_change() {
    let db = backup_test_db().await;
    let conn = db.conn.clone();
    let sink: Arc<dyn AuditSink> = Arc::new(DbAuditSink::new(conn.clone()));
    let store = DbAllocationStore::new(
        conn.clone(),
        sink,
        Arc::new(super::support::FixedClock(t0())),
        INSTANCE,
        AllocationLimits::default(),
    );
    let id = seed_alloc(
        &conn,
        SeedAlloc::new(AllocationState::Written, 17).version(2),
    )
    .await;

    let lost = store
        .reconcile_state(
            id,
            (AllocationState::Written, 1, 1),
            AllocationState::ReleasePending,
            t0(),
        )
        .await
        .expect("lost CAS is a result, not an error");

    assert_eq!(lost, TransitionResult::Lost);
    let after = raw_alloc(&conn, id).await.unwrap();
    assert_eq!((after.state.as_str(), after.state_version), ("written", 2));
    assert!(audit_events_for_allocation(&conn, id).await.is_empty());
}

/// A rejected reconciliation audit must roll back both the allocation update and the
/// audit row written by the wrapper before it reports failure.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn reconciliation_audit_failure_rolls_back_allocation_and_audit() {
    let db = backup_test_db().await;
    let conn = db.conn.clone();
    let sink: Arc<dyn AuditSink> = Arc::new(WriteThenFailSink::new(&conn));
    let store = DbAllocationStore::new(
        conn.clone(),
        sink,
        Arc::new(super::support::FixedClock(t0())),
        INSTANCE,
        AllocationLimits::default(),
    );
    let id = seed_alloc(
        &conn,
        SeedAlloc::new(AllocationState::Written, 17).version(2),
    )
    .await;
    let before = raw_alloc(&conn, id).await.unwrap();
    let audit_before = audit_rows_for_allocation(&conn, id).await;

    let failed = store
        .reconcile_state(
            id,
            (AllocationState::Written, 2, 1),
            AllocationState::ReleasePending,
            t0(),
        )
        .await;

    assert!(
        failed.is_err(),
        "audit rejection must refuse the reconciliation"
    );
    assert_eq!(raw_alloc(&conn, id).await.unwrap(), before);
    assert_eq!(audit_rows_for_allocation(&conn, id).await, audit_before);
}
