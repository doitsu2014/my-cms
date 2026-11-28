//! Acceptance of `POST /backup/jobs` (FN-07, FN-08): replay or conflict first, then
//! the concurrency check, then one transaction holding the `queued` job, the
//! idempotency row and the `job.accepted` audit row.

use std::sync::Arc;

use chrono::Duration;
use sea_orm::{DatabaseConnection, TransactionTrait};
use uuid::Uuid;

use super::{CreateJobError, CreateRequest};
use crate::domain::clock::Clock;
use crate::domain::idempotency::{key_digest, request_fingerprint, OPERATION};
use crate::ports::audit::{
    AuditError, AuditEvent, AuditEventKind, AuditOutcome, AuditSink, Digest,
};
use crate::ports::job_store::{AcceptResult, IdemRecord, JobRow, JobStore, NewJob, StoreError};

/// The authenticated caller and the request correlation id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptContext {
    pub actor_sub: String,
    pub correlation_id: String,
}

/// `Created` is answered 202; `Replayed` is answered 200 with `Idempotent-Replay: true`.
#[derive(Debug, Clone, PartialEq)]
pub enum Accepted {
    Created(JobRow),
    Replayed(JobRow),
}

pub struct CreateJobHandler {
    conn: DatabaseConnection,
    store: Arc<dyn JobStore>,
    audit: Arc<dyn AuditSink>,
    clock: Arc<dyn Clock>,
    retention: Duration,
    epoch: i64,
}

impl std::fmt::Debug for CreateJobHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateJobHandler").finish_non_exhaustive()
    }
}

/// Why one acceptance attempt did not create a job.
enum AttemptFailure {
    /// A unique violation: the active-job partial index or the idempotency UNIQUE.
    /// The caller re-reads the idempotency row to choose replay, reuse or the limit.
    Conflict,
    Refused(CreateJobError),
}

impl From<CreateJobError> for AttemptFailure {
    fn from(e: CreateJobError) -> Self {
        AttemptFailure::Refused(e)
    }
}

/// A store failure that is not a conflict. A reachable database that rejected the audit
/// statement is `AUDIT_UNAVAILABLE`; everything else, including a persistence error the
/// caller cannot act on, is the outage path `STORE_UNAVAILABLE` (FD 6.6).
fn refusal(e: StoreError) -> CreateJobError {
    match e {
        StoreError::AuditUnavailable => CreateJobError::AuditUnavailable,
        _ => CreateJobError::StoreUnavailable,
    }
}

fn audit_refusal(e: AuditError) -> CreateJobError {
    match e {
        AuditError::Rejected => CreateJobError::AuditUnavailable,
        AuditError::Unavailable => CreateJobError::StoreUnavailable,
    }
}

impl CreateJobHandler {
    /// `retention_days` is CFG-14: it bounds both the job metadata and the idempotency
    /// validity window. `epoch` is the lifecycle epoch the new job is stamped with.
    pub fn new(
        conn: DatabaseConnection,
        store: Arc<dyn JobStore>,
        audit: Arc<dyn AuditSink>,
        clock: Arc<dyn Clock>,
        retention_days: u32,
        epoch: i64,
    ) -> Self {
        Self {
            conn,
            store,
            audit,
            clock,
            retention: Duration::days(i64::from(retention_days)),
            epoch,
        }
    }

    pub async fn create(
        &self,
        ctx: &AcceptContext,
        req: CreateRequest,
    ) -> Result<Accepted, CreateJobError> {
        let digest = key_digest(&ctx.actor_sub, OPERATION, &req.idempotency_key);
        let fingerprint = request_fingerprint(OPERATION, &req.instance_id);

        // Replay and conflict are decided before the concurrency check, so a replay of an
        // accepted job is never answered CONCURRENCY_LIMIT (IFC section 4).
        if let Some(found) = self.replay_or_reuse(ctx, &digest, &fingerprint).await? {
            return Ok(found);
        }
        match self.accept_new(ctx, &req, &digest, &fingerprint).await {
            Ok(job) => Ok(Accepted::Created(job)),
            Err(AttemptFailure::Refused(e)) => Err(e),
            // On any unique violation the transaction was rolled back; re-read the
            // idempotency row in a fresh read. Only when no row exists is it the limit.
            Err(AttemptFailure::Conflict) => {
                match self.replay_or_reuse(ctx, &digest, &fingerprint).await? {
                    Some(found) => Ok(found),
                    None => Err(CreateJobError::ConcurrencyLimit),
                }
            }
        }
    }

