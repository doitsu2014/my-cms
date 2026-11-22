# GVR-02 Green verification report: WU-02

- **ID:** GVR-02 (WU-02)
- **Stage:** Develop / build-and-verify, unit WU-02 of WU-01..WU-15. Not the release-candidate gate.
- **Status:** WU-02 green on this host, 2026-11-22.
- **Inputs:** [red-tests/WU-02](../../red-tests/WU-02/failing-tests.md), [implementation.md](implementation.md), [test-strategy](../../implementation-design/test-strategy.md), [acceptance-criteria](../../../analyze/problem-framing/acceptance-criteria.md), [quality-requirements](../../../analyze/context-and-risk/quality-requirements.md), [threat-model](../../../ideate/direction-selection/threat-model.md).
- **Logs:**
  - [evidence/green-wu02-cargo-test.log](../evidence/green-wu02-cargo-test.log)
  - [evidence/mutation-wu02/](../evidence/mutation-wu02/)
  - [evidence/mutation-wu02-test-after.log](../evidence/mutation-wu02-test-after.log)

Provenance: Observed from the conductor's runs on 2026-11-22. Host: Windows 11, cargo 1.100.0, Docker 29.7.2, tests on `postgres:15-alpine`. The mutation table rows marked "agent" were run and reported by the mutation and fixer agents (logs above). The conductor re-ran the two test-after mutations itself.

## 1. Red then green, same tests

- **Red** (2026-11-22, [log](../../red-tests/evidence/red-wu02-cargo-test.log)): 1 unit test and 33 DB tests failed on missing schema.
- **Green** (from `apps/api`):
  - `cargo test -p domain_backup --no-fail-fast`: exit 0, **37 passed, 0 failed, 34 ignored**.
  - `-- --ignored`: exit 0, **34 passed, 0 failed** (28 s).
- **Red tests unchanged except one recorded amendment.** `tests.rs` and `test_db.rs` were byte-identical to the red snapshot through the build and fix steps (checked by `cmp`). After that, the conductor made two changes:
  - **D-16 amendment, `columns()` helper.** It now sorts the fetched rows in Rust, so both sides of the comparison use byte order. SQL `ORDER BY column_name` puts `state` before `state_version`, while the expected list (the joined `name|type|null` strings, sorted in Rust) puts it after. The same exact set of columns, types, lengths and nullability is still compared, so nothing is weakened. The defect was invisible at red because the tables did not exist.
  - **One test-after test,** `idempotency_scope_ignores_request_fingerprint` (section 3).

  No other test was changed, skipped or ignored.

## 2. Results per test level

| Level | Result |
|---|---|
| Unit (no Docker) | 37 passed (35 WU-01 tests, table name, structural migrator list) |
| Integration (DB, PostgreSQL 15) | 34 passed, 0 failed |
| End to end | not run: no WU-02 E2E. Gateway composition with backup is WU-10. |
| Performance | not run: not in scope |
| Security | DB-level guards verified: U1 status guard (T-06), audit append-only incl. TRUNCATE (T-08), revert keeps audit (T-08/T-09), separate migration table (T-17). Owner or superuser bypass is out of reach of DB triggers (known limitation L-05). |
| Static | `cargo fmt -- --check` exit 0; `cargo clippy --all-targets` exit 0, no warnings from `domain_backup` |

## 3. Mutation proof (red-integrity follow-up)

Most WU-02 DB tests were red only on missing schema. Each guard was therefore weakened on the green schema to show its target assertion fails. The code was then restored and confirmed byte-identical with `cmp`.

| Guard | Mutations | Caught | By |
|---|---|---|---|
| Own migration table | 1 | yes | `backup_uses_its_own_migration_table` and 8 DB tests (agent) |
| Status values CHECK (D-10) | 3 (drop `incomplete`, drop `recovery-required`, `CHECK (true)`) | yes | `status_column_check_keeps_all_six_documented_values` and others (agent) |
| U1 status guard | 2 | yes | `u1_job_cannot_be_inserted_as_succeeded_or_recovery_required` and others (agent) |
| Terminal-immutable trigger | 3 (no-op, `IF false`, `UPDATE OF id`) | yes | `terminal_job_cannot_be_updated`, `every_status_transition_pair_is_allowed_or_rejected_exactly` (agent) |
| Transition rule | 3 (allow queued to incomplete, running to queued, running to succeeded) | 2 of 3 | `illegal_non_terminal_transitions_are_rejected`. **running to succeeded is not caught** because the U1 CHECK rejects it independently (defence in depth). Accepted. |
| Audit UPDATE/DELETE | 3 | yes | `audit_rows_cannot_be_updated_or_deleted`, `both_guard_triggers_are_present` (agent) |
| Audit TRUNCATE (D-14) | 1 | yes | `audit_cannot_be_truncated` (agent) |
| Partial unique concurrency index | 4 | yes | `only_one_active_job_per_instance` (agent) |
| Idempotency UNIQUE scope | 3 | **2 of 3 by agent; widened-scope mutation caught after test-after** | `idempotency_unique_and_restrict_fk`. Widening with `request_fingerprint` was missed at first and is now caught by `idempotency_scope_ignores_request_fingerprint` (conductor run, [log](../evidence/mutation-wu02-test-after.log)). |
| Idempotency FK RESTRICT | 2 | yes | `idempotency_unique_and_restrict_fk` (agent) |
| Down refusal: audit rows | 2 | yes | `down_refuses_while_audit_rows_exist_and_keeps_everything` (agent) |
| Down refusal: allocations | 3 | yes | `down_refuses_while_an_allocation_is_not_deleted` (agent) |
| Down keeps `backup_audit` | 1 | yes | `down_without_audit_rows_keeps_audit_table_and_trigger_and_drops_the_rest` (agent) |
| Idempotent re-up | 4 | 3 of 4 | `down_without_audit_rows_...` (42P07/42710). **Jobs trigger without `DROP TRIGGER IF EXISTS` is not caught**: every revert path drops that trigger with its function or table, so no reachable re-up collides. Accepted as unreachable. |
| Column types and lengths | 3 | **2 of 3 by agent; `key_digest` caught after D-16** | `audit_digest_is_char_64`, `backup_jobs_columns_match_the_contract`. `key_digest char(64)` to `text` was masked by the D-16 defect, and is now caught by `other_backup_table_columns_match_the_contract` (conductor run). |

