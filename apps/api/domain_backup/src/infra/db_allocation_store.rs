//! `DbAllocationStore`: PostgreSQL adapter for `AllocationStore` (FN-21, FN-23).
//! WU-04 compile shell: permissive stubs, no SQL yet.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::DatabaseConnection;
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::clock::Clock;
use crate::ports::allocation_store::{
    Allocation, AllocationError, AllocationLimits, AllocationState, AllocationStore,
    AllocationTotals, CleanupFailureOutcome,
};
use crate::ports::audit::AuditSink;
use crate::ports::fixture_storage::StorageKey;
use crate::ports::job_store::ReasonCode;

pub struct DbAllocationStore {
    _conn: DatabaseConnection,
    _audit: Arc<dyn AuditSink>,
    _clock: Arc<dyn Clock>,
    _instance_id: String,
    _limits: AllocationLimits,
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
            _conn: conn,
            _audit: audit,
            _clock: clock,
            _instance_id: instance_id.into(),
            _limits: limits,
        }
    }
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
        let id = Uuid::new_v4();
        let now = Utc::now(); // shell: deliberately not the injected clock
        Ok(Allocation {
            id,
            job_id: job,
            storage_key: StorageKey::from_allocation_id(id),
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

    async fn mark_written(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Ok(())
    }

    async fn release(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Ok(())
    }

    async fn claim_for_delete(
        &self,
        _id: Uuid,
        _ver: i64,
    ) -> Result<Option<Allocation>, AllocationError> {
        Ok(None)
    }

    async fn mark_deleted(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Ok(())
    }

    async fn record_cleanup_failure(
        &self,
        _id: Uuid,
        _ver: i64,
        _reason: ReasonCode,
        _now: DateTime<Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError> {
        Ok(CleanupFailureOutcome::Retry)
    }

    async fn insert_orphan(
        &self,
        _id: Uuid,
        _key: &StorageKey,
        _bytes: u64,
        _now: DateTime<Utc>,
    ) -> Result<(), AllocationError> {
        Ok(())
    }

    async fn due_for_cleanup(
        &self,
        _now: DateTime<Utc>,
    ) -> Result<Vec<Allocation>, AllocationError> {
        Ok(Vec::new())
    }

    async fn totals(&self) -> Result<AllocationTotals, AllocationError> {
        Ok(AllocationTotals::default())
    }

    async fn get(&self, _id: Uuid) -> Result<Option<Allocation>, AllocationError> {
        Ok(None)
    }
}
