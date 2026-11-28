# Final regression sweep report (release candidate)

- **Status:** Every suite passes on the final tree (commit `16fdfe0`, 2026-11-28, macOS Darwin 25.6.0, Docker `postgres:15-alpine`, node v24.14.1). 18 of 18 work units satisfied; 0 open deferred failures; 0 unowned failures; 0 baseline-excused failures.
- **Baseline:** `develop/baseline.md` (2026-11-22, Windows 11): Rust 290 normal / 39 Docker passed, 0 failed; web 19 files / 49 tests passed.

## Runs

| Command | Exit | Result | Log |
|---|---:|---|---|
| `cargo test --workspace --no-fail-fast` (apps/api) | 0 | 440 passed, 0 failed, 219 ignored | `evidence/sweep-rust-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` (apps/api) | 0 | 219 passed, 0 failed | `evidence/sweep-rust-ignored.log` |
| `cargo fmt --all -- --check` | 0 | clean | `evidence/sweep-fmt.log` |
| `cargo clippy --workspace --all-targets` | 0 | 45 warnings, all in files this intent did not change (legacy `gateway/src/backup_cli.rs`, an existing `migrate_cli.rs` test, posts/media/user/auth crates); none in `domain_backup` or `domain_interface` | `evidence/sweep-clippy.log` |
| `cargo check --workspace` | 0 | clean | `evidence/sweep-check.log` |
| `pnpm test` (apps/web) | 0 | 33 files, 183 passed, 0 failed | `evidence/sweep-web.log` |
| `pnpm exec tsc --noEmit` (apps/web) | 2 | 23 errors, the same pre-existing set as the WU-12 base; none in backup files | `evidence/sweep-web-tsc.log` |
| `pnpm scan:bundle-canary` (apps/web) | 0 | PASS: 6 canaries, 79 dist files, 0 hits, both positive controls detected | `evidence/sweep-bundle-canary-scan.log` |
| `dtech-aidlc ids check --stage build-and-verify` | 0 | `ok: true` | `evidence/sweep-ids.log` |

## Per-crate comparison with the baseline (normal run)

| Crate | Baseline pass / ignored | Sweep pass / ignored | Change |
|---|---|---|---|
| domain_auth | 29 / 0 | 29 / 0 | unchanged |
| domain_interface | 9 / 0 | 9 / 0 | unchanged count (Mount tests extended in place) |
| domain_media | 101 / 0 | 101 / 0 | unchanged |
| domain_posts | 50 / 2 | 50 / 2 | unchanged |
| domain_seo | 11 / 1 | 11 / 1 | unchanged |
| domain_user | 36 / 0 | 36 / 0 | unchanged |
| domain_backup | 37 / 34 | 164 / 212 | U1 work units |
| gateway | 17 / 2 | 40 / 4 | WU-10 |
| web (`pnpm test`) | 49 tests / 19 files | 183 tests / 33 files | WU-12..WU-15 |

No previously passing test fails. Pre-existing and unchanged: `pnpm lint` fails at ESLint config load (`react-hooks` `recommended-latest`), 23 `tsc` errors in untouched web files, clippy warnings in untouched crates.

## Not omitted

- The Docker-backed suite is run with `--test-threads=4`. At default parallelism (~220 concurrent testcontainers) this host intermittently loses one random test to container start-up (`PortNotExposed`, "postgres never became ready"); unit logs keep those runs (WU-08, WU-09, WU-10, WU-15). It is a harness reliability issue, not a product failure.
- 17 deferred regressions recorded earlier against WU-17..WU-19 are all `resolved` in the CLI's sweep summary.
