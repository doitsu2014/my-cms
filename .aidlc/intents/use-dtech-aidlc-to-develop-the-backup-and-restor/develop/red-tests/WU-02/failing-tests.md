# FT-02 Failing tests (code manifest): WU-02

- **ID:** FT-02 (WU-02)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-02 of WU-01..WU-15.
- **Status:** red run recorded 2026-11-22 on PostgreSQL 15 (Docker available).
- **Inputs:** [work-units](../../implementation-design/work-units.md) WU-02, [test-strategy](../../implementation-design/test-strategy.md) TS-11 (DB part), TS-34, TS-66, [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) DB schema, status contract (D-10) and audit rules, [functional-design](../../implementation-design/functional-design.md) F-37.
- **Evidence:** [red-evidence.md](red-evidence.md), raw log [../evidence/red-wu02-cargo-test.log](../evidence/red-wu02-cargo-test.log).

Provenance: results are Observed from the conductor's run on 2026-11-22. The AC and threat (T) mappings follow test-strategy.

## Files

All new files are under `apps/api/domain_backup/`.

| Path | Change | Notes |
|---|---|---|
| `src/migrations/mod.rs` | new | `Migrator` listing the two migrations, `BACKUP_MIGRATION_IDS`, `migration_descriptors()`. `migration_table_name` not overridden yet. |
| `src/migrations/m20261122_000001_backup_core.rs` | new | shell: `up()`/`down()` return `Ok(())` |
| `src/migrations/m20261122_000002_backup_terminal_guards.rs` | new | shell: `up()`/`down()` return `Ok(())` |
| `src/migrations/tests.rs` | new | all WU-02 tests |
| `src/migrations_cli.rs` | new | `run`, `revert`, `list_identities` (thin wrappers over the Migrator) |
| `src/test_db.rs` | new, `#[cfg(test)]` | own testcontainer, `postgres:15-alpine`, connect with retry, no `test_helpers`, no posts migrations |
| `src/lib.rs` | modified | declares `migrations`, `migrations_cli`, test-only `test_db` |
| `Cargo.toml` | modified | adds `async-trait`, `domain_interface`, `sea-orm`, `sea-orm-migration`, `tokio`; dev-deps `testcontainers`, `testcontainers-modules`. Versions match the workspace lock. |
| `apps/api/Cargo.lock` | modified | lock entries for the above |

No WU-01 test module, gateway, other domain, `test_helpers` or web file is touched by WU-02. Entities (`src/entities/*`) are left to the build stage; no WU-02 test needs them.

## Tests

Command, from `apps/api`: `cargo test -p domain_backup --no-fail-fast` and `cargo test -p domain_backup --no-fail-fast -- --ignored`. All DB tests carry `#[ignore = "requires Docker for PostgreSQL testcontainer"]`. All tests are in `migrations::tests::`.

