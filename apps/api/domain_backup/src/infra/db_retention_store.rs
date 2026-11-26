//! PostgreSQL adapter for the cross-table retention purge contract (FN-33).

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, Statement,
    TransactionTrait, Value,
};
use std::fmt;
use std::sync::Arc;

use crate::infra::db_allocation_store::{map_audit, map_db};
use crate::ports::allocation_store::AllocationError;
use crate::ports::audit::{AuditEvent, AuditEventKind, AuditOutcome, AuditSink};
use crate::ports::retention_store::{PurgeReport, RetentionStore};

pub struct DbRetentionStore {
    conn: DatabaseConnection,
    audit: Arc<dyn AuditSink>,
    instance_id: String,
}

impl fmt::Debug for DbRetentionStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbRetentionStore").finish_non_exhaustive()
    }
}

impl DbRetentionStore {
    pub fn new(
        conn: DatabaseConnection,
        audit: Arc<dyn AuditSink>,
        instance_id: impl Into<String>,
    ) -> Self {
        Self {
            conn,
            audit,
            instance_id: instance_id.into(),
        }
    }
}

fn pg(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Postgres, sql, values)
}

fn purge_correlation(now: DateTime<Utc>) -> String {
    format!("retention-{}", now.timestamp_micros())
}

async fn abort<T>(tx: DatabaseTransaction, err: AllocationError) -> Result<T, AllocationError> {
    let _ = tx.rollback().await;
    Err(err)
}

#[async_trait]
impl RetentionStore for DbRetentionStore {
    async fn purge_retained(&self, now: DateTime<Utc>) -> Result<PurgeReport, AllocationError> {
        let tx = self.conn.begin().await.map_err(|e| map_db(&e))?;
        // Eligible jobs are terminal, past retention, and have no allocation that still
        // represents owned/unresolved state. CTEs materialize the exact target set before
        // dependent rows are removed, preserving FK-safe order in this transaction.
        let ids = match tx
            .query_all(pg(
                "SELECT j.id FROM backup_jobs j \
                 WHERE j.status IN ('failed', 'incomplete') AND j.retain_until < $1 \
                 AND NOT EXISTS (SELECT 1 FROM backup_allocations a \
                                 WHERE a.job_id = j.id AND a.state <> 'deleted') \
                 ORDER BY j.id",
                vec![now.into()],
            ))
            .await
        {
            Ok(rows) => rows
                .iter()
                .map(|row| row.try_get::<uuid::Uuid>("", "id"))
                .collect::<Result<Vec<_>, _>>(),
            Err(error) => return abort(tx, map_db(&error)).await,
        };
        let ids = match ids {
            Ok(ids) => ids,
            Err(_) => return abort(tx, AllocationError::Persistence).await,
        };
        let mut report = PurgeReport::default();
        if !ids.is_empty() {
            let audit = AuditEvent {
                occurred_at: now,
                actor_sub: None,
                instance_id: self.instance_id.clone(),
                job_id: None,
                allocation_id: None,
                operation: "retention".to_owned(),
                event: AuditEventKind::RetentionPurged,
                outcome: AuditOutcome::Success,
                reason_code: None,
                digest: None,
                correlation_id: purge_correlation(now),
            };
            if let Err(error) = self.audit.record(Some(&tx), audit).await {
                let mapped = map_audit(error);
                let _ = tx.rollback().await;
                return Err(mapped);
            }
            let id_values = ids.iter().copied().map(Value::from).collect::<Vec<_>>();
            let placeholders = (1..=ids.len())
                .map(|i| format!("${i}"))
                .collect::<Vec<_>>()
                .join(", ");
            let allocation_sql = format!(
                "DELETE FROM backup_allocations WHERE state = 'deleted' AND job_id IN ({placeholders})"
            );
            let idem_sql =
                format!("DELETE FROM backup_idempotency WHERE job_id IN ({placeholders})");
            // Test strategy requires idempotency rows first, then deleted allocations,
            // then jobs. The audit is already in this same transaction and precedes all.
            let idem_result = match tx.execute(pg(&idem_sql, id_values.clone())).await {
                Ok(result) => result,
                Err(error) => return abort(tx, map_db(&error)).await,
            };
            report.idempotency_rows_purged = idem_result.rows_affected();
            let allocation_result = match tx.execute(pg(&allocation_sql, id_values.clone())).await {
                Ok(result) => result,
                Err(error) => return abort(tx, map_db(&error)).await,
            };
            report.allocations_purged = allocation_result.rows_affected();
            let job_sql = format!("DELETE FROM backup_jobs WHERE id IN ({placeholders})");
            let job_result = match tx.execute(pg(&job_sql, id_values)).await {
                Ok(result) => result,
                Err(error) => return abort(tx, map_db(&error)).await,
            };
            report.jobs_purged = job_result.rows_affected();
        }
        tx.commit().await.map_err(|e| map_db(&e))?;
        Ok(report)
    }
}
