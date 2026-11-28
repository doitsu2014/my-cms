//! Cross-table retention purge port (FN-33; IFC section 9).

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::allocation_store::AllocationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PurgeReport {
    pub jobs_purged: u64,
    pub idempotency_rows_purged: u64,
    pub allocations_purged: u64,
}

#[async_trait]
pub trait RetentionStore: Send + Sync {
    async fn purge_retained(&self, now: DateTime<Utc>) -> Result<PurgeReport, AllocationError>;
}
