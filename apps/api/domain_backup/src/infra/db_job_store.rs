//! `DbJobStore`: PostgreSQL adapter for `JobStore` (FN-15, FN-16).
//!
//! Every statement is parameterised. The claim is one atomic `UPDATE ... WHERE id =
//! (SELECT ... FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING`. Transitions are
//! compare-and-set on `(id, status, state_version)`. The `job.*` audit row is written on
//! the store's own transaction, so an audit failure rolls the change back.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, QueryResult,
    Statement, TransactionTrait, Value,
};
use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use super::pg_error::{classify, PgFailure};
use crate::domain::clock::Clock;
use crate::domain::state_machine::is_allowed;
use crate::domain::status::BackupJobStatus;
use crate::ports::audit::{
    AuditError, AuditEvent, AuditEventKind, AuditOutcome, AuditSink, Digest,
};
use crate::ports::job_store::{
    AcceptResult, Components, IdemMatch, IdemRecord, JobRow, JobStore, NewJob, ReasonCode,
    StoreError, TransitionResult,
};

const OPERATION: &str = "backup_db_slice";

/// Columns returned by every statement that yields a `JobRow`.
const JOB_COLUMNS: &str = "id, actor_sub, instance_id, operation, synthetic, status, \
     state_version, reason_code, components::text AS components, epoch, correlation_id, \
     created_at, updated_at, started_at, finished_at, deadline_at, retain_until";

pub struct DbJobStore {
    conn: DatabaseConnection,
    audit: Arc<dyn AuditSink>,
    clock: Arc<dyn Clock>,
}

impl fmt::Debug for DbJobStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbJobStore").finish_non_exhaustive()
    }
}

impl DbJobStore {
    pub fn new(conn: DatabaseConnection, audit: Arc<dyn AuditSink>, clock: Arc<dyn Clock>) -> Self {
        Self { conn, audit, clock }
    }
}

fn pg(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Postgres, sql, values)
}

/// Store-level mapping of a database error: outage or a bare persistence error.
fn map_db(err: &DbErr) -> StoreError {
    match classify(err) {
        PgFailure::Outage => StoreError::StoreUnavailable,
        _ => StoreError::Persistence,
    }
}

fn map_audit(err: AuditError) -> StoreError {
    match err {
        AuditError::Rejected => StoreError::AuditUnavailable,
        AuditError::Unavailable => StoreError::StoreUnavailable,
    }
}

/// Any unique violation on insert is a concurrency conflict, except a primary-key clash
/// (a duplicate generated id), which is a persistence error (IFC section 4).
fn map_insert(err: &DbErr) -> StoreError {
    match classify(err) {
        PgFailure::Outage => StoreError::StoreUnavailable,
        PgFailure::Database { code, constraint } => {
            let pkey = constraint.as_deref().is_some_and(|c| c.ends_with("_pkey"));
            if code == "23505" && !pkey {
                StoreError::ConcurrencyConflict
            } else {
                StoreError::Persistence
            }
        }
        PgFailure::Other => StoreError::Persistence,
    }
}

fn status_from_wire(s: &str) -> Result<BackupJobStatus, StoreError> {
    match s {
        "queued" => Ok(BackupJobStatus::Queued),
        "running" => Ok(BackupJobStatus::Running),
        "succeeded" => Ok(BackupJobStatus::Succeeded),
        "failed" => Ok(BackupJobStatus::Failed),
        "recovery-required" => Ok(BackupJobStatus::RecoveryRequired),
        "incomplete" => Ok(BackupJobStatus::Incomplete),
        _ => Err(StoreError::Persistence),
    }
}

fn status_to_wire(s: BackupJobStatus) -> &'static str {
    match s {
        BackupJobStatus::Queued => "queued",
        BackupJobStatus::Running => "running",
        BackupJobStatus::Succeeded => "succeeded",
        BackupJobStatus::Failed => "failed",
        BackupJobStatus::RecoveryRequired => "recovery-required",
        BackupJobStatus::Incomplete => "incomplete",
    }
}

