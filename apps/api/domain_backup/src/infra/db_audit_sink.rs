//! `DbAuditSink`: PostgreSQL adapter for `AuditSink` (FN-34).
//!
//! Plain parameterised INSERT only: the `backup_audit` append-only trigger rejects any
//! UPDATE or DELETE, and the sink never issues one.

use async_trait::async_trait;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, Statement,
};
use uuid::Uuid;

use super::pg_error::{classify, PgFailure};
use crate::ports::audit::{AuditError, AuditEvent, AuditSink};

const INSERT_AUDIT: &str = "INSERT INTO backup_audit (id, occurred_at, actor_sub, instance_id, \
     job_id, allocation_id, operation, event, outcome, reason_code, digest, correlation_id) \
     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)";

/// Classify a failed audit statement (FD 6.6): a rejection on a healthy connection is
/// `Rejected`; connection loss, pool exhaustion, statement timeout, and anything that
/// cannot be told apart are `Unavailable` (fail closed).
pub(crate) fn classify_audit_error(err: &DbErr) -> AuditError {
    match classify(err) {
        PgFailure::Database { .. } => AuditError::Rejected,
        PgFailure::Outage | PgFailure::Other => AuditError::Unavailable,
    }
}

#[derive(Debug, Clone)]
pub struct DbAuditSink {
    conn: DatabaseConnection,
}

impl DbAuditSink {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

fn statement(ev: AuditEvent) -> Statement {
    Statement::from_sql_and_values(
        DbBackend::Postgres,
        INSERT_AUDIT,
        vec![
            Uuid::new_v4().into(),
            ev.occurred_at.into(),
            ev.actor_sub.into(),
            ev.instance_id.into(),
            ev.job_id.into(),
            ev.allocation_id.into(),
            ev.operation.into(),
            ev.event.as_str().into(),
            ev.outcome.as_str().into(),
            ev.reason_code.into(),
            ev.digest.map(|d| d.as_str().to_owned()).into(),
            ev.correlation_id.into(),
        ],
    )
}

#[async_trait]
impl AuditSink for DbAuditSink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        let stmt = statement(ev);
        let result = match tx {
            Some(tx) => tx.execute(stmt).await,
            None => self.conn.execute(stmt).await,
        };
        result.map(|_| ()).map_err(|e| classify_audit_error(&e))
    }
}
