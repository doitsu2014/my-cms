//! `FsNamespaceLock`: exclusive namespace ownership through `<root>.lock` (FN-24).
//!
//! The lock file is a sibling of the root, so inventory never sees it. Exclusion is the
//! operating system's advisory file lock (`std::fs::File::try_lock`, RK-06): it holds
//! across processes and is released by the OS when the holder ends. The lock path is never
//! followed through a link: the file is created with `create_new`, and an existing entry
//! must be a regular, non-link file both before and after it is opened.

use std::ffi::OsString;
use std::fs::{File, Metadata, OpenOptions, TryLockError};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::ports::namespace_lock::{LockError, LockGuard, NamespaceLock};

#[derive(Debug, Clone, Copy)]
pub struct FsNamespaceLock;

impl FsNamespaceLock {
    /// The lock file: a sibling of the root (`<root>.lock`), outside the namespace. The
    /// root's whole final component is kept (`store.v1` -> `store.v1.lock`) and a trailing
    /// separator is ignored.
    pub fn lock_path(root: &Path) -> PathBuf {
        match (root.parent(), root.file_name()) {
            (Some(parent), Some(name)) => {
                let mut leaf: OsString = name.to_os_string();
                leaf.push(".lock");
                parent.join(leaf)
            }
            _ => {
                let mut whole = root.as_os_str().to_os_string();
                whole.push(".lock");
                PathBuf::from(whole)
            }
        }
    }
}

/// True for a symlink, Windows junction or any reparse point.
pub(crate) fn is_link(meta: &Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if meta.file_attributes() & 0x400 != 0 {
            return true;
        }
    }
    meta.file_type().is_symlink()
}

fn is_plain_file(meta: &Metadata) -> bool {
    meta.is_file() && !is_link(meta)
}

#[cfg(unix)]
fn same_file(a: &Metadata, b: &Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    a.dev() == b.dev() && a.ino() == b.ino()
}

#[cfg(not(unix))]
fn same_file(_a: &Metadata, _b: &Metadata) -> bool {
    true
}

fn open_new(path: &Path) -> std::io::Result<File> {
    let mut o = OpenOptions::new();
    o.read(true).write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.mode(0o600);
    }
    o.open(path)
}

/// Open the lock file without following a link at its path.
fn open_lock_file(path: &Path) -> Result<File, LockError> {
    match open_new(path) {
        Ok(f) => return Ok(f),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {}
        Err(_) => return Err(LockError::Unavailable),
    }
    // Something exists at the lock path (possibly a link, even a dangling one).
    let before = std::fs::symlink_metadata(path).map_err(|_| LockError::Unavailable)?;
    if !is_plain_file(&before) {
        return Err(LockError::Unavailable);
    }
    let mut o = OpenOptions::new();
    o.read(true).write(true);
    #[cfg(windows)]
    {
        // Open the link itself, never its target; the metadata check below then rejects it.
        use std::os::windows::fs::OpenOptionsExt;
        o.custom_flags(0x0020_0000); // FILE_FLAG_OPEN_REPARSE_POINT
    }
    let file = o.open(path).map_err(|_| LockError::Unavailable)?;
    // Defend against a swap between the check and the open: the opened file must be the
    // regular file still at the path.
    let opened = file.metadata().map_err(|_| LockError::Unavailable)?;
    let after = std::fs::symlink_metadata(path).map_err(|_| LockError::Unavailable)?;
    if !is_plain_file(&opened) || !is_plain_file(&after) || !same_file(&opened, &after) {
        return Err(LockError::Unavailable);
    }
    Ok(file)
}

impl NamespaceLock for FsNamespaceLock {
    fn try_acquire(root: &Path) -> Result<LockGuard, LockError> {
        if !root.is_absolute() || root.file_name().is_none() || root.parent().is_none() {
            return Err(LockError::InvalidRoot);
        }
        let file = open_lock_file(&Self::lock_path(root))?;
        match file.try_lock() {
            Ok(()) => Ok(LockGuard { _file: Some(file) }),
            Err(TryLockError::WouldBlock) => Err(LockError::AlreadyHeld),
            Err(TryLockError::Error(_)) => Err(LockError::Unavailable),
        }
    }
}
