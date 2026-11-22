//! `FsNamespaceLock` red tests (no Docker). TS-32 (lock part: a second holder is
//! refused ownership), including the two-process cross-check (RK-06), and the
//! lock file placement of FD 4.4 (a sibling of the root, outside the namespace).
//! AC-003 S3, AC-002 S2; T-11, T-12. FN-24.
//!
//! The two-process tests re-run this test binary as a child (the pattern of WU-01's
//! non-UTF-8 test): `lock_child_probe` is inert unless `MAKARA_LOCK_MODE` is set.
//! The hand-off is by a line on the child's stdout and EOF on its stdin, never a sleep.

use super::fs_support::*;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::infra::fs_namespace_lock::FsNamespaceLock;
use crate::ports::fixture_storage::FixtureStorage;
use crate::ports::namespace_lock::{LockError, NamespaceLock};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Kills and reaps the child on drop, so a failed assertion never leaks a process.
struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn spawn_probe(mode: &str, root: &Path) -> ChildGuard {
    let child = Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg(format!(
            "{}::lock_child_probe",
            module_path!().split_once("::").unwrap().1
        ))
        .arg("--nocapture")
        .arg("--test-threads=1")
        .env("MAKARA_LOCK_MODE", mode)
        .env("MAKARA_LOCK_ROOT", root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn child test binary");
    ChildGuard(child)
}

/// Probe run in a child process by the two-process tests (inert alone).
///
/// - `expect_refused`: the parent holds the lock; this process must be refused with
///   `AlreadyHeld`.
/// - `hold`: acquire, announce `HOLDING`, keep the guard until stdin reaches EOF.
#[test]
fn lock_child_probe() {
    let Ok(mode) = std::env::var("MAKARA_LOCK_MODE") else {
        return;
    };
    let root = std::path::PathBuf::from(std::env::var_os("MAKARA_LOCK_ROOT").unwrap());
    match mode.as_str() {
        "expect_refused" => {
            let r = FsNamespaceLock::try_acquire(&root);
            assert!(
                matches!(r, Err(LockError::AlreadyHeld)),
                "second process must be refused with AlreadyHeld, got {r:?}"
            );
            println!("CHILD_PROBE_RAN expect_refused");
        }
        "hold" => {
            let guard = FsNamespaceLock::try_acquire(&root).expect("child acquires the lock");
            println!("HOLDING");
            std::io::stdout().flush().unwrap();
            let mut sink = Vec::new();
            let _ = std::io::stdin().read_to_end(&mut sink);
            drop(guard);
            println!("CHILD_PROBE_RAN hold");
        }
        other => panic!("unknown mode {other}"),
    }
}

/// FD 4.4, FN-24: the lock file is `<root>.lock`, a sibling of the root and never inside
/// it, so inventory cannot see it.
#[test]
fn lock_path_is_a_sibling_of_the_root() {
    let tmp = TempDir::new("lock-path");
    let root = tmp.root();
    let lock = FsNamespaceLock::lock_path(&root);
    assert_eq!(lock, tmp.path().join("root.lock"));
    assert!(!lock.starts_with(&root));
}

/// FN-24: acquiring creates the lock file outside the root and the lock raises no
/// ownership incident (the namespace inventory stays empty).
#[tokio::test]
async fn acquire_creates_the_sibling_lock_file_outside_the_namespace() {
    let tmp = TempDir::new("lock-file");
    let root = tmp.root();
    let storage = FsFixtureStorage::open(&root, FakeLedger::empty()).expect("open storage");
    let _guard = FsNamespaceLock::try_acquire(&root).expect("acquire");
    let lock = FsNamespaceLock::lock_path(&root);
    assert!(lock.is_file(), "{lock:?} must exist while held");
    let page = storage.inventory(100).await.expect("inventory");
    assert!(
        page.entries.is_empty(),
        "lock file must not appear in inventory"
    );
    assert!(!page.truncated);
}

/// TS-32, FN-24: while one holder has the namespace, a second holder in the same
/// process gets a clear refusal; dropping the first releases it.
#[test]
fn second_holder_is_refused_and_drop_releases_the_lock() {
    let tmp = TempDir::new("lock-exclusive");
    let root = tmp.root();
    let first = FsNamespaceLock::try_acquire(&root).expect("first acquire");
    let second = FsNamespaceLock::try_acquire(&root);
    assert!(
        second.is_err(),
        "second holder must be refused, got {second:?}"
    );
    let err = second.unwrap_err();
    assert_eq!(err, LockError::AlreadyHeld);
    let shown = err.to_string();
    assert!(!shown.is_empty());
    assert!(
        !shown.contains(&*tmp.path().to_string_lossy()),
        "the refusal must not leak the path: {shown}"
    );
    drop(first);
    let third = FsNamespaceLock::try_acquire(&root);
    assert!(third.is_ok(), "released on drop, got {:?}", third.err());
}

/// TS-32, RK-06: the lock is exclusive across processes. The parent holds it; a child
/// process is refused with `AlreadyHeld`; after the parent drops it a child can take it.
#[test]
fn lock_held_by_parent_refuses_a_second_process() {
    let tmp = TempDir::new("lock-parent-holds");
    let root = tmp.root();
    let guard = FsNamespaceLock::try_acquire(&root).expect("parent acquires");
    assert!(
        FsNamespaceLock::lock_path(&root).is_file(),
        "a held lock must leave its file in place"
    );

    let mut child = spawn_probe("expect_refused", &root);
    let out = finish(&mut child);
    assert!(
        out.status.success() && out.stdout.contains("CHILD_PROBE_RAN expect_refused"),
        "child was not refused: {}{}",
        out.stdout,
        out.stderr
    );

    drop(guard);
    let mut after = spawn_probe("hold", &root);
    let mut lines = BufReader::new(after.0.stdout.take().unwrap());
    assert!(
        wait_for_holding(&mut lines),
        "child must acquire a released lock"
    );
    drop(after.0.stdin.take());
    let status = after.0.wait().unwrap();
    assert!(status.success());
}

/// TS-32, RK-06: the lock is exclusive in the other direction too. A child process holds
/// it; the parent is refused; once the child exits (stdin EOF) the parent can take it,
/// which proves the OS releases a lock when its holder ends.
#[test]
fn lock_held_by_child_process_refuses_the_parent_until_it_exits() {
    let tmp = TempDir::new("lock-child-holds");
    let root = tmp.root();
    let mut child = spawn_probe("hold", &root);
    let mut stdout = BufReader::new(child.0.stdout.take().unwrap());
    assert!(
        wait_for_holding(&mut stdout),
        "child did not acquire the lock"
    );

    let refused = FsNamespaceLock::try_acquire(&root);
    assert!(
        matches!(refused, Err(LockError::AlreadyHeld)),
        "parent must be refused while the child holds the namespace, got {refused:?}"
    );

    drop(child.0.stdin.take());
    let status = child.0.wait().unwrap();
    assert!(status.success(), "child exited with {status:?}");
    let acquired = FsNamespaceLock::try_acquire(&root);
    assert!(
        acquired.is_ok(),
        "free after the holder exits, got {:?}",
        acquired.err()
    );
}

/// Block until the child announces `HOLDING` (the test harness prints its own lines
/// first, so scan for it). False if the child's stdout ends without it.
fn wait_for_holding(out: &mut impl BufRead) -> bool {
    let mut line = String::new();
    loop {
        line.clear();
        match out.read_line(&mut line) {
            Ok(0) | Err(_) => return false,
            Ok(_) if line.contains("HOLDING") => return true,
            Ok(_) => {}
        }
    }
}

/// Output of a finished child as text.
struct ProbeOutput {
    status: std::process::ExitStatus,
    stdout: String,
    stderr: String,
}

/// Close the child's stdin, read its (small) output to the end and reap it.
fn finish(child: &mut ChildGuard) -> ProbeOutput {
    drop(child.0.stdin.take());
    let mut stdout = String::new();
    let mut stderr = String::new();
    if let Some(mut o) = child.0.stdout.take() {
        let _ = o.read_to_string(&mut stdout);
    }
    if let Some(mut e) = child.0.stderr.take() {
        let _ = e.read_to_string(&mut stderr);
    }
    let status = child.0.wait().expect("wait for child");
    ProbeOutput {
        status,
        stdout,
        stderr,
    }
}

// ---------- lock placement and refusals (FD 4.4, T-11) ----------

/// FD 4.4: the lock is `<root>.lock` with the root's FULL final component kept, so a root
/// with an extension (`store.v1`) maps to `store.v1.lock`, not `store.lock`.
#[test]
fn lock_path_appends_to_the_whole_root_name() {
    let tmp = TempDir::new("lock-path-ext");
    assert_eq!(
        FsNamespaceLock::lock_path(&tmp.path().join("store.v1")),
        tmp.path().join("store.v1.lock")
    );
}

/// FD 4.4: a root spelled with a trailing separator still gets the sibling lock, never a
/// file inside the root.
#[test]
fn lock_path_for_a_root_with_a_trailing_separator_is_still_a_sibling() {
    let tmp = TempDir::new("lock-path-trailing");
    let root = tmp.root();
    let spelled =
        std::path::PathBuf::from(format!("{}{}", root.display(), std::path::MAIN_SEPARATOR));
    let lock = FsNamespaceLock::lock_path(&spelled);
    assert_eq!(lock, tmp.path().join("root.lock"));
    assert!(!lock.starts_with(&root));
}

/// FN-24, IFC 9: a relative root is refused with `InvalidRoot` and no lock file is
/// created in the working directory.
#[test]
fn a_relative_root_is_refused_with_invalid_root() {
    let relative = std::path::PathBuf::from(format!(
        "relative-makara-wu04-lock-{}",
        uuid::Uuid::new_v4()
    ));
    let _cleanup = CwdCleanup::new(relative.clone());
    let r = FsNamespaceLock::try_acquire(&relative);
    assert!(
        matches!(r, Err(LockError::InvalidRoot)),
        "relative root must be InvalidRoot, got {r:?}"
    );
    let mut lock = relative.clone().into_os_string();
    lock.push(".lock");
    assert!(
        !std::path::Path::new(&lock).exists(),
        "no lock file may be created for a refused root"
    );
}

/// T-11, FD 4.4: a FILE symlink planted at `<root>.lock` is not followed: acquire is
/// refused and the link target is neither created nor truncated. Skipped (visibly) where
/// the host cannot create file symlinks.
#[test]
fn a_file_symlink_at_the_lock_path_is_refused_and_not_followed() {
    let tmp = TempDir::new("lock-file-link");
    let target = tmp.path().join("precious.txt");
    std::fs::write(&target, b"precious").unwrap();
    if !make_file_link(&FsNamespaceLock::lock_path(&tmp.root()), &target) {
        return;
    }
    let r = FsNamespaceLock::try_acquire(&tmp.root());
    assert!(r.is_err(), "a linked lock path must be refused, got {r:?}");
    assert_eq!(std::fs::read(&target).unwrap(), b"precious");
}

/// T-11: a DIRECTORY link (junction on Windows) at `<root>.lock` is refused and nothing
/// is created behind it.
#[test]
fn a_directory_link_at_the_lock_path_is_refused_and_not_followed() {
    let tmp = TempDir::new("lock-dir-link");
    let outside = tmp.path().join("outside");
    std::fs::create_dir(&outside).unwrap();
    make_dir_link(&FsNamespaceLock::lock_path(&tmp.root()), &outside);
    let r = FsNamespaceLock::try_acquire(&tmp.root());
    assert!(r.is_err(), "a linked lock path must be refused, got {r:?}");
    assert!(names_in(&outside).is_empty());
}
