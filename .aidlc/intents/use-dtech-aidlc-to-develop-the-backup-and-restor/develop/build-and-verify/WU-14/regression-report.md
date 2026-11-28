# WU-14 regression report

- **Status:** All suites pass on the integrated tree after WU-13 became satisfied (2026-11-28).

| Command | Exit | Result | Log |
|---|---:|---|---|
| `pnpm test` (apps/web) | 0 | 32 files, 179 passed, 0 failed (WU-13: 102) | `evidence/regression-wu14-web.log` |
| `cargo test --workspace --no-fail-fast` (apps/api) | 0 | 439 passed, 0 failed, 216 ignored | `evidence/regression-wu14-rust-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` (apps/api) | 0 | 216 passed, 0 failed | `evidence/regression-wu14-rust-ignored.log` |

No previously passing test fails; WU-12 and WU-13 tests are unchanged and green. Pre-existing: 23 `tsc` errors in untouched files and the `pnpm lint` config-load failure.
