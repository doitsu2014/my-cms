//! WU-02 red tests: TS-34 (migrations), TS-11 (DB-level), TS-66 (revert keeps
//! audit). DB-backed tests need Docker and are opt-in (`-- --ignored`).

use super::{Migrator, BACKUP_MIGRATION_IDS};
use crate::test_db::{backup_test_db, untouched_db};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use sea_orm_migration::MigratorTrait;

const POSTS_IDS: &[&str] = &[
    "m20240409_151952_release_100",
    "m20250330_151455_release_110",
    "m20260126_040610_release_300",
    "m20260531_000001_pgvector",
];
const SEO_IDS: &[&str] = &["m20260822_000001_seo_head_assets"];

const TABLES: &[&str] = &[
    "backup_allocations",
    "backup_audit",
    "backup_epoch",
    "backup_idempotency",
    "backup_jobs",
];

// ---------- helpers ----------

async fn try_exec(conn: &DatabaseConnection, sql: &str) -> Result<(), String> {
    conn.execute_unprepared(sql)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

async fn exec_in(conn: &impl ConnectionTrait, sql: &str) {
    if let Err(e) = conn.execute_unprepared(sql).await {
        panic!("expected success for `{sql}`: {e}");
    }
}

async fn exec_ok(conn: &DatabaseConnection, sql: &str) {
    if let Err(e) = try_exec(conn, sql).await {
        panic!("expected success for `{sql}`: {e}");
    }
}

#[track_caller]
fn assert_rejected(result: Result<(), String>, needle: &str, what: &str) {
    match result {
        Ok(()) => panic!("{what}: expected rejection containing `{needle}`, but it succeeded"),
        Err(e) => assert!(
            e.contains(needle),
            "{what}: expected error containing `{needle}`, got: {e}"
        ),
    }
}

#[track_caller]
fn assert_rejected_any(result: Result<(), String>, needles: &[&str], what: &str) {
    match result {
        Ok(()) => {
            panic!("{what}: expected rejection containing one of {needles:?}, but it succeeded")
        }
        Err(e) => assert!(
            needles.iter().any(|n| e.contains(n)),
            "{what}: expected error containing one of {needles:?}, got: {e}"
        ),
    }
}

/// Name of the constraint whose `name => definition` line satisfies `pred`.
#[track_caller]
fn constraint_named(defs: &[String], what: &str, pred: impl Fn(&str) -> bool) -> String {
    let line = defs
        .iter()
        .find(|d| pred(d))
        .unwrap_or_else(|| panic!("{what} missing: {defs:?}"));
    line.split(" => ").next().expect("name").to_owned()
}

fn is_status_values_check(d: &str) -> bool {
    d.contains("'queued'") && !d.contains("operation") && !d.contains("finished_at")
}

fn is_u1_guard_check(d: &str) -> bool {
    d.contains("operation") && d.contains("'succeeded'") && d.contains("'recovery-required'")
}

fn is_operation_check(d: &str) -> bool {
    d.contains("operation") && !d.contains("status")
}

async fn strings(conn: &DatabaseConnection, sql: &str, col: &str) -> Vec<String> {
    conn.query_all(Statement::from_string(DbBackend::Postgres, sql.to_owned()))
        .await
        .unwrap_or_else(|e| panic!("query `{sql}`: {e}"))
        .iter()
        .map(|r| r.try_get::<String>("", col).expect("column"))
        .collect()
}

async fn one(conn: &DatabaseConnection, sql: &str, col: &str) -> String {
    let mut v = strings(conn, sql, col).await;
    assert_eq!(v.len(), 1, "expected one row for `{sql}`, got {v:?}");
    v.remove(0)
}

async fn existing_tables(conn: &DatabaseConnection) -> Vec<String> {
    strings(
        conn,
        "SELECT table_name::text AS t FROM information_schema.tables \
         WHERE table_schema = 'public' AND table_name LIKE 'backup\\_%' ORDER BY table_name",
        "t",
    )
    .await
}

async fn columns(conn: &DatabaseConnection, table: &str) -> Vec<String> {
    // Sorted in Rust so both sides use byte order: SQL `ORDER BY column_name`
    // puts `state` before `state_version`, while sorting the joined
    // `name|type|null` strings puts it after (amendment D-16).
    let mut cols = strings(
        conn,
        &format!(
            "SELECT column_name || '|' || data_type \
             || COALESCE('(' || character_maximum_length || ')', '') || '|' || is_nullable AS c \
             FROM information_schema.columns \
             WHERE table_schema = 'public' AND table_name = '{table}' ORDER BY column_name"
        ),
        "c",
    )
    .await;
    cols.sort();
    cols
}

fn expected(cols: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = cols.iter().map(|s| s.to_string()).collect();
    v.sort();
    v
}

async fn constraint_defs(conn: &DatabaseConnection, table: &str, kind: &str) -> Vec<String> {
    strings(
        conn,
        &format!(
            "SELECT conname || ' => ' || pg_get_constraintdef(oid) AS d FROM pg_constraint \
             WHERE conrelid = 'public.{table}'::regclass AND contype = '{kind}' ORDER BY conname"
        ),
        "d",
    )
    .await
}

async fn index_defs(conn: &DatabaseConnection, table: &str) -> Vec<String> {
    strings(
        conn,
        &format!(
            "SELECT indexdef AS d FROM pg_indexes \
             WHERE schemaname = 'public' AND tablename = '{table}' ORDER BY indexname"
        ),
        "d",
    )
    .await
}

async fn trigger_defs(conn: &DatabaseConnection) -> Vec<String> {
    strings(
        conn,
        "SELECT c.relname || ' / ' || t.tgname || ' / ' || pg_get_triggerdef(t.oid) AS d \
         FROM pg_trigger t JOIN pg_class c ON c.oid = t.tgrelid \
         WHERE NOT t.tgisinternal AND c.relname LIKE 'backup\\_%' ORDER BY t.tgname",
        "d",
    )
    .await
}

async fn versions(conn: &DatabaseConnection, table: &str) -> Vec<String> {
    strings(
        conn,
        &format!("SELECT version::text AS v FROM {table} ORDER BY version"),
        "v",
    )
    .await
}

fn uuid(n: u32) -> String {
    format!("00000000-0000-0000-0000-{n:012}")
}

fn job_sql(n: u32, instance: &str, status: &str, finished: bool) -> String {
    let fin = if finished { "now()" } else { "NULL" };
    format!(
        "INSERT INTO backup_jobs (id, actor_sub, instance_id, operation, status, epoch, \
         correlation_id, created_at, updated_at, finished_at, retain_until) VALUES \
         ('{}', 'admin', '{instance}', 'backup_db_slice', '{status}', 1, 'corr-{n}', now(), \
         now(), {fin}, now() + interval '30 days')",
        uuid(n)
    )
}

fn audit_sql(n: u32) -> String {
    format!(
        "INSERT INTO backup_audit (id, occurred_at, actor_sub, instance_id, job_id, operation, \
         event, outcome, correlation_id) VALUES ('{}', now(), 'admin', 'inst-a', NULL, \
         'backup_db_slice', 'job.accepted', 'accepted', 'corr-{n}')",
        uuid(n)
    )
}

fn allocation_sql(n: u32, state: &str, bytes: i64, attempts: i32) -> String {
    format!(
        "INSERT INTO backup_allocations (id, job_id, storage_key, state, epoch, bytes_reserved, \
         expires_at, cleanup_attempts, created_at, updated_at, deleted_at) VALUES ('{}', NULL, \
         'alloc/{n}.bin', '{state}', 1, {bytes}, now() + interval '1 day', {attempts}, now(), \
         now(), CASE WHEN '{state}' = 'deleted' THEN now() END)",
        uuid(n)
    )
}

async fn create_sea_table(conn: &DatabaseConnection, table: &str, ids: &[&str]) {
    exec_ok(
        conn,
        &format!(
            "CREATE TABLE {table} (version varchar NOT NULL PRIMARY KEY, applied_at bigint NOT NULL)"
        ),
    )
    .await;
    for id in ids {
        exec_ok(
            conn,
            &format!("INSERT INTO {table} (version, applied_at) VALUES ('{id}', 1700000000)"),
        )
        .await;
    }
}

fn set_job(n: u32, set: &str) -> String {
    format!("UPDATE backup_jobs SET {set} WHERE id = '{}'", uuid(n))
}

// ---------- non-Docker unit tests ----------

/// TS-34 (separate migration table). RED: default `seaql_migrations` until overridden.
#[test]
fn backup_uses_its_own_migration_table() {
    assert_eq!(
        sea_orm_migration::prelude::Iden::to_string(&*Migrator::migration_table_name()),
        "seaql_migrations_backup"
    );
}

/// TS-34. Structural (passes against the shell): declared ids match the migration names.
#[test]
fn migrator_lists_the_two_backup_migrations_in_order() {
    let names: Vec<String> = Migrator::migrations()
        .iter()
        .map(|m| m.name().to_string())
        .collect();
    assert_eq!(names, BACKUP_MIGRATION_IDS);
    assert_eq!(
        BACKUP_MIGRATION_IDS,
        &[
            "m20261122_000001_backup_core",
            "m20261122_000002_backup_terminal_guards"
        ]
    );
    assert_eq!(
        crate::migrations_cli::list_identities(),
        BACKUP_MIGRATION_IDS.to_vec()
    );
    let descriptors = super::migration_descriptors();
    assert_eq!(descriptors.len(), 2);
    for (descriptor, id) in descriptors.iter().zip(BACKUP_MIGRATION_IDS) {
        assert_eq!(descriptor.id, *id);
        assert!(descriptor.depends_on.is_empty(), "{}", descriptor.id);
    }
}

// ---------- TS-34 Docker: fresh, separate table, upgrade, idempotent up ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn fresh_database_gets_all_five_backup_tables() {
    let db = backup_test_db().await;
    assert_eq!(existing_tables(&db.conn).await, expected(TABLES));
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn migration_records_only_in_seaql_migrations_backup() {
    let db = backup_test_db().await;
    assert_eq!(
        versions(&db.conn, "seaql_migrations_backup").await,
        expected(BACKUP_MIGRATION_IDS)
    );
    let default_table = one(
        &db.conn,
        "SELECT COALESCE(to_regclass('public.seaql_migrations')::text, 'absent') AS t",
        "t",
    )
    .await;
    assert_eq!(
        default_table, "absent",
        "backup must not create seaql_migrations"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn second_up_is_a_no_op() {
    let db = backup_test_db().await;
    let sql = "SELECT version || '@' || applied_at::text AS v FROM seaql_migrations_backup ORDER BY version";
    let before = strings(&db.conn, sql, "v").await;
    assert_eq!(before.len(), 2);
    Migrator::up(&db.conn, None).await.expect("second up");
    assert_eq!(before, strings(&db.conn, sql, "v").await);
    assert_eq!(existing_tables(&db.conn).await, expected(TABLES));
}

/// TS-34 via `migrations_cli`: `run` applies on a fresh database and is repeatable.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn migrations_cli_run_applies_on_a_fresh_database() {
    let db = untouched_db().await;
    crate::migrations_cli::run(&db.conn).await.expect("run");
    assert_eq!(existing_tables(&db.conn).await, expected(TABLES));
    assert_eq!(
        versions(&db.conn, "seaql_migrations_backup").await,
        expected(BACKUP_MIGRATION_IDS)
    );
    crate::migrations_cli::run(&db.conn)
        .await
        .expect("run again");
    assert_eq!(
        versions(&db.conn, "seaql_migrations_backup").await,
        expected(BACKUP_MIGRATION_IDS)
    );
}

/// TS-34 upgrade: 000001 alone (tables, constraints, indexes), then 000002 (both triggers).
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn upgrade_from_000001_to_000002() {
    let db = untouched_db().await;
    Migrator::up(&db.conn, Some(1)).await.expect("up 1");
    assert_eq!(
        versions(&db.conn, "seaql_migrations_backup").await,
        vec![BACKUP_MIGRATION_IDS[0].to_string()]
    );
    assert_eq!(existing_tables(&db.conn).await, expected(TABLES));
    let checks = constraint_defs(&db.conn, "backup_jobs", "c").await;
    constraint_named(&checks, "status CHECK", is_status_values_check);
    constraint_named(&checks, "U1 guard CHECK", is_u1_guard_check);
    let idx = index_defs(&db.conn, "backup_jobs").await;
    assert!(
        idx.iter()
            .any(|d| d.contains("UNIQUE") && d.contains("(instance_id)") && d.contains("WHERE")),
        "concurrency index belongs to 000001: {idx:?}"
    );
    assert!(
        trigger_defs(&db.conn).await.is_empty(),
        "triggers belong to 000002"
    );

    Migrator::up(&db.conn, None).await.expect("up 2");
    assert_eq!(
        versions(&db.conn, "seaql_migrations_backup").await,
        expected(BACKUP_MIGRATION_IDS)
    );
    let triggers = trigger_defs(&db.conn).await;
    assert_eq!(triggers.len(), 2, "{triggers:?}");
    assert!(triggers
        .iter()
        .any(|t| t.contains("backup_jobs_terminal_immutable")));
    assert!(triggers
        .iter()
        .any(|t| t.contains("backup_audit_append_only")));
}

/// TS-34/TS-66 mirror: reverting only 000002 drops the jobs guard, keeps the audit guard.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn down_one_step_removes_only_the_jobs_trigger() {
    let db = backup_test_db().await;
    Migrator::down(&db.conn, Some(1)).await.expect("down 1");
    assert_eq!(
        versions(&db.conn, "seaql_migrations_backup").await,
        vec![BACKUP_MIGRATION_IDS[0].to_string()]
    );
    assert_eq!(existing_tables(&db.conn).await, expected(TABLES));
    let triggers = trigger_defs(&db.conn).await;
    assert_eq!(triggers.len(), 1, "{triggers:?}");
    assert!(
        triggers[0].contains("backup_audit_append_only"),
        "{triggers:?}"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn upgrade_beside_posts_and_seo_leaves_their_tables_untouched() {
    let db = untouched_db().await;
    create_sea_table(&db.conn, "seaql_migrations", POSTS_IDS).await;
    create_sea_table(&db.conn, "seaql_migrations_seo", SEO_IDS).await;
    exec_ok(
        &db.conn,
        "CREATE TABLE posts_sentinel (id int PRIMARY KEY, title text NOT NULL)",
    )
    .await;
    exec_ok(&db.conn, "INSERT INTO posts_sentinel VALUES (1, 'kept')").await;
    let tables_before = strings(
        &db.conn,
        "SELECT table_name::text AS t FROM information_schema.tables \
         WHERE table_schema = 'public' ORDER BY table_name",
        "t",
    )
    .await;

    Migrator::up(&db.conn, None)
        .await
        .expect("backup migrator applies beside posts and seo");

    assert_eq!(
        versions(&db.conn, "seaql_migrations_backup").await,
        expected(BACKUP_MIGRATION_IDS)
    );
    assert_eq!(
        versions(&db.conn, "seaql_migrations").await,
        expected(POSTS_IDS)
    );
    assert_eq!(
        versions(&db.conn, "seaql_migrations_seo").await,
        expected(SEO_IDS)
    );
    assert_eq!(existing_tables(&db.conn).await, expected(TABLES));
    assert_eq!(
        one(
            &db.conn,
            "SELECT id::text || title AS r FROM posts_sentinel",
            "r"
        )
        .await,
        "1kept"
    );
    assert_eq!(
        columns(&db.conn, "posts_sentinel").await,
        expected(&["id|integer|NO", "title|text|NO"])
    );
    // only backup_* tables were added to the public schema
    let mut expected_tables = tables_before;
    expected_tables.extend(TABLES.iter().map(|t| t.to_string()));
    expected_tables.push("seaql_migrations_backup".to_string());
    expected_tables.sort();
    assert_eq!(
        strings(
            &db.conn,
            "SELECT table_name::text AS t FROM information_schema.tables \
             WHERE table_schema = 'public' ORDER BY table_name",
            "t",
        )
        .await,
        expected_tables
    );
}

// ---------- TS-34 Docker: structure ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn backup_jobs_columns_match_the_contract() {
    let db = backup_test_db().await;
    assert_eq!(
        columns(&db.conn, "backup_jobs").await,
        expected(&[
            "id|uuid|NO",
            "actor_sub|text|NO",
            "instance_id|text|NO",
            "operation|text|NO",
            "status|text|NO",
            "state_version|bigint|NO",
            "synthetic|boolean|NO",
            "reason_code|text|YES",
            "components|jsonb|NO",
            "counters|jsonb|NO",
            "epoch|bigint|NO",
            "correlation_id|text|NO",
            "created_at|timestamp with time zone|NO",
            "updated_at|timestamp with time zone|NO",
            "started_at|timestamp with time zone|YES",
            "finished_at|timestamp with time zone|YES",
            "deadline_at|timestamp with time zone|YES",
            "retain_until|timestamp with time zone|NO",
        ])
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn other_backup_table_columns_match_the_contract() {
    let db = backup_test_db().await;
    assert_eq!(
        columns(&db.conn, "backup_idempotency").await,
        expected(&[
            "id|uuid|NO",
            "actor_sub|text|NO",
            "operation|text|NO",
            "key_digest|character(64)|NO",
            "request_fingerprint|character(64)|NO",
            "job_id|uuid|NO",
            "created_at|timestamp with time zone|NO",
            "expires_at|timestamp with time zone|NO",
        ])
    );
    assert_eq!(
        columns(&db.conn, "backup_audit").await,
        expected(&[
            "id|uuid|NO",
            "occurred_at|timestamp with time zone|NO",
            "actor_sub|text|YES",
            "instance_id|text|NO",
            "job_id|uuid|YES",
            "allocation_id|uuid|YES",
            "operation|text|NO",
            "event|text|NO",
            "outcome|text|NO",
            "reason_code|text|YES",
            "digest|character(64)|YES",
            "correlation_id|text|NO",
        ])
    );
    assert_eq!(
        columns(&db.conn, "backup_allocations").await,
        expected(&[
            "id|uuid|NO",
            "job_id|uuid|YES",
            "storage_key|text|NO",
            "state|text|NO",
            "state_version|bigint|NO",
            "epoch|bigint|NO",
            "bytes_reserved|bigint|NO",
            "expires_at|timestamp with time zone|NO",
            "orphan_grace_until|timestamp with time zone|YES",
            "cleanup_attempts|integer|NO",
            "next_attempt_at|timestamp with time zone|YES",
            "last_reason_code|text|YES",
            "created_at|timestamp with time zone|NO",
            "updated_at|timestamp with time zone|NO",
            "deleted_at|timestamp with time zone|YES",
        ])
    );
    assert_eq!(
        columns(&db.conn, "backup_epoch").await,
        expected(&[
            "id|integer|NO",
            "epoch|bigint|NO",
            "started_at|timestamp with time zone|NO",
        ])
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn backup_jobs_defaults_apply() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    assert_eq!(
        one(
            &db.conn,
            "SELECT state_version::text || '|' || synthetic::text || '|' || components::text \
             || '|' || counters::text AS d FROM backup_jobs",
            "d"
        )
        .await,
        "0|true|[]|{}"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn secondary_indexes_exist() {
    let db = backup_test_db().await;
    let has = |defs: &[String], needle: &str| defs.iter().any(|d| d.contains(needle));
    let jobs = index_defs(&db.conn, "backup_jobs").await;
    assert!(has(&jobs, "(instance_id, created_at DESC)"), "{jobs:?}");
    assert!(has(&jobs, "(status)"), "{jobs:?}");
    assert!(has(&jobs, "(retain_until)"), "{jobs:?}");
    let idem = index_defs(&db.conn, "backup_idempotency").await;
    assert!(has(&idem, "(expires_at)"), "{idem:?}");
    let audit = index_defs(&db.conn, "backup_audit").await;
    assert!(has(&audit, "(job_id)"), "{audit:?}");
    assert!(has(&audit, "(occurred_at)"), "{audit:?}");
    let alloc = index_defs(&db.conn, "backup_allocations").await;
    assert!(has(&alloc, "(state, next_attempt_at)"), "{alloc:?}");
    assert!(has(&alloc, "(job_id)"), "{alloc:?}");
    assert!(has(&alloc, "(expires_at)"), "{alloc:?}");
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn both_guard_triggers_are_present() {
    let db = backup_test_db().await;
    let triggers = trigger_defs(&db.conn).await;
    let jobs = triggers
        .iter()
        .find(|t| t.starts_with("backup_jobs / backup_jobs_terminal_immutable / "))
        .unwrap_or_else(|| panic!("jobs trigger missing: {triggers:?}"));
    assert!(
        jobs.contains("BEFORE") && jobs.contains("UPDATE"),
        "timing BEFORE, event UPDATE: {jobs}"
    );
    let audit = triggers
        .iter()
        .find(|t| t.starts_with("backup_audit / backup_audit_append_only / "))
        .unwrap_or_else(|| panic!("audit trigger missing: {triggers:?}"));
    assert!(
        audit.contains("BEFORE") && audit.contains("UPDATE") && audit.contains("DELETE"),
        "{audit}"
    );
}

// ---------- TS-11 DB level (D-10) ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn status_column_check_keeps_all_six_documented_values() {
    let db = backup_test_db().await;
    let defs = constraint_defs(&db.conn, "backup_jobs", "c").await;
    let status_check = defs
        .iter()
        .find(|d| is_status_values_check(d))
        .unwrap_or_else(|| panic!("status CHECK missing: {defs:?}"));
    for value in [
        "queued",
        "running",
        "succeeded",
        "failed",
        "recovery-required",
        "incomplete",
    ] {
        assert!(
            status_check.contains(&format!("'{value}'")),
            "status CHECK must keep `{value}`: {status_check}"
        );
    }
    let u1 = defs
        .iter()
        .find(|d| is_u1_guard_check(d))
        .unwrap_or_else(|| panic!("U1 guard CHECK missing: {defs:?}"));
    assert!(u1.contains("backup_db_slice") && u1.contains("NOT"), "{u1}");
}

/// D-10 behavioural proof, independent of CHECK evaluation order: with the U1 guard
/// and the operation CHECK removed (inside a rolled-back transaction), the status
/// column and its own CHECK still accept every documented value.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn status_column_accepts_every_documented_value_when_u1_guard_is_removed() {
    let db = backup_test_db().await;
    let defs = constraint_defs(&db.conn, "backup_jobs", "c").await;
    let u1 = constraint_named(&defs, "U1 guard CHECK", is_u1_guard_check);
    let operation = constraint_named(&defs, "operation CHECK", is_operation_check);
    let txn = sea_orm::TransactionTrait::begin(&db.conn)
        .await
        .expect("begin");
    exec_in(
        &txn,
        &format!("ALTER TABLE backup_jobs DROP CONSTRAINT {u1}"),
    )
    .await;
    exec_in(
        &txn,
        &format!("ALTER TABLE backup_jobs DROP CONSTRAINT {operation}"),
    )
    .await;
    for (i, status) in [
        "queued",
        "running",
        "succeeded",
        "recovery-required",
        "failed",
        "incomplete",
    ]
    .iter()
    .enumerate()
    {
        let finished = matches!(*status, "failed" | "incomplete");
        let sql = job_sql(500 + i as u32, &format!("inst-d10-{i}"), status, finished)
            .replace("'backup_db_slice'", "'other_kind'");
        exec_in(&txn, &sql).await;
    }
    txn.rollback().await.expect("rollback");
    assert_eq!(
        one(&db.conn, "SELECT count(*)::text AS c FROM backup_jobs", "c").await,
        "0"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn u1_job_cannot_be_inserted_as_succeeded_or_recovery_required() {
    let db = backup_test_db().await;
    // Positive controls: the allowed values insert fine.
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    exec_ok(&db.conn, &job_sql(2, "inst-b", "incomplete", true)).await;
    exec_ok(&db.conn, &job_sql(3, "inst-c", "failed", true)).await;
    for (n, status) in [(10, "succeeded"), (11, "recovery-required")] {
        assert_rejected(
            try_exec(&db.conn, &job_sql(n, "inst-z", status, false)).await,
            "violates check constraint",
            status,
        );
    }
    assert_rejected(
        try_exec(&db.conn, &job_sql(12, "inst-z", "bogus", false)).await,
        "violates check constraint",
        "unknown status",
    );
    assert_eq!(
        one(&db.conn, "SELECT count(*)::text AS c FROM backup_jobs", "c").await,
        "3"
    );
}

const GUARD_NEEDLES: &[&str] = &[
    "backup_jobs_terminal_immutable",
    "violates check constraint",
];

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn u1_job_cannot_be_updated_to_succeeded_or_recovery_required() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    exec_ok(
        &db.conn,
        &set_job(1, "status = 'running', state_version = 1"),
    )
    .await;
    for status in ["succeeded", "recovery-required"] {
        assert_rejected_any(
            try_exec(&db.conn, &set_job(1, &format!("status = '{status}'"))).await,
            GUARD_NEEDLES,
            status,
        );
        assert_eq!(
            one(&db.conn, "SELECT status AS s FROM backup_jobs", "s").await,
            "running"
        );
    }
    // positive control after the rejected updates: the legal edge still works
    exec_ok(
        &db.conn,
        &set_job(1, "status = 'failed', finished_at = now()"),
    )
    .await;
}

/// TS-11 DB level, exhaustive: every (source, target) pair over the four insertable
/// sources and all six statuses. Exactly the four documented edges succeed; every
/// other pair (including every target of succeeded or recovery-required and every
/// terminal source) is rejected and leaves the stored status unchanged.
/// [D] a non-terminal row set to its own status is a no-op, so those pairs are skipped.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn every_status_transition_pair_is_allowed_or_rejected_exactly() {
    let db = backup_test_db().await;
    let all = [
        "queued",
        "running",
        "succeeded",
        "failed",
        "recovery-required",
        "incomplete",
    ];
    let legal = [
        ("queued", "running"),
        ("queued", "failed"),
        ("running", "incomplete"),
        ("running", "failed"),
    ];
    let is_terminal = |s: &str| matches!(s, "failed" | "incomplete");
    let mut n = 1000;
    for source in ["queued", "running", "failed", "incomplete"] {
        for target in all {
            if source == target && !is_terminal(source) {
                continue;
            }
            n += 1;
            let instance = format!("inst-pair-{n}");
            exec_ok(
                &db.conn,
                &job_sql(n, &instance, source, is_terminal(source)),
            )
            .await;
            let finished = if is_terminal(target) {
                "finished_at = now()"
            } else {
                "finished_at = NULL"
            };
            let result = try_exec(
                &db.conn,
                &set_job(n, &format!("status = '{target}', {finished}")),
            )
            .await;
            let stored = one(
                &db.conn,
                &format!(
                    "SELECT status AS s FROM backup_jobs WHERE id = '{}'",
                    uuid(n)
                ),
                "s",
            )
            .await;
            let what = format!("{source} -> {target}");
            if legal.contains(&(source, target)) {
                result.unwrap_or_else(|e| panic!("{what} must be allowed: {e}"));
                assert_eq!(stored, target, "{what}");
            } else {
                assert_rejected_any(result, GUARD_NEEDLES, &what);
                assert_eq!(stored, source, "{what} must leave the status unchanged");
            }
        }
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn backup_jobs_row_checks_reject_bad_rows() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    // operation is fixed to backup_db_slice
    assert_rejected(
        try_exec(
            &db.conn,
            &job_sql(2, "inst-b", "queued", false).replace("'backup_db_slice'", "'restore'"),
        )
        .await,
        "violates check constraint",
        "operation",
    );
    // synthetic must be true
    assert_rejected(
        try_exec(
            &db.conn,
            &job_sql(3, "inst-c", "queued", false)
                .replace("epoch, ", "epoch, synthetic, ")
                .replace("1, 'corr-3'", "1, false, 'corr-3'"),
        )
        .await,
        "violates check constraint",
        "synthetic false",
    );
    // terminal status requires finished_at, non-terminal forbids it
    assert_rejected(
        try_exec(&db.conn, &job_sql(4, "inst-d", "failed", false)).await,
        "violates check constraint",
        "failed without finished_at",
    );
    assert_rejected(
        try_exec(&db.conn, &job_sql(5, "inst-e", "queued", true)).await,
        "violates check constraint",
        "queued with finished_at",
    );
}

// ---------- terminal-state guard trigger ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn legal_transitions_are_allowed() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    // a non-status update on a non-terminal row is fine (CAS bumps state_version)
    exec_ok(
        &db.conn,
        &set_job(1, "state_version = 1, updated_at = now()"),
    )
    .await;
    exec_ok(
        &db.conn,
        &set_job(
            1,
            "status = 'running', state_version = 2, started_at = now()",
        ),
    )
    .await;
    exec_ok(
        &db.conn,
        &set_job(
            1,
            "status = 'incomplete', state_version = 3, finished_at = now()",
        ),
    )
    .await;
    exec_ok(&db.conn, &job_sql(2, "inst-b", "queued", false)).await;
    exec_ok(
        &db.conn,
        &set_job(2, "status = 'failed', finished_at = now()"),
    )
    .await;
    exec_ok(&db.conn, &job_sql(3, "inst-c", "queued", false)).await;
    exec_ok(&db.conn, &set_job(3, "status = 'running'")).await;
    exec_ok(
        &db.conn,
        &set_job(3, "status = 'failed', finished_at = now()"),
    )
    .await;
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn terminal_job_cannot_be_updated() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "failed", true)).await;
    exec_ok(&db.conn, &job_sql(2, "inst-b", "incomplete", true)).await;
    for n in [1, 2] {
        for set in [
            "status = 'running', finished_at = NULL",
            "status = 'failed'",
            "status = 'incomplete'",
            "reason_code = 'tamper'",
            "state_version = 99",
        ] {
            assert_rejected(
                try_exec(&db.conn, &set_job(n, set)).await,
                "backup_jobs_terminal_immutable",
                set,
            );
        }
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn illegal_non_terminal_transitions_are_rejected() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    // queued -> incomplete is not an allowed edge
    assert_rejected(
        try_exec(
            &db.conn,
            &set_job(1, "status = 'incomplete', finished_at = now()"),
        )
        .await,
        "backup_jobs_terminal_immutable",
        "queued->incomplete",
    );
    exec_ok(&db.conn, &set_job(1, "status = 'running'")).await;
    // running -> queued is not allowed
    assert_rejected(
        try_exec(&db.conn, &set_job(1, "status = 'queued'")).await,
        "backup_jobs_terminal_immutable",
        "running->queued",
    );
    assert_eq!(
        one(&db.conn, "SELECT status AS s FROM backup_jobs", "s").await,
        "running"
    );
}

// ---------- concurrency index ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn only_one_active_job_per_instance() {
    let db = backup_test_db().await;
    let defs = index_defs(&db.conn, "backup_jobs").await;
    let idx = defs
        .iter()
        .find(|d| d.contains("UNIQUE") && d.contains("(instance_id)") && d.contains("WHERE"))
        .unwrap_or_else(|| panic!("partial unique index missing: {defs:?}"));
    assert!(
        idx.contains("'queued'")
            && idx.contains("'running'")
            && !idx.contains("'failed'")
            && !idx.contains("'incomplete'"),
        "predicate must be exactly the two active statuses: {idx}"
    );

    const DUP: &str = "duplicate key value violates unique constraint";
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    assert_rejected(
        try_exec(&db.conn, &job_sql(2, "inst-a", "queued", false)).await,
        DUP,
        "second queued",
    );
    assert_rejected(
        try_exec(&db.conn, &job_sql(3, "inst-a", "running", false)).await,
        DUP,
        "queued + running",
    );
    // running first, then queued or running
    exec_ok(&db.conn, &job_sql(20, "inst-r", "running", false)).await;
    assert_rejected(
        try_exec(&db.conn, &job_sql(21, "inst-r", "queued", false)).await,
        DUP,
        "running + queued",
    );
    assert_rejected(
        try_exec(&db.conn, &job_sql(22, "inst-r", "running", false)).await,
        DUP,
        "running + running",
    );
    // an incomplete row does not count as active
    exec_ok(&db.conn, &job_sql(30, "inst-i", "incomplete", true)).await;
    exec_ok(&db.conn, &job_sql(31, "inst-i", "queued", false)).await;
    // a different instance is independent
    exec_ok(&db.conn, &job_sql(4, "inst-b", "queued", false)).await;
    // terminal rows do not count, and finishing the active one frees the slot
    exec_ok(&db.conn, &job_sql(5, "inst-a", "failed", true)).await;
    exec_ok(
        &db.conn,
        &set_job(1, "status = 'failed', finished_at = now()"),
    )
    .await;
    exec_ok(&db.conn, &job_sql(6, "inst-a", "queued", false)).await;
}

// ---------- other tables constraints ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn idempotency_unique_and_restrict_fk() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    let idem = |n: u32, job: u32, actor: &str, operation: &str, digest_char: char| {
        let digest = digest_char.to_string().repeat(64);
        format!(
            "INSERT INTO backup_idempotency (id, actor_sub, operation, key_digest, \
             request_fingerprint, job_id, created_at, expires_at) VALUES ('{}', '{actor}', \
             '{operation}', '{digest}', '{digest}', '{}', now(), now() + interval '30 days')",
            uuid(n),
            uuid(job)
        )
    };
    exec_ok(&db.conn, &idem(100, 1, "admin", "backup_db_slice", 'a')).await;
    assert_rejected(
        try_exec(&db.conn, &idem(101, 1, "admin", "backup_db_slice", 'a')).await,
        "duplicate key value violates unique constraint",
        "duplicate scope",
    );
    // the UNIQUE scope is the full triple: changing any one member is allowed
    exec_ok(
        &db.conn,
        &idem(103, 1, "other-admin", "backup_db_slice", 'a'),
    )
    .await;
    exec_ok(&db.conn, &idem(104, 1, "admin", "other_operation", 'a')).await;
    // unknown job with its own scope, so the FK is the only possible violation
    assert_rejected(
        try_exec(&db.conn, &idem(102, 999, "admin", "backup_db_slice", 'b')).await,
        "violates foreign key constraint",
        "unknown job",
    );
    assert_rejected(
        try_exec(
            &db.conn,
            &format!("DELETE FROM backup_jobs WHERE id = '{}'", uuid(1)),
        )
        .await,
        "violates foreign key constraint",
        "ON DELETE RESTRICT",
    );
    let fk = constraint_defs(&db.conn, "backup_idempotency", "f").await;
    assert!(
        fk.iter().any(|d| d.contains("ON DELETE RESTRICT")),
        "{fk:?}"
    );
}

/// The idempotency scope is exactly (actor_sub, operation, key_digest). A replay with
/// the same key but a different request body must hit the UNIQUE constraint, so the
/// store can answer 409 instead of creating a second job (test-after, D-16).
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn idempotency_scope_ignores_request_fingerprint() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "queued", false)).await;
    let insert = |n: u32, fingerprint_char: char| {
        format!(
            "INSERT INTO backup_idempotency (id, actor_sub, operation, key_digest, \
             request_fingerprint, job_id, created_at, expires_at) VALUES ('{}', 'admin', \
             'backup_db_slice', '{}', '{}', '{}', now(), now() + interval '30 days')",
            uuid(n),
            "a".repeat(64),
            fingerprint_char.to_string().repeat(64),
            uuid(1)
        )
    };
    exec_ok(&db.conn, &insert(200, 'f')).await;
    assert_rejected(
        try_exec(&db.conn, &insert(201, 'e')).await,
        "duplicate key value violates unique constraint",
        "same key, different request body",
    );
}

