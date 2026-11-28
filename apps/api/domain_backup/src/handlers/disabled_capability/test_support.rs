//! Shared HTTP test fixtures for the WU-09 handler tests (disabled capabilities, download
//! denial, previews). Test-only. Builds the real `routes()` router over a configured
//! `BackupApiState`, with a spy audit sink and a body-stream spy.

use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

use async_trait::async_trait;
use axum::body::{to_bytes, Body, Bytes};
use axum::http::{header, request::Builder, Request, StatusCode};
use domain_interface::{AuthenticatedActor, DomainContext};
use sea_orm::DatabaseTransaction;
use serde_json::Value;
use tower::ServiceExt;

use crate::domain::config::{BackupConfig, BackupConfigState};
use crate::handlers::http::{routes, BackupApiState};
use crate::infra::fs_namespace_lock::FsNamespaceLock;
use crate::lifecycle::cleaner::Cleaner;
use crate::lifecycle::reconciler::{Reconciler, ReconcilerConfig};
use crate::lifecycle::runtime::{Runtime, RuntimeConfig, RuntimeState};
use crate::lifecycle::sweeper::{Sweeper, SweeperConfig};
use crate::lifecycle::test_support::{
    log_lines, new_log, t0, FakeClock, Log, MemJobStore, MemStorage,
};
use crate::ports::allocation_store::{
    Allocation, AllocationError, AllocationStore, AllocationTotals, CleanupFailureOutcome,
};
use crate::ports::audit::{AuditError, AuditEvent, AuditSink};
use crate::ports::namespace_lock::NamespaceLock;
use crate::ports::retention_store::{PurgeReport, RetentionStore};

pub(crate) const INSTANCE_ID: &str = "local-1";
pub(crate) const INSTANCE_LABEL: &str = "Test local instance";

/// A unique, not-yet-created storage root under the link-free OS temp dir.
pub(crate) fn unique_root(tag: &str) -> PathBuf {
    crate::test_temp_dir().join(format!(
        "wu09-{tag}-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ))
}

pub(crate) fn enabled_config(root: &Path) -> BackupConfigState {
    BackupConfig::load(|key| {
        Some(match key {
            "BACKUP_ENABLED" => "true".into(),
            "BACKUP_INSTANCE_ID" => INSTANCE_ID.into(),
            "BACKUP_INSTANCE_LABEL" => INSTANCE_LABEL.into(),
            "BACKUP_STORAGE_ROOT" => root.to_string_lossy().into_owned(),
            "BACKUP_ALLOWED_ORIGINS" => "http://localhost:3002".into(),
            "BACKUP_REQUEST_TIMEOUT_SECS" => "30".into(),
            "BACKUP_MAX_HEADER_BYTES" => "16384".into(),
            _ => return None,
        })
    })
}

pub(crate) fn disabled_config() -> BackupConfigState {
    BackupConfig::load(|_| None)
}

pub(crate) fn admin() -> AuthenticatedActor {
    AuthenticatedActor {
        user_id: "test-admin".into(),
        email: None,
        primary_role: "authenticated".into(),
        app_roles: vec!["my-headless-cms-administrator".into()],
    }
}

pub(crate) fn writer() -> AuthenticatedActor {
    AuthenticatedActor {
        user_id: "test-writer".into(),
        email: None,
        primary_role: "authenticated".into(),
        app_roles: vec!["my-headless-cms-writer".into()],
    }
}

pub(crate) fn test_domain_context(conn: sea_orm::DatabaseConnection) -> DomainContext {
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

/// Audit sink that records every attempt and can be told to fail.
#[derive(Debug, Default)]
pub(crate) struct SpyAudit {
    events: Mutex<Vec<AuditEvent>>,
    fail: bool,
}

impl SpyAudit {
    pub(crate) fn recording() -> Arc<Self> {
        Arc::default()
    }

    pub(crate) fn failing() -> Arc<Self> {
        Arc::new(Self {
            events: Mutex::default(),
            fail: true,
        })
    }

    pub(crate) fn events(&self) -> Vec<AuditEvent> {
        self.events.lock().unwrap().clone()
    }
}

#[async_trait]
impl AuditSink for SpyAudit {
    async fn record(
        &self,
        _tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        self.events.lock().unwrap().push(ev);
        if self.fail {
            Err(AuditError::Unavailable)
        } else {
            Ok(())
        }
    }
}

/// A request body that records whether anyone polled it.
#[derive(Debug, Clone, Default)]
pub(crate) struct BodySpy(Arc<AtomicBool>);

impl BodySpy {
    pub(crate) fn polled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    pub(crate) fn body(&self) -> Body {
        Body::from_stream(PolledStream(self.0.clone()))
    }
}

struct PolledStream(Arc<AtomicBool>);

impl futures_core::Stream for PolledStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.store(true, Ordering::SeqCst);
        Poll::Ready(Some(Ok(Bytes::from_static(b"{}"))))
    }
}

/// Router over the real `routes()` with no database (`Disconnected`), a spy audit sink and an
/// in-memory job store. Without a lifecycle runtime the backup state is `not ready` when the
/// config is enabled.
pub(crate) fn offline_app(config: BackupConfigState, audit: Arc<SpyAudit>) -> axum::Router {
    let clock = FakeClock::new(t0());
    let store = MemJobStore::new(clock, new_log());
    let state = BackupApiState::new(sea_orm::DatabaseConnection::Disconnected, config)
        .with_persistence(store, audit);
    app_from_state(state)
}

pub(crate) fn app_from_state(state: BackupApiState) -> axum::Router {
    routes(state).with_state(test_domain_context(
        sea_orm::DatabaseConnection::Disconnected,
    ))
}

/// Request builder with the loopback Host header and an administrator actor.
pub(crate) fn builder(method: &str, uri: &str) -> Builder {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::HOST, "localhost:8989")
        .extension(admin())
}

