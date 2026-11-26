# REG-02 Regression report: WU-02

- **Status:** No migration-specific regression observed. Historical WU-02 full-workspace regression and baseline remain the cross-unit baseline; this refresh did not rerun or overwrite the baseline.
- **Baseline:** [Intent develop baseline](../../baseline.md), recorded once on 2026-11-22. Rust workspace was 290 passed, 0 failed, 39 ignored; all 39 ignored Docker tests passed. `cargo fmt -- --check` and `cargo clippy --all-targets` exited 0. Historical WU-02 report records a post-WU-02 state of 290/0/39 with 39/39 ignored passing.
- **Current source scope:** No WU-02 migration source or tests changed. Later WU-06 and WU-16 lifecycle work exists in the current tree, so the full crate is no longer representative of the WU-02-only change comparison.

## Current observed regression-focused checks

| Command, from repository root | Result | Log |
|---|---|---|
| `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup migrations::tests:: --no-fail-fast` | Exit 0; 2 passed, 34 ignored | [normal migration filter](../evidence/refresh-wu02-domain-backup-normal.log) |
| `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup migrations::tests:: --no-fail-fast -- --ignored --nocapture` | Exit 0; 34 PostgreSQL 15 tests passed | [Docker migration filter](../evidence/refresh-wu02-migrations-ignored.log) |
| `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` | Exit 0 | [format](../evidence/refresh-wu02-cargo-fmt.log) |
| `cargo clippy --manifest-path apps/api/Cargo.toml -p domain_backup --all-targets` | Exit 0, warnings | [clippy](../evidence/refresh-wu02-cargo-clippy.log) |

The focused migration tests cover the WU-02 target and pass. Historical WU-02 evidence establishes the wider regression result at completion. This verification refresh did not rerun the full Rust workspace, web checks, or baseline suites.

## Current worktree delta outside WU-02

`cargo test --manifest-path apps/api/Cargo.toml -p domain_backup --no-fail-fast` exited 101 with 116 passed, 13 failed and 128 ignored. All 13 failures are lifecycle test functions for later WU-06/WU-16 behavior (TS-20/21/22/23/25/26/27a/29/30/32/42/56/65); no migration test failed. These shell/lifecycle failures are current later-unit worktree state, not attributed to WU-02 or the migration baseline. They do mean the present whole domain_backup crate and consequently the overall current regression are not green.

**WU-02 regression conclusion:** green for focused migration tests and consistent with its historical full-workspace baseline; broader current-worktree regression is blocked by later-unit lifecycle tests and is not a WU-02 regression finding.
