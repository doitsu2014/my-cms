//! `NamespaceLock` port (FN-24; IFC section 9, FD 4.4). WU-04 compile shell.

use std::path::Path;
use thiserror::Error;

/// Held for the life of the guard; dropping it releases the lock.
#[derive(Debug)]
pub struct LockGuard {
    pub(crate) _file: Option<std::fs::File>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum LockError {
    #[error("storage namespace is owned by another holder")]
    AlreadyHeld,
    #[error("namespace lock unavailable")]
    Unavailable,
    #[error("invalid namespace root")]
    InvalidRoot,
}

pub trait NamespaceLock {
    fn try_acquire(root: &Path) -> Result<LockGuard, LockError>;
}