/// F-33 purge needs a terminal job without an idempotency row to be deletable, so the
/// terminal guard must not fire on DELETE. backup_audit.job_id has no FK for the same reason.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn terminal_job_without_idempotency_row_can_be_deleted_and_audit_has_no_fk() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &job_sql(1, "inst-a", "failed", true)).await;
    exec_ok(
        &db.conn,
        &format!("DELETE FROM backup_jobs WHERE id = '{}'", uuid(1)),
    )
    .await;
    assert!(
        constraint_defs(&db.conn, "backup_audit", "f")
            .await
            .is_empty(),
        "backup_audit must have no foreign keys"
    );
    exec_ok(
        &db.conn,
        &audit_sql(2).replace("'inst-a', NULL", &format!("'inst-a', '{}'", uuid(999))),
    )
    .await;
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn allocation_job_fk_and_column_defaults() {
    let db = backup_test_db().await;
    assert_rejected(
        try_exec(
            &db.conn,
            &allocation_sql(1, "registered", 1, 0)
                .replace("NULL, 'alloc/", &format!("'{}', 'alloc/", uuid(999))),
        )
        .await,
        "violates foreign key constraint",
        "unknown job",
    );
    exec_ok(&db.conn, &job_sql(2, "inst-a", "queued", false)).await;
    exec_ok(
        &db.conn,
        &allocation_sql(3, "registered", 1, 0)
            .replace("NULL, 'alloc/", &format!("'{}', 'alloc/", uuid(2))),
    )
    .await;
    // omit state_version and cleanup_attempts: both DEFAULT 0
    exec_ok(
        &db.conn,
        &format!(
            "INSERT INTO backup_allocations (id, storage_key, state, epoch, bytes_reserved, \
             expires_at, created_at, updated_at) VALUES ('{}', 'alloc/4.bin', 'registered', 1, \
             1, now(), now(), now())",
            uuid(4)
        ),
    )
    .await;
    assert_eq!(
        one(
            &db.conn,
            &format!(
                "SELECT state_version::text || '|' || cleanup_attempts::text AS d \
                 FROM backup_allocations WHERE id = '{}'",
                uuid(4)
            ),
            "d"
        )
        .await,
        "0|0"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn allocation_checks_and_epoch_singleton() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &allocation_sql(1, "registered", 268_435_456, 3)).await;
    assert_rejected(
        try_exec(&db.conn, &allocation_sql(2, "exploded", 1, 0)).await,
        "violates check constraint",
        "state",
    );
    assert_rejected(
        try_exec(&db.conn, &allocation_sql(3, "registered", 268_435_457, 0)).await,
        "violates check constraint",
        "bytes above ceiling",
    );
    assert_rejected(
        try_exec(&db.conn, &allocation_sql(4, "registered", -1, 0)).await,
        "violates check constraint",
        "negative bytes",
    );
    assert_rejected(
        try_exec(&db.conn, &allocation_sql(5, "registered", 1, 4)).await,
        "violates check constraint",
        "attempts above ceiling",
    );
    assert_rejected(
        try_exec(
            &db.conn,
            &allocation_sql(6, "registered", 1, 0).replace("alloc/6.bin", "alloc/1.bin"),
        )
        .await,
        "duplicate key value violates unique constraint",
        "storage_key unique",
    );
    exec_ok(
        &db.conn,
        "INSERT INTO backup_epoch (id, epoch, started_at) VALUES (1, 1, now())",
    )
    .await;
    assert_rejected(
        try_exec(
            &db.conn,
            "INSERT INTO backup_epoch (id, epoch, started_at) VALUES (2, 1, now())",
        )
        .await,
        "violates check constraint",
        "epoch singleton",
    );
}