| Test | TS | AC | T | Observed (red) |
|---|---|---|---|---|
| `backup_uses_its_own_migration_table` | TS-34 | none | T-17 | FAILED: assertion, table name is the default (no Docker) |
| `migrator_lists_the_two_backup_migrations_in_order` | TS-34 | none | T-17 | passed: structural, not red evidence (no Docker) |
| `fresh_database_gets_all_five_backup_tables` | TS-34 | none | T-17 | FAILED: assertion, no tables |
| `migration_records_only_in_seaql_migrations_backup` | TS-34 | none | T-17 | FAILED: version table missing |
| `second_up_is_a_no_op` | TS-34 | none | T-17 | FAILED: version table missing |
| `migrations_cli_run_applies_on_a_fresh_database` | TS-34 | none | T-17 | FAILED: assertion |
| `upgrade_from_000001_to_000002` | TS-34 | none | T-17 | FAILED: version table missing |
| `down_one_step_removes_only_the_jobs_trigger` | TS-34, TS-66 | AC-005 S2 | T-08 | FAILED: version table missing |
| `upgrade_beside_posts_and_seo_leaves_their_tables_untouched` | TS-34 | none | T-17 | FAILED: "Migration file of version ... is missing" (the shell still shares the default table) |
| `backup_jobs_columns_match_the_contract` | TS-34 | AC-002 S3 | T-17 | FAILED: assertion, no columns |
| `other_backup_table_columns_match_the_contract` | TS-34 | AC-005 S2 | T-17 | FAILED: assertion, no columns |
| `backup_jobs_defaults_apply` | TS-34 | none | T-17 | FAILED: positive-control insert |
| `secondary_indexes_exist` | TS-34 | none | T-17 | FAILED: assertion |
| `both_guard_triggers_are_present` | TS-34 | AC-005 S2 | T-17, T-08 | FAILED: "jobs trigger missing" |
| `allocation_job_fk_and_column_defaults` | TS-34 | none | T-17 | FAILED: relation missing |
| `allocation_checks_and_epoch_singleton` | TS-34 | none | T-17 | FAILED: positive-control insert |
| `idempotency_unique_and_restrict_fk` | TS-34 | AC-005 S1 | T-07 | FAILED: positive-control insert |
| `terminal_job_without_idempotency_row_can_be_deleted_and_audit_has_no_fk` | TS-34 | AC-005 S2 | T-08 | FAILED: positive-control insert |
| `status_column_check_keeps_all_six_documented_values` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: relation missing |
| `status_column_accepts_every_documented_value_when_u1_guard_is_removed` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: relation missing |
| `u1_job_cannot_be_inserted_as_succeeded_or_recovery_required` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: positive-control insert |
| `u1_job_cannot_be_updated_to_succeeded_or_recovery_required` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: positive-control insert |
| `every_status_transition_pair_is_allowed_or_rejected_exactly` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: first insert |
| `backup_jobs_row_checks_reject_bad_rows` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: positive-control insert |
| `legal_transitions_are_allowed` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: first insert |
| `terminal_job_cannot_be_updated` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: first insert |
| `illegal_non_terminal_transitions_are_rejected` | TS-11 (DB) | AC-002 S3 | T-06 | FAILED: first insert |
| `only_one_active_job_per_instance` | TS-34 (index; behaviour is WU-03) | AC-002 S3 | T-07 | FAILED: "partial unique index missing" |
| `audit_rows_cannot_be_updated_or_deleted` | TS-66 | AC-005 S2 | T-08 | FAILED: positive-control insert |
| `audit_cannot_be_truncated` | TS-66 (D-14) | AC-005 S2 | T-08 | FAILED: positive-control insert |
| `audit_digest_is_char_64` | TS-34 | AC-005 S2 | T-09 | FAILED: positive-control insert |
| `down_refuses_while_audit_rows_exist_and_keeps_everything` | TS-66 | AC-005 S2 | T-08, T-09 | FAILED: positive-control insert |
| `down_without_audit_rows_keeps_audit_table_and_trigger_and_drops_the_rest` | TS-66 | AC-005 S2 | T-08 | FAILED: assertion, `backup_audit` absent |
| `down_refuses_while_an_allocation_is_not_deleted` | TS-66 | none | T-17 | FAILED: positive-control insert |
| `down_succeeds_when_every_allocation_is_deleted` | TS-66 | none | T-17 | FAILED: positive-control insert |

Totals (Observed):
- No-Docker run: 36 passed (35 WU-01 tests and 1 structural) and 1 failed.
- Docker run (`--ignored`): 33 failed and 0 passed.

**What the red proves and does not prove.** About 25 of the 33 DB tests fail first at a positive-control insert or a catalog query, because the shell creates no schema. That is the honest red for a migration that does nothing, but their target assertions (a specific rejection or a trigger message) have not yet been seen failing for the right reason. Build-and-verify must therefore add a mutation check for each guard once the schema is green: drop or weaken the guard, show its test fails, then restore it. Same practice as WU-01.
