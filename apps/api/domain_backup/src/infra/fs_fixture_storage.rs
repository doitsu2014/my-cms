//! `FsFixtureStorage`: file-system adapter for `FixtureStorage` (FN-22).
//!
//! Only opaque keys reach the disk (`StorageKey` is the sole path input). No component is
//! ever followed through a link: the root, `alloc/` and the key path are inspected with
//! `symlink_metadata` (reparse points and junctions count as links), files are created with
//! `create_new`, and `delete` unlinks only a regular file whose directory entry name equals
//! the key name exactly (found by listing, so a case-folded forged twin is never removed).
//! Directories are 0700 and files 0600 on Unix. Errors carry no path text.
//!
//! The adapter does blocking `std::fs` calls inside `async fn` (no async runtime is a
//! runtime dependency of this crate).

use async_trait::async_trait;
use std::ffi::OsStr;
use std::fmt;
use std::fs::{Metadata, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::fs_namespace_lock::is_link;
use crate::ports::allocation_store::{AllocationError, AllocationState, AllocationStore};
use crate::ports::fixture_storage::{
    DeleteOutcome, EntryKind, FixtureStorage, InventoryEntry, InventoryPage, StorageError,
    StorageKey,
};

const ALLOC_DIR: &str = "alloc";

pub struct FsFixtureStorage {
    root: PathBuf,
    ledger: Arc<dyn AllocationStore>,
}

impl fmt::Debug for FsFixtureStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FsFixtureStorage").finish_non_exhaustive()
    }
}

/// Result of inspecting a directory path without following a link.
enum DirState {
    Missing,
    Ready,
}

/// `Ambiguous` for a link or a non-directory, `Unavailable` for a Unix mode that others
/// can use or any other failure; `Missing` when nothing is there.
fn inspect_dir(path: &Path) -> Result<DirState, StorageError> {
    match std::fs::symlink_metadata(path) {
        Ok(meta) => {
            if is_link(&meta) || !meta.is_dir() {
                return Err(StorageError::Ambiguous);
            }
            if !is_private_dir(&meta) {
                return Err(StorageError::Unavailable);
            }
            Ok(DirState::Ready)
        }
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(DirState::Missing),
        Err(_) => Err(StorageError::Unavailable),
    }
}

#[cfg(unix)]
fn is_private_dir(meta: &Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    meta.permissions().mode() & 0o077 == 0
}

#[cfg(not(unix))]
fn is_private_dir(_meta: &Metadata) -> bool {
    true
}

fn create_private_dir(path: &Path, recursive: bool) -> Result<(), StorageError> {
    let mut b = std::fs::DirBuilder::new();
    b.recursive(recursive);
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        b.mode(0o700);
    }
    match b.create(path) {
        Ok(()) => Ok(()),
        // Lost a creation race: the re-inspection that follows decides.
        Err(e) if e.kind() == ErrorKind::AlreadyExists => Ok(()),
        Err(_) => Err(StorageError::Unavailable),
    }
}

/// Make sure `path` is a private, non-link directory, creating it when absent.
fn ensure_dir(path: &Path, recursive: bool) -> Result<(), StorageError> {
    if let DirState::Missing = inspect_dir(path)? {
        create_private_dir(path, recursive)?;
        // Re-verify what is now there: it must be ours, not a link planted meanwhile.
        match inspect_dir(path)? {
            DirState::Ready => {}
            DirState::Missing => return Err(StorageError::Unavailable),
        }
    }
    Ok(())
}

