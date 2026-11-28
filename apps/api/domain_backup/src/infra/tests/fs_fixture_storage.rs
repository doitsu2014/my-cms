//! `FsFixtureStorage` and `StorageKey` red tests (no Docker; a temp dir and an
//! in-memory ledger). TS-19 (path safety, forged names, ambiguous ownership never
//! deleted), TS-31 (private permissions, Unix modes only), TS-56 (bounded inventory),
//! and the register-before-write rule of FN-22 and FN-21. AC-003 S3, AC-002 S2;
//! T-11, T-13, T-14 (partial on Windows).
//!
//! T-14 residual (owner decision of 2026-11-22, WU-01 red evidence section 1): on
//! Windows no owner-only ACL check exists and none is tested here; the Unix mode tests
//! are `cfg(unix)` and run only on a Unix host.
//!
//! Every filesystem fixture lives in a unique temp dir; links are created with
//! `mklink /J` on Windows and symlinks on Unix, and the `TempDir` guard removes links
//! before trees and never follows one.

use super::fs_support::*;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::ports::allocation_store::AllocationState::{self, *};
use crate::ports::fixture_storage::{
    classify_entry, DeleteOutcome, EntryKind, FixtureStorage, InventoryEntry, Ownership,
    StorageError, StorageKey,
};
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

fn alloc_dir(tmp: &TempDir) -> PathBuf {
    tmp.root().join("alloc")
}

fn file_name(id: Uuid) -> String {
    format!("{id}.bin")
}

/// Create `<root>/alloc` if needed and put a regular file in it.
fn plant(tmp: &TempDir, name: &str, bytes: &[u8]) {
    let dir = alloc_dir(tmp);
    std::fs::create_dir_all(&dir).expect("create alloc dir");
    std::fs::write(dir.join(name), bytes).expect("plant file");
}

fn open(
    tmp: &TempDir,
    ledger: Arc<dyn crate::ports::allocation_store::AllocationStore>,
) -> FsFixtureStorage {
    FsFixtureStorage::open(&tmp.root(), ledger).expect("open storage")
}

fn registered(id: Uuid, bytes: u64) -> Arc<FakeLedger> {
    FakeLedger::with(vec![ledger_row(id, Registered, bytes)])
}

fn entry(name: &str, kind: EntryKind) -> InventoryEntry {
    InventoryEntry {
        name: name.to_owned(),
        kind,
        size_bytes: 1,
    }
}

// ---------- StorageKey (T-11, TS-19) ----------

/// TS-19, FN-22, T-11: the key is derived from the allocation id only.
#[test]
fn storage_key_is_derived_from_the_allocation_id_only() {
    let id = Uuid::new_v4();
    let key = StorageKey::from_allocation_id(id);
    assert_eq!(key.as_str(), format!("alloc/{id}.bin"));
    assert_eq!(key.allocation_id(), id);
    assert_eq!(StorageKey::parse(key.as_str()), Ok(key));
}

/// TS-19, FN-22, T-11: traversal, absolute paths, separators, streams and any other
/// non-canonical spelling can never become a key.
#[test]
fn storage_key_parse_refuses_every_non_opaque_name() {
    let id = Uuid::new_v4();
    let upper = id.to_string().to_uppercase();
    let simple = id.simple().to_string();
    let bad = [
        String::new(),
        "alloc/".to_owned(),
        "alloc/x.bin".to_owned(),
        format!("../alloc/{id}.bin"),
        format!("alloc/../{id}.bin"),
        format!("alloc/../../{id}.bin"),
        format!("/alloc/{id}.bin"),
        format!("/etc/{id}.bin"),
        format!("alloc//{id}.bin"),
        format!("alloc\\{id}.bin"),
        format!("alloc/sub/{id}.bin"),
        format!("C:\\alloc\\{id}.bin"),
        format!("C:/alloc/{id}.bin"),
        format!("\\\\?\\C:\\alloc\\{id}.bin"),
        format!("\\\\server\\share\\{id}.bin"),
        format!("{id}.bin"),
        format!("alloc/{id}.bin/"),
        format!("alloc/{id}.bin."),
        format!("alloc/{id}.bin "),
        format!("alloc/{id}.bin:stream"),
        format!("alloc/{id}.bin::$DATA"),
        format!("alloc/{id}.BIN"),
        format!("alloc/{id}.bin.tmp"),
        format!("alloc/{upper}.bin"),
        format!("alloc/{simple}.bin"),
        format!("alloc/{id}.bin\0"),
        format!("alloc/{id}\n.bin"),
        format!("ALLOC/{id}.bin"),
    ];
    for raw in &bad {
        assert!(
            StorageKey::parse(raw).is_err(),
            "must refuse non-opaque key {raw:?}"
        );
    }
}

