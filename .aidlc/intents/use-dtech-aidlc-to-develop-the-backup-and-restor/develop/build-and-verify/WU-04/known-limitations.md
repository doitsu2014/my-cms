# KL-04 Known limitations: WU-04

- **ID:** KL-04 (WU-04)
- **Stage:** Develop / build-and-verify, unit WU-04.
- **Status:** Open items carried to later units or to the release-candidate gate. Earlier items: [../WU-03/known-limitations.md](../WU-03/known-limitations.md).

| # | Limitation | Impact | Owner or next step |
|---|---|---|---|
| L-01 | **Directory link swapped between check and use.** `check_dirs` inspects `<root>` and `alloc/` with `symlink_metadata`. Later calls re-resolve the full path (`create_new`, `read_dir`, `remove_file`). | Someone with write access to the storage root's parent could swap `alloc/` for a link in that window. | Mitigated on Unix by the private 0700 root; on Windows this is the owner-accepted ACL residual. A full fix needs held directory handles (`openat`-style calls, e.g. `cap-std` or `rustix`), which means a new dependency. Owner decision before real-data staging. |
| L-02 | **A held lock file can be deleted or renamed.** Windows opens it with `FILE_SHARE_DELETE`. On Unix `flock` does not stop unlink. | A second process could then create a new lock file and acquire it, breaking exclusivity. | Requires write access next to the storage root, which already defeats L-01. Recorded; revisit with L-01. |
| L-03 | **Lock link handling and the write pre-check's file-link handling are not verifiable on Windows.** The file-symlink tests skip because the host lacks symlink privilege. | No executable evidence on this host for those guards | CI on Ubuntu, or a Linux run, before the release-candidate gate |
| L-04 | **Unix permission tests (TS-31) never run green.** | 0700/0600 enforcement unverified | CI on Ubuntu before the release-candidate gate |
| L-05 | **`create_new` has no test of its own.** It overlaps with the key-path pre-check. | A regression in only one of the two would not be caught. | A concurrent check-then-create test |
| L-06 | **Write racing release-and-delete.** A write that passes the ledger `get` can create a file after the row moves to `release_pending` or `deleted`. | A late file with no live row | WU-06 TS-28 must force this interleaving and prove the late file is cleaned up |
| L-07 | **Blocking filesystem calls inside `async fn`.** `tokio` is a dev-dependency only, so `spawn_blocking` is not available. | Can block an async worker thread on slow disks | WU-05/WU-10 decide whether to wrap calls in `spawn_blocking` (needs `tokio` as a dependency) |
| L-08 | **`delete` scans `alloc/` (O(n)) and has no entry cap.** `inventory` does not count NotFound-skipped entries toward its bound. | Slow on a very large namespace; the bound is approximate | A test with more than `MAX_INVENTORY_ENTRIES + 1` junk entries, plus a removal hook during listing |
| L-09 | **Root-level files are invisible to inventory.** Only `alloc/` is listed (FD F-22). | A stray file at the root is never reconciled | Accepted per design |
| L-10 | **Design documents disagree on classification.** The WU-04 Done-when says an unregistered valid key is "ambiguous"; FD 4.4, IFC 6.1 and 6.2 and TS-23 say "orphan". The implementation follows the detailed design. | Documentation inconsistency only | Correct the Done-when line in a later design touch-up |
| L-11 | **Five port-shape deviations from IFC section 9** (Dev-1..Dev-5 in the red evidence) | The contract and the code differ in signatures | Accepted and recorded. Fold into the contract when next revised. |
| L-12 | **The two-process lock test spawns the test binary.** It relies on the `lock_child_probe` helper. | none | Accepted |
| L-13 | **Not composed into the gateway** | No effect on the running API | WU-10 |

## Rollback

The ledger, storage and lock are not referenced outside `domain_backup`, and the gateway does not compose the crate. To roll back, revert the WU-04 files. No schema change was made in WU-04, because the `backup_allocations` and `backup_epoch` tables are from WU-02.
