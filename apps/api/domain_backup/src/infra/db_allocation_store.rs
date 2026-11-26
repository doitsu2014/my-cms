//! `DbAllocationStore`: PostgreSQL adapter for `AllocationStore` (FN-21, FN-23).
//!
//! Every statement is parameterised. Each ledger write and its audit row share one
//! transaction, so a failed audit rolls the ledger change back (IFC 6.3). Every move is a
//! compare-and-set on `(id, state, state_version)`. The total budget check is serialised
//! across all registering transactions with a transaction-scoped advisory lock, which is
//! released by the commit or rollback that ends the registration.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, DbErr, QueryResult,
    Statement, TransactionTrait, Value,
};
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

use super::pg_error::{classify, PgFailure};
use crate::domain::clock::Clock;
use crate::domain::status::BackupJobStatus;
use crate::ports::allocation_store::{
    Allocation, AllocationError, AllocationLimits, AllocationRecoveryRow, AllocationState,
    AllocationStore, AllocationTotals, BudgetKind, CleanupFailureOutcome,
};
use crate::ports::audit::{AuditError, AuditEvent, AuditEventKind, AuditOutcome, AuditSink};
use crate::ports::fixture_storage::StorageKey;
use crate::ports::job_store::ReasonCode;

const OPERATION: &str = "backup_db_slice";

/// Key of the advisory lock that serialises the total-budget check.
const BUDGET_LOCK_KEY: i64 = 0x4d4b_5f41_4c4c_4f43;

/// Upper bound on the rows one `due_for_cleanup` call returns (the next sweep takes more).
const DUE_BATCH: i64 = 1000;

/// Epoch given to a discovered orphan when `backup_epoch` has no row yet.
const FALLBACK_EPOCH: i64 = 1;

const COLUMNS: &str = "id, job_id, storage_key, state, state_version, epoch, bytes_reserved, \
     expires_at, orphan_grace_until, cleanup_attempts, next_attempt_at, last_reason_code, \
     created_at, updated_at, deleted_at";

const CURRENT_EPOCH: &str = "(SELECT epoch FROM backup_epoch WHERE id = 1)";

pub struct DbAllocationStore {
    conn: DatabaseConnection,
    audit: Arc<dyn AuditSink>,
    clock: Arc<dyn Clock>,
    instance_id: String,
    limits: AllocationLimits,
}

impl fmt::Debug for DbAllocationStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbAllocationStore").finish_non_exhaustive()
    }
}

impl DbAllocationStore {
    pub fn new(
        conn: DatabaseConnection,
        audit: Arc<dyn AuditSink>,
        clock: Arc<dyn Clock>,
        instance_id: impl Into<String>,
        limits: AllocationLimits,
    ) -> Self {
        Self {
            conn,
            audit,
            clock,
            instance_id: instance_id.into(),
            limits,
        }
    }

    async fn begin(&self) -> Result<DatabaseTransaction, AllocationError> {
        self.conn.begin().await.map_err(|e| map_db(&e))
    }

    /// Record one audit row on the caller's transaction.
    async fn audit_in(
        &self,
        tx: &DatabaseTransaction,
        event: AuditEventKind,
        at: DateTime<Utc>,
        job_id: Option<Uuid>,
        allocation_id: Uuid,
        reason_code: Option<String>,
    ) -> Result<(), AllocationError> {
        self.audit
            .record(
                Some(tx),
                AuditEvent {
                    occurred_at: at,
                    actor_sub: None,
                    instance_id: self.instance_id.clone(),
                    job_id,
                    allocation_id: Some(allocation_id),
                    operation: OPERATION.to_owned(),
                    event,
                    outcome: AuditOutcome::Success,
                    reason_code,
                    digest: None,
                    correlation_id: allocation_id.to_string(),
                },
            )
            .await
            .map_err(map_audit)
    }

    /// One compare-and-set `UPDATE` with parameters `$1 = id`, `$2 = version`, `$3 = now`;
    /// no affected row is a lost race or a state the move does not allow.
    async fn simple_move(&self, sql: &str, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        let now = self.clock.now();
        let res = self
            .conn
            .execute(pg(sql, vec![id.into(), ver.into(), now.into()]))
            .await
            .map_err(|e| map_db(&e))?;
        if res.rows_affected() == 1 {
            Ok(())
        } else {
            Err(AllocationError::Lost)
        }
    }
}

