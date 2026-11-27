//! WU-08 REST contract tests against the API adapter and PostgreSQL.

use super::alloc_support::{seed_alloc, SeedAlloc};
use super::support::*;
use crate::domain::config::{BackupConfig, BackupConfigState};
use crate::domain::idempotency::{key_digest, OPERATION};
use crate::domain::status::BackupJobStatus;
use crate::handlers::create_job::{AcceptContext, CreateJobHandler};
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
use sea_orm::ConnectionTrait;
use serde_json::Value;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;
use tracing_subscriber::layer::SubscriberExt;
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

#[derive(Clone)]
struct CapturedEvents(Arc<Mutex<Vec<String>>>);

impl<S> tracing_subscriber::Layer<S> for CapturedEvents
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _context: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut fields = EventFields::default();
        event.record(&mut fields);
        self.0.lock().unwrap().push(fields.0);
    }
}

#[derive(Default)]
struct EventFields(String);

impl tracing::field::Visit for EventFields {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        use std::fmt::Write as _;
        let _ = write!(self.0, "{}={value:?};", field.name());
    }
}

fn scan_canary<'a>(contents: impl IntoIterator<Item = &'a str>, canary: &str) -> bool {
    contents.into_iter().any(|value| value.contains(canary))
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
    let foreign_job = seed(
        &db.conn,
        Seed::new("other-instance", BackupJobStatus::Failed),
    )
    .await;
    seed_alloc(
        &db.conn,
        SeedAlloc::new(
            crate::ports::allocation_store::AllocationState::ReleasePending,
            100,
        )
        .job_id(foreign_job),
    )
    .await;
    seed_alloc(
        &db.conn,
        SeedAlloc::new(
            crate::ports::allocation_store::AllocationState::CleanupFailed,
            100,
        )
        .job_id(foreign_job),
    )
    .await;
    let app_state = BackupApiState::new(db.conn.clone(), config).with_runtime(runtime);
    let app = routes(app_state.clone()).with_state(test_domain_context(db.conn.clone()));
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
    assert_eq!(status_json["cleanup"]["pending"], 0);
    assert_eq!(status_json["cleanup"]["exhausted"], 0);
    assert_eq!(status_json["cleanup"]["orphan_discovered"], 0);
    assert_eq!(status_json["cleanup"]["ownership_incidents"], 0);
    assert_eq!(
        status_json["cleanup"]["pending"], 0,
        "foreign pending rows are excluded"
    );
    assert_eq!(
        status_json["cleanup"]["exhausted"], 0,
        "foreign cleanup failures are excluded"
    );

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

    let failed_job = seed(&db.conn, Seed::new("local-1", BackupJobStatus::Failed)).await;
    let _cleanup_failed = seed_alloc(
        &db.conn,
        SeedAlloc::new(
            crate::ports::allocation_store::AllocationState::CleanupFailed,
            100,
        )
        .job_id(failed_job),
    )
    .await;
    let (status, durable_status) = body(
        app.clone()
            .oneshot(request("GET", "/backup/status"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(durable_status["cleanup"]["exhausted"], 1);
    assert_eq!(durable_status["cleanup"]["ownership_incidents"], 0);
    assert_eq!(durable_status["escalation"]["visible"], true);

    for uri in [
        "/backup/jobs?limit=101",
        "/backup/jobs?limit=abc",
        "/backup/jobs?cursor=not-a-cursor",
        "/backup/jobs?unknown=value",
    ] {
        let (status, error) = body(app.clone().oneshot(request("GET", uri)).await.unwrap()).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(error["reason_code"], "MALFORMED_REQUEST");
    }
    for bad_json in [
        br#"{"#.as_slice(),
        br#"{"instance_id":"local-1","extra":true}"#.as_slice(),
        br#"{"instance_id":1}"#.as_slice(),
    ] {
        let invalid = Request::builder()
            .method("POST")
            .uri("/backup/jobs")
            .header(header::HOST, "localhost:8989")
            .header("Origin", "http://localhost:3002")
            .header(header::CONTENT_TYPE, "application/json")
            .header("Idempotency-Key", "key-12345678")
            .extension(admin())
            .body(Body::from(bad_json.to_vec()))
            .unwrap();
        let (status, error) = body(app.clone().oneshot(invalid).await.unwrap()).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(error["reason_code"], "MALFORMED_REQUEST");
        assert!(!error.to_string().contains("extra"));
    }
    let wrong_length_body = Request::builder()
        .method("POST")
        .uri("/backup/jobs")
        .header(header::HOST, "localhost:8989")
        .header("Origin", "http://localhost:3002")
        .header(header::CONTENT_TYPE, "application/json")
        .header("Idempotency-Key", "key-12345678")
        .header(header::CONTENT_LENGTH, "1")
        .extension(admin())
        .body(Body::from(vec![
            b' ';
            crate::handlers::create_job::MAX_BODY_BYTES
                + 1
        ]))
        .unwrap();
    let (status, error) = body(app.clone().oneshot(wrong_length_body).await.unwrap()).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(error["reason_code"], "PAYLOAD_TOO_LARGE");

    for (payload, reason) in [
        (
            br#"{"instance_id":"local-1","target_url":"http://attacker"}"#.to_vec(),
            "MALFORMED_REQUEST",
        ),
        (
            br#"{"instance_id":"local-1","url":"http://CANARY.invalid"}"#.to_vec(),
            "MALFORMED_REQUEST",
        ),
        (
            br#"{"instance_id":"local-1","dsn":"postgres://CANARY.invalid"}"#.to_vec(),
            "MALFORMED_REQUEST",
        ),
        (
            br#"{"instance_id":"local-1","key":"CANARY-secret"}"#.to_vec(),
            "MALFORMED_REQUEST",
        ),
        (
            br#"{"instance_id":"local-1","synthetic":false}"#.to_vec(),
            "MALFORMED_REQUEST",
        ),
        (
            br#"{"instance_id":"other"}"#.to_vec(),
            "INSTANCE_NOT_ALLOWED",
        ),
    ] {
        let before_jobs = jobs_for_instance(&db.conn, "local-1").await;
        let before_idem = idempotency_rows(&db.conn).await;
        let before_audit = audit_count_for_instance(&db.conn, "local-1").await;
        let response = Request::builder()
            .method("POST")
            .uri("/backup/jobs")
            .header(header::HOST, "localhost:8989")
            .header("Origin", "http://localhost:3002")
            .header(header::CONTENT_TYPE, "application/json")
            .header("Idempotency-Key", "key-12345678")
            .extension(admin())
            .body(Body::from(payload))
            .unwrap();
        let (status, error) = body(app.clone().oneshot(response).await.unwrap()).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(error["reason_code"], reason);
        assert!(!error.to_string().contains("attacker"));
        assert!(!error.to_string().contains("other"));
        assert_eq!(jobs_for_instance(&db.conn, "local-1").await, before_jobs);
        assert_eq!(idempotency_rows(&db.conn).await, before_idem);
        assert_eq!(
            audit_count_for_instance(&db.conn, "local-1").await,
            before_audit
        );
    }

    // Install a scoped tracing subscriber; positive control exercises the actual scanner.
    let captured_events = Arc::new(Mutex::new(Vec::new()));
    let subscriber = tracing_subscriber::registry().with(CapturedEvents(captured_events.clone()));
    let _subscriber_guard = tracing::subscriber::set_default(subscriber);
    let log_canary = "CANARY-positive-control-0001";
    tracing::info!(canary = log_canary, "positive-control");
    let positive_control_events = captured_events.lock().unwrap().clone();
    assert!(scan_canary(
        positive_control_events.iter().map(String::as_str),
        log_canary
    ));
    captured_events.lock().unwrap().clear();

    let valid_replay_key = "CANARY-valid-route-key-0001";
    let request_canary = "CANARY-request-body-0001";
    let jobs_before_accept = jobs_for_instance(&db.conn, "local-1").await;
    let idempotency_before_accept = idempotency_rows(&db.conn).await;
    let audit_before_accept = audit_count_for_instance(&db.conn, "local-1").await;
    let valid_request = || {
        Request::builder()
            .method("POST")
            .uri("/backup/jobs")
            .header(header::HOST, "localhost:8989")
            .header("Origin", "http://localhost:3002")
            .header(header::CONTENT_TYPE, "application/json")
            .header("Idempotency-Key", valid_replay_key)
            .extension(admin())
            .body(Body::from(br#"{"instance_id":"local-1"}"#.to_vec()))
            .unwrap()
    };
    let created_response = app.clone().oneshot(valid_request()).await.unwrap();
    let created_headers = format!("{:?}", created_response.headers());
    let (created_status, created) = body(created_response).await;
    assert_eq!(created_status, StatusCode::ACCEPTED);
    let created_id = created["job"]["id"].as_str().unwrap().to_owned();
    let digest = key_digest("test-admin", OPERATION, valid_replay_key);
    assert!(
        !audit_rows_for_job(&db.conn, Uuid::parse_str(&created_id).unwrap())
            .await
            .iter()
            .any(|event| event.digest.as_deref() == Some(valid_replay_key)),
        "raw idempotency key must not appear in audit"
    );
    let (replay_status, replay) = body(app.clone().oneshot(valid_request()).await.unwrap()).await;
    assert_eq!(replay_status, StatusCode::OK);
    assert_eq!(replay["job"]["id"], created_id);
    assert!(!created.to_string().contains(valid_replay_key));
    assert!(!replay.to_string().contains(valid_replay_key));
    let audit_text: String = db
        .conn
        .query_one(pg(
            "SELECT COALESCE(string_agg(row_to_json(a)::text, ''), '') AS content FROM backup_audit a",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "content")
        .unwrap();
    let idempotency_text: String = db
        .conn
        .query_one(pg(
            "SELECT COALESCE(string_agg(row_to_json(i)::text, ''), '') AS content FROM backup_idempotency i",
            vec![],
        ))
        .await
        .unwrap()
        .unwrap()
        .try_get("", "content")
        .unwrap();
    assert!(!audit_text.contains(valid_replay_key));
    assert!(!idempotency_text.contains(valid_replay_key));
    for table in ["backup_jobs", "backup_allocations", "backup_epoch"] {
        let sql = format!(
            "SELECT COALESCE(string_agg(row_to_json(r)::text, ''), '') AS content FROM {table} r"
        );
        let persisted: String = db
            .conn
            .query_one(pg(&sql, vec![]))
            .await
            .unwrap()
            .unwrap()
            .try_get("", "content")
            .unwrap();
        assert!(
            !persisted.contains(valid_replay_key),
            "raw key must not appear in {table}"
        );
    }
    assert_eq!(
        idempotency_rows(&db.conn).await,
        idempotency_before_accept + 1
    );
    assert_eq!(
        jobs_for_instance(&db.conn, "local-1").await,
        jobs_before_accept + 1
    );
    assert_eq!(
        audit_count_for_instance(&db.conn, "local-1").await,
        audit_before_accept + 1
    );
    assert_eq!(digest.len(), 64);
    assert!(!created.to_string().contains("CANARY-valid-route-key-0001"));
    assert!(!replay.to_string().contains("CANARY-valid-route-key-0001"));

    let jobs_after_accept = jobs_for_instance(&db.conn, "local-1").await;
    let idempotency_after_accept = idempotency_rows(&db.conn).await;
    let audit_after_accept = audit_count_for_instance(&db.conn, "local-1").await;
    let concurrent_create = Request::builder()
        .method("POST")
        .uri("/backup/jobs")
        .header(header::HOST, "localhost:8989")
        .header("Origin", "http://localhost:3002")
        .header(header::CONTENT_TYPE, "application/json")
        .header("Idempotency-Key", "second-route-key-0001")
        .extension(admin())
        .body(Body::from(br#"{"instance_id":"local-1"}"#.to_vec()))
        .unwrap();
    let (conflict_status, conflict) =
        body(app.clone().oneshot(concurrent_create).await.unwrap()).await;
    assert_eq!(conflict_status, StatusCode::CONFLICT);
    assert_eq!(conflict["reason_code"], "CONCURRENCY_LIMIT");
    assert_eq!(
        jobs_for_instance(&db.conn, "local-1").await,
        jobs_after_accept
    );
    assert_eq!(idempotency_rows(&db.conn).await, idempotency_after_accept);
    assert_eq!(
        audit_count_for_instance(&db.conn, "local-1").await,
        audit_after_accept
    );

    let canary_body_request = Request::builder()
        .method("POST")
        .uri("/backup/jobs")
        .header(header::HOST, "localhost:8989")
        .header("Origin", "http://localhost:3002")
        .header(header::CONTENT_TYPE, "application/json")
        .header("Idempotency-Key", "CANARY-invalid-key-0001")
        .extension(admin())
        .body(Body::from(format!(
            r#"{{"instance_id":"local-1","secret":"{request_canary}"}}"#
        )))
        .unwrap();
    let (canary_status, canary_error) =
        body(app.clone().oneshot(canary_body_request).await.unwrap()).await;
    assert_eq!(canary_status, StatusCode::BAD_REQUEST);
    assert_eq!(canary_error["reason_code"], "MALFORMED_REQUEST");
    assert!(!canary_error.to_string().contains(request_canary));
    let invalid_path_canary = "/backup/jobs/CANARY-invalid-path-0001";
    let (invalid_path_status, invalid_path_error) = body(
        app.clone()
            .oneshot(request("GET", invalid_path_canary))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(invalid_path_status, StatusCode::NOT_FOUND);
    assert_eq!(invalid_path_error["reason_code"], "JOB_NOT_FOUND");
    assert!(!invalid_path_error
        .to_string()
        .contains("CANARY-invalid-path-0001"));

    let wrong_role = Request::builder()
        .method("GET")
        .uri("/backup/instances")
        .header(header::HOST, "localhost:8989")
        .extension(AuthenticatedActor {
            user_id: "writer-sub".into(),
            email: None,
            primary_role: "authenticated".into(),
            app_roles: vec!["my-headless-cms-writer".into()],
        })
        .body(Body::empty())
        .unwrap();
    let (denied_status, denied) = body(app.clone().oneshot(wrong_role).await.unwrap()).await;
    assert_eq!(denied_status, StatusCode::FORBIDDEN);
    assert_eq!(denied["reason_code"], "ROLE_REQUIRED");

    let wrong_host = Request::builder()
        .method("GET")
        .uri("/backup/instances")
        .header(header::HOST, "public.example:8989")
        .extension(admin())
        .body(Body::empty())
        .unwrap();
    let (host_status, host_error) = body(app.clone().oneshot(wrong_host).await.unwrap()).await;
    assert_eq!(host_status, StatusCode::FORBIDDEN);
    assert_eq!(host_error["reason_code"], "HOST_NOT_ALLOWED");

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

    for content_type in [None, Some("text/plain")] {
        let mut builder = Request::builder()
            .method("POST")
            .uri("/backup/jobs")
            .header(header::HOST, "localhost:8989")
            .header("Origin", "http://localhost:3002")
            .header("Idempotency-Key", "key-12345678")
            .extension(admin());
        if let Some(content_type) = content_type {
            builder = builder.header(header::CONTENT_TYPE, content_type);
        }
        let request = builder
            .body(Body::from(br#"{"instance_id":"local-1"}"#.to_vec()))
            .unwrap();
        let (status, error) = body(app.clone().oneshot(request).await.unwrap()).await;
        assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(error["reason_code"], "UNSUPPORTED_MEDIA_TYPE");
    }

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
    let (status, error) = body(app.clone().oneshot(oversized).await.unwrap()).await;
    assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(error["reason_code"], "PAYLOAD_TOO_LARGE");

    let (status, status_json) = body(
        app.clone()
            .oneshot(request("GET", "/backup/status"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let metrics = app_state.metric_snapshot();
    assert!(metrics.iter().any(|(labels, count)| {
        labels.job_kind.as_str() == "backup_db_slice"
            && labels.outcome == crate::domain::metrics::MetricOutcome::Accepted
            && labels.error_code == crate::domain::metrics::MetricErrorCode::None
            && *count == 1
    }));
    assert!(metrics.iter().any(|(labels, _)| {
        labels.outcome == crate::domain::metrics::MetricOutcome::Refused
            && labels.error_code == crate::domain::metrics::MetricErrorCode::ValidationError
    }));
    assert!(metrics.iter().any(|(labels, _)| {
        labels.outcome == crate::domain::metrics::MetricOutcome::Denied
            && labels.error_code == crate::domain::metrics::MetricErrorCode::Forbidden
    }));
    for error_code in [
        crate::domain::metrics::MetricErrorCode::UnsupportedMediaType,
        crate::domain::metrics::MetricErrorCode::PayloadTooLarge,
    ] {
        assert!(metrics.iter().any(|(labels, _)| {
            labels.outcome == crate::domain::metrics::MetricOutcome::Refused
                && labels.error_code == error_code
        }));
    }
    assert!(metrics.iter().all(|(labels, _)| {
        labels
            .attributes()
            .iter()
            .all(|(key, _)| ["job_kind", "outcome", "error_code"].contains(key))
    }));
    for (_, metric_value) in &metrics {
        assert!(!metric_value.to_string().contains("CANARY"));
    }
    for (labels, _) in &metrics {
        for (_, value) in labels.attributes() {
            assert!(!value.contains("CANARY"));
        }
    }
    let metric_text = format!("{metrics:?}");
    assert!(!metric_text.contains("CANARY"));
    let status_text = status_json.to_string();
    assert!(!status_text.contains("CANARY"));

    let event_snapshot = captured_events.lock().unwrap().clone();
    for canary in [
        valid_replay_key,
        request_canary,
        "CANARY-invalid-key-0001",
        "CANARY-invalid-path-0001",
        log_canary,
    ] {
        assert!(!scan_canary(
            event_snapshot.iter().map(String::as_str),
            canary
        ));
    }

    for table in [
        "backup_jobs",
        "backup_idempotency",
        "backup_audit",
        "backup_allocations",
        "backup_epoch",
    ] {
        let sql = format!(
            "SELECT COALESCE(string_agg(row_to_json(r)::text, ''), '') AS content FROM {table} r"
        );
        let persisted: String = db
            .conn
            .query_one(pg(&sql, vec![]))
            .await
            .unwrap()
            .unwrap()
            .try_get("", "content")
            .unwrap();
        for canary in [
            valid_replay_key,
            request_canary,
            "CANARY-invalid-key-0001",
            "CANARY-invalid-path-0001",
            log_canary,
        ] {
            assert!(
                !persisted.contains(canary),
                "canary {canary} must not appear in {table}"
            );
        }
    }
    let response_text = format!(
        "headers={created_headers}; created={created}; replay={replay}; conflict={conflict}; \
         denials={denied}{host_error}{error}; status={status_json}"
    );
    for canary in [
        valid_replay_key,
        request_canary,
        "CANARY-invalid-key-0001",
        "CANARY-invalid-path-0001",
        log_canary,
    ] {
        assert!(!response_text.contains(canary));
    }
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
        jobs.clone(),
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
        clock.clone(),
        RuntimeConfig::new(temp.root()),
    ));
    runtime.start().await.unwrap();
    let app = routes(BackupApiState::new(db.conn.clone(), config).with_runtime(runtime))
        .with_state(test_domain_context(db.conn.clone()));
    let foreign_id = seed(&db.conn, Seed::new("foreign", BackupJobStatus::Queued)).await;
    let replay_key = "foreign-replay-key-01";
    let replay_request = crate::handlers::create_job::request::parse_create_request(
        Some("application/json"),
        Some(replay_key),
        br#"{"instance_id":"local-1"}"#,
        "local-1",
    )
    .unwrap();
    let replay_handler = CreateJobHandler::new(
        db.conn.clone(),
        jobs.clone(),
        audit.clone(),
        clock.clone(),
        30,
        1,
    );
    let replayed_job = replay_handler
        .create(
            &AcceptContext {
                actor_sub: "test-admin".to_owned(),
                correlation_id: "replay-test".to_owned(),
            },
            replay_request,
        )
        .await
        .expect("seed replayable job");
    let replay_job_id = match replayed_job {
        crate::handlers::create_job::Accepted::Created(row) => row.id,
        _ => panic!("first acceptance must create a job"),
    };
    let own_id = replay_job_id;
    let replay_http_request = Request::builder()
        .method("POST")
        .uri("/backup/jobs")
        .header(header::HOST, "localhost:8989")
        .header("Origin", "http://localhost:3002")
        .header(header::CONTENT_TYPE, "application/json")
        .header("Idempotency-Key", replay_key)
        .extension(admin())
        .body(Body::from(br#"{"instance_id":"local-1"}"#.to_vec()))
        .unwrap();
    let (replay_status, replay_body) =
        body(app.clone().oneshot(replay_http_request).await.unwrap()).await;
    assert_eq!(replay_status, StatusCode::OK);
    assert_eq!(replay_body["job"]["id"], replay_job_id.to_string());
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
    let (status, foreign_list) = body(
        app.clone()
            .oneshot(request("GET", "/backup/jobs?limit=20"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let listed = foreign_list["jobs"].as_array().unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0]["id"], own_id.to_string());
    assert!(listed.iter().all(|job| job["id"] != foreign_id.to_string()));
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
