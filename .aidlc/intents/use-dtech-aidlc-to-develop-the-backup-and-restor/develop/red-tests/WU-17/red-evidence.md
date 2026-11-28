# RE-17 Red evidence: WU-17

- **ID:** RE-17 (WU-17)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-17.
- **Recorded:** 2026-11-26, Windows 11, cargo 1.100.0, Docker for PostgreSQL 15.
- **Manifest:** [failing-tests.md](failing-tests.md)

## Runs (Observed)

| Command (from `apps/api`) | Exit | Result | Log |
|---|---|---|---|
| `cargo test -p domain_backup infra::tests::reconciler -- --ignored` | 101 | 19 run: 15 failed, 4 passed | `build-and-verify/evidence/red-wu17-reconciler-db-ignored.log` |
| `cargo test -p domain_backup infra::tests::reconciliation -- --ignored` | 101 | 7 run: 3 failed (`job_store_reconciliation_inventory_and_epoch_are_durable`, `ts22_job_fence_cas_...`, `ts30_deadline_inventory_...`), 4 passed | `build-and-verify/evidence/red-wu17-jobstore-db-ignored.log` |
| `cargo test -p domain_backup` | 101 | 133 passed, 7 failed, 162 ignored. The 7 failures are the WU-18 and WU-19 shell tests (`ts26`, `ts27a`, `ts29`, `ts30`, `ts32`, `ts42`, `ts65`). None belongs to WU-17. | `build-and-verify/evidence/red-wu17-cargo-test.log` |

The code compiled before every run. The failing tests fail on assertions about durable state or returned results, not on setup, compile or import errors.

## Why each failure is behavioral

The shell does nothing and returns defaults, so each test fails at its first behavioral assertion:

| Failure | First failing assertion |
|---|---|
| `ts20`, `ts21` | job still `running`, expected `failed` |
| `ts22_restart_advances_epoch...` | `advance_epoch` returned 0, expected 1 |
| `ts22_lost_*_cas...` | the stale-snapshot double never fired because the reconciler never read the inventory (version 4 or 2 where 5 or 3 was expected) |
| `ts23_*orphan*`, `ts23_sweep_*` | no orphan row persisted (0 of 1) |
| `ts23_truncated_sweep_*`, `ts56_*` | `Ok(..)` where `Err(InventoryTruncated)` was required |
| `ts24_unexpired_*_audits_each_full_scan` | `missing_files` 0, expected 1 per scan |
| `ts24_failed_owner_or_expired_*` | allocation still `written`, expected `release_pending` |
| `ts24_missing_file_audit_failure_*` | `Ok(..)` where `Err(AuditUnavailable)` was required |
| `ts25_stale_deleting_*` | allocation still `deleting`, expected `release_pending` |
| `ts44b_ownerless_*` | allocation still `registered`, expected `release_pending` |
| `advance_epoch_is_durable...` | `Ok(0)`, expected `Ok(1)` |
| store-level (existing) `job_store_reconciliation_inventory_and_epoch_are_durable`, `ts22_job_fence_cas_...` | epoch 0 instead of 1; fence returns `Lost` instead of `Applied` (inert `JobStore` defaults) |

## The four tests that pass against the shell

`ts24_unexpired_incomplete_allocation_with_its_file_present_is_retained_silently`, `current_epoch_running_job_and_its_registered_allocation_are_left_to_the_worker`, `cleanup_failed_allocation_is_terminal_for_automation_and_never_touched` and `ts25_current_epoch_deleting_claim_belongs_to_the_live_cleaner` assert that rows stay unchanged and no audit is written. A function that does nothing satisfies them. They are guards against over-eager reconciliation, not red evidence. The green stage must show they can fail (mutation checks) before they are counted.

## Decisions taken in the tests (the design is silent on them)

- `reconcile.fenced` is written with outcome `success`. IFC 6.1a fixes the outcome set at `success`, `failure`, `denied` and names no outcome for the fence event; a completed fence is a success.
- `ownership.ambiguous` is written with outcome `failure`, no job id, the allocation id only when the name parses as a UUID, and the operation `reconciliation`. The design asks for the incident and the audit event but not its fields.
- A stale `deleting` row of an older epoch always returns to `release_pending`, also when its file is absent. IFC 6.1a lists "absent -> deleted"; the tests route it through `release_pending` so the cleaner (WU-18) writes the audited claim and `allocation.deleted` in one place. The end state is the same (`deleted`).
- The `delete_claimed` and `deleted` half of the TS-20, TS-21 and TS-22 audit sequence needs the cleaner (WU-18). WU-17 asserts the reconciler leaves the file in place and writes neither event; WU-18 extends the same scenarios to the full sequence.

## Not covered by this red run

- `JobStore::list_deadline_expired` (`ts30_deadline_inventory_includes_only_due_jobs_at_epoch`) is owned by WU-19 and stays red.
- Namespace-lock and runtime sequencing (TS-29, TS-32) belong to WU-19.
- Platform: Windows only.
