//! WU-06 reconciliation shell (FN-25..FN-28). No reconciliation behavior in red stage.
use crate::ports::allocation_store::AllocationError;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReconcileReport {
    pub fenced_jobs: usize,
    pub discovered_orphans: usize,
    pub missing_files: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconcileError {
    StoreUnavailable,
    AuditUnavailable,
    InventoryTruncated,
    StorageUnavailable,
}

pub struct Reconciler;
impl Reconciler {
    pub fn new() -> Self {
        Self
    }
    pub async fn reconcile(&self, _now: DateTime<Utc>) -> Result<ReconcileReport, ReconcileError> {
        Ok(ReconcileReport {
            fenced_jobs: 0,
            discovered_orphans: 0,
            missing_files: 0,
        })
    }
}
impl From<AllocationError> for ReconcileError {
    fn from(_: AllocationError) -> Self {
        Self::StoreUnavailable
    }
}
