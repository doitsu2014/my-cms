# WU-12 regression report

- **Status:** All suites pass on the integrated tree (2026-11-28), after WU-09 became satisfied.

| Command | Exit | Result | Log |
|---|---:|---|---|
| `pnpm test` (apps/web) | 0 | 20 files, 82 passed, 0 failed | `evidence/regression-wu12-web.log` |
| `cargo test --workspace --no-fail-fast` (apps/api) | 0 | 420 passed, 0 failed, 213 ignored | `evidence/regression-wu12-rust-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` (apps/api) | 0 | 213 passed, 0 failed | `evidence/regression-wu12-rust-ignored.log` |

Pre-existing and unchanged: 23 `tsc` errors in other web files and the `pnpm lint` config-load failure (see green-verification-report.md). No previously passing test fails.
