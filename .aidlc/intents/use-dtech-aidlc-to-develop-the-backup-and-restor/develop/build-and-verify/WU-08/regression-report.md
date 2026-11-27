# WU-08 regression report

- **Status:** Full workspace normal and ignored suites passed on the final WU-08 working tree.
- **Comparison baseline:** `develop/baseline.md` (2026-11-22 baseline: 290 normal passes, 0 failures; 39/39 ignored pass). Baseline was preserved.

## Commands and results

| Command (from `apps/api`) | Observed result | Attribution |
|---|---|---|
| `cargo test --workspace --no-fail-fast` | Pass: 399 passed, 0 failed, 207 ignored across workspace. `domain_backup`: 141 passed, 0 failed, 202 ignored. | No regression failures. |
| `cargo test --workspace --no-fail-fast -- --ignored` | Pass: 207 ignored tests passed, 0 failed. `domain_backup`: 202 passed, 0 failed. | No regression failures. |
| Focused `cargo test -p domain_backup infra::tests::http_api:: -- --ignored --nocapture` | Pass: all 3 WU-08 HTTP integration tests. | TS-04/TS-49, TS-45/IF-11 assertions, TS-63 invalid UUID mapping. |

`failures` is empty in `regression.json`. The baseline's existing ignored tests remain ignored in the normal run and passed in the ignored run. Regression results establish suite health only; they do not close the WU-08 gaps recorded in `implementation.md` and `known-limitations.md`.