// ---------- open: root and namespace directory (TS-19, TS-31) ----------

/// FN-22: `open` prepares the private root and the single `alloc/` namespace directory.
#[test]
fn open_creates_root_and_alloc_directories() {
    let tmp = TempDir::new("open-create");
    let _storage = open(&tmp, FakeLedger::empty());
    assert!(tmp.root().is_dir(), "root must exist after open");
    assert!(alloc_dir(&tmp).is_dir(), "alloc/ must exist after open");
}

/// TS-19, T-11: a relative root is refused (CFG-04 is absolute).
#[test]
fn open_refuses_a_relative_root() {
    // Resolves against the working directory, outside any temp dir: unique name, and a
    // link-safe guard removes whatever a faulty implementation would have created.
    let relative = PathBuf::from(format!("relative-makara-wu04-root-{}", Uuid::new_v4()));
    let _cleanup = CwdCleanup::new(relative.clone());
    let r = FsFixtureStorage::open(&relative, FakeLedger::empty());
    assert!(r.is_err(), "relative root must be refused");
    assert!(
        !relative.exists(),
        "nothing may be created for a refused root"
    );
}

/// TS-19: a root that is a regular file is refused and left alone.
#[test]
fn open_refuses_a_root_that_is_a_file() {
    let tmp = TempDir::new("open-file-root");
    std::fs::write(tmp.root(), b"not a directory").unwrap();
    assert!(FsFixtureStorage::open(&tmp.root(), FakeLedger::empty()).is_err());
    assert_eq!(std::fs::read(tmp.root()).unwrap(), b"not a directory");
}

/// TS-19, T-11: a root that is a junction or symlink is refused, and nothing is
/// created behind the link.
#[test]
fn open_refuses_a_root_that_is_a_link() {
    let tmp = TempDir::new("open-link-root");
    let real = tmp.path().join("real");
    std::fs::create_dir(&real).unwrap();
    make_dir_link(&tmp.root(), &real);
    assert!(FsFixtureStorage::open(&tmp.root(), FakeLedger::empty()).is_err());
    assert!(
        names_in(&real).is_empty(),
        "nothing may be created behind a link"
    );
}

/// TS-19, T-11: `alloc/` replaced by a link to a directory outside the root is
/// refused, and nothing is created in the target.
#[test]
fn open_refuses_an_alloc_directory_that_is_a_link() {
    let tmp = TempDir::new("open-link-alloc");
    let outside = tmp.path().join("outside");
    std::fs::create_dir(&outside).unwrap();
    std::fs::create_dir(tmp.root()).unwrap();
    make_dir_link(&alloc_dir(&tmp), &outside);
    assert!(FsFixtureStorage::open(&tmp.root(), FakeLedger::empty()).is_err());
    assert!(names_in(&outside).is_empty());
}

/// TS-31, T-14 (Unix modes only): root and `alloc/` are created 0700.
#[cfg(unix)]
#[test]
fn open_creates_private_directories_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new("open-mode");
    let _storage = open(&tmp, FakeLedger::empty());
    let mode = |p: &std::path::Path| {
        let meta = std::fs::metadata(p);
        assert!(meta.is_ok(), "{p:?} must exist");
        meta.unwrap().permissions().mode() & 0o777
    };
    assert_eq!(mode(&tmp.root()), 0o700);
    assert_eq!(mode(&alloc_dir(&tmp)), 0o700);
}

/// TS-31, T-14 (Unix modes only): an existing root that others can read is refused.
#[cfg(unix)]
#[test]
fn open_refuses_a_group_or_world_accessible_root_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new("open-mode-open");
    std::fs::create_dir(tmp.root()).unwrap();
    std::fs::set_permissions(tmp.root(), std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(FsFixtureStorage::open(&tmp.root(), FakeLedger::empty()).is_err());
}

