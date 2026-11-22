//! Trigger guards (IFC sections 5.1 and 7).
//!
//! - `backup_jobs_terminal_immutable`: BEFORE UPDATE row trigger. A terminal row
//!   (`failed`, `incomplete`) is immutable; a status change must be one of
//!   queued->running, queued->failed, running->incomplete, running->failed.
//!   Anything else, including any move to `succeeded` or `recovery-required`, raises.
//!   A non-terminal row set to its own status is a no-op.
//! - `backup_audit_append_only`: one statement-level BEFORE UPDATE OR DELETE OR
//!   TRUNCATE trigger (D-14). It is statement-level because a row trigger does
//!   not fire on TRUNCATE; it therefore also fires for statements matching no row.
//!   Consequences for callers: never issue any UPDATE, DELETE or MERGE against
//!   `backup_audit` (a filtered DELETE matching nothing, `INSERT .. ON CONFLICT
//!   DO UPDATE` and `MERGE` all raise); use plain INSERT or ON CONFLICT DO NOTHING.
//!
//! Both functions pin `search_path` and are not SECURITY DEFINER. `down` removes
//! only the jobs guard and keeps the audit guard (RK-13, TS-66). `up` is
//! idempotent because the audit trigger survives a revert.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName, Debug)]
pub struct Migration;

const UP_STATEMENTS: &[&str] = &[
    "CREATE OR REPLACE FUNCTION backup_jobs_terminal_immutable_fn() RETURNS trigger
     LANGUAGE plpgsql
     SET search_path = pg_catalog, public
     AS $$
     BEGIN
         IF OLD.status IN ('failed', 'incomplete') THEN
             RAISE EXCEPTION 'backup_jobs_terminal_immutable: terminal job % cannot be updated', OLD.id;
         END IF;
         IF NEW.status IS DISTINCT FROM OLD.status AND NOT (
             (OLD.status = 'queued' AND NEW.status IN ('running', 'failed'))
             OR (OLD.status = 'running' AND NEW.status IN ('incomplete', 'failed'))
         ) THEN
             RAISE EXCEPTION 'backup_jobs_terminal_immutable: status transition % -> % is not allowed', OLD.status, NEW.status;
         END IF;
         RETURN NEW;
     END
     $$",
    "DROP TRIGGER IF EXISTS backup_jobs_terminal_immutable ON backup_jobs",
    "CREATE TRIGGER backup_jobs_terminal_immutable
     BEFORE UPDATE ON backup_jobs
     FOR EACH ROW EXECUTE FUNCTION backup_jobs_terminal_immutable_fn()",
    "CREATE OR REPLACE FUNCTION backup_audit_append_only_fn() RETURNS trigger
     LANGUAGE plpgsql
     SET search_path = pg_catalog, public
     AS $$
     BEGIN
         RAISE EXCEPTION 'backup_audit_append_only: backup_audit rows cannot be updated, deleted or truncated';
     END
     $$",
    "DROP TRIGGER IF EXISTS backup_audit_append_only ON backup_audit",
    "CREATE TRIGGER backup_audit_append_only
     BEFORE UPDATE OR DELETE OR TRUNCATE ON backup_audit
     FOR EACH STATEMENT EXECUTE FUNCTION backup_audit_append_only_fn()",
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let conn = manager.get_connection();
        for sql in UP_STATEMENTS {
            conn.execute_unprepared(sql).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Checked in both steps so every revert path, including a one-step
        // revert of this migration, is refused while audit rows or live
        // allocations exist. Atomicity comes from sea-orm's single transaction
        // per run on Postgres.
        super::refuse_unsafe_down(manager).await?;
        let conn = manager.get_connection();
        conn.execute_unprepared(
            "DROP TRIGGER IF EXISTS backup_jobs_terminal_immutable ON backup_jobs",
        )
        .await?;
        conn.execute_unprepared("DROP FUNCTION IF EXISTS backup_jobs_terminal_immutable_fn()")
            .await?;
        Ok(())
    }
}
