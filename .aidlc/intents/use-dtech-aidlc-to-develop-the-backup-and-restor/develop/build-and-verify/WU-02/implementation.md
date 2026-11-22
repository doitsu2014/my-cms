# IMPL-02 Implementation manifest: WU-02

- **ID:** IMPL-02 (WU-02)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-02 of WU-01..WU-15.
- **Status:** WU-02 implemented and verified on this host, 2026-11-22 (PostgreSQL 15 via Docker). Uncommitted, on branch `doitsu2014/migrate-exported-documents-to` (owner rule: branch-only until Launch).
- **Inputs:** [red-tests/WU-02/failing-tests.md](../../red-tests/WU-02/failing-tests.md), [red-evidence.md](../../red-tests/WU-02/red-evidence.md) (decisions D-14, D-15 and the builder [D] list), [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) DB schema, [work-units](../../implementation-design/work-units.md) WU-02.
- **Evidence:** [green-verification-report.md](green-verification-report.md), [regression-report.md](regression-report.md), [known-limitations.md](known-limitations.md).

## Files (WU-02 only)

| Path (under `apps/api/domain_backup/`) | Lines | Content |
|---|---|---|
| `src/migrations/mod.rs` | 88 | `Migrator` (two migrations), `migration_table_name()` = `seaql_migrations_backup`, `BACKUP_MIGRATION_IDS`, `migration_descriptors()`, `refuse_unsafe_down()` |
| `src/migrations/m20261122_000001_backup_core.rs` | 128 | tables, CHECKs, FKs, indexes; `down` drops everything except `backup_audit` |
| `src/migrations/m20261122_000002_backup_terminal_guards.rs` | 85 | guard functions and triggers; `down` drops only the jobs trigger and function |
| `src/migrations_cli.rs` | 12 | `run`, `revert`, `list_identities` |
| `src/migrations/tests.rs` | 1431 | WU-02 tests (red tests plus one amendment and one test-after, see the green report) |
| `src/test_db.rs` | 62 | test-only fixture: own `postgres:15-alpine` container, connect with retry |
| `src/lib.rs` | +5 | module wiring |
| `Cargo.toml` | +7 | `async-trait`, `domain_interface`, `sea-orm`, `sea-orm-migration`; dev: `testcontainers`, `testcontainers-modules`, `tokio`. All versions match the workspace lock. |
| `apps/api/Cargo.lock` | +7 | lock entries |

Entities (`src/entities/*`) were not added, because no WU-02 test needs them. They move to WU-03, which first reads and writes rows. The backup crate is still not registered in the gateway (WU-10), so the running API and its migrations are unchanged by WU-02.

## Schema implemented

| Object | Definition (summary) |
|---|---|
| `backup_jobs` | 18 contract columns. Defaults: `state_version` 0, `synthetic` true, `components` `'[]'`, `counters` `'{}'`. CHECKs: `operation = 'backup_db_slice'`; status in the six documented values (D-10); U1 guard `NOT (operation = 'backup_db_slice' AND status IN ('succeeded','recovery-required'))`; `synthetic = true`; `finished_at` set exactly when the status is terminal. Indexes: `(instance_id, created_at DESC)`, `(status)`, `(retain_until)`, and the partial unique `(instance_id) WHERE status IN ('queued','running')`. |
| `backup_idempotency` | `UNIQUE (actor_sub, operation, key_digest)`. `job_id` FK to `backup_jobs` `ON DELETE RESTRICT`. Index `(expires_at)`. |
| `backup_audit` | `IF NOT EXISTS`, no FKs, `digest char(64)`. Indexes `(job_id)`, `(occurred_at)`. |
| `backup_allocations` | Nullable `job_id` FK to `backup_jobs`. `UNIQUE (storage_key)`. CHECKs: `state` (7 values), `bytes_reserved` 0..268435456 (256 MiB), `cleanup_attempts <= 3`. Indexes `(state, next_attempt_at)`, `(job_id)`, `(expires_at)`. |
| `backup_epoch` | `id` primary key, `CHECK (id = 1)` (singleton) |
| `backup_jobs_terminal_immutable` | BEFORE UPDATE, FOR EACH ROW. Rejects any update of a terminal row. Allows only queued to running, queued to failed, running to incomplete and running to failed. A non-terminal same-status update is allowed (CAS bump). |
| `backup_audit_append_only` | BEFORE UPDATE OR DELETE OR TRUNCATE, FOR EACH STATEMENT (covers D-14). Every UPDATE, DELETE or TRUNCATE statement is rejected, even one matching zero rows. |
| Guard functions | plpgsql, `SET search_path = pg_catalog, public`, not SECURITY DEFINER, no dynamic SQL. Messages start with the trigger name. Triggers are re-created idempotently. |
| `refuse_unsafe_down()` | Takes `LOCK TABLE backup_audit, backup_allocations, backup_jobs IN SHARE ROW EXCLUSIVE MODE`. Refuses while `backup_audit` has rows or any allocation is not `deleted`. Called by both migrations' `down`. |

## How it was built (process record)

1. A builder (sonnet, `stage.model`) implemented the DDL. It got 32 of 33 DB tests green, then stopped on one test it judged defective without editing it, as instructed.
2. Two independent reviewers reported 9 findings:
   - the test defect, with the correct root cause;
   - a race in the down refusal;
   - wrong comments;
   - 5 design residuals.
3. The fixer:
   - added the `LOCK TABLE` (hardens the tested refusal; no change to tested behaviour);
   - corrected the comments;
   - ran a first mutation set (7 guards);
   - recorded the residuals.
4. A mutation agent tried 33 mutations across 15 guards. 29 were caught, and the 4 that were not are dispositioned in the green report.
5. The conductor:
   - amended the defective test helper (D-16);
   - added one test-after test;
   - proved both by mutation;
   - ran the final green, regression, fmt and clippy runs independently.
