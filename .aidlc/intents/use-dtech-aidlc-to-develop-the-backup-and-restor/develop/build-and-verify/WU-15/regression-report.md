# WU-15 regression report

- **Status:** All suites pass on the integrated tree after WU-14 became satisfied (2026-11-28).

| Command | Exit | Result | Log |
|---|---:|---|---|
| `cargo test --workspace --no-fail-fast` (apps/api) | 0 | 440 passed, 0 failed, 219 ignored (WU-14: 439 / 216) | `evidence/regression-wu15-rust-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` (apps/api) | 0 | 219 passed, 0 failed | `evidence/regression-wu15-rust-ignored.log` |
| `pnpm test` (apps/web) | 0 | 33 files, 183 passed, 0 failed (WU-14: 179) | `evidence/regression-wu15-web.log` |

No previously passing test fails. Pre-existing and unchanged: 23 `tsc` errors in untouched web files, the `pnpm lint` config-load failure, and Docker start-up flakiness at high parallelism (runs use `--test-threads=4`).