**Result:** 31 of 33 mutations are caught. The 2 uncaught are explained (defence in depth; unreachable path). No guard is left unproven.

## 4. Quality requirements

| QR | Status after WU-02 |
|---|---|
| A-01 Job idempotency | Partial: the DB scope (UNIQUE triple, FK RESTRICT) is verified. Admission behaviour comes in WU-07. |
| P-05 Concurrent job limits | Partial: the partial unique index enforces one active job per instance at DB level. Admission handling comes in WU-03/WU-05. |
| PR-04 Audit content safety | Partial: the audit table is append-only (UPDATE, DELETE, TRUNCATE) and `digest` is fixed length. Content rules come in WU-03/WU-08. |
| A-03 Crash recovery | Partial: the allocation ledger schema and singleton epoch exist. Reconciliation is WU-06. |
| O-07 Configuration validation on startup | unchanged since WU-01 |
| Others | not in WU-02 scope; not run |

## 5. Codebase map

`domain_backup` gains migrations, but it is still not composed into the gateway. The map is not updated in this unit and will be updated in WU-10 (same decision as WU-01). `PROJECT.md` already names `seaql_migrations_backup` in its conventions.

## 6. Trace table

One row per acceptance criterion, cumulative over completed units WU-01 and WU-02.

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-01, US-05 | AC-001 Instance selection and authorization | F-01 (instance config) | WU-01 config tests. Listing and authorization in WU-08/WU-13. | config part passed; rest not run |
| US-02, US-06 | AC-002 S1 Job status is observable | F-37 (schema) | `backup_jobs_columns_match_the_contract`. Behaviour in WU-05/WU-08/WU-14. | schema passed; behaviour not run |
| US-02, US-06 | AC-002 S2 Required capture fails | F-01 (bounds) | WU-01 bounds tests. Outcome in WU-05. | bounds passed; outcome not run |
| US-02, US-06 | AC-002 S3 U1 database-only result | F-15, F-37 | WU-01 guard tests; `u1_job_cannot_be_inserted_as_succeeded_or_recovery_required`, `u1_job_cannot_be_updated_to_succeeded_or_recovery_required`, `every_status_transition_pair_is_allowed_or_rejected_exactly`, `status_column_check_keeps_all_six_documented_values`, `terminal_job_cannot_be_updated` | passed (pure guards and DB guards) |
| US-03 | AC-003 S1 Invalid or incompatible package | none (rejection previews in WU-09) | none yet | not run |
| US-03 | AC-003 S2 Unsafe archive | none (deferred) | none | not run (deferred) |
| US-02, US-03, US-04, US-06 | AC-003 S3 U1 real-data staging is not enabled | F-01; allocation schema | WU-01 config tests; `allocation_checks_and_epoch_singleton` (256 MiB cap) | passed (config fail-closed; byte cap in schema) |
| US-04 | AC-004 S1 Confirmation describes impact | none yet (WU-09) | none yet | not run |
| US-04 | AC-004 S2 Restore is cancelled | none yet (WU-09) | none yet | not run |
| US-04 | AC-004 S3 Restore failure | none (restore not authorized) | none | not run (deferred) |
| US-05, US-06 | AC-005 S1 Unauthorized operation or resource access | F-39, F-01; idempotency scope | WU-01 loopback/origin tests; `idempotency_unique_and_restrict_fk`, `idempotency_scope_ignores_request_fingerprint` | passed (guard, config, DB scope); authorization not run |
| US-05, US-06 | AC-005 S2 Audit records a sensitive operation | F-37 (audit schema) | `audit_rows_cannot_be_updated_or_deleted`, `audit_cannot_be_truncated`, `audit_digest_is_char_64`, `down_refuses_while_audit_rows_exist_and_keeps_everything`, `down_without_audit_rows_keeps_audit_table_and_trigger_and_drops_the_rest` | passed (retention and append-only); writing audit records in WU-03/WU-05/WU-07 |
| US-05, US-06 | AC-005 S3 Audit store unavailable before effect | none yet (WU-07/WU-06) | none yet | not run |
| US-02, US-03, US-04 | AC-006 S1 Production restore readiness | none (production not authorized) | none | not run (out of scope) |
| US-02, US-03, US-04 | AC-006 S2 Unresolved U1 design blockers | F-01, F-37 | `defaults_match_owner_values`; `backup_uses_its_own_migration_table`; `upgrade_beside_posts_and_seo_leaves_their_tables_untouched` | passed |
