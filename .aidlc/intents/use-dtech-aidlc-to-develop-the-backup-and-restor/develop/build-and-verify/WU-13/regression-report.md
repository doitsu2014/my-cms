# WU-13 regression report

- **Status:** All suites pass on the integrated tree after WU-10 became satisfied (2026-11-28).

| Command | Exit | Result | Log |
|---|---:|---|---|
| `pnpm test` (apps/web) | 0 | 24 files, 102 passed, 0 failed (WU-12: 82) | `evidence/regression-wu13-web.log` |
| `cargo test --workspace --no-fail-fast` (apps/api) | 0 | 439 passed, 0 failed, 216 ignored | `evidence/regression-wu13-rust-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` (apps/api) | 0 | 216 passed, 0 failed | `evidence/regression-wu13-rust-ignored.log` |

No previously passing test fails. Pre-existing: 23 `tsc` errors in untouched files and the `pnpm lint` config-load failure.
