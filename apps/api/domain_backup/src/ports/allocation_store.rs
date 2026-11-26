//! `AllocationStore` port (FN-21, FN-23; IFC sections 5.4, 6.1 and 9). WU-04 compile
//! shell: types and the trait; behaviour lives in `infra::db_allocation_store`.
//!
//! Deviations from IFC section 9 awaiting conductor sign-off: `AllocationError` instead of
//! `StoreError`; `AllocationStore::get`; `AllocationLimits` and the `DbAllocationStore::new`
//! shape; `StorageKey::parse` and the ledger argument of `FsFixtureStorage::open` (see
//! `ports::fixture_storage` and `infra::fs_fixture_storage`).

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

use super::fixture_storage::StorageKey;
use super::job_store::{ReasonCode, TransitionResult};
use crate::domain::status::BackupJobStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AllocationState {
    Registered,
    Written,
    ReleasePending,
    Deleting,
    Deleted,
    OrphanDiscovered,
    CleanupFailed,
}

impl AllocationState {
    pub fn as_str(self) -> &'static str {
        match self {
            AllocationState::Registered => "registered",
            AllocationState::Written => "written",
            AllocationState::ReleasePending => "release_pending",
            AllocationState::Deleting => "deleting",
            AllocationState::Deleted => "deleted",
            AllocationState::OrphanDiscovered => "orphan_discovered",
            AllocationState::CleanupFailed => "cleanup_failed",
        }
    }
}

/// One `backup_allocations` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allocation {
    pub id: Uuid,
    pub job_id: Option<Uuid>,
    pub storage_key: StorageKey,
    pub state: AllocationState,
    pub state_version: i64,
    pub epoch: i64,
    pub bytes_reserved: u64,
    pub expires_at: DateTime<Utc>,
    pub orphan_grace_until: Option<DateTime<Utc>>,
    pub cleanup_attempts: i32,
    pub next_attempt_at: Option<DateTime<Utc>>,
    pub last_reason_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Allocation plus its optional owning job state, returned by the recovery LEFT JOIN.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationRecoveryRow {
    pub allocation: Allocation,
    pub owner_status: Option<BackupJobStatus>,
    pub owner_epoch: Option<i64>,
}

/// Live = every state other than `deleted` (orphans and `cleanup_failed` included).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AllocationTotals {
    pub live_bytes: u64,
    pub live_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanupFailureOutcome {
    Retry,
    Exhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetKind {
    PerAllocation,
    Total,
}

/// Configured bounds (CFG-06, CFG-08, CFG-09, CFG-12, CFG-13); defaults are the Q-05 values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationLimits {
    pub alloc_max_bytes: u64,
    pub total_max_bytes: u64,
    pub cleanup_max_attempts: i32,
    pub cleanup_backoff: Duration,
    /// Orphan grace (CFG-06): `orphan_grace_until` and `expires_at` of a discovered orphan.
    pub orphan_grace: Duration,
}

impl Default for AllocationLimits {
    fn default() -> Self {
        Self {
            alloc_max_bytes: 256 * 1024 * 1024,
            total_max_bytes: 1024 * 1024 * 1024,
            cleanup_max_attempts: 3,
            cleanup_backoff: Duration::from_secs(60),
            orphan_grace: Duration::from_secs(3600),
        }
    }
}

/// Display strings carry no database detail.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AllocationError {
    #[error("allocation budget exceeded")]
    BudgetExceeded(BudgetKind),
    /// Compare-and-set lost, or the row is not in a state the move allows.
    #[error("allocation state changed")]
    Lost,
    #[error("audit unavailable")]
    AuditUnavailable,
    #[error("store unavailable")]
    StoreUnavailable,
    #[error("allocation persistence error")]
    Persistence,
}

#[async_trait]
pub trait AllocationStore: Send + Sync {
    /// Budget check, `registered` row and `allocation.registered` audit in one
    /// transaction (FN-21). Fail-closed when the ledger or audit is unavailable.
    async fn register(
        &self,
        job: Option<Uuid>,
        bytes: u64,
        epoch: i64,
        expires: DateTime<Utc>,
    ) -> Result<Allocation, AllocationError>;
    /// Conditional `registered -> written` on `state_version` (FN-23).
    async fn mark_written(&self, id: Uuid, ver: i64) -> Result<(), AllocationError>;
    /// Conditional `registered|written -> release_pending`.
    async fn release(&self, id: Uuid, ver: i64) -> Result<(), AllocationError>;
    /// Conditional `release_pending|orphan_discovered(after grace) -> deleting`, with
    /// `allocation.delete_claimed` in the same transaction. `None` when not claimed.
    async fn claim_for_delete(
        &self,
        id: Uuid,
        ver: i64,
    ) -> Result<Option<Allocation>, AllocationError>;
    /// Conditional `deleting -> deleted`, with `allocation.deleted` in the same transaction.
    async fn mark_deleted(&self, id: Uuid, ver: i64) -> Result<(), AllocationError>;
    async fn record_cleanup_failure(
        &self,
        id: Uuid,
        ver: i64,
        reason: ReasonCode,
        now: DateTime<Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError>;
    async fn insert_orphan(
        &self,
        id: Uuid,
        key: &StorageKey,
        bytes: u64,
        now: DateTime<Utc>,
    ) -> Result<(), AllocationError>;
    async fn due_for_cleanup(&self, now: DateTime<Utc>)
        -> Result<Vec<Allocation>, AllocationError>;
    async fn totals(&self) -> Result<AllocationTotals, AllocationError>;
    /// Read one row (added to the IFC list so `FsFixtureStorage` can refuse a write
    /// without a registered allocation).
    async fn get(&self, id: Uuid) -> Result<Option<Allocation>, AllocationError>;

    /// Complete recovery inventory joined to optional job owner data. LEFT JOIN semantics
    /// preserve allocations with NULL or missing owners; epoch is returned as data.
    async fn list_for_reconciliation(&self) -> Result<Vec<AllocationRecoveryRow>, AllocationError> {
        Ok(Vec::new())
    }

    /// CAS state transition on allocation id, state, version and epoch. Applied changes
    /// and the corresponding allocation audit event are one transaction.
    async fn reconcile_state(
        &self,
        _id: Uuid,
        _expected: (AllocationState, i64, i64),
        _to: AllocationState,
        _now: DateTime<Utc>,
    ) -> Result<TransitionResult, AllocationError> {
        Ok(TransitionResult::Lost)
    }
}