fn pg(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Postgres, sql, values)
}

/// Outage or a bare persistence error; no driver text is carried.
pub(crate) fn map_db(err: &DbErr) -> AllocationError {
    match classify(err) {
        PgFailure::Outage => AllocationError::StoreUnavailable,
        _ => AllocationError::Persistence,
    }
}

pub(crate) fn map_audit(err: AuditError) -> AllocationError {
    match err {
        AuditError::Rejected => AllocationError::AuditUnavailable,
        AuditError::Unavailable => AllocationError::StoreUnavailable,
    }
}

async fn rollback(tx: DatabaseTransaction) {
    // The original failure is what the caller needs; a failed rollback changes nothing
    // (an uncommitted transaction never becomes visible).
    let _ = tx.rollback().await;
}

/// Rolls back and passes the error through.
async fn abort<T>(tx: DatabaseTransaction, err: AllocationError) -> Result<T, AllocationError> {
    rollback(tx).await;
    Err(err)
}

fn state_from_wire(s: &str) -> Result<AllocationState, AllocationError> {
    match s {
        "registered" => Ok(AllocationState::Registered),
        "written" => Ok(AllocationState::Written),
        "release_pending" => Ok(AllocationState::ReleasePending),
        "deleting" => Ok(AllocationState::Deleting),
        "deleted" => Ok(AllocationState::Deleted),
        "orphan_discovered" => Ok(AllocationState::OrphanDiscovered),
        "cleanup_failed" => Ok(AllocationState::CleanupFailed),
        _ => Err(AllocationError::Persistence),
    }
}

fn allocation(r: &QueryResult) -> Result<Allocation, AllocationError> {
    let get_err = |_: DbErr| AllocationError::Persistence;
    let state: String = r.try_get("", "state").map_err(get_err)?;
    let key: String = r.try_get("", "storage_key").map_err(get_err)?;
    let bytes: i64 = r.try_get("", "bytes_reserved").map_err(get_err)?;
    Ok(Allocation {
        id: r.try_get("", "id").map_err(get_err)?,
        job_id: r.try_get("", "job_id").map_err(get_err)?,
        storage_key: StorageKey::parse(&key).map_err(|_| AllocationError::Persistence)?,
        state: state_from_wire(&state)?,
        state_version: r.try_get("", "state_version").map_err(get_err)?,
        epoch: r.try_get("", "epoch").map_err(get_err)?,
        bytes_reserved: u64::try_from(bytes).map_err(|_| AllocationError::Persistence)?,
        expires_at: r.try_get("", "expires_at").map_err(get_err)?,
        orphan_grace_until: r.try_get("", "orphan_grace_until").map_err(get_err)?,
        cleanup_attempts: r.try_get("", "cleanup_attempts").map_err(get_err)?,
        next_attempt_at: r.try_get("", "next_attempt_at").map_err(get_err)?,
        last_reason_code: r.try_get("", "last_reason_code").map_err(get_err)?,
        created_at: r.try_get("", "created_at").map_err(get_err)?,
        updated_at: r.try_get("", "updated_at").map_err(get_err)?,
        deleted_at: r.try_get("", "deleted_at").map_err(get_err)?,
    })
}

fn to_chrono(d: std::time::Duration) -> Result<chrono::Duration, AllocationError> {
    chrono::Duration::from_std(d).map_err(|_| AllocationError::Persistence)
}

fn bytes_i64(bytes: u64) -> Result<i64, AllocationError> {
    i64::try_from(bytes).map_err(|_| AllocationError::Persistence)
}

