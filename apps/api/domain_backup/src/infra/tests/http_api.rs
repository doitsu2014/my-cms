//! WU-08 REST contract tests against the API adapter and PostgreSQL.

use super::support::*;
use crate::domain::config::{BackupConfig, BackupConfigState};
use crate::domain::status::BackupJobStatus;
use crate::handlers::http::{routes, BackupApiState};
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::infra::db_retention_store::DbRetentionStore;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::infra::fs_namespace_lock::FsNamespaceLock;
use crate::lifecycle::cleaner::Cleaner;
use crate::lifecycle::reconciler::{Reconciler, ReconcilerConfig};
use crate::lifecycle::runtime::RuntimeState;
use crate::lifecycle::runtime::{Runtime, RuntimeConfig};
use crate::lifecycle::sweeper::{Sweeper, SweeperConfig};
use crate::ports::allocation_store::{AllocationLimits, AllocationStore};
use crate::ports::fixture_storage::FixtureStorage;
use crate::ports::job_store::JobStore;
use crate::ports::retention_store::RetentionStore;
use crate::test_db::backup_test_db;
use axum::body::{to_bytes, Body};
use axum::http::{header, Request, StatusCode};
use domain_interface::{AuthenticatedActor, DomainContext};
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

fn config(root: &Path) -> BackupConfigState {
    BackupConfig::load(|key| {
        Some(match key {
            "BACKUP_ENABLED" => "true".into(),
            "BACKUP_INSTANCE_ID" => "local-1".into(),
            "BACKUP_INSTANCE_LABEL" => "Test local instance".into(),
            "BACKUP_STORAGE_ROOT" => root.to_string_lossy().into_owned(),
            "BACKUP_ALLOWED_ORIGINS" => "http://localhost:3002".into(),
            "BACKUP_REQUEST_TIMEOUT_SECS" => "30".into(),
            "BACKUP_MAX_HEADER_BYTES" => "16384".into(),
            _ => return None,
        })
    })
}

fn test_domain_context(conn: sea_orm::DatabaseConnection) -> DomainContext {
    use async_graphql::dynamic::{Field, FieldFuture, Object, Schema, TypeRef};
    use async_graphql::Value;
    let schema = || {
        Arc::new(
            Schema::build("Query", None, None)
                .register(Object::new("Query").field(Field::new(
                    "placeholder",
                    TypeRef::named_nn("Boolean"),
                    |_| FieldFuture::new(async { Ok(Some(Value::from(true))) }),
                )))
                .finish()
                .unwrap(),
        )
    };
    DomainContext {
        conn: Arc::new(conn),
        graphql_immutable: schema(),
        graphql_mutable: schema(),
    }
}

fn admin() -> AuthenticatedActor {
    AuthenticatedActor {
        user_id: "test-admin".into(),
        email: None,
        primary_role: "authenticated".into(),
        app_roles: vec!["my-headless-cms-administrator".into()],
    }
}

fn request(method: &str, uri: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::HOST, "localhost:8989")
        .extension(admin())
        .body(Body::empty())
        .unwrap()
}