/// Same as [`builder`] with a caller-chosen Host header (the only Host the request carries).
pub(crate) fn builder_with_host(method: &str, uri: &str, host: &str) -> Builder {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::HOST, host)
        .extension(admin())
}

pub(crate) fn get(uri: &str) -> Request<Body> {
    builder("GET", uri).body(Body::empty()).unwrap()
}

pub(crate) fn post_json(uri: &str, json: &str) -> Request<Body> {
    builder("POST", uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json.to_owned()))
        .unwrap()
}

pub(crate) async fn send(app: &axum::Router, request: Request<Body>) -> axum::response::Response {
    app.clone().oneshot(request).await.unwrap()
}

pub(crate) async fn json_of(response: axum::response::Response) -> (StatusCode, Value) {
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 64 * 1024).await.unwrap();
    let text = String::from_utf8_lossy(&bytes).into_owned();
    let value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| panic!("unexpected {status} response body: {text}"));
    (status, value)
}

/// Assert a crate error body: status, coarse `error_code`, `reason_code`, one fixed message
/// and a non-empty `correlation_id`. Returns the correlation id.
pub(crate) fn assert_error(
    actual: &(StatusCode, Value),
    status: StatusCode,
    error_code: &str,
    reason_code: &str,
    context: &str,
) -> String {
    let (got_status, json) = actual;
    assert_eq!(*got_status, status, "{context}: {json}");
    assert_eq!(json["error_code"], error_code, "{context}: {json}");
    assert_eq!(json["reason_code"], reason_code, "{context}: {json}");
    assert_eq!(
        json["errors"].as_array().map(Vec::len),
        Some(1),
        "{context}: exactly one safe message in {json}"
    );
    let correlation = json["correlation_id"].as_str().unwrap_or_default();
    assert!(
        !correlation.is_empty(),
        "{context}: correlation_id in {json}"
    );
    correlation.to_owned()
}

/// Allocation ledger with no rows. Every call is logged, so a test can prove the preview and
/// denial paths never reach the ledger (`MemAllocStore` fails its cleanup scan).
#[derive(Debug)]
struct IdleAllocs(Log);

impl IdleAllocs {
    fn note(&self, call: &str) {
        self.0.lock().unwrap().push(format!("alloc:{call}"));
    }
}

