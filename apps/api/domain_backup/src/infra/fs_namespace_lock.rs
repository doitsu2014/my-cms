//! `FsNamespaceLock`: exclusive namespace ownership through `<root>.lock` (FN-24).
//! WU-04 compile shell: always succeeds, takes no lock.

use std::path::{Path, PathBuf};

use crate::ports::namespace_lock::{LockError, LockGuard, NamespaceLock};

#[derive(Debug, Clone, Copy)]
pub struct FsNamespaceLock;

impl FsNamespaceLock {
    /// The lock file: a sibling of the root (`<root>.lock`), outside the namespace.
    pub fn lock_path(root: &Path) -> PathBuf {
        root.to_path_buf()
    }
}

impl NamespaceLock for FsNamespaceLock {
    fn try_acquire(_root: &Path) -> Result<LockGuard, LockError> {
        Ok(LockGuard { _file: None })
    }
}
