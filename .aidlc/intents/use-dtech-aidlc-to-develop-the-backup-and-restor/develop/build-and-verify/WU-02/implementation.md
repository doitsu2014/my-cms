# IMPL-02 Implementation manifest: WU-02

- **ID:** IMPL-02 (WU-02)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-02.
- **Status:** Existing implementation retained; no source or test edits made for this verification refresh (2026-11-24).
- **Scope:** Migration/schema implementation recorded below. The subsequent WU-06 design rework did not change WU-02 source. Current worktree contains later-unit WU-06 lifecycle and reconciliation tests/shells; their failures are reported separately and are not migration test failures.
- **Evidence:** [green verification](green-verification-report.md), [regression report](regression-report.md), [known limitations](known-limitations.md), and refreshed raw test logs under [evidence](../evidence/).

## WU-02 implementation retained

| Path (under `apps/api/domain_backup/`) | WU-02 responsibility |
|---|---|
| `src/migrations/mod.rs` | Backup migrator, isolated `seaql_migrations_backup` table, descriptors and guarded down migration |
| `src/migrations/m20261122_000001_backup_core.rs` | Core tables, constraints, foreign keys and indexes |
| `src/migrations/m20261122_000002_backup_terminal_guards.rs` | Terminal-state and append-only audit triggers |
| `src/migrations_cli.rs` | Migration run, revert and identity helpers |
| `src/migrations/tests.rs` | Migration contract and PostgreSQL integration tests |
| `src/test_db.rs` | PostgreSQL 15 testcontainer fixture |
| `src/lib.rs`, `Cargo.toml`, `apps/api/Cargo.lock` | Module and dependency wiring |

The schema includes `backup_jobs`, `backup_idempotency`, `backup_audit`, `backup_allocations`, and singleton `backup_epoch`; status and U1 safety checks; a partial active-job uniqueness index; audit immutability including TRUNCATE; and guarded down migrations that retain audit data. No WU-02 behavior was changed in this refresh. The implementation remains uncomposed into the gateway, as planned for WU-10.