fn is_terminal(s: BackupJobStatus) -> bool {
    matches!(s, BackupJobStatus::Failed | BackupJobStatus::Incomplete)
}

fn job_row(r: &QueryResult) -> Result<JobRow, StoreError> {
    let get_err = |_: DbErr| StoreError::Persistence;
    let status: String = r.try_get("", "status").map_err(get_err)?;
    let components: String = r.try_get("", "components").map_err(get_err)?;
    Ok(JobRow {
        id: r.try_get("", "id").map_err(get_err)?,
        actor_sub: r.try_get("", "actor_sub").map_err(get_err)?,
        instance_id: r.try_get("", "instance_id").map_err(get_err)?,
        operation: r.try_get("", "operation").map_err(get_err)?,
        synthetic: r.try_get("", "synthetic").map_err(get_err)?,
        status: status_from_wire(&status)?,
        state_version: r.try_get("", "state_version").map_err(get_err)?,
        reason_code: r.try_get("", "reason_code").map_err(get_err)?,
        components: serde_json::from_str(&components).map_err(|_| StoreError::Persistence)?,
        epoch: r.try_get("", "epoch").map_err(get_err)?,
        correlation_id: r.try_get("", "correlation_id").map_err(get_err)?,
        created_at: r.try_get("", "created_at").map_err(get_err)?,
        updated_at: r.try_get("", "updated_at").map_err(get_err)?,
        started_at: r.try_get("", "started_at").map_err(get_err)?,
        finished_at: r.try_get("", "finished_at").map_err(get_err)?,
        deadline_at: r.try_get("", "deadline_at").map_err(get_err)?,
        retain_until: r.try_get("", "retain_until").map_err(get_err)?,
    })
}

/// The audit event written for a transition to `to`.
fn transition_event(
    from: BackupJobStatus,
    to: BackupJobStatus,
    reason: Option<ReasonCode>,
) -> (AuditEventKind, AuditOutcome) {
    match to {
        BackupJobStatus::Running => (AuditEventKind::JobRunning, AuditOutcome::Success),
        BackupJobStatus::Incomplete => (AuditEventKind::JobIncomplete, AuditOutcome::Failure),
        _ if from == BackupJobStatus::Queued && reason == Some(ReasonCode::DispatchFailed) => {
            (AuditEventKind::DispatchFailed, AuditOutcome::Failure)
        }
        _ => (AuditEventKind::JobFailed, AuditOutcome::Failure),
    }
}

async fn rollback(tx: DatabaseTransaction) {
    // The original failure is what the caller needs; a failed rollback changes nothing
    // because the server discards an unfinished transaction when the connection returns.
    let _ = tx.rollback().await;
}

impl DbJobStore {
    /// Write the `job.*` audit row on `tx` for a job this store just changed. A failure
    /// rolls the whole transaction back and is mapped per FD 6.6.
    #[allow(clippy::too_many_arguments)]
    async fn audit_on(
        &self,
        tx: &DatabaseTransaction,
        job: &JobRow,
        from: BackupJobStatus,
        to: BackupJobStatus,
        reason: Option<ReasonCode>,
        at: DateTime<Utc>,
    ) -> Result<(), StoreError> {
        let (event, outcome) = transition_event(from, to, reason);
        self.audit
            .record(
                Some(tx),
                AuditEvent {
                    occurred_at: at,
                    actor_sub: None,
                    instance_id: job.instance_id.clone(),
                    job_id: Some(job.id),
                    allocation_id: None,
                    operation: OPERATION.to_owned(),
                    event,
                    outcome,
                    reason_code: job.reason_code.clone(),
                    digest: None,
                    correlation_id: job.correlation_id.clone(),
                },
            )
            .await
            .map_err(map_audit)
    }
}