#[async_trait]
impl AllocationStore for DbAllocationStore {
    async fn register(
        &self,
        job: Option<Uuid>,
        bytes: u64,
        epoch: i64,
        expires: DateTime<Utc>,
    ) -> Result<Allocation, AllocationError> {
        if bytes > self.limits.alloc_max_bytes {
            return Err(AllocationError::BudgetExceeded(BudgetKind::PerAllocation));
        }
        let wanted = bytes_i64(bytes)?;
        let id = Uuid::new_v4();
        let now = self.clock.now();
        // READ COMMITTED is required: the SUM below must read a statement snapshot taken
        // after the advisory lock is granted, so it sees the previous holder's insert.
        let tx = self
            .conn
            .begin_with_config(Some(sea_orm::IsolationLevel::ReadCommitted), None)
            .await
            .map_err(|e| map_db(&e))?;

        // Serialise every registration's budget check until this transaction ends.
        if let Err(e) = tx
            .execute(pg(
                "SELECT pg_advisory_xact_lock($1)",
                vec![BUDGET_LOCK_KEY.into()],
            ))
            .await
        {
            return abort(tx, map_db(&e)).await;
        }
        let live = match tx
            .query_one(pg(
                "SELECT COALESCE(SUM(bytes_reserved), 0)::bigint AS n \
                 FROM backup_allocations WHERE state <> 'deleted'",
                vec![],
            ))
            .await
        {
            Ok(Some(row)) => match row.try_get::<i64>("", "n") {
                Ok(n) => n,
                Err(_) => return abort(tx, AllocationError::Persistence).await,
            },
            Ok(None) => return abort(tx, AllocationError::Persistence).await,
            Err(e) => return abort(tx, map_db(&e)).await,
        };
        let total_max = bytes_i64(self.limits.total_max_bytes).unwrap_or(i64::MAX);
        if live.saturating_add(wanted) > total_max {
            return abort(tx, AllocationError::BudgetExceeded(BudgetKind::Total)).await;
        }

        let key = StorageKey::from_allocation_id(id);
        if let Err(e) = tx
            .execute(pg(
                "INSERT INTO backup_allocations (id, job_id, storage_key, state, state_version, \
                 epoch, bytes_reserved, expires_at, cleanup_attempts, created_at, updated_at) \
                 VALUES ($1, $2, $3, 'registered', 0, $4, $5, $6, 0, $7, $7)",
                vec![
                    id.into(),
                    job.into(),
                    key.as_str().to_owned().into(),
                    epoch.into(),
                    wanted.into(),
                    expires.into(),
                    now.into(),
                ],
            ))
            .await
        {
            return abort(tx, map_db(&e)).await;
        }
        if let Err(e) = self
            .audit_in(
                &tx,
                AuditEventKind::AllocationRegistered,
                now,
                job,
                id,
                None,
            )
            .await
        {
            return abort(tx, e).await;
        }
        tx.commit().await.map_err(|e| map_db(&e))?;
        Ok(Allocation {
            id,
            job_id: job,
            storage_key: key,
            state: AllocationState::Registered,
            state_version: 0,
            epoch,
            bytes_reserved: bytes,
            expires_at: expires,
            orphan_grace_until: None,
            cleanup_attempts: 0,
            next_attempt_at: None,
            last_reason_code: None,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        })
    }

    async fn mark_written(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        self.simple_move(
            "UPDATE backup_allocations SET state = 'written', state_version = state_version + 1, \
             updated_at = $3 WHERE id = $1 AND state = 'registered' AND state_version = $2",
            id,
            ver,
        )
        .await
    }

    async fn release(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        self.simple_move(
            "UPDATE backup_allocations SET state = 'release_pending', \
             state_version = state_version + 1, updated_at = $3 \
             WHERE id = $1 AND state IN ('registered', 'written') AND state_version = $2",
            id,
            ver,
        )
        .await
    }

    async fn claim_for_delete(
        &self,
        id: Uuid,
        ver: i64,
    ) -> Result<Option<Allocation>, AllocationError> {
        let now = self.clock.now();
        let tx = self.begin().await?;
        let sql = format!(
            "UPDATE backup_allocations SET state = 'deleting', state_version = state_version + 1, \
             epoch = COALESCE({CURRENT_EPOCH}, epoch), updated_at = $3 \
             WHERE id = $1 AND state_version = $2 AND ( \
               (state = 'release_pending' AND (next_attempt_at IS NULL OR next_attempt_at <= $3)) \
               OR (state = 'orphan_discovered' AND orphan_grace_until IS NOT NULL \
                   AND orphan_grace_until <= $3)) \
             RETURNING {COLUMNS}"
        );
        let row = match tx
            .query_one(pg(&sql, vec![id.into(), ver.into(), now.into()]))
            .await
        {
            Ok(Some(r)) => r,
            Ok(None) => {
                rollback(tx).await;
                return Ok(None);
            }
            Err(e) => return abort(tx, map_db(&e)).await,
        };
        let claimed = match allocation(&row) {
            Ok(a) => a,
            Err(e) => return abort(tx, e).await,
        };
        if let Err(e) = self
            .audit_in(
                &tx,
                AuditEventKind::AllocationDeleteClaimed,
                now,
                claimed.job_id,
                id,
                None,
            )
            .await
        {
            return abort(tx, e).await;
        }
        tx.commit().await.map_err(|e| map_db(&e))?;
        Ok(Some(claimed))
    }

