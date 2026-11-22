//! `FsFixtureStorage`: file-system adapter for `FixtureStorage` (FN-22).
//! WU-04 compile shell: permissive stubs, touches no file.

use async_trait::async_trait;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::ports::allocation_store::AllocationStore;
use crate::ports::fixture_storage::{
    DeleteOutcome, FixtureStorage, InventoryPage, StorageError, StorageKey,
};

pub struct FsFixtureStorage {
    root: PathBuf,
    _ledger: Arc<dyn AllocationStore>,
}

impl fmt::Debug for FsFixtureStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FsFixtureStorage").finish_non_exhaustive()
    }
}

impl FsFixtureStorage {
    /// Prepare `<root>` and `<root>/alloc` (private on Unix) and refuse a relative root,
    /// a non-directory, or a link or reparse point at the root or at `alloc`.
    pub fn open(root: &Path, ledger: Arc<dyn AllocationStore>) -> Result<Self, StorageError> {
        Ok(Self {
            root: root.to_path_buf(),
            _ledger: ledger,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[async_trait]
impl FixtureStorage for FsFixtureStorage {
    async fn write(&self, _key: &StorageKey, _bytes: &[u8]) -> Result<(), StorageError> {
        Ok(())
    }

    async fn delete(&self, _key: &StorageKey) -> Result<DeleteOutcome, StorageError> {
        Ok(DeleteOutcome::AlreadyAbsent)
    }

    async fn inventory(&self, _max_entries: usize) -> Result<InventoryPage, StorageError> {
        Ok(InventoryPage {
            entries: Vec::new(),
            truncated: false,
        })
    }
}
