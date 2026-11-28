# WU-09 regression report

- **Status:** Full workspace normal and Docker-backed suites pass on the integrated tree (2026-11-28).
- **Baseline:** `develop/baseline.md`; previous unit run (WU-08): 404 normal / 208 Docker passed. Now 420 / 213 (16 new unit tests, 5 new Docker tests). No previously passing test fails.

| Command (from `apps/api`) | Exit | Result | Log |
|---|---:|---|---|
| `cargo test --workspace --no-fail-fast` | 0 | 420 passed, 0 failed, 213 ignored | `evidence/regression-wu09-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` | 0 | 213 passed, 0 failed | `evidence/regression-wu09-ignored.log` |

## Not omitted

- Run 1 (default parallelism): `http_api::ts63_...` and `reconciler::ts25_...` failed in `test_db.rs` before any assertion ("postgres never became ready ... unexpected response from SSLRequest").
- Run 2 (default parallelism): `job_store::terminal_rows_are_immutable_through_the_store` failed with `PortNotExposed` at container start.
- All three tests passed in the reduced-concurrency run on the same tree. These are test-container start-up failures, not product regressions; logs kept.
