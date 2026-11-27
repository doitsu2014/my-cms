# RG-19 Regression report: WU-19

- **ID:** RG-19 (WU-19)
- **Stage:** Develop / build-and-verify, unit WU-19.
- **Baseline:** [../../baseline.md](../../baseline.md), recorded 2026-11-22: 290 normal passes / 0 failures / 39 ignored; all 39 ignored passed. The baseline was not overwritten.
- **Attribution:** [regression.json](regression.json), `scope: full`.

## Whole-workspace runs observed 2026-11-26

Commands run from `apps/api` after WU-19 build-and-verify was selected and rerun after the final deadline-release correction.

| Command | Exit | Result | Log |
|---|---:|---|---|
| `cargo test --workspace --no-fail-fast` | 0 | workspace passes; domain_backup 132 passed, 0 failed, 199 ignored; all other crate suites pass | `evidence/regression-wu19-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored` | 0 | all ignored workspace tests pass; domain_backup 199 passed, including new WU-19 coverage; all other ignored suites pass | `evidence/regression-wu19-ignored.log` |

No failures remain in either run, including the five WU-19-owned deferred failures listed in WU-18's earlier report. Relative to the unchanged baseline, every baseline-passing test still passes; the suite now includes the additional WU-17/18/19 Docker-backed tests. Existing ignored counts vary because the lifecycle shell tests were replaced with persisted-state integration tests.

## Conclusion

The full normal and ignored workspace regressions pass after WU-19's final fix. No baseline failure is newly introduced, and no test was skipped, weakened, or reclassified. The earlier red logs and WU-17/18 regression reports show the lifecycle failures present before WU-19; the final run resolves them.