impl FsFixtureStorage {
    /// Prepare `<root>` and `<root>/alloc` (private on Unix) and refuse a relative root,
    /// a non-directory, or a link or reparse point at the root or at `alloc`.
    pub fn open(root: &Path, ledger: Arc<dyn AllocationStore>) -> Result<Self, StorageError> {
        if !root.is_absolute() {
            return Err(StorageError::Unavailable);
        }
        ensure_dir(root, true)?;
        ensure_dir(&root.join(ALLOC_DIR), false)?;
        Ok(Self {
            root: root.to_path_buf(),
            ledger,
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn alloc_dir(&self) -> PathBuf {
        self.root.join(ALLOC_DIR)
    }

    /// Re-verify at use time that the root and `alloc/` are still plain directories.
    /// `Ok(false)` when `alloc/` is gone.
    fn check_dirs(&self) -> Result<bool, StorageError> {
        match inspect_dir(&self.root)? {
            DirState::Missing => return Ok(false),
            DirState::Ready => {}
        }
        Ok(matches!(inspect_dir(&self.alloc_dir())?, DirState::Ready))
    }
}

/// The file name inside `alloc/` for a key (`<uuid>.bin`).
fn key_name(key: &StorageKey) -> &str {
    key.as_str()
        .strip_prefix("alloc/")
        .unwrap_or_else(|| key.as_str())
}

fn kind_of(meta: &Metadata) -> EntryKind {
    if is_link(meta) {
        EntryKind::Link
    } else if meta.is_file() {
        EntryKind::RegularFile
    } else if meta.is_dir() {
        EntryKind::Directory
    } else {
        EntryKind::Other
    }
}

fn ledger_error(e: AllocationError) -> StorageError {
    match e {
        AllocationError::StoreUnavailable
        | AllocationError::AuditUnavailable
        | AllocationError::Persistence
        | AllocationError::BudgetExceeded(_)
        | AllocationError::Lost => StorageError::LedgerUnavailable,
    }
}

fn create_new_file(path: &Path) -> std::io::Result<std::fs::File> {
    let mut o = OpenOptions::new();
    o.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    o.open(path)
}

#[async_trait]
impl FixtureStorage for FsFixtureStorage {
    async fn write(&self, key: &StorageKey, bytes: &[u8]) -> Result<(), StorageError> {
        // Register before write: the ledger row must exist and still be `registered`.
        let row = self
            .ledger
            .get(key.allocation_id())
            .await
            .map_err(ledger_error)?
            .ok_or(StorageError::NotRegistered)?;
        if row.state != AllocationState::Registered || row.storage_key != *key {
            return Err(StorageError::NotRegistered);
        }
        let len = u64::try_from(bytes.len()).map_err(|_| StorageError::ExceedsReservation)?;
        if len > row.bytes_reserved {
            return Err(StorageError::ExceedsReservation);
        }
        if !self.check_dirs()? {
            return Err(StorageError::Unavailable);
        }
        let path = self.alloc_dir().join(key_name(key));
        match std::fs::symlink_metadata(&path) {
            Ok(meta) => {
                return Err(if kind_of(&meta) == EntryKind::RegularFile {
                    StorageError::AlreadyExists
                } else {
                    StorageError::Ambiguous
                });
            }
            Err(e) if e.kind() == ErrorKind::NotFound => {}
            Err(_) => return Err(StorageError::Unavailable),
        }
        let mut file = match create_new_file(&path) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::AlreadyExists => {
                return Err(StorageError::AlreadyExists)
            }
            Err(_) => return Err(StorageError::Unavailable),
        };
        let written = file.write_all(bytes).and_then(|()| file.sync_all());
        drop(file);
        if written.is_err() {
            // Remove only the file this call created; never leave a partial fixture.
            let _ = std::fs::remove_file(&path);
            return Err(StorageError::Unavailable);
        }
        Ok(())
    }

    async fn delete(&self, key: &StorageKey) -> Result<DeleteOutcome, StorageError> {
        // Only a missing file counts as already absent (IFC 6.1 step 6). A missing root or
        // `alloc/` means the namespace is gone, so fail closed.
        if !self.check_dirs()? {
            return Err(StorageError::Unavailable);
        }
        let want = OsStr::new(key_name(key));
        let entries = std::fs::read_dir(self.alloc_dir()).map_err(|_| StorageError::Unavailable)?;
        for entry in entries {
            let entry = entry.map_err(|_| StorageError::Unavailable)?;
            // Exact on-disk name only: a case-folded twin is a different entry.
            if entry.file_name() != want {
                continue;
            }
            let path = entry.path();
            let meta = match std::fs::symlink_metadata(&path) {
                Ok(m) => m,
                Err(e) if e.kind() == ErrorKind::NotFound => {
                    return Ok(DeleteOutcome::AlreadyAbsent)
                }
                Err(_) => return Err(StorageError::Unavailable),
            };
            if kind_of(&meta) != EntryKind::RegularFile {
                return Err(StorageError::Ambiguous);
            }
            return match std::fs::remove_file(&path) {
                Ok(()) => Ok(DeleteOutcome::Removed),
                Err(e) if e.kind() == ErrorKind::NotFound => Ok(DeleteOutcome::AlreadyAbsent),
                Err(_) => Err(StorageError::Unavailable),
            };
        }
        Ok(DeleteOutcome::AlreadyAbsent)
    }

    async fn inventory(&self, max_entries: usize) -> Result<InventoryPage, StorageError> {
        if !self.check_dirs()? {
            return Err(StorageError::Unavailable);
        }
        let read = std::fs::read_dir(self.alloc_dir()).map_err(|_| StorageError::Unavailable)?;
        let mut entries = Vec::new();
        let mut truncated = false;
        // Stop after max_entries + 1 entries: the extra one only proves truncation.
        for entry in read {
            let entry = entry.map_err(|_| StorageError::Unavailable)?;
            if entries.len() == max_entries {
                truncated = true;
                break;
            }
            let meta = match std::fs::symlink_metadata(entry.path()) {
                Ok(m) => m,
                // Removed since it was listed: not part of the namespace any more.
                Err(e) if e.kind() == ErrorKind::NotFound => continue,
                Err(_) => return Err(StorageError::Unavailable),
            };
            let kind = kind_of(&meta);
            entries.push(InventoryEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                kind,
                size_bytes: if kind == EntryKind::RegularFile {
                    meta.len()
                } else {
                    0
                },
            });
        }
        Ok(InventoryPage { entries, truncated })
    }
}