#[async_trait]
impl JobStore for DbJobStore {
    async fn accept(
        &self,
        tx: &DatabaseTransaction,
        new: NewJob,
        idem: IdemRecord,
    ) -> Result<AcceptResult, StoreError> {
        // Only sha-256 hex digests may reach the char(64) columns: a raw key is never stored.
        if Digest::new(&idem.key_digest).is_err() || Digest::new(&idem.request_fingerprint).is_err()
        {
            return Err(StoreError::Persistence);
        }
        let insert_job = format!(
            "INSERT INTO backup_jobs (id, actor_sub, instance_id, operation, status, \
             state_version, synthetic, components, epoch, correlation_id, created_at, \
             updated_at, retain_until) \
             VALUES ($1, $2, $3, $4, 'queued', 0, true, '[]'::jsonb, $5, $6, $7, $7, $8) \
             RETURNING {JOB_COLUMNS}"
        );
        let row = tx
            .query_one(pg(
                &insert_job,
                vec![
                    new.id.into(),
                    new.actor_sub.into(),
                    new.instance_id.into(),
                    OPERATION.into(),
                    new.epoch.into(),
                    new.correlation_id.into(),
                    new.created_at.into(),
                    new.retain_until.into(),
                ],
            ))
            .await
            .map_err(|e| map_insert(&e))?
            .ok_or(StoreError::Persistence)?;
        let job = job_row(&row)?;

        tx.execute(pg(
            "INSERT INTO backup_idempotency (id, actor_sub, operation, key_digest, \
             request_fingerprint, job_id, created_at, expires_at) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            vec![
                idem.id.into(),
                idem.actor_sub.into(),
                idem.operation.into(),
                idem.key_digest.into(),
                idem.request_fingerprint.into(),
                new.id.into(),
                new.created_at.into(),
                idem.expires_at.into(),
            ],
        ))
        .await
        .map_err(|e| map_insert(&e))?;
        Ok(AcceptResult::Created(job))
    }

    async fn find_idempotency(
        &self,
        actor_sub: &str,
        operation: &str,
        key_digest: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<IdemMatch>, StoreError> {
        let row = self
            .conn
            .query_one(pg(
                "SELECT job_id, request_fingerprint FROM backup_idempotency                  WHERE actor_sub = $1 AND operation = $2 AND key_digest = $3                  AND expires_at > $4",
                vec![
                    actor_sub.into(),
                    operation.into(),
                    key_digest.into(),
                    now.into(),
                ],
            ))
            .await
            .map_err(|e| map_db(&e))?;
        row.map(|r| {
            Ok(IdemMatch {
                job_id: r
                    .try_get("", "job_id")
                    .map_err(|_| StoreError::Persistence)?,
                request_fingerprint: r
                    .try_get("", "request_fingerprint")
                    .map_err(|_| StoreError::Persistence)?,
            })
        })
        .transpose()
    }

    async fn delete_expired_idempotency(
        &self,
        tx: &DatabaseTransaction,
        actor_sub: &str,
        operation: &str,
        key_digest: &str,
        now: DateTime<Utc>,
    ) -> Result<(), StoreError> {
        tx.execute(pg(
            "DELETE FROM backup_idempotency              WHERE actor_sub = $1 AND operation = $2 AND key_digest = $3 AND expires_at <= $4",
            vec![
                actor_sub.into(),
                operation.into(),
                key_digest.into(),
                now.into(),
            ],
        ))
        .await
        .map_err(|e| map_db(&e))?;
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Option<JobRow>, StoreError> {
        let sql = format!("SELECT {JOB_COLUMNS} FROM backup_jobs WHERE id = $1");
        let row = self
            .conn
            .query_one(pg(&sql, vec![id.into()]))
            .await
            .map_err(|e| map_db(&e))?;
        row.as_ref().map(job_row).transpose()
    }

    async fn transition(
        &self,
        id: Uuid,
        expect: (BackupJobStatus, i64),
        to: BackupJobStatus,
        reason: Option<ReasonCode>,
        comps: Option<Components>,
    ) -> Result<TransitionResult, StoreError> {
        let (from, version) = expect;
        if !is_allowed(from, to) {
            return Err(StoreError::IllegalTransition { from, to });
        }
        let components = match comps {
            Some(c) => Some(serde_json::to_string(&c).map_err(|_| StoreError::Persistence)?),
            None => None,
        };
        let now = self.clock.now();
        let finished = is_terminal(to).then_some(now);
        let started = (to == BackupJobStatus::Running).then_some(now);

        let tx = self.conn.begin().await.map_err(|e| map_db(&e))?;
        let sql = format!(
            "UPDATE backup_jobs SET status = $1, state_version = state_version + 1, \
             updated_at = $2, finished_at = COALESCE($3, finished_at), \
             started_at = COALESCE(started_at, $4), \
             reason_code = COALESCE($5, reason_code), \
             components = COALESCE($6::jsonb, components) \
             WHERE id = $7 AND status = $8 AND state_version = $9 \
             RETURNING {JOB_COLUMNS}"
        );
        let updated = tx
            .query_one(pg(
                &sql,
                vec![
                    status_to_wire(to).into(),
                    now.into(),
                    finished.into(),
                    started.into(),
                    reason.map(|r| r.as_str().to_owned()).into(),
                    components.into(),
                    id.into(),
                    status_to_wire(from).into(),
                    version.into(),
                ],
            ))
            .await;
        let row = match updated {
            Ok(r) => r,
            Err(e) => {
                rollback(tx).await;
                return Err(map_db(&e));
            }
        };
        let Some(row) = row else {
            rollback(tx).await;
            // No row matched the expectation: lost the CAS, or the job does not exist.
            let exists = self
                .conn
                .query_one(pg(
                    "SELECT 1 AS one FROM backup_jobs WHERE id = $1",
                    vec![id.into()],
                ))
                .await
                .map_err(|e| map_db(&e))?;
            return if exists.is_some() {
                Ok(TransitionResult::Lost)
            } else {
                Err(StoreError::NotFound)
            };
        };
        let job = match job_row(&row) {
            Ok(j) => j,
            Err(e) => {
                rollback(tx).await;
                return Err(e);
            }
        };
        if let Err(e) = self.audit_on(&tx, &job, from, to, reason, now).await {
            rollback(tx).await;
            return Err(e);
        }
        tx.commit().await.map_err(|e| map_db(&e))?;
        Ok(TransitionResult::Applied)
    }

    async fn claim_next(
        &self,
        epoch: i64,
        deadline: Duration,
    ) -> Result<Option<JobRow>, StoreError> {
        let now = self.clock.now();
        let deadline_at = chrono::Duration::from_std(deadline)
            .ok()
            .and_then(|d| now.checked_add_signed(d))
            .ok_or(StoreError::Persistence)?;

        let tx = self.conn.begin().await.map_err(|e| map_db(&e))?;
        let sql = format!(
            "UPDATE backup_jobs SET status = 'running', state_version = state_version + 1, \
             epoch = $1, started_at = $2, deadline_at = $3, updated_at = $2 \
             WHERE status = 'queued' AND id = ( \
                 SELECT id FROM backup_jobs WHERE status = 'queued' \
                 ORDER BY created_at, id FOR UPDATE SKIP LOCKED LIMIT 1) \
             RETURNING {JOB_COLUMNS}"
        );
        let claimed = tx
            .query_one(pg(&sql, vec![epoch.into(), now.into(), deadline_at.into()]))
            .await;
        let row = match claimed {
            Ok(r) => r,
            Err(e) => {
                rollback(tx).await;
                return Err(map_db(&e));
            }
        };
        let Some(row) = row else {
            rollback(tx).await;
            return Ok(None);
        };
        let job = match job_row(&row) {
            Ok(j) => j,
            Err(e) => {
                rollback(tx).await;
                return Err(e);
            }
        };
        if let Err(e) = self
            .audit_on(
                &tx,
                &job,
                BackupJobStatus::Queued,
                BackupJobStatus::Running,
                None,
                now,
            )
            .await
        {
            rollback(tx).await;
            return Err(e);
        }
        tx.commit().await.map_err(|e| map_db(&e))?;
        Ok(Some(job))
    }
}