/// TS-31, T-14 (Unix modes only): a pre-existing `alloc/` that others can read is
/// refused too (TS-02 definition: the directories are 0700).
#[cfg(unix)]
#[test]
fn open_refuses_a_group_or_world_accessible_alloc_directory_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new("open-mode-alloc");
    std::fs::create_dir(tmp.root()).unwrap();
    std::fs::set_permissions(tmp.root(), std::fs::Permissions::from_mode(0o700)).unwrap();
    std::fs::create_dir(alloc_dir(&tmp)).unwrap();
    std::fs::set_permissions(alloc_dir(&tmp), std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(FsFixtureStorage::open(&tmp.root(), FakeLedger::empty()).is_err());
}

/// TS-31, T-14 (Unix modes only): a written fixture file is 0600.
#[cfg(unix)]
#[tokio::test]
async fn written_file_is_private_on_unix() {
    use std::os::unix::fs::PermissionsExt;
    let tmp = TempDir::new("write-mode");
    let id = Uuid::new_v4();
    let storage = open(&tmp, registered(id, 4));
    storage
        .write(&StorageKey::from_allocation_id(id), b"data")
        .await
        .expect("write");
    let meta = std::fs::metadata(alloc_dir(&tmp).join(file_name(id)));
    assert!(meta.is_ok(), "the written file must exist");
    assert_eq!(meta.unwrap().permissions().mode() & 0o777, 0o600);
}

// ---------- write: register before write, opaque names (FN-22, FN-21) ----------

/// FN-21, FN-22, T-11: a write without a registered allocation is refused and not a
/// single byte (or empty file) reaches the namespace.
#[tokio::test]
async fn write_without_a_registered_allocation_is_refused_and_writes_nothing() {
    let tmp = TempDir::new("write-unregistered");
    let storage = open(&tmp, FakeLedger::empty());
    std::fs::create_dir_all(alloc_dir(&tmp)).unwrap();
    let key = StorageKey::from_allocation_id(Uuid::new_v4());
    assert_eq!(
        storage.write(&key, b"bytes").await,
        Err(StorageError::NotRegistered)
    );
    assert!(
        names_in(&alloc_dir(&tmp)).is_empty(),
        "nothing may be written"
    );
}

/// FN-21, FN-22: only an allocation still in `registered` may be written; every other
/// ledger state refuses the write and leaves the namespace empty.
#[tokio::test]
async fn write_is_refused_for_every_state_except_registered() {
    let tmp = TempDir::new("write-states");
    let states: [AllocationState; 6] = [
        Written,
        ReleasePending,
        Deleting,
        Deleted,
        OrphanDiscovered,
        CleanupFailed,
    ];
    let rows: Vec<_> = states
        .iter()
        .map(|s| ledger_row(Uuid::new_v4(), *s, 16))
        .collect();
    let storage = open(&tmp, FakeLedger::with(rows.clone()));
    std::fs::create_dir_all(alloc_dir(&tmp)).unwrap();
    for row in &rows {
        assert_eq!(
            storage.write(&row.storage_key, b"x").await,
            Err(StorageError::NotRegistered),
            "state {:?} must refuse a write",
            row.state
        );
    }
    assert!(names_in(&alloc_dir(&tmp)).is_empty());
}

/// FN-22, T-11: a registered allocation is written create-new under exactly
/// `alloc/<uuid>.bin`, with no temp or extra file left behind.
#[tokio::test]
async fn write_creates_exactly_the_opaque_key_file() {
    let tmp = TempDir::new("write-ok");
    let id = Uuid::new_v4();
    let storage = open(&tmp, registered(id, 16));
    let key = StorageKey::from_allocation_id(id);
    storage
        .write(&key, b"0123456789abcdef")
        .await
        .expect("write");
    let path = alloc_dir(&tmp).join(file_name(id));
    assert!(path.is_file(), "{path:?} must exist");
    assert_eq!(std::fs::read(&path).unwrap(), b"0123456789abcdef");
    assert_eq!(names_in(&alloc_dir(&tmp)), vec![file_name(id)]);
    assert_eq!(names_in(&tmp.root()), vec!["alloc".to_owned()]);
}

