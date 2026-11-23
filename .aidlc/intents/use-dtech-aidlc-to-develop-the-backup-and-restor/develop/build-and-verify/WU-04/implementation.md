# IMPL-04 Implementation manifest: WU-04

- **ID:** IMPL-04 (WU-04)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-04 of WU-01..WU-15.
- **Status:** WU-04 is implemented and verified on 2026-11-22. Tested on Windows with PostgreSQL 15 via Docker. Uncommitted, on branch `doitsu2014/migrate-exported-documents-to` (owner rule: branch-only until Launch).
- **Inputs:**
  - [red-tests/WU-04](../../red-tests/WU-04/failing-tests.md) (decisions D1..D6 and Dev-1..Dev-5)
  - [ids.json](../../implementation-design/ids.json)
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) 6.1, 6.1a, 6.2, 6.3, 8, 9
  - [functional-design](../../implementation-design/functional-design.md) FN-21..FN-24 (legacy F-21..F-24) and 4.4
- **Evidence:** [green-verification-report.md](green-verification-report.md), [regression-report.md](regression-report.md), [known-limitations.md](known-limitations.md).

## Files (production; tests are in the red manifest plus two additions)

| Path (under `apps/api/domain_backup/src/`) | Lines | Content |
|---|---|---|
| `infra/db_allocation_store.rs` | 583 | `DbAllocationStore`: the allocation ledger (FN-21, FN-23) |
| `infra/fs_fixture_storage.rs` | 295 | `FsFixtureStorage`: private opaque-key files (FN-22) |
| `infra/fs_namespace_lock.rs` | 120 | `FsNamespaceLock`: exclusive namespace ownership (FN-24) |
| `ports/fixture_storage.rs` | 144 | `StorageKey::parse`, `allocation_id`, `classify_entry` filled in. No signature changed. |
| `ports/allocation_store.rs`, `ports/namespace_lock.rs` | 165, 24 | Unchanged from red apart from doc notes |

- **Dependencies:** none added. `Cargo.toml` and `Cargo.lock` are unchanged.
- **Untouched:** WU-01..WU-03 code and tests are byte-identical (checked against the red snapshot).
- **Not yet wired in:** the gateway does not compose the crate yet (WU-10).

## Behaviour

- **Ledger (`DbAllocationStore`):**
  - All SQL is parameterised. Errors map through `pg_error` and the FD 6.6 split:
    - an audit rejection becomes `AuditUnavailable`;
    - an audit outage, or a store outage, becomes `StoreUnavailable`.
  - **`register`:**
    - Refuses a reservation above 256 MiB as `PerAllocation` before any SQL.
    - Otherwise runs one READ COMMITTED transaction. It takes `pg_advisory_xact_lock`, sums live bytes (every state except `deleted`), and refuses with `Total` above 1 GiB.
    - It then inserts the `registered` row, writes `allocation.registered` on the same transaction, and commits. Any audit failure rolls back.
  - **`mark_written` and `release`:** single-statement compare-and-set on `state` and `state_version`. Zero rows affected returns `Lost`.
  - **`claim_for_delete`:**
    - Claims a due `release_pending` row, or an `orphan_discovered` row past its grace, into `deleting`.
    - Stamps the epoch from `backup_epoch`, or keeps the row's own epoch when there is none.
    - Writes `allocation.delete_claimed` in the same transaction.
  - **`mark_deleted`:** moves `deleting` to `deleted` with `deleted_at` set, and writes `allocation.deleted` in the same transaction.
  - **`record_cleanup_failure`:**
    - A retry returns the row to `release_pending` with a 1-minute backoff.
    - The third failure moves it to `cleanup_failed` and writes `cleanup.exhausted`.
    - Time arithmetic is checked (`checked_add_signed`), so it never panics.
  - **`insert_orphan`:**
    - Idempotent: `ON CONFLICT (id) DO NOTHING`, so the first grace period stands.
    - Writes `orphan.discovered` in its own transaction and fails closed when audit fails.
    - Is never refused for budget.
  - **`due_for_cleanup`:** returns due rows, capped at 1000 per call.
- **Fixture storage (`FsFixtureStorage`):**
  - **`open`:**
    - Refuses a relative root, and any link, reparse point or non-directory.
    - Creates the root and `alloc/`, private (0700) on Unix. On Unix it also refuses existing directories that are group- or world-accessible.
  - **`write`:**
    - Asks the ledger first and fails closed: `NotRegistered` or `LedgerUnavailable`.
    - Requires state `registered` and size within the reservation.
    - Re-checks the directories for links.
    - Refuses anything already at the key path (`AlreadyExists` or `Ambiguous`).
    - Creates the file with `create_new` (0600 on Unix), syncs it, and removes it on failure.
  - **`delete`:**
    - Fails closed with `Unavailable` when the root or `alloc/` is missing (fixed in this stage, see below).
    - Matches the exact directory-entry name, so a case-folded twin is never removed.
    - Unlinks only a regular non-link file. Anything else is `Ambiguous`; a missing file is `AlreadyAbsent`.
  - **`inventory`:** stops after `max_entries + 1` entries and sets `truncated`. It reads `symlink_metadata`, so it never follows a link.
- **Classification (`classify_entry`):**

  | Entry | Class |
  |---|---|
  | Non-regular entry, or non-canonical name | `Ambiguous` |
  | Canonical name, no ledger row | `Orphan` (D6) |
  | Row is `registered`, `written`, `release_pending` or `orphan_discovered` | `Owned` |
  | Row is `deleting` | `OwnedInCleanup` |
  | Row is `deleted` or `cleanup_failed` | `Ambiguous` |
- **Namespace lock (`FsNamespaceLock`):**
  - The lock file is `<parent>/<root name>.lock`: a sibling of the root, outside the namespace.
  - It is created with `create_new` (0600 on Unix). An existing lock file must be a plain non-link file both before and after opening. On Windows the file is opened with `FILE_FLAG_OPEN_REPARSE_POINT`; on Unix the device and inode are compared.
  - The lock itself is `std::fs::File::try_lock`: `WouldBlock` means `AlreadyHeld`, and any other error is `Unavailable`. It is released on drop.

## How it was built (process record)

1. **Builder** (sonnet): briefed with the CLI directive verbatim, the `ids.json` rule, and an explicit ban on environment changes. All tests passed on its first pass.
2. **Reviewers:**
   - Integrity: no defect; tests byte-identical.
   - Correctness, security and concurrency: 3 major and 6 minor findings.
3. **Fixer:**
   - Applied 3 hardening changes: the Windows reparse-point flag on the lock open, READ COMMITTED pinned for `register`, and checked time arithmetic.
   - Handed back 3 as "needs test", and noted 2.
4. **Mutation pass:** 48 mutations across 24 guards, each confirmed applied before tests ran. 20 guards caught; 2 test gaps, 1 equivalent mutant, 4 not verifiable on Windows.
5. **Conductor:**
   - **Delete fail-open:** wrote a failing test (`delete_fails_closed_when_the_namespace_directories_are_missing`), saw it fail for the intended reason, then applied the one-line fix.
   - **Version check:** added the test-after `mark_written_with_a_wrong_version_on_a_registered_row_loses`.
   - Proved both by mutation, then ran the final green, regression, fmt, clippy and `ids check` itself.
