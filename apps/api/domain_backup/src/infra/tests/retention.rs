//! WU-06 retention-store adapter contracts (TS-42 and TS-65 store portion).

use super::alloc_support::{raw_alloc, seed_alloc, RetentionProbeSink, SeedAlloc, INSTANCE};
use super::support::{count, exec, fx, pg, raw_job, seed, t0, AuditRow, Seed};
use crate::domain::status::BackupJobStatus;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_retention_store::DbRetentionStore;
use crate::ports::allocation_store::AllocationState;
use crate::ports::audit::{AuditError, AuditEvent, AuditEventKind, AuditOutcome, AuditSink};
use crate::ports::retention_store::RetentionStore;
use chrono::Duration;
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, Value};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use uuid::Uuid;

/// Counts the exact retention audit invocation, then refuses it.
#[derive(Debug)]
struct RejectingRetentionAudit {
    calls: Arc<AtomicUsize>,
}

#[async_trait::async_trait]
impl AuditSink for RejectingRetentionAudit {
    async fn record(
        &self,
        _tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        if ev.event == AuditEventKind::RetentionPurged {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Err(AuditError::Rejected)
        } else {
            Ok(())
        }
    }
}

async fn install_delete_order_probe(conn: &DatabaseConnection) {
    exec(
        conn,
        "CREATE TABLE retention_delete_order (sequence bigserial PRIMARY KEY, table_name text NOT NULL)",
        vec![],
    )
    .await;
    exec(
        conn,
        "CREATE FUNCTION capture_retention_delete_order() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN INSERT INTO retention_delete_order (table_name) VALUES (TG_TABLE_NAME); RETURN OLD; END $$",
        vec![],
    )
    .await;
    for table in ["backup_idempotency", "backup_allocations", "backup_jobs"] {
        exec(
            conn,
            &format!(
                "CREATE TRIGGER {table}_retention_order BEFORE DELETE ON {table} FOR EACH ROW EXECUTE FUNCTION capture_retention_delete_order()"
            ),
            vec![],
        )
        .await;
    }
}

async fn retention_delete_order(conn: &DatabaseConnection) -> Vec<String> {
    conn.query_all(pg(
        "SELECT table_name FROM retention_delete_order ORDER BY sequence",
        vec![],
    ))
    .await
    .expect("query retention delete order")
    .iter()
    .map(|row| row.try_get::<String>("", "table_name").unwrap())
    .collect()
}

fn retention_audit(instance_id: &str) -> AuditEvent {
    AuditEvent {
        occurred_at: t0(),
        actor_sub: None,
        instance_id: instance_id.to_owned(),
        job_id: None,
        allocation_id: None,
        operation: "backup_db_slice".to_owned(),
        event: AuditEventKind::JobAccepted,
        outcome: AuditOutcome::Success,
        reason_code: None,
        digest: None,
        correlation_id: "historical-audit".to_owned(),
    }
}

async fn seed_idempotency(
    conn: &DatabaseConnection,
    job_id: Uuid,
    expires_at: chrono::DateTime<chrono::Utc>,
) {
    exec(
        conn,
        "INSERT INTO backup_idempotency (id, actor_sub, operation, key_digest, \
         request_fingerprint, job_id, created_at, expires_at) \
         VALUES ($1, $2, 'backup_db_slice', $3, $4, $5, $6, $7)",
        vec![
            Uuid::new_v4().into(),
            format!("actor-{job_id}").into(),
            format!("{:064x}", job_id.as_u128()).into(),
            format!("{:064x}", job_id.as_u128() ^ 1).into(),
            job_id.into(),
            t0().into(),
            expires_at.into(),
        ],
    )
    .await;
}

async fn idempotency_for_job(conn: &impl ConnectionTrait, job_id: Uuid) -> i64 {
    count(
        conn,
        "SELECT COUNT(*)::bigint AS n FROM backup_idempotency WHERE job_id = $1",
        vec![job_id.into()],
    )
    .await
}

