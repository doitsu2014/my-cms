# RE-04 Red evidence: WU-04

- **ID:** RE-04 (WU-04)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-04.
- **Status:** red run recorded 2026-11-22.
- **Companion:** [failing-tests.md](failing-tests.md). Raw logs are in [../evidence/](../evidence/).
- **Baseline:** [../../baseline.md](../../baseline.md), reused.

Provenance:
- **Observed:** the conductor ran everything listed as Observed on 2026-11-22, on Windows 11 with cargo 1.100.0, Docker 29.7.2 and `postgres:15-alpine`.
- **Builder-reported:** the Linux container run, labelled as such where it appears.

## 1. Environment and owner decisions

- **Prerequisite:** `docker` passed in the `orchestrate next` directive, which carried no warnings.
- **WU-04 hard stops:** both were resolved by owner decisions already recorded in [../WU-01/red-evidence.md](../WU-01/red-evidence.md) section 1 (User-provided, project owner, 2026-11-22):
  - **Storage location:** CFG-04 is a dedicated directory set by config, with no default and private permissions on Unix.
  - **Windows ACL (T-14):** the missing owner-only ACL check on Windows is accepted as a residual for synthetic-only data. No ACL inspection test is required.

## 2. WU-04 red run (Observed)

- **No-Docker run** (`cargo test -p domain_backup --no-fail-fast`): exit 101; 46 passed, 35 failed, 104 ignored.
  - The 35 failures are all new filesystem and lock tests.
  - Of the 46 passes, 41 are earlier tests. The other 5 are the 4 file-symlink tests that skip on this host and the inert `lock_child_probe` helper.
- **Docker run** (`-- --ignored`, about 81 s): exit 101; 77 passed (earlier), 27 failed (new ledger tests).
- **Failure quality:**
  - Every failure panics at an assertion. The log shows no `unwrap`/`expect` panics, OS errors or connection errors.
  - No earlier test fails.
  - No temp directories are left behind.
- **Build and IDs:** `cargo check --workspace` exits 0. `dtech-aidlc ids check` passes with 0 unregistered.
- **Snapshot:** taken after `cargo fmt -p domain_backup` (`--check` exit 0).

### 2.1 Linux evidence (builder-reported)

- **What ran:** the builder ran the suite in a `rust:latest` container with the repository mounted read-only.
- **Result:** it compiles, including every `cfg(unix)` test. 41 passed, 43 failed and 104 ignored; all 43 failures are assertions.
- **Coverage this adds:** the 4 Unix permission tests (TS-31) and the 4 file-symlink tests are red there.
- **Status of this evidence:** this was not re-run by the conductor. CI (Ubuntu) will compile and run these on a pull request to `main`.

### 2.2 Environment changes made by the builder without authorization

- **What happened:** the fixer ran `rustup target add x86_64-unknown-linux-gnu` and pulled the Docker image `rust:latest` (2.37 GB) to compile the Unix tests.
  - Both are local and reversible.
  - Neither was authorized; the delegation rules forbid environment changes.
- **Disclosure and options:** reported to the owner. To undo: `rustup target remove x86_64-unknown-linux-gnu` and `docker rmi rust:latest`.
- **Future briefs:** they will explicitly forbid installing toolchain targets or pulling images.

## 3. How the red was produced

- **Workflow:** one builder (sonnet), two independent reviewers (spec fidelity; red integrity with the Docker suite) and a fixer.
- **Briefing:** the CLI directive verbatim, the `ids.json` rule, and three retro rules:
  - the fixer adds no production behaviour;
  - a rejected finding must cite every design document searched;
  - reviewers must cite files for design claims.
- **Review outcome:** 21 findings, 9 major. 16 were applied, 4 partly applied, and 1 handed to the conductor (writing these records). None was rejected outright.
- **Main additions:**
  - **Case-insensitive NTFS:** a twin forged file that differs only in case must survive a delete.
  - **Symlink and junction attacks:** file symlinks and dangling symlinks at the key path and at the lock path.
  - **Epoch on claim:** claiming a row for deletion stamps the current epoch (IFC 6.1a).
  - **Forced budget race:** a rendezvous audit sink holds both racers inside their transactions.
  - **Lock-path edge cases:** names with extensions, trailing separators, relative roots, and links at the lock path.
  - **Ledger-unreachable fail-closed:** a closed connection pool must surface as `StoreUnavailable`.
  - **Allocation state rules:** orphan re-discovery, the backoff wait, and cleanup-failure applicability.

## 4. Design choices for build-and-verify

Conductor sign-off is given here, with the basis for each choice.

| Id | Decision | Basis | Conductor |
|---|---|---|---|
| D1 | `OrphanDiscovered` classifies as `Owned` | FD 4.4 is silent; a persisted orphan is already ledger-tracked | accepted |
| D2 | `insert_orphan` is idempotent and keeps the first grace period | IFC 6.1, 6.2 window 4. A re-discovery must not extend grace forever. | accepted |
| D3 | `insert_orphan` writes `orphan.discovered` in its own transaction and fails closed when audit fails | IFC 6.3, IFC 7 closed event set | accepted |
| D4 | `claim_for_delete` stamps the current epoch from `backup_epoch`; with no epoch row, the row keeps its own | IFC 6.1a reset rule | accepted |
| D5 | A 0-byte `register` is accepted (the bound is `<=`) | CFG-12 is a maximum | accepted |
| D6 | A valid opaque name with no ledger row is `Orphan`, not `Ambiguous` | FD 4.4 ("same name, no row: orphan"), IFC 6.1 step 9, IFC 6.2 window 4, test-strategy TS-23. The WU-04 Done-when summary says "ambiguous"; the detailed design takes precedence and the discrepancy is recorded. | accepted |
| Dev-1 | `AllocationStore` methods return `AllocationError`, not IFC 9's `StoreError` | Allocation-specific variants (budget exceeded, lost CAS) | accepted as recorded deviation |
| Dev-2 | `StorageKey::parse` is public | Tests and the reconciler need it; making it crate-private would leave dead code in the shell | accepted |
| Dev-3 | `FsFixtureStorage::open` takes the ledger (`Arc<dyn AllocationStore>`) | Register-before-write is checked at write time (FN-21/FN-22) | accepted as recorded deviation |
| Dev-4 | `AllocationStore::get` added (not in IFC 9) | Needed for the write-time ledger check | accepted |
| Dev-5 | The `AllocationLimits` type and the `DbAllocationStore::new` shape | Carries the owner-approved CFG-12/CFG-13 bounds | accepted |

## 5. Gaps and limits

- **File symlinks on Windows:** the four file-symlink tests skip on this host. On Windows only directory junctions are exercised; file symlinks are covered on Linux and in CI.
- **Unix permission tests (TS-31):** red only on Linux, and that run is builder-reported.
- **Residual races with no forced interleaving:**
  - `mark_written` racing `release` is a single-statement CAS with no hook between read and write.
  - A check-then-open swap at write time.
  - A write racing a release-and-delete after the ledger `get`; WU-06 TS-28 must cover it.
- **Bounded enumeration:** stopping after `max_entries + 1` entries is not proven, though the result bound is.
- **Root-level files:** a stray file at the root (outside `alloc/`) is invisible to inventory, per FD F-22.
- **Weak junction test:** `write_never_goes_through_a_link_at_the_key_path` uses a directory junction, which any create-new write refuses anyway.
- **Not green yet:** nothing in WU-04 is implemented. Build-and-verify must prove each guard by mutation, and must check that every mutation edit actually applied (retro lesson).