/// FN-22: a write beyond the reserved bytes is refused with no partial file.
#[tokio::test]
async fn write_beyond_the_reserved_bytes_is_refused_without_a_partial_file() {
    let tmp = TempDir::new("write-over");
    let id = Uuid::new_v4();
    let storage = open(&tmp, registered(id, 8));
    std::fs::create_dir_all(alloc_dir(&tmp)).unwrap();
    let key = StorageKey::from_allocation_id(id);
    assert_eq!(
        storage.write(&key, b"123456789").await,
        Err(StorageError::ExceedsReservation)
    );
    assert!(names_in(&alloc_dir(&tmp)).is_empty());
}

/// FN-22: create-new semantics; a second write never overwrites.
#[tokio::test]
async fn write_never_overwrites_an_existing_file() {
    let tmp = TempDir::new("write-twice");
    let id = Uuid::new_v4();
    let storage = open(&tmp, registered(id, 8));
    let key = StorageKey::from_allocation_id(id);
    storage.write(&key, b"first").await.expect("first write");
    let path = alloc_dir(&tmp).join(file_name(id));
    assert!(path.is_file(), "first write must create the file");
    assert_eq!(
        storage.write(&key, b"second").await,
        Err(StorageError::AlreadyExists)
    );
    assert_eq!(std::fs::read(&path).unwrap(), b"first");
}

/// FN-21, T-12: an unavailable ledger fails closed; nothing is written.
#[tokio::test]
async fn write_fails_closed_when_the_ledger_is_unavailable() {
    let tmp = TempDir::new("write-ledger-down");
    let storage = open(&tmp, Arc::new(DownLedger));
    std::fs::create_dir_all(alloc_dir(&tmp)).unwrap();
    let key = StorageKey::from_allocation_id(Uuid::new_v4());
    assert_eq!(
        storage.write(&key, b"x").await,
        Err(StorageError::LedgerUnavailable)
    );
    assert!(names_in(&alloc_dir(&tmp)).is_empty());
}

/// Plant a FILE link at `alloc/<id>.bin` pointing at `<tmp>/outside.txt` (existing, with
/// known content) or, when `dangling`, at a name that does not exist. `None` when the
/// host cannot create file symlinks (skip recorded on stderr).
fn plant_file_link(tmp: &TempDir, id: Uuid, dangling: bool) -> Option<(PathBuf, PathBuf)> {
    std::fs::create_dir_all(alloc_dir(tmp)).unwrap();
    let target = tmp.path().join("outside.txt");
    if !dangling {
        std::fs::write(&target, b"precious").unwrap();
    }
    let link = alloc_dir(tmp).join(file_name(id));
    make_file_link(&link, &target).then_some((link, target))
}

fn assert_link_and_target_untouched(
    link: &std::path::Path,
    target: &std::path::Path,
    dangling: bool,
) {
    assert!(
        std::fs::symlink_metadata(link)
            .map(|m| is_link(&m))
            .unwrap_or(false),
        "the planted link must remain a link"
    );
    if dangling {
        assert!(!target.exists(), "a dangling target must not be created");
    } else {
        assert_eq!(std::fs::read(target).unwrap(), b"precious");
    }
}

/// TS-19, T-11 (T-11 core case): a FILE symlink at the key path that points at an existing
/// file outside the namespace is never truncated, overwritten or written through. A
/// naive `File::create` would follow it. Skipped (visibly) where the host cannot create
/// file symlinks.
#[tokio::test]
async fn write_never_goes_through_a_file_symlink_to_an_existing_file() {
    let tmp = TempDir::new("write-file-link");
    let id = Uuid::new_v4();
    let storage = open(&tmp, registered(id, 8));
    let Some((link, target)) = plant_file_link(&tmp, id, false) else {
        return;
    };
    let r = storage
        .write(&StorageKey::from_allocation_id(id), b"x")
        .await;
    assert!(
        r.is_err(),
        "write through a file symlink must be refused: {r:?}"
    );
    assert_link_and_target_untouched(&link, &target, false);
}

