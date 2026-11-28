# RG-07 Regression report: WU-07

- **ID:** RG-07 (WU-07)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-07.
- **Baseline:** [../../baseline.md](../../baseline.md) (2026-11-22: 290 normal passed, 39 of 39 ignored passed).
- **Attribution:** [regression.json](regression.json), `scope: full`.

## Whole-workspace runs (Observed, 2026-11-26, from `apps/api`)

| Command | Exit | Result | Log |
|---|---|---|---|
| `cargo test --workspace --no-fail-fast` | 101 | `domain_backup` 134 passed / 13 failed / 143 ignored; every other crate passed | `evidence/regression-wu07-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored` | 101 | `domain_backup` 140 passed / 3 failed; every other crate passed | `evidence/regression-wu07-ignored.log` |

Compared with the previous full run (WU-01 refresh, same day): `domain_backup` normal 116 to 134 passed (+18, the new pure tests) and ignored 126 to 140 passed (+14, the new Docker tests). The failing set is unchanged: the same 13 and 3 tests.

## Failures and ownership

All 16 failures are lifecycle tests whose implementation does not exist yet. None is a WU-07 test, and none is in the baseline pass set (checked by name: 0 matches in `baseline.md`).

| Owner | Count | Tests |
|---|---|---|
| WU-17 (reconciler) | 8 | `lifecycle_tests::ts20`, `ts21`, `ts22`, `ts23`, `ts25`, `ts56`; `infra::tests::reconciliation::job_store_reconciliation_inventory_and_epoch_are_durable`, `...ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically` |
| WU-18 (cleaner) | 2 | `lifecycle_tests::ts26`, `ts27a` |
| WU-19 (sweeper and runtime) | 6 | `lifecycle_tests::ts29`, `ts30`, `ts32`, `ts42`, `ts65`; `infra::tests::reconciliation::ts30_deadline_inventory_includes_only_due_jobs_at_epoch` |

They are deferred failures: they are attributed to those units in `regression.json`, remain failures, and the final sweep re-runs both suites and reopens the owning unit if any is still red. `ts65` is declared by both WU-18 and WU-19 and is attributed to WU-19, the later unit.

## Conclusion

WU-07 added 32 tests, all green, and changed no existing test result. The release-candidate regression criterion is not met yet because of the 16 deferred failures; that is for the final sweep.

Note: the whole-workspace suites were run twice on this unchanged tree, once before and once after the CLI selected this step (the CLI requires the attribution file to be written after selection); both runs gave the same 134/13 and 140/3. The logs above are from the second run.
