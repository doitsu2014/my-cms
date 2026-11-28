//! WU-09 contract tests against PostgreSQL and a real file system: download denial (IF-05),
//! disabled capabilities (IF-06..IF-08) and the adapter-selection guard (TS-09, TS-10).
//!
//! The Docker-free halves (precedence while disabled or not ready, body-not-polled spies,
//! previews) live next to the handlers in `handlers/*/tests.rs`.

use super::fs_support::TempDir;
use super::http_api::ready_runtime;
use super::support::*;
use crate::domain::status::BackupJobStatus;
use crate::handlers::disabled_capability::test_support::*;
use crate::handlers::http::BackupApiState;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::ports::audit::AuditSink;
use crate::ports::job_store::JobStore;
use crate::test_db::backup_test_db;
use axum::http::{header, StatusCode};
use sea_orm::{ConnectionTrait, DatabaseConnection};
use serde_json::json;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

/// Every file and directory under `root`, as sorted relative paths.
fn tree(root: &Path) -> Vec<String> {
    fn walk(dir: &Path, base: &Path, out: &mut Vec<String>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            out.push(
                path.strip_prefix(base)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned(),
            );
            if path.is_dir() {
                walk(&path, base, out);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}

async fn table_count(conn: &DatabaseConnection, table: &str) -> i64 {
    count(
        conn,
        &format!("SELECT COUNT(*)::bigint AS n FROM {table}"),
        vec![],
    )
    .await
}

#[derive(Debug, PartialEq, Eq)]
struct AuditRow {
    job_id: String,
    allocation_id: String,
    actor: String,
    instance_id: String,
    operation: String,
    outcome: String,
    reason_code: String,
    digest: String,
    correlation_id: String,
}

async fn audit_rows(conn: &DatabaseConnection, event: &str) -> Vec<AuditRow> {
    let rows = conn
        .query_all(pg(
            "SELECT COALESCE(job_id::text, '') AS job_id, \
                    COALESCE(allocation_id::text, '') AS allocation_id, \
                    COALESCE(actor_sub, '') AS actor, instance_id, operation, outcome, \
                    COALESCE(reason_code, '') AS reason_code, \
                    COALESCE(digest::text, '') AS digest, correlation_id \
             FROM backup_audit WHERE event = $1 ORDER BY occurred_at, id",
            vec![event.into()],
        ))
        .await
        .unwrap();
    rows.iter()
        .map(|row| {
            let text = |column: &str| row.try_get::<String>("", column).unwrap();
            AuditRow {
                job_id: text("job_id"),
                allocation_id: text("allocation_id"),
                actor: text("actor"),
                instance_id: text("instance_id"),
                operation: text("operation"),
                outcome: text("outcome"),
                reason_code: text("reason_code"),
                digest: text("digest"),
                correlation_id: text("correlation_id"),
            }
        })
        .collect()
}

async fn job_snapshot(conn: &DatabaseConnection, id: Uuid) -> (String, i64) {
    let row = conn
        .query_one(pg(
            "SELECT status, state_version FROM backup_jobs WHERE id = $1",
            vec![id.into()],
        ))
        .await
        .unwrap()
        .unwrap();
    (
        row.try_get::<String>("", "status").unwrap(),
        row.try_get::<i64>("", "state_version").unwrap(),
    )
}

/// TS-09 (IF-05, F-11, T-05, T-06): once enabled and ready, a download of an existing job is
/// 403 `DOWNLOAD_DENIED` for every U1-reachable status. Nothing is read, written or granted:
/// no URL, no redirect, no ledger row, no file, no job change; the denial is audited.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts09_download_is_denied_for_every_status_and_changes_nothing() {
    let db = backup_test_db().await;
    let temp = TempDir::new("cap-download");
    let root = temp.root();
    let runtime = ready_runtime(&db.conn, &root).await;
    let app = app_from_state(
        BackupApiState::new(db.conn.clone(), enabled_config(&root)).with_runtime(runtime),
    );
    let tree_before = tree(&root);
    let audit_before = table_count(&db.conn, "backup_audit").await;

    // One active job per instance is a database rule, so the two active statuses are seeded
    // one after the other and each is removed once it has been checked.
    let order = [
        BackupJobStatus::Failed,
        BackupJobStatus::Incomplete,
        BackupJobStatus::Queued,
        BackupJobStatus::Running,
    ];
    assert!(
        SEEDABLE.iter().all(|status| order.contains(status)) && order.len() == SEEDABLE.len(),
        "every U1-reachable status is covered"
    );
    let mut seeded = Vec::new();
    let mut correlations = Vec::new();
    let mut jobs_before = 0;
    for status in order {
        let id = seed(&db.conn, Seed::new(INSTANCE_ID, status)).await;
        let snapshot = job_snapshot(&db.conn, id).await;
        let rows_before = table_count(&db.conn, "backup_jobs").await;
        let response = send(&app, get(&format!("/backup/jobs/{id}/download"))).await;
        for forbidden in [
            header::LOCATION,
            header::CONTENT_DISPOSITION,
            header::CONTENT_RANGE,
        ] {
            assert!(
                response.headers().get(&forbidden).is_none(),
                "{status:?}: no {forbidden} header on a denial"
            );
        }
        let result = json_of(response).await;
        let correlation = assert_error(
            &result,
            StatusCode::FORBIDDEN,
            "Forbidden",
            "DOWNLOAD_DENIED",
            &format!("{status:?}"),
        );
        assert_eq!(
            result.1["errors"],
            json!(["Download is not permitted for backup jobs."])
        );
        let text = result.1.to_string();
        for leak in ["http", "/", "\\", ".bin", "alloc", id.to_string().as_str()] {
            assert!(
                !text.contains(leak),
                "{status:?}: {leak:?} leaked in {text}"
            );
        }
        assert_eq!(
            job_snapshot(&db.conn, id).await,
            snapshot,
            "{status:?}: the job is unchanged"
        );
        assert_eq!(table_count(&db.conn, "backup_jobs").await, rows_before);
        correlations.push(correlation);
        seeded.push((status, id, snapshot));
        if matches!(status, BackupJobStatus::Queued | BackupJobStatus::Running) {
            exec(
                &db.conn,
                "DELETE FROM backup_jobs WHERE id = $1",
                vec![id.into()],
            )
            .await;
        } else {
            jobs_before += 1;
        }
    }
    assert_eq!(table_count(&db.conn, "backup_jobs").await, jobs_before);
    assert_eq!(table_count(&db.conn, "backup_allocations").await, 0);
    assert_eq!(table_count(&db.conn, "backup_idempotency").await, 0);
    assert_eq!(tree(&root), tree_before, "no file is created or touched");

    let rows = audit_rows(&db.conn, "download.denied").await;
    assert_eq!(rows.len(), 4, "{rows:?}");
    assert_eq!(
        table_count(&db.conn, "backup_audit").await,
        audit_before + 4,
        "only the four denials are audited"
    );
    for (((_, id, _), correlation), row) in seeded.iter().zip(&correlations).zip(&rows) {
        assert_eq!(
            *row,
            AuditRow {
                job_id: id.to_string(),
                allocation_id: String::new(),
                actor: "test-admin".into(),
                instance_id: INSTANCE_ID.into(),
                operation: "download".into(),
                outcome: "denied".into(),
                reason_code: "DOWNLOAD_DENIED".into(),
                digest: String::new(),
                correlation_id: correlation.clone(),
            }
        );
    }
}

/// F-04 and FD F-11: an unknown, foreign or malformed job id is the same 404
/// `JOB_NOT_FOUND`, never a 403, and nothing is audited as a denial.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts09_download_unknown_foreign_and_malformed_ids_are_404_and_unaudited() {
    let db = backup_test_db().await;
    let temp = TempDir::new("cap-download-404");
    let root = temp.root();
    let runtime = ready_runtime(&db.conn, &root).await;
    let app = app_from_state(
        BackupApiState::new(db.conn.clone(), enabled_config(&root)).with_runtime(runtime),
    );
    let foreign = seed(
        &db.conn,
        Seed::new("other-instance", BackupJobStatus::Failed),
    )
    .await;
    for id in [
        Uuid::new_v4().to_string(),
        foreign.to_string(),
        "not-a-uuid".to_owned(),
    ] {
        let result = json_of(send(&app, get(&format!("/backup/jobs/{id}/download"))).await).await;
        assert_error(
            &result,
            StatusCode::NOT_FOUND,
            "NotFound",
            "JOB_NOT_FOUND",
            &id,
        );
    }
    assert!(audit_rows(&db.conn, "download.denied").await.is_empty());
}