/// TS-19, T-11: a DANGLING file symlink at the key path is not written through either
/// (the target is never created).
#[tokio::test]
async fn write_never_goes_through_a_dangling_symlink() {
    let tmp = TempDir::new("write-dangling-link");
    let id = Uuid::new_v4();
    let storage = open(&tmp, registered(id, 8));
    let Some((link, target)) = plant_file_link(&tmp, id, true) else {
        return;
    };
    let r = storage
        .write(&StorageKey::from_allocation_id(id), b"x")
        .await;
    assert!(
        r.is_err(),
        "write through a dangling symlink must be refused: {r:?}"
    );
    assert_link_and_target_untouched(&link, &target, true);
}

/// TS-19, T-11: a DIRECTORY link (junction on Windows) at the key path is refused and
/// nothing reaches its target. Weak on its own (a create-new write fails on any
/// existing path); the file-symlink tests above and the `alloc/` swap test below carry the
/// link-following evidence.
#[tokio::test]
async fn write_never_goes_through_a_link_at_the_key_path() {
    let tmp = TempDir::new("write-key-link");
    let id = Uuid::new_v4();
    let storage = open(&tmp, registered(id, 8));
    let outside = tmp.path().join("outside");
    std::fs::create_dir(&outside).unwrap();
    std::fs::create_dir_all(alloc_dir(&tmp)).unwrap();
    make_dir_link(&alloc_dir(&tmp).join(file_name(id)), &outside);
    let r = storage
        .write(&StorageKey::from_allocation_id(id), b"x")
        .await;
    assert!(
        r.is_err(),
        "write through a planted link must be refused: {r:?}"
    );
    assert!(
        names_in(&outside).is_empty(),
        "nothing may reach the link target"
    );
}

/// TS-19, T-11: `alloc/` swapped for a link after `open` is detected at write time.
#[tokio::test]
async fn write_refuses_when_alloc_is_swapped_for_a_link_after_open() {
    let tmp = TempDir::new("write-swap");
    let id = Uuid::new_v4();
    let storage = open(&tmp, registered(id, 8));
    let outside = tmp.path().join("outside");
    std::fs::create_dir(&outside).unwrap();
    std::fs::create_dir_all(alloc_dir(&tmp)).unwrap();
    std::fs::remove_dir(alloc_dir(&tmp)).unwrap();
    make_dir_link(&alloc_dir(&tmp), &outside);
    let r = storage
        .write(&StorageKey::from_allocation_id(id), b"x")
        .await;
    assert!(
        r.is_err(),
        "write under a swapped alloc/ link must be refused: {r:?}"
    );
    assert!(names_in(&outside).is_empty());
}

// ---------- delete (FN-22, TS-19) ----------

/// FN-22: `delete` unlinks a verified regular file; a missing file counts as removed.
#[tokio::test]
async fn delete_removes_a_regular_file_and_a_missing_file_is_already_absent() {
    let tmp = TempDir::new("delete-ok");
    let id = Uuid::new_v4();
    let storage = open(&tmp, FakeLedger::empty());
    plant(&tmp, &file_name(id), b"payload");
    let key = StorageKey::from_allocation_id(id);
    assert_eq!(storage.delete(&key).await, Ok(DeleteOutcome::Removed));
    assert!(!alloc_dir(&tmp).join(file_name(id)).exists());
    assert_eq!(storage.delete(&key).await, Ok(DeleteOutcome::AlreadyAbsent));
}

/// FN-22, IFC 6.1 step 6: only a missing FILE counts as already absent. A missing `alloc/`
/// or storage root means the namespace is gone (unmounted, deleted, misconfigured), so a
/// delete must fail closed instead of reporting a successful cleanup. Red-first, added in
/// build-and-verify WU-04 after review.
#[tokio::test]
async fn delete_fails_closed_when_the_namespace_directories_are_missing() {
    let tmp = TempDir::new("delete-missing-dirs");
    let key = StorageKey::from_allocation_id(Uuid::new_v4());

    let storage = open(&tmp, FakeLedger::empty());
    std::fs::remove_dir(alloc_dir(&tmp)).expect("remove empty alloc dir");
    assert_eq!(
        storage.delete(&key).await,
        Err(StorageError::Unavailable),
        "missing alloc/ must not look like a deleted file"
    );

    std::fs::remove_dir(tmp.root()).expect("remove empty root");
    assert_eq!(
        storage.delete(&key).await,
        Err(StorageError::Unavailable),
        "missing storage root must not look like a deleted file"
    );
}

