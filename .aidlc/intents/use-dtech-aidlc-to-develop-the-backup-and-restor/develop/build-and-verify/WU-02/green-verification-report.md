# GVR-02 Green verification report: WU-02

- **Stage:** Develop / build-and-verify, unit WU-02; not the release-candidate gate.
- **Status:** WU-02 migration-specific verification is green on Windows 11 with Docker Desktop/PostgreSQL 15, observed 2026-11-24. No WU-02 source or test changes were made; this refresh validates the historical implementation against current focused migration checks.
- **Environment:** `dtech-aidlc config prerequisite check` passed (`docker`, `docker info`); `docker info` passed. PostgreSQL 15 testcontainers ran successfully.
- **Historical evidence:** Original WU-02 green run (2026-11-22) recorded 37 normal tests passing and 34 ignored PostgreSQL tests passing, including the broader full-workspace baseline comparison. This refreshed run is limited to migration filters because later WU-06 lifecycle shells now fail in the current worktree.

## Current focused results

Commands run from the repository root:

| Command | Exit/result | Log |
|---|---:|---|
| `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup migrations::tests:: --no-fail-fast` | 0; 2 passed, 0 failed, 34 ignored | [normal filter](../evidence/refresh-wu02-domain-backup-normal.log) |
| `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup migrations::tests:: --no-fail-fast -- --ignored --nocapture` | 0; 34 passed, 0 failed, 0 ignored (PostgreSQL 15) | [Docker migration tests](../evidence/refresh-wu02-migrations-ignored.log) |
| `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` | 0 | [format](../evidence/refresh-wu02-cargo-fmt.log) |
| `cargo clippy --manifest-path apps/api/Cargo.toml -p domain_backup --all-targets` | 0, warnings | [clippy](../evidence/refresh-wu02-cargo-clippy.log) |
| `dtech-aidlc ids check --stage build-and-verify` | `ok: true`, no unregistered identifiers | CLI output observed; uncovered function IDs are reported by the tool and are not unregistered identifiers |

Clippy warnings are in later-unit WU-06/lifecycle and reconciliation seams in `allocation_store.rs`, `job_store.rs`, and lifecycle shell types (including unused parameters, missing `Debug`, `new_without_default`, and `too_many_arguments`). They do not originate in WU-02 migration files; this focused command was not a claim of a warning-free workspace.

## Separation from current later-unit failures

`cargo test --manifest-path apps/api/Cargo.toml -p domain_backup --no-fail-fast` was also run to determine the current crate state. It exited 101: 116 passed, 13 failed, 128 ignored. Every observed failure is in `lifecycle::lifecycle_tests` (TS-20/21/22/23/25/26/27a/29/30/32/42/56/65), not in `migrations::tests`. These are later WU-06/WU-17/18/19 lifecycle reconciliation/retention shells and are outside WU-02. Full output is [the normal crate log](../evidence/refresh-wu02-domain-backup-normal.log). This failure set is not attributed to WU-02, but the full current crate is not green.

## Coverage and trace

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-02, US-06 | AC-002 S3 — U1 database-only result / forbidden successful states | F-37 | `u1_job_cannot_be_inserted_as_succeeded_or_recovery_required`; `u1_job_cannot_be_updated_to_succeeded_or_recovery_required`; `status_column_check_keeps_all_six_documented_values`; `terminal_job_cannot_be_updated` | Passed in historical WU-02 evidence; this refreshed filter selected migration tests and ignored the DB-backed instances. |
| US-02, US-06 | AC-005 S1 — isolated migration identity and noninterference | F-37 | `backup_uses_its_own_migration_table`; `migration_records_only_in_seaql_migrations_backup`; `upgrade_beside_posts_and_seo_leaves_their_tables_untouched` | Passed in refreshed focused tests (migration table test observed in normal run; Docker test observed in ignored run). |
| US-05, US-06 | AC-005 S2 — durable append-only audit | F-37 | `audit_rows_cannot_be_updated_or_deleted`; `audit_cannot_be_truncated`; `down_without_audit_rows_keeps_audit_table_and_trigger_and_drops_the_rest` | Passed in refreshed Docker migration run. |
| US-05, US-06 | AC-005 S3 — audit unavailable before effect | WU-02 schema only | No WU-02 migration test independently proves runtime audit-before-effect behavior; that behavior belongs to later units. | Not run / outside WU-02 schema scope. |
| US-02, US-03, US-04 | AC-006 S2 — unresolved design blockers | F-37 | `defaults_match_owner_values`; `backup_uses_its_own_migration_table`; `upgrade_beside_posts_and_seo_leaves_their_tables_untouched` | Historical WU-02 evidence passed; current migration tests pass as above. |

**WU-02 disposition:** migration contract and PostgreSQL verification remain green. This is not an overall Develop/release-candidate approval; the current broader crate fails on later lifecycle work and WU-06 remains the active blocker.

## Refresh 2026-11-26

Re-verified after the 1.1.0 migration with no WU-02 source or test changes: migrations::tests 2 passed (non-ignored) and 34 passed (Docker/PostgreSQL, ignored). Whole-workspace regression and its attribution are in `regression-report.md` and `regression.json`.