/// FD F-11 and IFC 6.3: a failing audit write never turns the denial into anything else.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts09_download_and_capability_denials_survive_an_audit_failure() {
    let db = backup_test_db().await;
    let temp = TempDir::new("cap-audit-fail");
    let root = temp.root();
    let runtime = ready_runtime(&db.conn, &root).await;
    let clock = Arc::new(FixedClock(t0()));
    let real_audit: Arc<dyn AuditSink> = Arc::new(DbAuditSink::new(db.conn.clone()));
    let store: Arc<dyn JobStore> = Arc::new(DbJobStore::new(db.conn.clone(), real_audit, clock));
    let failing = SpyAudit::failing();
    let app = app_from_state(
        BackupApiState::new(db.conn.clone(), enabled_config(&root))
            .with_runtime(runtime)
            .with_persistence(store, failing.clone()),
    );
    let job = seed(
        &db.conn,
        Seed::new(INSTANCE_ID, BackupJobStatus::Incomplete),
    )
    .await;
    let result = json_of(send(&app, get(&format!("/backup/jobs/{job}/download"))).await).await;
    assert_error(
        &result,
        StatusCode::FORBIDDEN,
        "Forbidden",
        "DOWNLOAD_DENIED",
        "download",
    );
    let result = json_of(send(&app, post_json("/backup/restore", "{}")).await).await;
    assert_error(
        &result,
        StatusCode::FORBIDDEN,
        "Forbidden",
        "CAPABILITY_DISABLED",
        "restore",
    );
    assert_eq!(
        failing.events().len(),
        2,
        "both audit writes were attempted"
    );
}