    async fn mark_deleted(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        let now = self.clock.now();
        let tx = self.begin().await?;
        let row = match tx
            .query_one(pg(
                "UPDATE backup_allocations SET state = 'deleted', \
                 state_version = state_version + 1, deleted_at = $3, updated_at = $3 \
                 WHERE id = $1 AND state = 'deleting' AND state_version = $2 RETURNING job_id",
                vec![id.into(), ver.into(), now.into()],
            ))
            .await
        {
            Ok(Some(r)) => r,
            Ok(None) => return abort(tx, AllocationError::Lost).await,
            Err(e) => return abort(tx, map_db(&e)).await,
        };
        let job: Option<Uuid> = match row.try_get("", "job_id") {
            Ok(j) => j,
            Err(_) => return abort(tx, AllocationError::Persistence).await,
        };
        if let Err(e) = self
            .audit_in(&tx, AuditEventKind::AllocationDeleted, now, job, id, None)
            .await
        {
            return abort(tx, e).await;
        }
        tx.commit().await.map_err(|e| map_db(&e))
    }

    async fn record_cleanup_failure(
        &self,
        id: Uuid,
        ver: i64,
        reason: ReasonCode,
        now: DateTime<Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError> {
        let retry_at = now
            .checked_add_signed(to_chrono(self.limits.cleanup_backoff)?)
            .ok_or(AllocationError::Persistence)?;
        let tx = self.begin().await?;
        let row = match tx
            .query_one(pg(
                "UPDATE backup_allocations SET \
                   cleanup_attempts = cleanup_attempts + 1, \
                   state = CASE WHEN cleanup_attempts + 1 >= $5::int \
                                THEN 'cleanup_failed' ELSE 'release_pending' END, \
                   next_attempt_at = CASE WHEN cleanup_attempts + 1 >= $5::int \
                                          THEN NULL ELSE $6::timestamptz END, \
                   last_reason_code = $4, state_version = state_version + 1, updated_at = $3 \
                 WHERE id = $1 AND state = 'deleting' AND state_version = $2 \
                 RETURNING state, job_id",
                vec![
                    id.into(),
                    ver.into(),
                    now.into(),
                    reason.as_str().to_owned().into(),
                    self.limits.cleanup_max_attempts.into(),
                    retry_at.into(),
                ],
            ))
            .await
        {
            Ok(Some(r)) => r,
            Ok(None) => return abort(tx, AllocationError::Lost).await,
            Err(e) => return abort(tx, map_db(&e)).await,
        };
        let state: String = match row.try_get("", "state") {
            Ok(s) => s,
            Err(_) => return abort(tx, AllocationError::Persistence).await,
        };
        if state == "cleanup_failed" {
            let job: Option<Uuid> = match row.try_get("", "job_id") {
                Ok(j) => j,
                Err(_) => return abort(tx, AllocationError::Persistence).await,
            };
            if let Err(e) = self
                .audit_in(
                    &tx,
                    AuditEventKind::CleanupExhausted,
                    now,
                    job,
                    id,
                    Some(reason.as_str().to_owned()),
                )
                .await
            {
                return abort(tx, e).await;
            }
            tx.commit().await.map_err(|e| map_db(&e))?;
            Ok(CleanupFailureOutcome::Exhausted)
        } else {
            tx.commit().await.map_err(|e| map_db(&e))?;
            Ok(CleanupFailureOutcome::Retry)
        }
    }

    async fn insert_orphan(
        &self,
        id: Uuid,
        key: &StorageKey,
        bytes: u64,
        now: DateTime<Utc>,
    ) -> Result<(), AllocationError> {
        if *key != StorageKey::from_allocation_id(id) {
            return Err(AllocationError::Persistence);
        }
        let capped = bytes.min(self.limits.alloc_max_bytes);
        let grace_until = now
            .checked_add_signed(to_chrono(self.limits.orphan_grace)?)
            .ok_or(AllocationError::Persistence)?;
        let tx = self.begin().await?;
        let sql = format!(
            "INSERT INTO backup_allocations (id, job_id, storage_key, state, state_version, \
             epoch, bytes_reserved, expires_at, orphan_grace_until, cleanup_attempts, \
             created_at, updated_at) \
             VALUES ($1, NULL, $2, 'orphan_discovered', 0, COALESCE({CURRENT_EPOCH}, $3), $4, \
             $5, $5, 0, $6, $6) ON CONFLICT (id) DO NOTHING"
        );
        let inserted = match tx
            .execute(pg(
                &sql,
                vec![
                    id.into(),
                    key.as_str().to_owned().into(),
                    FALLBACK_EPOCH.into(),
                    bytes_i64(capped)?.into(),
                    grace_until.into(),
                    now.into(),
                ],
            ))
            .await
        {
            Ok(r) => r.rows_affected(),
            Err(e) => return abort(tx, map_db(&e)).await,
        };
        if inserted == 0 {
            // Already ledger-tracked: keep the first grace, write no second audit row.
            rollback(tx).await;
            return Ok(());
        }
        if let Err(e) = self
            .audit_in(&tx, AuditEventKind::OrphanDiscovered, now, None, id, None)
            .await
        {
            return abort(tx, e).await;
        }
        tx.commit().await.map_err(|e| map_db(&e))
    }

    async fn due_for_cleanup(
        &self,
        now: DateTime<Utc>,
    ) -> Result<Vec<Allocation>, AllocationError> {
        let sql = format!(
            "SELECT {COLUMNS} FROM backup_allocations WHERE \
             (state = 'release_pending' AND (next_attempt_at IS NULL OR next_attempt_at <= $1)) \
             OR (state = 'orphan_discovered' AND orphan_grace_until IS NOT NULL \
                 AND orphan_grace_until <= $1) \
             ORDER BY created_at, id LIMIT $2"
        );
        let rows = self
            .conn
            .query_all(pg(&sql, vec![now.into(), DUE_BATCH.into()]))
            .await
            .map_err(|e| map_db(&e))?;
        rows.iter().map(allocation).collect()
    }

    async fn totals(&self) -> Result<AllocationTotals, AllocationError> {
        let row = self
            .conn
            .query_one(pg(
                "SELECT COALESCE(SUM(bytes_reserved), 0)::bigint AS bytes, \
                 COUNT(*)::bigint AS n FROM backup_allocations WHERE state <> 'deleted'",
                vec![],
            ))
            .await
            .map_err(|e| map_db(&e))?
            .ok_or(AllocationError::Persistence)?;
        let bytes: i64 = row
            .try_get("", "bytes")
            .map_err(|_| AllocationError::Persistence)?;
        let n: i64 = row
            .try_get("", "n")
            .map_err(|_| AllocationError::Persistence)?;
        Ok(AllocationTotals {
            live_bytes: u64::try_from(bytes).map_err(|_| AllocationError::Persistence)?,
            live_count: u64::try_from(n).map_err(|_| AllocationError::Persistence)?,
        })
    }

    async fn get(&self, id: Uuid) -> Result<Option<Allocation>, AllocationError> {
        let sql = format!("SELECT {COLUMNS} FROM backup_allocations WHERE id = $1");
        match self
            .conn
            .query_one(pg(&sql, vec![id.into()]))
            .await
            .map_err(|e| map_db(&e))?
        {
            Some(r) => Ok(Some(allocation(&r)?)),
            None => Ok(None),
        }
    }

    async fn list_for_reconciliation(&self) -> Result<Vec<AllocationRecoveryRow>, AllocationError> {
        let sql = "SELECT a.id, a.job_id, a.storage_key, a.state, a.state_version, a.epoch, \
             a.bytes_reserved, a.expires_at, a.orphan_grace_until, a.cleanup_attempts, \
             a.next_attempt_at, a.last_reason_code, a.created_at, a.updated_at, a.deleted_at, \
             j.status AS owner_status, j.epoch AS owner_epoch \
             FROM backup_allocations a LEFT JOIN backup_jobs j ON j.id = a.job_id \
             ORDER BY a.created_at, a.id";
        let rows = self
            .conn
            .query_all(pg(sql, vec![]))
            .await
            .map_err(|e| map_db(&e))?;
        rows.iter()
            .map(|row| {
                let allocation = allocation(row)?;
                let owner_status: Option<String> = row
                    .try_get("", "owner_status")
                    .map_err(|_| AllocationError::Persistence)?;
                let owner_status = owner_status
                    .map(|status| status_from_wire(&status))
                    .transpose()?;
                let owner_epoch = row
                    .try_get("", "owner_epoch")
                    .map_err(|_| AllocationError::Persistence)?;
                Ok(AllocationRecoveryRow {
                    allocation,
                    owner_status,
                    owner_epoch,
                })
            })
            .collect()
    }

    async fn reconcile_state(
        &self,
        id: Uuid,
        expected: (AllocationState, i64, i64),
        to: AllocationState,
        now: DateTime<Utc>,
    ) -> Result<crate::ports::job_store::TransitionResult, AllocationError> {
        use crate::ports::job_store::TransitionResult;

        let tx = self.begin().await?;
        let row = match tx
            .query_one(pg(
                "UPDATE backup_allocations SET state = $5, state_version = state_version + 1, \
                 updated_at = $6 WHERE id = $1 AND state = $2 AND state_version = $3 \
                 AND epoch = $4 RETURNING job_id",
                vec![
                    id.into(),
                    expected.0.as_str().into(),
                    expected.1.into(),
                    expected.2.into(),
                    to.as_str().into(),
                    now.into(),
                ],
            ))
            .await
        {
            Ok(Some(row)) => row,
            Ok(None) => {
                rollback(tx).await;
                return Ok(TransitionResult::Lost);
            }
            Err(error) => return abort(tx, map_db(&error)).await,
        };
        let job_id: Option<Uuid> = match row.try_get("", "job_id") {
            Ok(job_id) => job_id,
            Err(_) => return abort(tx, AllocationError::Persistence).await,
        };
        let correlation_id = match job_id {
            Some(job_id) => match tx
                .query_one(pg(
                    "SELECT correlation_id FROM backup_jobs WHERE id = $1",
                    vec![job_id.into()],
                ))
                .await
            {
                Ok(Some(owner)) => match owner.try_get::<String>("", "correlation_id") {
                    Ok(correlation_id) => correlation_id,
                    Err(_) => return abort(tx, AllocationError::Persistence).await,
                },
                Ok(None) => id.to_string(),
                Err(error) => return abort(tx, map_db(&error)).await,
            },
            None => id.to_string(),
        };
        if let Err(error) = self
            .audit
            .record(
                Some(&tx),
                AuditEvent {
                    occurred_at: now,
                    actor_sub: None,
                    instance_id: self.instance_id.clone(),
                    job_id,
                    allocation_id: Some(id),
                    operation: "reconciliation".to_owned(),
                    event: AuditEventKind::AllocationReconciled,
                    outcome: AuditOutcome::Success,
                    reason_code: Some("RECONCILED".to_owned()),
                    digest: None,
                    correlation_id,
                },
            )
            .await
        {
            return abort(tx, map_audit(error)).await;
        }
        tx.commit().await.map_err(|error| map_db(&error))?;
        Ok(TransitionResult::Applied)
    }
}

fn status_from_wire(value: &str) -> Result<BackupJobStatus, AllocationError> {
    match value {
        "queued" => Ok(BackupJobStatus::Queued),
        "running" => Ok(BackupJobStatus::Running),
        "succeeded" => Ok(BackupJobStatus::Succeeded),
        "failed" => Ok(BackupJobStatus::Failed),
        "recovery-required" => Ok(BackupJobStatus::RecoveryRequired),
        "incomplete" => Ok(BackupJobStatus::Incomplete),
        _ => Err(AllocationError::Persistence),
    }
}
