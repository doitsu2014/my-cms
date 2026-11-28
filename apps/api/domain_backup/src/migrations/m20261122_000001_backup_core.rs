//! Core backup schema: five tables, constraints and indexes (IFC section 5).
//! Triggers belong to `m20261122_000002_backup_terminal_guards`.
//!
//! `backup_audit` is created idempotently and is never dropped by `down`
//! (audit retention is an open owner decision, RK-13), so a later `up` after a
//! `down` must tolerate the surviving table.

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName, Debug)]
pub struct Migration;

const UP_STATEMENTS: &[&str] = &[
    "CREATE TABLE backup_jobs (
        id uuid PRIMARY KEY,
        actor_sub text NOT NULL,
        instance_id text NOT NULL,
        operation text NOT NULL,
        status text NOT NULL,
        state_version bigint NOT NULL DEFAULT 0,
        synthetic boolean NOT NULL DEFAULT true,
        reason_code text NULL,
        components jsonb NOT NULL DEFAULT '[]'::jsonb,
        counters jsonb NOT NULL DEFAULT '{}'::jsonb,
        epoch bigint NOT NULL,
        correlation_id text NOT NULL,
        created_at timestamptz NOT NULL,
        updated_at timestamptz NOT NULL,
        started_at timestamptz NULL,
        finished_at timestamptz NULL,
        deadline_at timestamptz NULL,
        retain_until timestamptz NOT NULL,
        CONSTRAINT backup_jobs_operation_check CHECK (operation = 'backup_db_slice'),
        CONSTRAINT backup_jobs_status_check CHECK (status IN ('queued', 'running', 'succeeded', 'failed', 'recovery-required', 'incomplete')),
        CONSTRAINT backup_jobs_u1_status_guard CHECK (NOT (operation = 'backup_db_slice' AND status IN ('succeeded', 'recovery-required'))),
        CONSTRAINT backup_jobs_synthetic_check CHECK (synthetic = true),
        CONSTRAINT backup_jobs_terminal_finished_check CHECK ((status IN ('incomplete', 'failed')) = (finished_at IS NOT NULL))
    )",
    "CREATE INDEX backup_jobs_instance_created_idx ON backup_jobs (instance_id, created_at DESC)",
    "CREATE INDEX backup_jobs_status_idx ON backup_jobs (status)",
    "CREATE INDEX backup_jobs_retain_until_idx ON backup_jobs (retain_until)",
    "CREATE UNIQUE INDEX backup_jobs_one_active_per_instance ON backup_jobs (instance_id) WHERE status IN ('queued', 'running')",
    "CREATE TABLE backup_idempotency (
        id uuid PRIMARY KEY,
        actor_sub text NOT NULL,
        operation text NOT NULL,
        key_digest char(64) NOT NULL,
        request_fingerprint char(64) NOT NULL,
        job_id uuid NOT NULL REFERENCES backup_jobs (id) ON DELETE RESTRICT,
        created_at timestamptz NOT NULL,
        expires_at timestamptz NOT NULL,
        CONSTRAINT backup_idempotency_scope_unique UNIQUE (actor_sub, operation, key_digest)
    )",
    "CREATE INDEX backup_idempotency_expires_at_idx ON backup_idempotency (expires_at)",
    "CREATE TABLE IF NOT EXISTS backup_audit (
        id uuid PRIMARY KEY,
        occurred_at timestamptz NOT NULL,
        actor_sub text NULL,
        instance_id text NOT NULL,
        job_id uuid NULL,
        allocation_id uuid NULL,
        operation text NOT NULL,
        event text NOT NULL,
        outcome text NOT NULL,
        reason_code text NULL,
        digest char(64) NULL,
        correlation_id text NOT NULL
    )",
    "CREATE INDEX IF NOT EXISTS backup_audit_job_id_idx ON backup_audit (job_id)",
    "CREATE INDEX IF NOT EXISTS backup_audit_occurred_at_idx ON backup_audit (occurred_at)",
    "CREATE TABLE backup_allocations (
        id uuid PRIMARY KEY,
        job_id uuid NULL REFERENCES backup_jobs (id),
        storage_key text NOT NULL,
        state text NOT NULL,
        state_version bigint NOT NULL DEFAULT 0,
        epoch bigint NOT NULL,
        bytes_reserved bigint NOT NULL,
        expires_at timestamptz NOT NULL,
        orphan_grace_until timestamptz NULL,
        cleanup_attempts integer NOT NULL DEFAULT 0,
        next_attempt_at timestamptz NULL,
        last_reason_code text NULL,
        created_at timestamptz NOT NULL,
        updated_at timestamptz NOT NULL,
        deleted_at timestamptz NULL,
        CONSTRAINT backup_allocations_storage_key_unique UNIQUE (storage_key),
        CONSTRAINT backup_allocations_state_check CHECK (state IN ('registered', 'written', 'release_pending', 'deleting', 'deleted', 'orphan_discovered', 'cleanup_failed')),
        CONSTRAINT backup_allocations_bytes_check CHECK (bytes_reserved >= 0 AND bytes_reserved <= 268435456),
        CONSTRAINT backup_allocations_attempts_check CHECK (cleanup_attempts <= 3)
    )",
    "CREATE INDEX backup_allocations_state_next_attempt_idx ON backup_allocations (state, next_attempt_at)",
    "CREATE INDEX backup_allocations_job_id_idx ON backup_allocations (job_id)",
    "CREATE INDEX backup_allocations_expires_at_idx ON backup_allocations (expires_at)",
    "CREATE TABLE backup_epoch (
        id integer PRIMARY KEY,
        epoch bigint NOT NULL,
        started_at timestamptz NOT NULL,
        CONSTRAINT backup_epoch_singleton CHECK (id = 1)
    )",
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
        super::refuse_unsafe_down(manager).await?;
        let conn = manager.get_connection();
        // Reverse dependency order. `backup_audit` and its indexes are kept.
        for table in [
            "backup_allocations",
            "backup_idempotency",
            "backup_jobs",
            "backup_epoch",
        ] {
            conn.execute_unprepared(&format!("DROP TABLE IF EXISTS {table}"))
                .await?;
        }
        Ok(())
    }
}