// ---------- audit append-only (T-08) ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn audit_rows_cannot_be_updated_or_deleted() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &audit_sql(1)).await;
    assert_rejected(
        try_exec(
            &db.conn,
            &format!(
                "UPDATE backup_audit SET outcome = 'tampered' WHERE id = '{}'",
                uuid(1)
            ),
        )
        .await,
        "backup_audit_append_only",
        "update",
    );
    assert_rejected(
        try_exec(
            &db.conn,
            &format!("DELETE FROM backup_audit WHERE id = '{}'", uuid(1)),
        )
        .await,
        "backup_audit_append_only",
        "delete",
    );
    assert_rejected(
        try_exec(&db.conn, "DELETE FROM backup_audit").await,
        "backup_audit_append_only",
        "bulk delete",
    );
    assert_eq!(
        one(&db.conn, "SELECT outcome AS o FROM backup_audit", "o").await,
        "accepted"
    );
    // inserts stay allowed
    exec_ok(&db.conn, &audit_sql(2)).await;
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn audit_cannot_be_truncated() {
    // A row-level trigger does not fire on TRUNCATE, so append-only needs a
    // statement-level guard too (conductor decision D-14, threat T-08).
    let db = backup_test_db().await;
    exec_ok(&db.conn, &audit_sql(1)).await;
    assert_rejected(
        try_exec(&db.conn, "TRUNCATE backup_audit").await,
        "backup_audit_append_only",
        "truncate",
    );
    assert_eq!(
        one(
            &db.conn,
            "SELECT count(*)::text AS n FROM backup_audit",
            "n"
        )
        .await,
        "1"
    );
}

