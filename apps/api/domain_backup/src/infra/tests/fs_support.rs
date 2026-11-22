//! File-system fixtures for the WU-04 storage and lock tests: unique temp dirs with
//! link-safe cleanup, directory links (Windows junctions, Unix symlinks) and an
//! in-memory ledger that answers only `get`.

use crate::ports::allocation_store::{
    Allocation, AllocationError, AllocationState, AllocationStore, AllocationTotals,
    CleanupFailureOutcome,
};
use crate::ports::fixture_storage::StorageKey;
use crate::ports::job_store::ReasonCode;
use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// True for a symlink, Windows junction or any reparse point. Never follows the link.
pub(super) fn is_link(meta: &std::fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if meta.file_attributes() & 0x400 != 0 {
            return true;
        }
    }
    meta.file_type().is_symlink()
}

/// Removes a tree without ever following a link: a link is removed as a link, a
/// directory is emptied first, so a junction can never lead the cleanup outside.
fn remove_tree(path: &Path) {
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return;
    };
    if is_link(&meta) {
        #[cfg(windows)]
        {
            let _ = std::fs::remove_dir(path).or_else(|_| std::fs::remove_file(path));
        }
        #[cfg(unix)]
        {
            let _ = std::fs::remove_file(path);
        }
    } else if meta.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                remove_tree(&entry.path());
            }
        }
        let _ = std::fs::remove_dir(path);
    } else {
        let _ = std::fs::remove_file(path);
    }
}

/// Unique directory under the OS temp dir, removed (links first) on drop. A lock file
/// at `<root>.lock` lives inside it and goes with it.
#[derive(Debug)]
pub(super) struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "makara-wu04-{tag}-{}-{}",
            std::process::id(),
            Uuid::new_v4().simple()
        ));
        std::fs::create_dir_all(&path).expect("create temp dir");
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// A not-yet-existing storage root inside the temp dir.
    pub fn root(&self) -> PathBuf {
        self.path.join("root")
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        remove_tree(&self.path);
    }
}

/// Create a directory link `link -> target` (junction on Windows via `mklink /J`,
/// symlink on Unix). `target` must be an existing directory.
pub(super) fn make_dir_link(link: &Path, target: &Path) {
    #[cfg(windows)]
    {
        let out = std::process::Command::new("cmd")
            .arg("/C")
            .arg("mklink")
            .arg("/J")
            .arg(link)
            .arg(target)
            .output()
            .expect("run cmd mklink");
        assert!(
            out.status.success(),
            "mklink /J failed: {}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(target, link).expect("create symlink");
    }
    let meta = std::fs::symlink_metadata(link).expect("link metadata");
    assert!(is_link(&meta), "test setup: {link:?} must be a link");
}

/// Names of the entries directly inside `dir` (no recursion, links not followed).
pub(super) fn names_in(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("read dir")
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

pub(super) fn t0() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 22, 12, 0, 0).unwrap()
}

pub(super) fn ledger_row(id: Uuid, state: AllocationState, bytes: u64) -> Allocation {
    Allocation {
        id,
        job_id: None,
        storage_key: StorageKey::from_allocation_id(id),
        state,
        state_version: 0,
        epoch: 1,
        bytes_reserved: bytes,
        expires_at: t0() + Duration::hours(24),
        orphan_grace_until: None,
        cleanup_attempts: 0,
        next_attempt_at: None,
        last_reason_code: None,
        created_at: t0(),
        updated_at: t0(),
        deleted_at: None,
    }
}

/// In-memory ledger answering `get` only. The storage tests use it so that the
/// register-before-write rule is exercised without Docker; every other method fails.
#[derive(Debug, Default)]
pub(super) struct FakeLedger {
    rows: Mutex<HashMap<Uuid, Allocation>>,
}

impl FakeLedger {
    pub fn with(rows: Vec<Allocation>) -> Arc<Self> {
        let ledger = Self::default();
        for r in rows {
            ledger.rows.lock().unwrap().insert(r.id, r);
        }
        Arc::new(ledger)
    }

    pub fn empty() -> Arc<Self> {
        Arc::new(Self::default())
    }
}

