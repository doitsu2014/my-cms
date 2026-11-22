//! `FixtureStorage` port (FN-22; IFC section 9, FD 4.4). Opaque-key file storage under a
//! private root. WU-04 compile shell: types and permissive stubs, no behaviour yet.

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use super::allocation_store::AllocationState;

/// Code constant for `FixtureStorage::inventory` callers (IFC section 9, T-13).
pub const MAX_INVENTORY_ENTRIES: usize = 10_000;

/// `alloc/<lowercase uuid>.bin`, constructed only from an allocation id (T-11).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StorageKey(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("not an opaque storage key")]
pub struct InvalidStorageKey;

impl StorageKey {
    pub fn from_allocation_id(id: Uuid) -> Self {
        Self(format!("alloc/{id}.bin"))
    }

    /// Accepts only the canonical form `alloc/<lowercase hyphenated uuid>.bin`.
    pub fn parse(raw: &str) -> Result<Self, InvalidStorageKey> {
        Ok(Self(raw.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The allocation id the key was derived from.
    pub fn allocation_id(&self) -> Uuid {
        Uuid::nil()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteOutcome {
    Removed,
    AlreadyAbsent,
}

/// What `inventory` saw on disk, never followed through a link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    RegularFile,
    Directory,
    /// Symlink, Windows junction or any reparse point.
    Link,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryEntry {
    /// File name inside `alloc/` (lossy for non-UTF-8), never a path.
    pub name: String,
    pub kind: EntryKind,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryPage {
    pub entries: Vec<InventoryEntry>,
    pub truncated: bool,
}

/// FD 4.4 classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ownership {
    Owned,
    OwnedInCleanup,
    /// Valid opaque name, regular file, no ledger row.
    Orphan,
    /// Never deleted, raises an ownership incident.
    Ambiguous,
}

/// FD 4.4 table for one inventory entry; `ledger_state` is the state of the row whose
/// id is the UUID in the name, if any.
pub fn classify_entry(entry: &InventoryEntry, ledger_state: Option<AllocationState>) -> Ownership {
    let _ = (entry, ledger_state);
    Ownership::Owned
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum StorageError {
    #[error("no registered allocation for this key")]
    NotRegistered,
    #[error("write exceeds the reserved bytes")]
    ExceedsReservation,
    #[error("file already exists")]
    AlreadyExists,
    /// Link, reparse point, directory or other non-regular entry on the path.
    #[error("ambiguous namespace entry")]
    Ambiguous,
    #[error("ledger unavailable")]
    LedgerUnavailable,
    #[error("storage unavailable")]
    Unavailable,
}

#[async_trait]
pub trait FixtureStorage: Send + Sync {
    /// Create-new write of `bytes` under the opaque key; refused without a registered
    /// allocation (register before write).
    async fn write(&self, key: &StorageKey, bytes: &[u8]) -> Result<(), StorageError>;
    async fn delete(&self, key: &StorageKey) -> Result<DeleteOutcome, StorageError>;
    /// Bounded: at most `max_entries` entries; `truncated = true` if more existed.
    async fn inventory(&self, max_entries: usize) -> Result<InventoryPage, StorageError>;
}
