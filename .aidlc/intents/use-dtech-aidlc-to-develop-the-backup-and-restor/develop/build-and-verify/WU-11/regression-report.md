# RG-11 Regression report: WU-11

- **ID:** RG-11 (WU-11)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-11.
- **Baseline:** [../../baseline.md](../../baseline.md) (2026-11-22: 290 normal passed, 39 of 39 ignored passed).
- **Attribution:** [regression.json](regression.json), `scope: full`.

## Whole-workspace runs (Observed, 2026-11-26, from `apps/api`, after the step was selected)

| Command | Exit | Result | Log |
|---|---|---|---|
| `cargo test --workspace --no-fail-fast` | 101 | `domain_backup` 134 passed / 13 failed / 143 ignored; the gateway binary 22 passed (was 17); every other crate passed | `evidence/regression-wu11-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored` | 101 | `domain_backup` 140 passed / 3 failed; every other crate passed | `evidence/regression-wu11-ignored.log` |

Compared with the previous full run (WU-07, same day): the `domain_backup` counts are identical and the failing set is the same 13 and 3 tests; the only difference is the gateway's +5 passing tests.

## Failures and ownership

All 16 failures are lifecycle tests whose implementation does not exist yet. None belongs to WU-11 and none is in the baseline pass set (checked by name earlier: 0 matches in `baseline.md`).

| Owner | Count | Tests |
|---|---|---|
| WU-17 (reconciler) | 8 | `lifecycle_tests::ts20`, `ts21`, `ts22`, `ts23`, `ts25`, `ts56`; `infra::tests::reconciliation::job_store_reconciliation_inventory_and_epoch_are_durable`, `...ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically` |
| WU-18 (cleaner) | 2 | `lifecycle_tests::ts26`, `ts27a` |
| WU-19 (sweeper and runtime) | 6 | `lifecycle_tests::ts29`, `ts30`, `ts32`, `ts42`, `ts65`; `infra::tests::reconciliation::ts30_deadline_inventory_includes_only_due_jobs_at_epoch` |

They are deferred failures: attributed to those units in `regression.json`, still failing, and re-run at the final sweep, which reopens the owning unit if any is still red.

## Conclusion

WU-11 added 5 passing tests and changed no existing test result. The release-candidate regression criterion is not met yet because of the 16 deferred failures; that is for the final sweep.