/// TS-19: delete touches only the exact key; a forged upper-case name of ANOTHER id, a
/// temp file and an other-key file stay on disk. (The upper-case name belongs to a
/// different id: on a case-insensitive file system the same id would be the same file.)
#[tokio::test]
async fn delete_leaves_every_other_entry_on_disk() {
    let tmp = TempDir::new("delete-others");
    let id = Uuid::new_v4();
    let other = Uuid::new_v4();
    let storage = open(&tmp, FakeLedger::empty());
    let forged_upper = format!("{}.bin", uuid_with_letters().to_string().to_uppercase());
    plant(&tmp, &file_name(id), b"mine");
    plant(&tmp, &file_name(other), b"other");
    plant(&tmp, "stray.tmp", b"tmp");
    plant(&tmp, &forged_upper, b"forged");
    let key = StorageKey::from_allocation_id(id);
    assert_eq!(storage.delete(&key).await, Ok(DeleteOutcome::Removed));
    let mut expect = vec![file_name(other), "stray.tmp".to_owned(), forged_upper];
    expect.sort();
    let mut left = names_in(&alloc_dir(&tmp));
    left.sort();
    assert_eq!(left, expect);
}

/// TS-19, FD F-22 ("unlinks only a verified regular file with the exact name"): after the
/// real file is gone, a forged file whose name is the UPPER-CASE spelling of the same id
/// must survive a `delete` of the lower-case key, even where the file system folds case
/// (NTFS): the implementation must compare the exact on-disk name, not open by path.
/// The first delete (red against the shell) frees the lower-case name so the forged
/// file can exist next to nothing on a case-insensitive host.
#[tokio::test]
async fn delete_never_removes_a_case_folded_forged_file() {
    let tmp = TempDir::new("delete-case-fold");
    let id = uuid_with_letters();
    let storage = open(&tmp, FakeLedger::empty());
    let key = StorageKey::from_allocation_id(id);
    plant(&tmp, &file_name(id), b"mine");
    assert_eq!(storage.delete(&key).await, Ok(DeleteOutcome::Removed));
    let forged_upper = format!("{}.bin", id.to_string().to_uppercase());
    plant(&tmp, &forged_upper, b"forged");
    let r = storage.delete(&key).await;
    assert!(
        matches!(
            r,
            Ok(DeleteOutcome::AlreadyAbsent) | Err(StorageError::Ambiguous)
        ),
        "an exact-name miss is AlreadyAbsent or Ambiguous, never Removed: {r:?}"
    );
    assert_eq!(names_in(&alloc_dir(&tmp)), vec![forged_upper.clone()]);
    assert_eq!(
        std::fs::read(alloc_dir(&tmp).join(&forged_upper)).unwrap(),
        b"forged"
    );
}

/// TS-19, T-11: a FILE symlink at the key path (to an existing outside file, or
/// dangling) is ambiguous: never followed, never deleted, the link stays and the target
/// is unchanged. Skipped (visibly) where the host cannot create file symlinks.
#[tokio::test]
async fn delete_never_follows_or_removes_a_file_symlink() {
    for dangling in [false, true] {
        let tmp = TempDir::new("delete-file-link");
        let id = Uuid::new_v4();
        let storage = open(&tmp, FakeLedger::empty());
        let Some((link, target)) = plant_file_link(&tmp, id, dangling) else {
            return;
        };
        let r = storage.delete(&StorageKey::from_allocation_id(id)).await;
        assert_eq!(r, Err(StorageError::Ambiguous), "dangling={dangling}");
        assert_link_and_target_untouched(&link, &target, dangling);
    }
}

/// TS-19, TS-23 classification: a directory at the key path is ambiguous, never
/// deleted, and its contents survive.
#[tokio::test]
async fn delete_refuses_a_directory_at_the_key_path() {
    let tmp = TempDir::new("delete-dir");
    let id = Uuid::new_v4();
    let storage = open(&tmp, FakeLedger::empty());
    let dir = alloc_dir(&tmp).join(file_name(id));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("inner.txt"), b"keep").unwrap();
    let r = storage.delete(&StorageKey::from_allocation_id(id)).await;
    assert_eq!(r, Err(StorageError::Ambiguous));
    assert_eq!(std::fs::read(dir.join("inner.txt")).unwrap(), b"keep");
}

