# RG-17 Regression report: WU-17

- **ID:** RG-17 (WU-17)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-17.
- **Baseline:** [../../baseline.md](../../baseline.md) (2026-11-22: 290 normal passed, 39 of 39 ignored passed).
- **Attribution:** [regression.json](regression.json), `scope: full`.

## Whole-workspace runs (Observed, 2026-11-26, from `apps/api`, after the step was selected)

| Command | Exit | Result | Log |
|---|---|---|---|
| `cargo test --workspace --no-fail-fast` | 101 | `domain_backup` 133 passed / 7 failed / 163 ignored; every other crate passed | `evidence/regression-wu17-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored` | 101 | `domain_backup` 162 passed / 1 failed; every other crate passed | `evidence/regression-wu17-ignored.log` |

Compared with the previous full run (WU-11, same day): normal failures went from 13 to 7 and ignored failures from 3 to 1. The normal pass count is 133 (was 134) because the seven WU-17 shell tests were replaced: six failing and one (`ts24`, which passed against a no-op) were removed, and the 20 new tests are Docker-backed so they sit in the ignored count (162 passed, was 140).

## Failures and ownership

All 8 failures are lifecycle tests whose implementation does not exist yet. None belongs to WU-17 and none is in the baseline pass set.

| Owner | Count | Tests |
|---|---|---|
| WU-18 (cleaner) | 2 | `lifecycle_tests::ts26`, `ts27a` |
| WU-19 (sweeper and runtime) | 6 | `lifecycle_tests::ts29`, `ts30`, `ts32`, `ts42`, `ts65`; `infra::tests::reconciliation::ts30_deadline_inventory_includes_only_due_jobs_at_epoch` |

They are deferred failures: attributed to those units in `regression.json`, still failing, and re-run at the final sweep.

## Conclusion

WU-17 added 20 passing tests, turned the 8 WU-17-owned deferred failures green and changed no existing test result. The release-candidate regression criterion is not met yet because of the 8 deferred failures; that is for the final sweep.