/// TS-09 (IF-06..IF-08): with a Ready runtime and real tables, the three disabled
/// capabilities never poll the body, create no job, idempotency or ledger row and no file,
/// and each denial is audited exactly once with the closed field set.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts09_disabled_capabilities_leave_no_ledger_rows_or_files() {
    let db = backup_test_db().await;
    let temp = TempDir::new("cap-ledger");
    let root = temp.root();
    let runtime = ready_runtime(&db.conn, &root).await;
    let app = app_from_state(
        BackupApiState::new(db.conn.clone(), enabled_config(&root)).with_runtime(runtime),
    );
    let tree_before = tree(&root);
    let tables = ["backup_jobs", "backup_idempotency", "backup_allocations"];
    let mut before = Vec::new();
    for table in tables {
        before.push(table_count(&db.conn, table).await);
    }

    let mut correlations = Vec::new();
    for (uri, _) in [
        ("/backup/packages", "upload"),
        ("/backup/preflight", "preflight"),
        ("/backup/restore", "restore"),
    ] {
        let spy = BodySpy::default();
        let request = builder("POST", uri)
            .header(header::CONTENT_TYPE, "multipart/form-data; boundary=x")
            .header(header::CONTENT_LENGTH, "1073741824")
            .body(spy.body())
            .unwrap();
        let result = json_of(send(&app, request).await).await;
        correlations.push(assert_error(
            &result,
            StatusCode::FORBIDDEN,
            "Forbidden",
            "CAPABILITY_DISABLED",
            uri,
        ));
        assert!(!spy.polled(), "{uri}: body stream was polled");
    }

    for (table, expected) in tables.iter().zip(before) {
        assert_eq!(table_count(&db.conn, table).await, expected, "{table}");
    }
    assert_eq!(tree(&root), tree_before, "no file is created");
    let rows = audit_rows(&db.conn, "capability.denied").await;
    assert_eq!(rows.len(), 3, "{rows:?}");
    for ((row, operation), correlation) in rows
        .iter()
        .zip(["upload", "preflight", "restore"])
        .zip(&correlations)
    {
        assert_eq!(
            *row,
            AuditRow {
                job_id: String::new(),
                allocation_id: String::new(),
                actor: "test-admin".into(),
                instance_id: INSTANCE_ID.into(),
                operation: operation.into(),
                outcome: "denied".into(),
                reason_code: "CAPABILITY_DISABLED".into(),
                digest: String::new(),
                correlation_id: correlation.clone(),
            }
        );
    }
}

