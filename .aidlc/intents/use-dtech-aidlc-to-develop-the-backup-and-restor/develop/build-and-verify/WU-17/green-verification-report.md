# GV-17 Green verification report: WU-17

- **ID:** GV-17 (WU-17)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-17.
- **Status:** green on the unit's own tests, 2026-11-26. Workspace regression: see [regression-report.md](regression-report.md).
- **Environment:** Windows 11, cargo 1.100.0, Docker for PostgreSQL 15. Logs are in [../evidence/](../evidence/).

## 1. Results (Observed)

| Command (from `apps/api`) | Exit | Result | Log |
|---|---|---|---|
| `cargo test -p domain_backup infra::tests::reconcil -- --ignored` | 101 | 25 passed, 1 failed. The 1 failure is `ts30_deadline_inventory_includes_only_due_jobs_at_epoch`, owned by WU-19. All 20 new reconciler tests and the 5 other store-level reconciliation tests pass, including the two WU-17-owned ones that were red (`job_store_reconciliation_inventory_and_epoch_are_durable`, `ts22_job_fence_cas_...`). | `green-wu17-focused-db.log` |
| `cargo fmt --all -- --check` | 0 | formatted | `green-wu17-fmt.log` |
| `cargo clippy --workspace --all-targets` | 0 | no warning in `reconciler.rs`, `db_job_store.rs` or `infra/tests/reconciler.rs`; the remaining warnings are in code that existed before WU-17 | `green-wu17-clippy.log` |
| `dtech-aidlc ids check --stage build-and-verify` | 0 | `ok: true` | n/a |

The focused run was made before the 20th test (`an_expired_written_allocation_of_a_running_job_is_not_released`) was added at green; that test passed on its own when added and in both full runs recorded in the regression report. It was added because the first mutation run showed no test covered a `written` allocation of a running job at expiry.

## 2. Tests assert durable state (Observed)

Each test seeds rows with raw SQL, puts real files in a real temp-root `FsFixtureStorage`, runs the reconciler and reads the rows, the audit table and the directory back. Audit order is read from a recording sink around the real `DbAuditSink`. Examples of what the green run proves:

- TS-20 and TS-21: the old-epoch running job ends `failed INTERRUPTED_RESTART` (version 4 to 5); its allocation ends `release_pending` (version 2 to 3); the audit order is `reconcile.fenced` then `allocation.reconciled`; the file is still where it was.
- TS-22: epoch 1 then 2; running and queued jobs are fenced with their own reason codes; a stale job snapshot or a stale allocation snapshot changes nothing and writes no event.
- TS-23: an unowned file becomes a graced orphan (`now + 1 h`) with one `orphan.discovered`; a stray file and a directory each raise `ownership.ambiguous`; nothing is deleted, also two hours later; the sweep scan finds a late file and does not fence or delete.
- TS-24: two full scans append two `reconcile.missing_file` rows with every IFC section 7 field; the allocation row is byte for byte unchanged; an audit failure fails the run.
- TS-25: a stale `deleting` row returns to `release_pending` with attempts unchanged.
- TS-56: a truncated listing leaves the job, allocation, row count, audit count and directory listing identical.

## 3. The tests can fail (mutation checks, Observed)

Each edit was applied to `reconciler.rs`, the 20 reconciler tests were run, and the file was restored byte for byte (checked with `cmp`). Raw result: `green-wu17-mutations.json`.

| Mutation | Tests that failed |
|---|---|
| a `registered` allocation is released although its owner is running | `current_epoch_running_job_and_its_registered_allocation_are_left_to_the_worker` |
| a `written` allocation is released although its owner is running | `an_expired_written_allocation_of_a_running_job_is_not_released` |
| the stale-claim reset ignores the epoch | `ts25_current_epoch_deleting_claim_belongs_to_the_live_cleaner` |
| a `cleanup_failed` allocation is released | `cleanup_failed_allocation_is_terminal_for_automation_and_never_touched` |
| the missing-file audit is written although the file is present | `ts24_unexpired_incomplete_allocation_with_its_file_present_is_retained_silently` |
| current-epoch jobs are fenced too | `ts23_sweep_discovery_...`, `current_epoch_running_job_...`, `an_expired_written_allocation_...` |
| truncation is ignored | `ts23_truncated_sweep_discovery_persists_nothing`, `ts56_truncated_inventory_fails_closed_before_any_write` |
| the missing-file audit skips the "owner is incomplete" check | none (survived) |

The surviving mutant is equivalent for every state U1 can reach: a `written` allocation whose owner is not running, not failed, not absent and not incomplete would need a `queued`, `succeeded` or `recovery-required` owner, and the state machine and the database constraint make those unreachable (D-10). The check is kept as a guard.

## 4. Not verified here

- The `delete_claimed` and `deleted` half of the TS-20, TS-21 and TS-22 audit sequence needs the cleaner (WU-18). WU-17 proves the reconciler leaves the file in place and writes neither event; WU-18 must extend the same scenarios.
- The reconciler is not called by any runtime yet (WU-19), so "admissions stay closed on failure" is shown as the returned error, not as a closed gate.
- A DB outage in the middle of a run is covered only by the store-level tests, not by a reconciler test.
- Windows only.
