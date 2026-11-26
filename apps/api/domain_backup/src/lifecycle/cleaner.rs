//! WU-06 cleaner shell (FN-29..FN-30). No cleanup behavior in red stage.
use crate::ports::allocation_store::AllocationError;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanOutcome {
    Deleted,
    RetryScheduled,
    Exhausted,
    NotDue,
    Lost,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanError {
    StoreUnavailable,
    AuditUnavailable,
    StorageUnavailable,
}

pub struct Cleaner;
impl Cleaner {
    pub fn new() -> Self {
        Self
    }
    pub async fn clean_due(&self, _now: DateTime<Utc>) -> Result<Vec<CleanOutcome>, CleanError> {
        Ok(Vec::new())
    }
    pub async fn clean_one(
        &self,
        _id: uuid::Uuid,
        _now: DateTime<Utc>,
    ) -> Result<CleanOutcome, CleanError> {
        Ok(CleanOutcome::NotDue)
    }
}
impl From<AllocationError> for CleanError {
    fn from(_: AllocationError) -> Self {
        Self::StoreUnavailable
    }
}