/// TS-10 (T-05): client-supplied flags cannot select a real adapter. A create request with
/// an adapter-like body field is refused and creates nothing; the same hints in the query
/// string or a header are ignored and the job is synthetic and not downloadable. The
/// instance advertises no real capability. (The create path was built in WU-07/WU-08; this is
/// the regression guard for it. The preview and capability routes are covered in
/// `handlers/disabled_capability/tests.rs` and `handlers/preview/tests.rs`.)
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts10_client_flags_cannot_select_a_real_adapter() {
    let db = backup_test_db().await;
    let temp = TempDir::new("cap-adapter");
    let root = temp.root();
    let runtime = ready_runtime(&db.conn, &root).await;
    let app = app_from_state(
        BackupApiState::new(db.conn.clone(), enabled_config(&root)).with_runtime(runtime),
    );
    let create = |body: &str, key: &str, uri: &str, extra_header: Option<(&str, &str)>| {
        let mut request = builder("POST", uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Idempotency-Key", key);
        if let Some((name, value)) = extra_header {
            request = request.header(name, value);
        }
        request
            .body(axum::body::Body::from(body.to_owned()))
            .unwrap()
    };

    for (index, extra) in [
        r#""adapter":"real""#,
        r#""real":true"#,
        r#""synthetic":false"#,
        r#""capture":"s3""#,
        r#""download_eligible":true"#,
    ]
    .iter()
    .enumerate()
    {
        let body = format!(r#"{{"instance_id":"local-1",{extra}}}"#);
        let result = json_of(
            send(
                &app,
                create(
                    &body,
                    &format!("adapter-key-{index:04}"),
                    "/backup/jobs",
                    None,
                ),
            )
            .await,
        )
        .await;
        assert_error(
            &result,
            StatusCode::BAD_REQUEST,
            "ValidationError",
            "MALFORMED_REQUEST",
            extra,
        );
    }
    assert_eq!(table_count(&db.conn, "backup_jobs").await, 0);

    let (status, json) = json_of(
        send(
            &app,
            create(
                r#"{"instance_id":"local-1"}"#,
                "adapter-key-hints",
                "/backup/jobs?adapter=real&synthetic=false",
                Some(("X-Backup-Adapter", "real")),
            ),
        )
        .await,
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED, "{json}");
    assert_eq!(json["job"]["synthetic"], true);
    assert_eq!(json["job"]["download_eligible"], false);

    let (status, json) = json_of(send(&app, get("/backup/instances")).await).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["synthetic"], true);
    assert_eq!(
        json["instances"][0]["capabilities"],
        json!({
            "create": true,
            "upload": false,
            "preflight": false,
            "restore": false,
            "download": false
        })
    );
}
