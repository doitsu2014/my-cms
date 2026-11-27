# WU-08 regression report

- **Status:** Full workspace normal and Docker-backed ignored suites passed on the final WU-08 working tree (2026-11-27).
- **Comparison baseline:** `develop/baseline.md` (2026-11-22: 290 normal passed, 0 failed; 39/39 ignored passed). Baseline was preserved.
- **Attribution:** `regression.json`, scope `full`, empty failures array.

## Final regression runs

Commands run from `apps/api` after the final WU-08 source/test changes:

| Command | Exit | Result | Log |
|---|---:|---|---|
| `cargo test --workspace --no-fail-fast` | 0 | 399 passed, 0 failed; 207 ignored | `evidence/regression-wu08-final-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored` | 0 | 207 ignored tests passed, 0 failed; domain_backup 202 passed | `evidence/regression-wu08-final-ignored.log` |

The domain_backup normal suite passes 142/142; its ignored suite passes 202/202. Focused HTTP integration tests pass 3/3, projection tests pass 3/3, and metric recorder tests pass 2/2. Formatting, crate clippy, workspace check and `ids check` pass (see `green-verification-report.md`).

No regression failure was observed against the preserved baseline. This demonstrates test-suite health only; it does not complete WU-08's remaining contract and production-composition work. No failures were omitted or reclassified.