/// TS-19, T-11: a link (junction, reparse point, symlink) at the key path is never
/// followed or deleted; the target and the link both survive.
#[tokio::test]
async fn delete_never_follows_or_removes_a_link() {
    let tmp = TempDir::new("delete-link");
    let id = Uuid::new_v4();
    let storage = open(&tmp, FakeLedger::empty());
    let outside = tmp.path().join("outside");
    std::fs::create_dir(&outside).unwrap();
    std::fs::write(outside.join("precious.txt"), b"keep").unwrap();
    std::fs::create_dir_all(alloc_dir(&tmp)).unwrap();
    let link = alloc_dir(&tmp).join(file_name(id));
    make_dir_link(&link, &outside);
    let r = storage.delete(&StorageKey::from_allocation_id(id)).await;
    assert_eq!(r, Err(StorageError::Ambiguous));
    assert!(
        is_link(&std::fs::symlink_metadata(&link).unwrap()),
        "link must remain"
    );
    assert_eq!(
        std::fs::read(outside.join("precious.txt")).unwrap(),
        b"keep"
    );
}

// ---------- inventory: bounded (TS-56, T-13) and classified (TS-19) ----------

/// TS-56, T-13: `inventory(max_entries)` never returns more than `max_entries` and sets
/// `truncated` exactly when more existed.
#[tokio::test]
async fn inventory_never_exceeds_max_entries_and_flags_truncation() {
    let tmp = TempDir::new("inv-bound");
    let storage = open(&tmp, FakeLedger::empty());
    for _ in 0..5 {
        plant(&tmp, &file_name(Uuid::new_v4()), b"x");
    }
    // (max_entries, expected len, expected truncated)
    for (max, len, truncated) in [
        (0usize, 0usize, true),
        (1, 1, true),
        (3, 3, true),
        (4, 4, true),
        (5, 5, false),
        (6, 5, false),
        (10_000, 5, false),
    ] {
        let page = storage.inventory(max).await.expect("inventory");
        assert!(
            page.entries.len() <= max,
            "max {max}: got {}",
            page.entries.len()
        );
        assert_eq!(page.entries.len(), len, "max {max}: entry count");
        assert_eq!(page.truncated, truncated, "max {max}: truncated flag");
    }
    let empty = TempDir::new("inv-bound-empty");
    let empty_storage = open(&empty, FakeLedger::empty());
    let page = empty_storage.inventory(0).await.expect("inventory");
    assert!(page.entries.is_empty() && !page.truncated);
}

/// TS-56, T-13: the bound holds for a large namespace.
#[tokio::test]
async fn inventory_bound_holds_for_a_large_namespace() {
    let tmp = TempDir::new("inv-large");
    let storage = open(&tmp, FakeLedger::empty());
    for _ in 0..600 {
        plant(&tmp, &file_name(Uuid::new_v4()), b"");
    }
    let page = storage.inventory(50).await.expect("inventory");
    assert_eq!(page.entries.len(), 50);
    assert!(page.truncated);
}

/// TS-19, FD 4.4: `inventory` reports what is on disk by kind, never follows a link,
/// and lists only the single `alloc/` directory.
#[tokio::test]
async fn inventory_reports_kinds_and_never_follows_links() {
    let tmp = TempDir::new("inv-kinds");
    let storage = open(&tmp, FakeLedger::empty());
    let valid = Uuid::new_v4();
    let dir_id = Uuid::new_v4();
    let link_id = Uuid::new_v4();
    plant(&tmp, &file_name(valid), b"0123456789");
    plant(&tmp, "stray.tmp", b"t");
    std::fs::create_dir(alloc_dir(&tmp).join(file_name(dir_id))).unwrap();
    let outside = tmp.path().join("outside");
    std::fs::create_dir(&outside).unwrap();
    for n in ["a", "b", "c"] {
        std::fs::write(outside.join(n), b"o").unwrap();
    }
    make_dir_link(&alloc_dir(&tmp).join(file_name(link_id)), &outside);
    // A file next to `alloc/` is outside the namespace and must not be listed.
    std::fs::write(tmp.root().join("not-in-namespace.txt"), b"n").unwrap();

    let page = storage.inventory(100).await.expect("inventory");
    assert!(!page.truncated);
    let kind_of = |name: &str| {
        page.entries
            .iter()
            .find(|e| e.name == name)
            .map(|e| (e.kind, e.size_bytes))
    };
    assert_eq!(
        kind_of(&file_name(valid)),
        Some((EntryKind::RegularFile, 10))
    );
    assert_eq!(kind_of("stray.tmp"), Some((EntryKind::RegularFile, 1)));
    assert_eq!(
        kind_of(&file_name(dir_id)).map(|k| k.0),
        Some(EntryKind::Directory)
    );
    assert_eq!(
        kind_of(&file_name(link_id)).map(|k| k.0),
        Some(EntryKind::Link)
    );
    assert_eq!(
        page.entries.len(),
        4,
        "link target and root files must not be listed"
    );
}