async fn body(response: axum::response::Response) -> (StatusCode, Value) {
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
    let payload = String::from_utf8_lossy(&bytes);
    let value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| panic!("unexpected {status} response body: {payload}"));
    (status, value)
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts63_invalid_uuid_is_mapped_to_crate_404_after_runtime_ready() {
    let db = backup_test_db().await;
    let temp = crate::infra::tests::fs_support::TempDir::new("http-invalid-path");
    let config = config(&temp.root());
    let clock = Arc::new(FixedClock(t0()));
    let audit = Arc::new(DbAuditSink::new(db.conn.clone()));
    let allocs: Arc<dyn AllocationStore> = Arc::new(DbAllocationStore::new(
        db.conn.clone(),
        audit.clone(),
        clock.clone(),
        "local-1",
        AllocationLimits::default(),
    ));
    let storage: Arc<dyn FixtureStorage> =
        Arc::new(FsFixtureStorage::open(&temp.root(), allocs.clone()).unwrap());
    let jobs: Arc<dyn JobStore> = Arc::new(DbJobStore::new(
        db.conn.clone(),
        audit.clone(),
        clock.clone(),
    ));
    let retention: Arc<dyn RetentionStore> = Arc::new(DbRetentionStore::new(
        db.conn.clone(),
        audit.clone(),
        "local-1",
    ));
    let reconciler = Arc::new(Reconciler::new(
        jobs.clone(),
        allocs.clone(),
        storage.clone(),
        audit.clone(),
        ReconcilerConfig::new("local-1"),
    ));
    let cleaner = Arc::new(Cleaner::new(allocs.clone(), storage));
    let sweeper = Arc::new(Sweeper::new(
        jobs,
        allocs,
        retention,
        reconciler.clone(),
        SweeperConfig::default(),
    ));
    let runtime = Arc::new(Runtime::new(
        reconciler,
        cleaner,
        sweeper,
        Arc::new(|root: &Path| {
            <FsNamespaceLock as crate::ports::namespace_lock::NamespaceLock>::try_acquire(root)
        }),
        clock,
        RuntimeConfig::new(temp.root()),
    ));
    runtime.start().await.unwrap();
    let app = routes(BackupApiState::new(db.conn.clone(), config).with_runtime(runtime))
        .with_state(test_domain_context(db.conn.clone()));
    let (status, json) = body(
        app.oneshot(request("GET", "/backup/jobs/not-a-uuid"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(json["reason_code"], "JOB_NOT_FOUND");
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts45_instances_and_status_return_the_configured_safe_contract() {
    let db = backup_test_db().await;
    let temp = crate::infra::tests::fs_support::TempDir::new("http-instance");
    let config = config(&temp.root());
    let clock = Arc::new(FixedClock(t0()));
    let audit = Arc::new(DbAuditSink::new(db.conn.clone()));
    let allocs: Arc<dyn AllocationStore> = Arc::new(DbAllocationStore::new(
        db.conn.clone(),
        audit.clone(),
        clock.clone(),
        "local-1",
        AllocationLimits::default(),
    ));
    let storage: Arc<dyn FixtureStorage> =
        Arc::new(FsFixtureStorage::open(&temp.root(), allocs.clone()).unwrap());
    let jobs: Arc<dyn JobStore> = Arc::new(DbJobStore::new(
        db.conn.clone(),
        audit.clone(),
        clock.clone(),
    ));
    let retention: Arc<dyn RetentionStore> = Arc::new(DbRetentionStore::new(
        db.conn.clone(),
        audit.clone(),
        "local-1",
    ));
    let reconciler = Arc::new(Reconciler::new(
        jobs.clone(),
        allocs.clone(),
        storage,
        audit.clone(),
        ReconcilerConfig::new("local-1"),
    ));
    let cleaner_storage = Arc::new(FsFixtureStorage::open(&temp.root(), allocs.clone()).unwrap());
    let cleaner = Arc::new(Cleaner::new(allocs.clone(), cleaner_storage));
    let sweeper = Arc::new(Sweeper::new(
        jobs,
        allocs,
        retention,
        reconciler.clone(),
        SweeperConfig::default(),
    ));
    let runtime = Arc::new(Runtime::new(
        reconciler,
        cleaner,
        sweeper,
        Arc::new(|root: &Path| {
            <FsNamespaceLock as crate::ports::namespace_lock::NamespaceLock>::try_acquire(root)
        }),
        clock,
        RuntimeConfig::new(temp.root()),
    ));
    runtime.start().await.unwrap();
    assert_eq!(runtime.state(), RuntimeState::Ready);
    let app = routes(BackupApiState::new(db.conn.clone(), config).with_runtime(runtime))
        .with_state(test_domain_context(db.conn.clone()));
    let (status, instance_json) = body(
        app.clone()
            .oneshot(request("GET", "/backup/instances"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(instance_json["instances"].as_array().unwrap().len(), 1);
    assert_eq!(instance_json["instances"][0]["id"], "local-1");
    assert_eq!(instance_json["instances"][0]["environment"], "local");
    assert_eq!(
        instance_json["instances"][0]["capabilities"]["download"],
        false
    );
    let (status, status_json) = body(
        app.clone()
            .oneshot(request("GET", "/backup/status"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(status_json["state"], "ready");
    assert_eq!(status_json["synthetic_only"], true);

    for uri in ["/backup/jobs?limit=101", "/backup/jobs?cursor=not-a-cursor"] {
        let (status, error) = body(app.clone().oneshot(request("GET", uri)).await.unwrap()).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(error["reason_code"], "MALFORMED_REQUEST");
    }
    let (status, empty_list) = body(
        app.clone()
            .oneshot(request("GET", "/backup/jobs"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(empty_list["jobs"].as_array().unwrap().len(), 0);
    assert_eq!(empty_list["next_cursor"], Value::Null);

    let wrong_origin = Request::builder()
        .method("POST")
        .uri("/backup/jobs")
        .header(header::HOST, "localhost:8989")
        .header("Origin", "http://untrusted.example")
        .header(header::CONTENT_TYPE, "application/json")
        .extension(admin())
        .body(Body::from(br#"{"instance_id":"local-1"}"#.to_vec()))
        .unwrap();
    let (status, error) = body(app.clone().oneshot(wrong_origin).await.unwrap()).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(error["reason_code"], "ORIGIN_NOT_ALLOWED");

    let wrong_content_type = Request::builder()
        .method("POST")
        .uri("/backup/jobs")
        .header(header::HOST, "localhost:8989")
        .header("Origin", "http://localhost:3002")
        .header(header::CONTENT_TYPE, "text/plain")
        .header("Idempotency-Key", "key-12345678")
        .extension(admin())
        .body(Body::from(br#"{"instance_id":"local-1"}"#.to_vec()))
        .unwrap();
    let (status, error) = body(app.clone().oneshot(wrong_content_type).await.unwrap()).await;
    assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(error["reason_code"], "UNSUPPORTED_MEDIA_TYPE");

    let oversized = Request::builder()
        .method("POST")
        .uri("/backup/jobs")
        .header(header::HOST, "localhost:8989")
        .header("Origin", "http://localhost:3002")
        .header(header::CONTENT_TYPE, "application/json")
        .header(
            header::CONTENT_LENGTH,
            (crate::handlers::create_job::MAX_BODY_BYTES + 1).to_string(),
        )
        .extension(admin())
        .body(Body::empty())
        .unwrap();
    let (status, error) = body(app.oneshot(oversized).await.unwrap()).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(error["reason_code"], "PAYLOAD_TOO_LARGE");
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts04_foreign_and_unknown_ids_are_the_same_404_and_ts49_is_safe() {
    let db = backup_test_db().await;
    let temp = crate::infra::tests::fs_support::TempDir::new("http-read");
    let config = config(&temp.root());
    let clock = Arc::new(FixedClock(t0()));
    let audit = Arc::new(DbAuditSink::new(db.conn.clone()));
    let allocs = Arc::new(DbAllocationStore::new(
        db.conn.clone(),
        audit.clone(),
        clock.clone(),
        "local-1",
        AllocationLimits::default(),
    ));
    let storage = Arc::new(FsFixtureStorage::open(&temp.root(), allocs.clone()).unwrap());
    let jobs = Arc::new(DbJobStore::new(
        db.conn.clone(),
        audit.clone(),
        clock.clone(),
    ));
    let retention = Arc::new(DbRetentionStore::new(
        db.conn.clone(),
        audit.clone(),
        "local-1",
    ));
    let reconciler = Arc::new(Reconciler::new(
        jobs.clone(),
        allocs.clone(),
        storage.clone(),
        audit.clone(),
        ReconcilerConfig::new("local-1"),
    ));
    let cleaner = Arc::new(Cleaner::new(allocs.clone(), storage));
    let sweeper = Arc::new(Sweeper::new(
        jobs,
        allocs,
        retention,
        reconciler.clone(),
        SweeperConfig::default(),
    ));
    let runtime = Arc::new(Runtime::new(
        reconciler,
        cleaner,
        sweeper,
        Arc::new(|root: &Path| {
            <FsNamespaceLock as crate::ports::namespace_lock::NamespaceLock>::try_acquire(root)
        }),
        clock,
        RuntimeConfig::new(temp.root()),
    ));
    runtime.start().await.unwrap();
    let app = routes(BackupApiState::new(db.conn.clone(), config).with_runtime(runtime))
        .with_state(test_domain_context(db.conn.clone()));
    let foreign_id = seed(&db.conn, Seed::new("foreign", BackupJobStatus::Queued)).await;
    let own_id = seed(&db.conn, Seed::new("local-1", BackupJobStatus::Queued)).await;
    for id in [foreign_id, Uuid::new_v4()] {
        let (status, json) = body(
            app.clone()
                .oneshot(request("GET", &format!("/backup/jobs/{id}")))
                .await
                .unwrap(),
        )
        .await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(json["reason_code"], "JOB_NOT_FOUND");
    }
    let (status, json) = body(
        app.oneshot(request("GET", &format!("/backup/jobs/{own_id}")))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["job"]["download_eligible"], false);
    assert_eq!(json["job"]["progress"], Value::Null);
    assert!(json["job"].get("actor_sub").is_none());
}