#[async_trait]
impl AllocationStore for FakeLedger {
    async fn register(
        &self,
        _job: Option<Uuid>,
        _bytes: u64,
        _epoch: i64,
        _expires: DateTime<Utc>,
    ) -> Result<Allocation, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn mark_written(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn release(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn claim_for_delete(
        &self,
        _id: Uuid,
        _ver: i64,
    ) -> Result<Option<Allocation>, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn mark_deleted(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn record_cleanup_failure(
        &self,
        _id: Uuid,
        _ver: i64,
        _reason: ReasonCode,
        _now: DateTime<Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn insert_orphan(
        &self,
        _id: Uuid,
        _key: &StorageKey,
        _bytes: u64,
        _now: DateTime<Utc>,
    ) -> Result<(), AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn due_for_cleanup(
        &self,
        _now: DateTime<Utc>,
    ) -> Result<Vec<Allocation>, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn totals(&self) -> Result<AllocationTotals, AllocationError> {
        Err(AllocationError::Persistence)
    }
    async fn get(&self, id: Uuid) -> Result<Option<Allocation>, AllocationError> {
        Ok(self.rows.lock().unwrap().get(&id).cloned())
    }
}

/// A ledger that is down: `get` reports an outage.
#[derive(Debug)]
pub(super) struct DownLedger;

#[async_trait]
impl AllocationStore for DownLedger {
    async fn register(
        &self,
        _job: Option<Uuid>,
        _bytes: u64,
        _epoch: i64,
        _expires: DateTime<Utc>,
    ) -> Result<Allocation, AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn mark_written(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn release(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn claim_for_delete(
        &self,
        _id: Uuid,
        _ver: i64,
    ) -> Result<Option<Allocation>, AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn mark_deleted(&self, _id: Uuid, _ver: i64) -> Result<(), AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn record_cleanup_failure(
        &self,
        _id: Uuid,
        _ver: i64,
        _reason: ReasonCode,
        _now: DateTime<Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn insert_orphan(
        &self,
        _id: Uuid,
        _key: &StorageKey,
        _bytes: u64,
        _now: DateTime<Utc>,
    ) -> Result<(), AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn due_for_cleanup(
        &self,
        _now: DateTime<Utc>,
    ) -> Result<Vec<Allocation>, AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn totals(&self) -> Result<AllocationTotals, AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
    async fn get(&self, _id: Uuid) -> Result<Option<Allocation>, AllocationError> {
        Err(AllocationError::StoreUnavailable)
    }
}

/// Create a FILE symlink `link -> target` (the target may not exist: a dangling link).
/// Returns false, after recording the reason on stderr, when the host cannot create one
/// (Windows without Developer Mode or the symlink privilege): the caller skips and the
/// skip is visible in the test output. Never follows the link.
pub(super) fn make_file_link(link: &Path, target: &Path) -> bool {
    #[cfg(windows)]
    let made = std::os::windows::fs::symlink_file(target, link);
    #[cfg(unix)]
    let made = std::os::unix::fs::symlink(target, link);
    match made {
        Ok(()) => {
            let meta = std::fs::symlink_metadata(link).expect("link metadata");
            assert!(is_link(&meta), "test setup: {link:?} must be a link");
            true
        }
        Err(e) => {
            eprintln!("SKIPPED_FILE_SYMLINK: cannot create a file symlink on this host: {e}");
            false
        }
    }
}

/// Removes a RELATIVE path (resolved against the process working directory) on drop,
/// link-safely. Used only for a path a faulty implementation might create; a correct
/// one leaves nothing behind and the guard does nothing.
pub(super) struct CwdCleanup(PathBuf);

impl CwdCleanup {
    pub fn new(relative: PathBuf) -> Self {
        assert!(relative.is_relative(), "CwdCleanup takes a relative path");
        Self(relative)
    }
}

impl Drop for CwdCleanup {
    fn drop(&mut self) {
        remove_tree(&self.0);
    }
}

/// A random UUID whose upper-case spelling differs from its lower-case one (it has at
/// least one hex letter), so a case-folded name is a different name on a case-sensitive
/// file system and the same file on a case-insensitive one.
pub(super) fn uuid_with_letters() -> Uuid {
    loop {
        let id = Uuid::new_v4();
        if id.to_string() != id.to_string().to_uppercase() {
            return id;
        }
    }
}
