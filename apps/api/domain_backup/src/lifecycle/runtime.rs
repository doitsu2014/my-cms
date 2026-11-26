//! WU-06 runtime shell (FN-31). Lifecycle behavior is intentionally absent in red stage.
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    NotReady,
    Ready,
    Degraded,
}

#[derive(Debug, Default)]
pub struct Runtime {
    admissions_open: AtomicBool,
}

impl Runtime {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn admissions_open(&self) -> bool {
        self.admissions_open.load(Ordering::SeqCst)
    }
    pub async fn start(&self) -> Result<(), RuntimeError> {
        Ok(())
    }
    pub async fn database_outage(&self) {}
    pub async fn database_recovered(&self) -> Result<(), RuntimeError> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeError {
    LockUnavailable,
    StoreUnavailable,
    AuditUnavailable,
    InventoryTruncated,
}