    /// The unexpired idempotency row for this key, if any: the same fingerprint replays
    /// the accepted job, a different one is `IDEMPOTENCY_KEY_REUSED`.
    async fn replay_or_reuse(
        &self,
        ctx: &AcceptContext,
        digest: &str,
        fingerprint: &str,
    ) -> Result<Option<Accepted>, CreateJobError> {
        let found = self
            .store
            .find_idempotency(&ctx.actor_sub, OPERATION, digest, self.clock.now())
            .await
            .map_err(refusal)?;
        let Some(found) = found else {
            return Ok(None);
        };
        if found.request_fingerprint != fingerprint {
            return Err(CreateJobError::IdempotencyKeyReused);
        }
        let job = self
            .store
            .get(found.job_id)
            .await
            .map_err(refusal)?
            .ok_or(CreateJobError::StoreUnavailable)?;
        Ok(Some(Accepted::Replayed(job)))
    }

    /// One transaction: drop an expired unpurged row for the key, insert the `queued` job
    /// and its idempotency row, write `job.accepted`, commit. Nothing is accepted until the
    /// commit succeeds.
    async fn accept_new(
        &self,
        ctx: &AcceptContext,
        req: &CreateRequest,
        digest: &str,
        fingerprint: &str,
    ) -> Result<JobRow, AttemptFailure> {
        let now = self.clock.now();
        let tx = self
            .conn
            .begin()
            .await
            .map_err(|_| CreateJobError::StoreUnavailable)?;

        let outcome: Result<JobRow, AttemptFailure> = async {
            self.store
                .delete_expired_idempotency(&tx, &ctx.actor_sub, OPERATION, digest, now)
                .await
                .map_err(refusal)?;
            let job_id = Uuid::new_v4();
            let new = NewJob {
                id: job_id,
                actor_sub: ctx.actor_sub.clone(),
                instance_id: req.instance_id.clone(),
                correlation_id: ctx.correlation_id.clone(),
                epoch: self.epoch,
                created_at: now,
                retain_until: now + self.retention,
            };
            let idem = IdemRecord {
                id: Uuid::new_v4(),
                actor_sub: ctx.actor_sub.clone(),
                operation: OPERATION.to_owned(),
                key_digest: digest.to_owned(),
                request_fingerprint: fingerprint.to_owned(),
                expires_at: now + self.retention,
            };
            let AcceptResult::Created(job) =
                self.store
                    .accept(&tx, new, idem)
                    .await
                    .map_err(|e| match e {
                        StoreError::ConcurrencyConflict => AttemptFailure::Conflict,
                        other => AttemptFailure::Refused(refusal(other)),
                    })?;
            self.audit
                .record(
                    Some(&tx),
                    AuditEvent {
                        occurred_at: now,
                        actor_sub: Some(ctx.actor_sub.clone()),
                        instance_id: job.instance_id.clone(),
                        job_id: Some(job.id),
                        allocation_id: None,
                        operation: OPERATION.to_owned(),
                        event: AuditEventKind::JobAccepted,
                        outcome: AuditOutcome::Success,
                        reason_code: None,
                        digest: Some(
                            Digest::new(digest).map_err(|_| CreateJobError::StoreUnavailable)?,
                        ),
                        correlation_id: ctx.correlation_id.clone(),
                    },
                )
                .await
                .map_err(audit_refusal)?;
            Ok(job)
        }
        .await;

        match outcome {
            Ok(job) => {
                // A failed commit means nothing was accepted; a retry with the same key
                // resolves (TS-16).
                tx.commit()
                    .await
                    .map_err(|_| CreateJobError::StoreUnavailable)?;
                Ok(job)
            }
            Err(failure) => {
                // The original failure is what the caller needs; a failed rollback changes
                // nothing because the server discards an unfinished transaction.
                let _ = tx.rollback().await;
                Err(failure)
            }
        }
    }
}