// ---------- classification (TS-19, TS-23 classification part) ----------

/// TS-19, FD 4.4: any name that is not exactly a lowercase `<uuid>.bin` is ambiguous
/// whatever the ledger says, so it is never deleted.
#[test]
fn classify_forged_and_non_opaque_names_as_ambiguous() {
    let id = Uuid::new_v4();
    let names = [
        format!("{}.bin", id.to_string().to_uppercase()),
        format!("{id}.BIN"),
        format!("{id}.bin.tmp"),
        format!("{id}.tmp"),
        format!(".{id}.bin"),
        "stray.tmp".to_owned(),
        "..".to_owned(),
        String::new(),
        format!("sub/{id}.bin"),
        format!("sub\\{id}.bin"),
        format!("{}.bin", id.simple()),
    ];
    for name in &names {
        for state in [None, Some(Registered), Some(Written), Some(ReleasePending)] {
            assert_eq!(
                classify_entry(&entry(name, EntryKind::RegularFile), state),
                Ownership::Ambiguous,
                "{name:?} with ledger {state:?}"
            );
        }
    }
}

/// TS-19, FD 4.4: a directory, link or other non-regular entry is ambiguous even when
/// its name is a valid key and a ledger row exists.
#[test]
fn classify_non_regular_entries_as_ambiguous_even_with_a_valid_name() {
    let name = file_name(Uuid::new_v4());
    for kind in [EntryKind::Directory, EntryKind::Link, EntryKind::Other] {
        for state in [None, Some(Registered), Some(Written), Some(Deleting)] {
            assert_eq!(
                classify_entry(&entry(&name, kind), state),
                Ownership::Ambiguous,
                "{kind:?} with ledger {state:?}"
            );
        }
    }
}

/// TS-19, TS-23 classification, FD 4.4: a regular file with a valid opaque name is
/// classified by its ledger row. A row in `deleted` or `cleanup_failed` is ambiguous
/// (never deleted); no row is an orphan (persisted with grace, deleted only later).
///
/// [D] A row already in `orphan_discovered` is `Owned` (FD 4.4 is silent): the orphan is
/// persisted and awaits its grace, so the 900 s discover-only scan (IFC 6.1a) must not
/// treat it as a new orphan and re-insert it. Ruling needed from the conductor.
///
/// Decision D3 (kept): a valid name with NO row is `Orphan`, not `Ambiguous`. Sources:
/// FD 4.4 table ("same name | no row | orphan"), IFC 6.1 step 9 and crash matrix window 4
/// (6.2), test-strategy TS-23 ("Window 4: orphan namespace entry"). The work-unit
/// Done-when wording ("not a ledger-registered opaque key is ambiguous") is read as the
/// invalid / forged / non-regular names of TS-19, which the tests above classify as
/// `Ambiguous`.
#[test]
fn classify_valid_opaque_name_follows_the_ledger_table() {
    let name = file_name(Uuid::new_v4());
    let e = entry(&name, EntryKind::RegularFile);
    for (state, expected) in [
        (Some(Registered), Ownership::Owned),
        (Some(Written), Ownership::Owned),
        (Some(ReleasePending), Ownership::Owned),
        (Some(Deleting), Ownership::OwnedInCleanup),
        (Some(OrphanDiscovered), Ownership::Owned),
        (Some(Deleted), Ownership::Ambiguous),
        (Some(CleanupFailed), Ownership::Ambiguous),
        (None, Ownership::Orphan),
    ] {
        assert_eq!(classify_entry(&e, state), expected, "ledger {state:?}");
    }
}