async fn audit_rows_for_event(conn: &impl ConnectionTrait, event: &str) -> Vec<AuditRow> {
    conn.query_all(pg(
        "SELECT occurred_at, actor_sub, instance_id, job_id, allocation_id, operation, event, \
         outcome, reason_code, digest::text AS digest, correlation_id FROM backup_audit \
         WHERE event = $1 ORDER BY occurred_at",
        vec![Value::from(event.to_owned())],
    ))
    .await
    .expect("query audit rows")
    .iter()
    .map(|row| AuditRow {
        occurred_at: row.try_get("", "occurred_at").unwrap(),
        actor_sub: row.try_get("", "actor_sub").unwrap(),
        instance_id: row.try_get("", "instance_id").unwrap(),
        job_id: row.try_get("", "job_id").unwrap(),
        allocation_id: row.try_get("", "allocation_id").unwrap(),
        operation: row.try_get("", "operation").unwrap(),
        event: row.try_get("", "event").unwrap(),
        outcome: row.try_get("", "outcome").unwrap(),
        reason_code: row.try_get("", "reason_code").unwrap(),
        digest: row.try_get("", "digest").unwrap(),
        correlation_id: row.try_get("", "correlation_id").unwrap(),
    })
    .collect()
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn retention_purge_orders_eligible_rows_and_retains_audit_and_ineligible_rows() {
    let f = fx().await;
    let conn = f.conn.clone();
    install_delete_order_probe(&conn).await;
    let now = t0() + Duration::days(31);

    let eligible = seed(
        &conn,
        Seed::new("retention-eligible", BackupJobStatus::Failed)
            .created_at(now - Duration::days(31)),
    )
    .await;
    let deleted = seed_alloc(
        &conn,
        SeedAlloc::new(AllocationState::Deleted, 11).job_id(eligible),
    )
    .await;
    seed_idempotency(&conn, eligible, now - Duration::seconds(1)).await;

    let eligible_without_allocation = seed(
        &conn,
        Seed::new(
            "retention-eligible-without-allocation",
            BackupJobStatus::Incomplete,
        )
        .created_at(now - Duration::days(31)),
    )
    .await;
    seed_idempotency(
        &conn,
        eligible_without_allocation,
        now - Duration::seconds(1),
    )
    .await;

    let probe = Arc::new(RetentionProbeSink::new(&conn, eligible, deleted));
    let sink: Arc<dyn AuditSink> = probe.clone();
    let store = DbRetentionStore::new(conn.clone(), sink.clone(), INSTANCE);

    let queued = seed(
        &conn,
        Seed::new("retention-queued", BackupJobStatus::Queued).created_at(now - Duration::days(31)),
    )
    .await;
    seed_idempotency(&conn, queued, now - Duration::seconds(1)).await;

    let mut excluded = Vec::new();
    for (index, state) in [
        AllocationState::Registered,
        AllocationState::Written,
        AllocationState::ReleasePending,
        AllocationState::Deleting,
        AllocationState::OrphanDiscovered,
        AllocationState::CleanupFailed,
    ]
    .into_iter()
    .enumerate()
    {
        let job = seed(
            &conn,
            Seed::new(
                &format!("retention-unclean-{index}"),
                BackupJobStatus::Failed,
            )
            .created_at(now - Duration::days(31)),
        )
        .await;
        let allocation = seed_alloc(&conn, SeedAlloc::new(state, 13).job_id(job)).await;
        excluded.push((job, allocation));
    }
    let running = seed(
        &conn,
        Seed::new("retention-running", BackupJobStatus::Running)
            .created_at(now - Duration::days(31)),
    )
    .await;
    seed_idempotency(&conn, running, now - Duration::seconds(1)).await;

    // Seed a prior audit row independently; the probe sink observes purge audits only.
    DbAuditSink::new(conn.clone())
        .record(None, retention_audit(INSTANCE))
        .await
        .expect("seed historical audit row");
    let audit_before = count(
        &conn,
        "SELECT COUNT(*)::bigint AS n FROM backup_audit",
        vec![],
    )
    .await;
    let probe_job_before = raw_job(&conn, eligible).await.unwrap();
    let probe_allocation_before = raw_alloc(&conn, deleted).await.unwrap();
    let probe_idempotency_before = idempotency_for_job(&conn, eligible).await;

    let report = store
        .purge_retained(now)
        .await
        .expect("purge retained rows");

    assert_eq!(
        report.jobs_purged, 2,
        "both eligible terminal jobs are purged, including one without allocations"
    );
    assert_eq!(report.idempotency_rows_purged, 2);
    assert_eq!(report.allocations_purged, 1);
    assert!(raw_job(&conn, eligible).await.is_none());
    assert!(raw_alloc(&conn, deleted).await.is_none());
    assert_eq!(idempotency_for_job(&conn, eligible).await, 0);
    assert!(
        raw_job(&conn, eligible_without_allocation).await.is_none(),
        "eligible terminal jobs do not require an allocation"
    );
    assert_eq!(
        idempotency_for_job(&conn, eligible_without_allocation).await,
        0
    );
    assert!(
        raw_job(&conn, queued).await.is_some(),
        "queued jobs are retained"
    );
    assert!(
        raw_job(&conn, running).await.is_some(),
        "running jobs are retained"
    );
    assert_eq!(idempotency_for_job(&conn, queued).await, 1);
    assert_eq!(idempotency_for_job(&conn, running).await, 1);
    for (job, allocation) in excluded {
        assert!(
            raw_job(&conn, job).await.is_some(),
            "unclean owner is retained"
        );
        assert!(
            raw_alloc(&conn, allocation).await.is_some(),
            "unclean allocation is retained"
        );
    }
    assert_eq!(
        count(
            &conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_audit",
            vec![]
        )
        .await,
        audit_before + 1,
        "purge audit is appended and prior audit history is retained"
    );
    let purge_audit = audit_rows_for_event(&conn, "retention.purged").await;
    assert_eq!(purge_audit.len(), 1);
    assert_eq!(purge_audit[0].instance_id, INSTANCE);
    {
        let audit_probe = probe.seen.lock().unwrap();
        assert_eq!(audit_probe.len(), 1);
        assert!(audit_probe[0].saw_tx);
        assert_eq!(
            audit_probe[0].event,
            crate::ports::audit::AuditEventKind::RetentionPurged
        );
        assert_eq!(probe_job_before.status, "failed");
        assert_eq!(probe_allocation_before.state, "deleted");
        assert_eq!(probe_idempotency_before, 1);
        assert!(
            audit_probe[0].target_job_exists
                && audit_probe[0].target_allocation_exists
                && audit_probe[0].target_idempotency_exists,
            "the retention.purged audit is recorded before the eligible targets are deleted"
        );
    }
    let delete_order = retention_delete_order(&conn).await;
    let first_idempotency = delete_order
        .iter()
        .position(|table| table == "backup_idempotency")
        .expect("idempotency deletion captured");
    let first_allocation = delete_order
        .iter()
        .position(|table| table == "backup_allocations")
        .expect("allocation deletion captured");
    let first_job = delete_order
        .iter()
        .position(|table| table == "backup_jobs")
        .expect("job deletion captured");
    assert!(
        first_idempotency < first_allocation && first_allocation < first_job,
        "FK-safe delete order is idempotency, deleted allocations, then jobs: {delete_order:?}"
    );

    let repeated = store.purge_retained(now).await.expect("repeat purge");
    assert_eq!(repeated.jobs_purged, 0);
    assert_eq!(repeated.idempotency_rows_purged, 0);
    assert_eq!(repeated.allocations_purged, 0);
    assert_eq!(
        audit_rows_for_event(&conn, "retention.purged").await.len(),
        1
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn retention_purge_audit_failure_rolls_back_every_table() {
    let f = fx().await;
    let conn = f.conn.clone();
    let audit_calls = Arc::new(AtomicUsize::new(0));
    let sink: Arc<dyn AuditSink> = Arc::new(RejectingRetentionAudit {
        calls: audit_calls.clone(),
    });
    let store = DbRetentionStore::new(conn.clone(), sink, INSTANCE);
    let now = t0() + Duration::days(31);
    let eligible = seed(
        &conn,
        Seed::new("retention-rollback", BackupJobStatus::Failed)
            .created_at(now - Duration::days(31)),
    )
    .await;
    let allocation = seed_alloc(
        &conn,
        SeedAlloc::new(AllocationState::Deleted, 19).job_id(eligible),
    )
    .await;
    seed_idempotency(&conn, eligible, now - Duration::seconds(1)).await;
    let before_job = raw_job(&conn, eligible).await.unwrap();
    let before_allocation = raw_alloc(&conn, allocation).await.unwrap();
    let before_idempotency = idempotency_for_job(&conn, eligible).await;
    let before_audit_rows = count(
        &conn,
        "SELECT COUNT(*)::bigint AS n FROM backup_audit",
        vec![],
    )
    .await;

    let result = store.purge_retained(now).await;

    assert_eq!(
        audit_calls.load(Ordering::SeqCst),
        1,
        "the retention.purged audit call must be reached exactly once before claiming audit refusal"
    );
    assert!(
        result.is_err(),
        "rejected purge audit must abort the transaction"
    );
    assert_eq!(raw_job(&conn, eligible).await.unwrap(), before_job);
    assert_eq!(
        raw_alloc(&conn, allocation).await.unwrap(),
        before_allocation
    );
    assert_eq!(
        idempotency_for_job(&conn, eligible).await,
        before_idempotency
    );
    assert_eq!(before_idempotency, 1);
    assert_eq!(
        count(
            &conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_audit",
            vec![]
        )
        .await,
        before_audit_rows,
        "the rejected audit does not leave a partial audit row"
    );
    assert!(audit_rows_for_event(&conn, "retention.purged")
        .await
        .is_empty());
}