#[async_trait]
impl AllocationStore for IdleAllocs {
    async fn register(
        &self,
        _job: Option<uuid::Uuid>,
        _bytes: u64,
        _epoch: i64,
        _expires: chrono::DateTime<chrono::Utc>,
    ) -> Result<Allocation, AllocationError> {
        self.note("register");
        Err(AllocationError::Persistence)
    }
    async fn mark_written(&self, _id: uuid::Uuid, _ver: i64) -> Result<(), AllocationError> {
        self.note("mark_written");
        Err(AllocationError::Persistence)
    }
    async fn release(&self, _id: uuid::Uuid, _ver: i64) -> Result<(), AllocationError> {
        self.note("release");
        Err(AllocationError::Persistence)
    }
    async fn claim_for_delete(
        &self,
        _id: uuid::Uuid,
        _ver: i64,
    ) -> Result<Option<Allocation>, AllocationError> {
        self.note("claim_for_delete");
        Err(AllocationError::Persistence)
    }
    async fn mark_deleted(&self, _id: uuid::Uuid, _ver: i64) -> Result<(), AllocationError> {
        self.note("mark_deleted");
        Err(AllocationError::Persistence)
    }
    async fn record_cleanup_failure(
        &self,
        _id: uuid::Uuid,
        _ver: i64,
        _reason: crate::ports::job_store::ReasonCode,
        _now: chrono::DateTime<chrono::Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError> {
        self.note("record_cleanup_failure");
        Err(AllocationError::Persistence)
    }
    async fn insert_orphan(
        &self,
        _id: uuid::Uuid,
        _key: &crate::ports::fixture_storage::StorageKey,
        _bytes: u64,
        _now: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AllocationError> {
        self.note("insert_orphan");
        Err(AllocationError::Persistence)
    }
    async fn due_for_cleanup(
        &self,
        _now: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<Allocation>, AllocationError> {
        self.note("due_for_cleanup");
        Ok(Vec::new())
    }
    async fn totals(&self) -> Result<AllocationTotals, AllocationError> {
        self.note("totals");
        Err(AllocationError::Persistence)
    }
    async fn get(&self, _id: uuid::Uuid) -> Result<Option<Allocation>, AllocationError> {
        self.note("get");
        Ok(None)
    }
}

/// Retention double: the preview and denial paths must never purge anything.
#[derive(Debug)]
struct NoRetention;

#[async_trait]
impl RetentionStore for NoRetention {
    async fn purge_retained(
        &self,
        _now: chrono::DateTime<chrono::Utc>,
    ) -> Result<PurgeReport, crate::ports::allocation_store::AllocationError> {
        Ok(PurgeReport::default())
    }
}

/// A Ready backup API over in-memory stores (no Docker). `log` is the shared call log of the
/// in-memory job and allocation stores; `api_audit` is the spy wired into the API state only
/// (the runtime's own start-up audit goes to a separate sink).
pub(crate) struct ReadyApp {
    pub app: axum::Router,
    pub state: BackupApiState,
    pub log: Log,
    pub api_audit: Arc<SpyAudit>,
    pub store: Arc<MemJobStore>,
    pub root: PathBuf,
    _runtime: Arc<Runtime>,
}

impl std::fmt::Debug for ReadyApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadyApp").finish_non_exhaustive()
    }
}

impl ReadyApp {
    /// Snapshot of every in-memory store call so far.
    pub(crate) fn calls(&self) -> Vec<String> {
        log_lines(&self.log)
    }
}

impl Drop for ReadyApp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
        let _ = std::fs::remove_file(FsNamespaceLock::lock_path(&self.root));
    }
}

pub(crate) async fn ready_offline(tag: &str, api_audit: Arc<SpyAudit>) -> ReadyApp {
    let root = unique_root(tag);
    std::fs::create_dir_all(&root).unwrap();
    // CFG-04: an existing storage root must be private (0700) on Unix.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).unwrap();
    }
    let clock = FakeClock::new(t0());
    let log = new_log();
    let store = MemJobStore::new(clock.clone(), log.clone());
    let allocs = Arc::new(IdleAllocs(log.clone()));
    let storage = MemStorage::logged(log.clone());
    let runtime_audit: Arc<dyn AuditSink> = SpyAudit::recording();
    let reconciler = Arc::new(Reconciler::new(
        store.clone(),
        allocs.clone(),
        storage.clone(),
        runtime_audit,
        ReconcilerConfig::new(INSTANCE_ID),
    ));
    let cleaner = Arc::new(Cleaner::new(allocs.clone(), storage));
    let sweeper = Arc::new(Sweeper::new(
        store.clone(),
        allocs,
        Arc::new(NoRetention),
        reconciler.clone(),
        SweeperConfig::default(),
    ));
    let runtime = Arc::new(Runtime::new(
        reconciler,
        cleaner,
        sweeper,
        Arc::new(|path: &Path| <FsNamespaceLock as NamespaceLock>::try_acquire(path)),
        clock,
        RuntimeConfig::new(root.clone()),
    ));
    runtime.start().await.expect("in-memory runtime starts");
    assert_eq!(runtime.state(), RuntimeState::Ready);
    let state = BackupApiState::new(
        sea_orm::DatabaseConnection::Disconnected,
        enabled_config(&root),
    )
    .with_runtime(runtime.clone())
    .with_persistence(store.clone(), api_audit.clone());
    ReadyApp {
        app: app_from_state(state.clone()),
        state,
        log,
        api_audit,
        store,
        root,
        _runtime: runtime,
    }
}