// ---------- TS-66 revert keeps audit ----------

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn down_refuses_while_audit_rows_exist_and_keeps_everything() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &audit_sql(1)).await;
    let err = crate::migrations_cli::revert(&db.conn)
        .await
        .expect_err("down must refuse while backup_audit has rows")
        .to_string();
    assert!(
        err.contains("backup_audit"),
        "refusal should name backup_audit: {err}"
    );
    assert_eq!(
        one(
            &db.conn,
            "SELECT count(*)::text AS c FROM backup_audit",
            "c"
        )
        .await,
        "1"
    );
    assert_eq!(existing_tables(&db.conn).await, expected(TABLES));
    assert_eq!(
        versions(&db.conn, "seaql_migrations_backup").await,
        expected(BACKUP_MIGRATION_IDS)
    );
    let triggers = trigger_defs(&db.conn).await;
    assert!(
        triggers
            .iter()
            .any(|t| t.contains("backup_audit_append_only")),
        "{triggers:?}"
    );
    assert_rejected(
        try_exec(&db.conn, "DELETE FROM backup_audit").await,
        "backup_audit_append_only",
        "append-only still enforced",
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn down_without_audit_rows_keeps_audit_table_and_trigger_and_drops_the_rest() {
    let db = backup_test_db().await;
    crate::migrations_cli::revert(&db.conn)
        .await
        .expect("down with no audit rows");
    assert_eq!(
        existing_tables(&db.conn).await,
        vec!["backup_audit".to_string()]
    );
    let triggers = trigger_defs(&db.conn).await;
    assert_eq!(triggers.len(), 1, "{triggers:?}");
    assert!(
        triggers[0].contains("backup_audit_append_only"),
        "{triggers:?}"
    );
    assert!(versions(&db.conn, "seaql_migrations_backup")
        .await
        .is_empty());
    // re-applying after a revert works and restores the full schema
    Migrator::up(&db.conn, None).await.expect("up after down");
    assert_eq!(existing_tables(&db.conn).await, expected(TABLES));
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn down_refuses_while_an_allocation_is_not_deleted() {
    let db = backup_test_db().await;
    for (i, state) in [
        "registered",
        "written",
        "release_pending",
        "deleting",
        "orphan_discovered",
        "cleanup_failed",
    ]
    .iter()
    .enumerate()
    {
        let n = 1 + i as u32;
        exec_ok(&db.conn, &allocation_sql(n, state, 10, 0)).await;
        let err = crate::migrations_cli::revert(&db.conn)
            .await
            .expect_err(&format!(
                "down must refuse while an allocation is `{state}`"
            ))
            .to_string();
        assert!(err.contains("backup_allocations"), "{state}: {err}");
        assert_eq!(existing_tables(&db.conn).await, expected(TABLES), "{state}");
        exec_ok(
            &db.conn,
            &format!("DELETE FROM backup_allocations WHERE id = '{}'", uuid(n)),
        )
        .await;
    }
}

/// Positive control: only `deleted` allocations (and no audit rows) do not block down.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn down_succeeds_when_every_allocation_is_deleted() {
    let db = backup_test_db().await;
    exec_ok(&db.conn, &allocation_sql(1, "deleted", 10, 0)).await;
    exec_ok(&db.conn, &allocation_sql(2, "deleted", 10, 1)).await;
    crate::migrations_cli::revert(&db.conn)
        .await
        .expect("down with only deleted allocations");
    assert_eq!(
        existing_tables(&db.conn).await,
        vec!["backup_audit".to_string()]
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn audit_digest_is_char_64() {
    let db = backup_test_db().await;
    let with_digest = |n: u32, len: usize| {
        audit_sql(n)
            .replace(
                "event, outcome, correlation_id)",
                "event, outcome, digest, correlation_id)",
            )
            .replace(
                "'accepted', 'corr-",
                &format!("'accepted', '{}', 'corr-", "d".repeat(len)),
            )
    };
    exec_ok(&db.conn, &with_digest(1, 64)).await;
    assert_rejected(
        try_exec(&db.conn, &with_digest(2, 65)).await,
        "value too long for type character(64)",
        "digest over 64",
    );
    assert_eq!(
        one(
            &db.conn,
            "SELECT length(digest)::text AS l FROM backup_audit",
            "l"
        )
        .await,
        "64"
    );
}
