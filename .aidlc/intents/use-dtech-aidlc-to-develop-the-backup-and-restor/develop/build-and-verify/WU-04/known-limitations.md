# KL-04 Known limitations: WU-04

- **ID:** KL-04 (WU-04)
- **Stage:** Develop / build-and-verify.
- **Status:** Open limitations and verification boundaries, refreshed 2026-11-24. Focused WU-04 verification passed; intent-wide regression remains blocked by later-unit failures documented in `regression-report.md`.
- **Implementation:** `implementation.md`.
- **Evidence:** `green-verification-report.md`, `regression-report.md`, and `develop/build-and-verify/evidence/refresh-wu04-*`.

| # | Limitation | Impact | Follow-up / owner |
|---|---|---|---|
| L-01 | Directory link can be swapped between path check and use. The implementation checks root and `alloc/` metadata, then later resolves by path. | A principal with write access to the storage-root parent could exploit the check/use window. | Private Unix directory permissions mitigate this on Unix; the owner-accepted Windows ACL residual applies. A stronger handle-relative implementation would require additional API/dependency work. Revisit before real-data staging. |
| L-02 | A held lock file may be unlinked or renamed by a principal with write access to its parent. | A second process might create a replacement lock inode and acquire it. | Requires parent-directory access that already defeats L-01; retain as residual and revisit alongside L-01. |
| L-03 | Windows file-symlink security cases depend on host symlink privilege and may skip. | When skipped, no executable Windows evidence for those cases. | Current focused Windows run passed; verify symlink-path behavior on a host that can create file symlinks or CI. Do not confuse skip with an exercised guard. |
| L-04 | Unix TS-31 permission checks (0700/0600) are compiled out on Windows. | Current refresh has no green evidence for Unix permissions. | Run on Unix CI or Linux before the release-candidate security gate. This refresh did not install a Rust target or pull an image. |
| L-05 | `create_new` is not independently isolated by a test; it overlaps with key-path pre-check coverage. | Removing either guard alone may not be detected; both removed is caught. | Add a concurrent check-then-create test if this becomes material. |
| L-06 | Write can race a release/delete after its ledger lookup. | A late file could appear after the row leaves a live state. | WU-06 TS-28 owns the forced interleaving and recovery proof. |
| L-07 | Synchronous filesystem operations occur inside async functions; no async filesystem dependency is present. | Slow filesystems can block an async worker thread. | Decide whether to use `spawn_blocking` in a later integration/runtime unit if justified. |
| L-08 | `delete` scans the allocation directory without a cap; inventory bound accounting can omit entries that disappear during enumeration. | Large or concurrently changing namespaces can cost extra time; bound is approximate in the noted edge case. | Consider bounded delete traversal and a removal-during-listing test if required. |
| L-09 | Inventory scans `alloc/`, not unrelated root-level files. | Root-level files are not reconciled. | Consistent with the specified inventory boundary; accepted design limit. |
| L-10 | Design's WU-04 Done-when wording conflicts with detailed classification rules for a valid unregistered key. | Documentation ambiguity only; implementation follows the detailed FD/IFC/TS behavior (`Orphan`). | Correct the WU-04 summary in a later design maintenance pass. |
| L-11 | Port API has recorded deviations Dev-1..Dev-5 from IFC section 9. | Signatures do not match the originally written contract exactly. | Retain the decisions in historical red evidence; update the contract at an authorized design maintenance point. |
| L-12 | Two-process lock test launches the test binary using the `lock_child_probe` helper. | None observed; test depends on subprocess invocation. | Accepted. |
| L-13 | `domain_backup` is not yet composed into the gateway. | WU-04 adapters are not reachable from the running API. | Gateway integration belongs to WU-10. |
| L-14 | Workspace test refresh fails in later-unit tests. | Overall develop regression cannot be declared clean. | Existing WU-17/18/19 lifecycle failures and WU-06 store reconciliation failures are documented by name in `regression-report.md`; same failure names occur in pre-refresh WU-03 logs. Resolve in owning units, then rerun full suites. They do not demonstrate a WU-04 regression. |
| L-15 | Full-workspace Clippy exits successfully but emits warnings, including later-unit code and other crates. | Warning-clean status is not claimed. | Review warnings in their owning units; WU-04's focused functional suites and formatting check pass. |

## Environment constraints

Host observed: Windows 11, Rust/Cargo 1.100.0, Docker 29.7.2. Docker prerequisite passed. No target installation or image pull was performed. Unix permission tests were not exercised. File-symlink tests may take their test-defined visible skip path if the host lacks privilege. Windows owner-only ACL verification remains the accepted synthetic-only U1 residual recorded in the design and historical WU-04 red evidence.

## Rollback

WU-04 contains allocation, fixture-storage and namespace-lock code; no schema change was made in this unit (the schema is from WU-02), and the gateway does not yet compose these adapters. Rollback is a source revert of WU-04 changes. Removing the database schema, files or unrelated artifacts is not part of this verification and was not performed.
