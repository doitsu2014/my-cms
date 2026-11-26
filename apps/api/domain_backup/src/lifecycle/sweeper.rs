//! WU-06 sweeper shell (FN-20, FN-32..FN-33). No scheduling or purge behavior in red stage.
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SweepReport {
    pub deadlines: usize,
    pub expired: usize,
    pub purged: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SweepError {
    StoreUnavailable,
    AuditUnavailable,
    InventoryTruncated,
}

pub struct Sweeper;
impl Sweeper {
    pub fn new() -> Self {
        Self
    }
    pub async fn sweep(&self, _now: DateTime<Utc>) -> Result<SweepReport, SweepError> {
        Ok(SweepReport::default())
    }
}
