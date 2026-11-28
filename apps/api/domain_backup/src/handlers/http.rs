//! Axum adapter for the backup REST routes: read/create (IF-01..IF-04, IF-11; FN-05, FN-09),
//! download denial (IF-05), disabled capabilities (IF-06..IF-08) and previews (IF-09, IF-10).

use std::sync::Arc;

use axum::body::{to_bytes, Body};
use axum::extract::{Extension, Path, Query, State};
use axum::http::{header, HeaderMap, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::Engine;
use chrono::{DateTime, Utc};
use domain_interface::{AuthenticatedActor, DomainContext};
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::domain::config::BackupConfigState;
use crate::domain::error::AppError;
use crate::domain::instance::{BackupInstance, InstanceRegistry};
use crate::domain::metrics::{
    MetricErrorCode, MetricJobKind, MetricLabels, MetricOutcome, MetricRecorder,
};
use crate::domain::policy::{authorize_operation, AuthorizationFailure, BackupPolicy};
use crate::domain::response::{project_create_error, project_error, SafeErrorBody};
use crate::handlers::create_job::{AcceptContext, Accepted, CreateJobError, CreateJobHandler};
use crate::handlers::job_response::JobResponse;
use crate::handlers::status::{project_status, CleanupStatus, StatusResponse};
use crate::handlers::{disabled_capability, download_denied, preview};
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::lifecycle::runtime::{Runtime, RuntimeState};
use crate::ports::audit::AuditSink;
use crate::ports::job_store::{JobCursor, JobRow, JobStore, StoreError};

pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 100;

#[derive(Clone)]
pub struct BackupApiState {
    pub conn: sea_orm::DatabaseConnection,
    pub config: BackupConfigState,
    pub runtime: Option<Arc<Runtime>>,
    pub configured_port: u16,
    pub correlation_id: Arc<dyn Fn() -> String + Send + Sync>,
    pub db_healthy: bool,
    pub cleanup: Option<CleanupStatus>,
    pub metrics: Arc<MetricRecorder>,
    pub(crate) store: Arc<dyn JobStore>,
    pub(crate) audit: Arc<dyn AuditSink>,
}

impl std::fmt::Debug for BackupApiState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("BackupApiState")
            .field("config", &self.config)
            .field("runtime", &self.runtime)
            .field("configured_port", &self.configured_port)
            .field("db_healthy", &self.db_healthy)
            .field("cleanup", &self.cleanup)
            .finish_non_exhaustive()
    }
}

fn nonnegative_count(value: i64) -> u64 {
    u64::try_from(value).unwrap_or(0)
}

fn create_metric_error(error: CreateJobError) -> MetricErrorCode {
    match error {
        CreateJobError::UnsupportedMediaType => MetricErrorCode::UnsupportedMediaType,
        CreateJobError::PayloadTooLarge => MetricErrorCode::PayloadTooLarge,
        CreateJobError::MalformedRequest
        | CreateJobError::IdempotencyKeyRequired
        | CreateJobError::IdempotencyKeyInvalid
        | CreateJobError::InstanceNotAllowed => MetricErrorCode::ValidationError,
        CreateJobError::IdempotencyKeyReused | CreateJobError::ConcurrencyLimit => {
            MetricErrorCode::Conflict
        }
        CreateJobError::AuditUnavailable => MetricErrorCode::AuditUnavailable,
        CreateJobError::StoreUnavailable => MetricErrorCode::StoreUnavailable,
    }
}

impl BackupApiState {
    pub fn new(conn: sea_orm::DatabaseConnection, config: BackupConfigState) -> Self {
        let clock: Arc<dyn crate::domain::clock::Clock> =
            Arc::new(crate::domain::clock::SystemClock);
        let audit: Arc<dyn AuditSink> = Arc::new(DbAuditSink::new(conn.clone()));
        let store: Arc<dyn JobStore> =
            Arc::new(DbJobStore::new(conn.clone(), audit.clone(), clock));
        Self {
            conn,
            config,
            runtime: None,
            configured_port: std::env::var("PORT")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(8989),
            correlation_id: Arc::new(|| Uuid::new_v4().to_string()),
            db_healthy: true,
            cleanup: None,
            metrics: Arc::new(MetricRecorder::default()),
            store,
            audit,
        }
    }

    pub fn with_runtime(mut self, runtime: Arc<Runtime>) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn with_persistence(mut self, store: Arc<dyn JobStore>, audit: Arc<dyn AuditSink>) -> Self {
        self.store = store;
        self.audit = audit;
        self
    }

    pub fn metric_snapshot(&self) -> Vec<(MetricLabels, u64)> {
        self.metrics.snapshot()
    }

    pub(crate) fn record_metric(&self, outcome: MetricOutcome, error_code: MetricErrorCode) {
        self.metrics.record(MetricLabels {
            job_kind: MetricJobKind::BackupDbSlice,
            outcome,
            error_code,
        });
    }

    pub(crate) fn record_refusal(&self, error: &AppError) {
        self.record_metric(
            MetricOutcome::Refused,
            match error {
                AppError::BackupDisabled => MetricErrorCode::BackupDisabled,
                AppError::BackupNotReady => MetricErrorCode::BackupNotReady,
                AppError::Forbidden { .. } | AppError::Unauthorized => MetricErrorCode::Forbidden,
                AppError::Validation { .. } => MetricErrorCode::ValidationError,
                AppError::PayloadTooLarge => MetricErrorCode::PayloadTooLarge,
                AppError::UnsupportedMediaType => MetricErrorCode::UnsupportedMediaType,
                AppError::Conflict { .. } => MetricErrorCode::Conflict,
                AppError::Unavailable { reason_code } if *reason_code == "AUDIT_UNAVAILABLE" => {
                    MetricErrorCode::AuditUnavailable
                }
                AppError::StoreUnavailable
                | AppError::Unavailable { .. }
                | AppError::NotFound
                | AppError::JobNotFound => MetricErrorCode::StoreUnavailable,
            },
        );
    }

    pub(crate) fn config_enabled(&self) -> Option<&crate::domain::config::BackupConfig> {
        match &self.config {
            BackupConfigState::Enabled(config) => Some(config),
            BackupConfigState::Disabled { .. } => None,
        }
    }

    pub(crate) fn registry(&self) -> Option<InstanceRegistry> {
        self.config_enabled().map(InstanceRegistry::from_config)
    }

    pub(crate) fn policy(&self) -> Option<BackupPolicy> {
        self.registry().map(BackupPolicy::new)
    }

    pub(crate) fn operational_error(&self) -> Option<AppError> {
        if self.config_enabled().is_none() {
            return Some(AppError::BackupDisabled);
        }
        if !self.db_healthy {
            return Some(AppError::StoreUnavailable);
        }
        if !self
            .runtime
            .as_ref()
            .is_some_and(|runtime| runtime.state() == RuntimeState::Ready)
        {
            return Some(AppError::BackupNotReady);
        }
        None
    }

    pub(crate) fn correlation_id(&self) -> String {
        (self.correlation_id)()
    }
}

pub fn routes(state: BackupApiState) -> Router<DomainContext> {
    let shared = Arc::new(state);
    Router::<DomainContext>::new()
        .route("/backup/instances", get(list_instances))
        .route("/backup/jobs", get(list_jobs).post(create_job))
        .route("/backup/jobs/{job_id}", get(get_job))
        .route(
            "/backup/jobs/{job_id}/download",
            get(download_denied::deny_download),
        )
        .route("/backup/packages", post(disabled_capability::upload))
        .route("/backup/preflight", post(disabled_capability::preflight))
        .route("/backup/restore", post(disabled_capability::restore))
        .route(
            "/backup/previews/preflight",
            post(preview::preview_preflight),
        )
        .route(
            "/backup/previews/restore-confirmation",
            post(preview::preview_restore_confirmation),
        )
        .route("/backup/status", get(status))
        .layer(middleware::from_fn_with_state(
            shared.clone(),
            authorization,
        ))
        .layer(Extension(shared.clone()))
}

async fn authorization(
    State(state): State<Arc<BackupApiState>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let is_post = request.method() == axum::http::Method::POST;
    let actor = request.extensions().get::<AuthenticatedActor>().cloned();
    let allowed_origins = state
        .config_enabled()
        .map(|config| config.allowed_origins().to_vec())
        .unwrap_or_default();
    if let Err(error) = authorize_operation(
        actor.as_ref(),
        request.headers(),
        is_post,
        state.configured_port,
        &allowed_origins,
    ) {
        let metric_error = match error {
            AuthorizationFailure::RoleRequired
            | AuthorizationFailure::HostNotAllowed
            | AuthorizationFailure::OriginNotAllowed => MetricErrorCode::Forbidden,
        };
        state.record_metric(MetricOutcome::Denied, metric_error);
        return project_auth_error(error, &state.correlation_id()).into_response();
    }
    if let Some(actor) = actor {
        request.extensions_mut().insert(actor);
    }
    next.run(request).await
}

fn project_auth_error(error: AuthorizationFailure, correlation_id: &str) -> SafeErrorBody {
    project_error(
        &AppError::Forbidden {
            reason_code: error.reason_code(),
        },
        correlation_id,
    )
}

async fn list_instances(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(_actor): Extension<AuthenticatedActor>,
) -> Response {
    if let Some(error) = state.operational_error() {
        state.record_refusal(&error);
        return project_error(&error, &state.correlation_id()).into_response();
    }
    let registry = state.registry().expect("enabled config checked");
    let owned: Vec<BackupInstance> = registry.list().into_iter().cloned().collect();
    (
        StatusCode::OK,
        Json(json!({ "synthetic": true, "instances": owned })),
    )
        .into_response()
}

async fn create_job(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(actor): Extension<AuthenticatedActor>,
    headers: HeaderMap,
    body: Body,
) -> Response {
    if let Some(error) = state.operational_error() {
        state.record_refusal(&error);
        return project_error(&error, &state.correlation_id()).into_response();
    }
    let config = state.config_enabled().expect("enabled config checked");
    let request_id = state.correlation_id();
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok());
    let key = headers
        .get("Idempotency-Key")
        .and_then(|value| value.to_str().ok());
    if !crate::handlers::create_job::request::is_json_content_type(content_type) {
        state.record_metric(
            MetricOutcome::Refused,
            MetricErrorCode::UnsupportedMediaType,
        );
        return project_create_error(
            crate::handlers::create_job::CreateJobError::UnsupportedMediaType,
            &request_id,
        )
        .into_response();
    }
    if headers
        .get(header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|n| n > crate::handlers::create_job::MAX_BODY_BYTES)
    {
        state.record_metric(MetricOutcome::Refused, MetricErrorCode::PayloadTooLarge);
        return project_create_error(
            crate::handlers::create_job::CreateJobError::PayloadTooLarge,
            &request_id,
        )
        .into_response();
    }
    let bytes = match to_bytes(body, crate::handlers::create_job::MAX_BODY_BYTES + 1).await {
        Ok(bytes) => bytes,
        Err(_) => {
            state.record_metric(MetricOutcome::Refused, MetricErrorCode::PayloadTooLarge);
            return project_create_error(
                crate::handlers::create_job::CreateJobError::PayloadTooLarge,
                &request_id,
            )
            .into_response();
        }
    };
    let parsed = match crate::handlers::create_job::parse_create_request(
        content_type,
        key,
        &bytes,
        config.instance_id(),
    ) {
        Ok(request) => request,
        Err(error) => {
            state.record_metric(MetricOutcome::Refused, create_metric_error(error));
            return project_create_error(error, &request_id).into_response();
        }
    };
    let handler = CreateJobHandler::new(
        state.conn.clone(),
        state.store.clone(),
        state.audit.clone(),
        Arc::new(crate::domain::clock::SystemClock),
        config.bounds().retention_days(),
        state
            .runtime
            .as_ref()
            .and_then(|runtime| runtime.epoch())
            .unwrap_or(0),
    );
    match handler
        .create(
            &AcceptContext {
                actor_sub: actor.user_id,
                correlation_id: request_id,
            },
            parsed,
        )
        .await
    {
        Ok(Accepted::Created(row)) => {
            state.record_metric(MetricOutcome::Accepted, MetricErrorCode::None);
            create_response(&state, row, StatusCode::ACCEPTED)
        }
        Ok(Accepted::Replayed(row)) => {
            let mut response = create_response(&state, row, StatusCode::OK);
            response.headers_mut().insert(
                "Idempotent-Replay",
                axum::http::HeaderValue::from_static("true"),
            );
            response
        }
        Err(error) => {
            state.record_metric(MetricOutcome::Refused, create_metric_error(error));
            project_create_error(error, &state.correlation_id()).into_response()
        }
    }
}

fn create_response(state: &BackupApiState, row: JobRow, status: StatusCode) -> Response {
    let Some(policy) = state.policy() else {
        return project_error(&AppError::BackupDisabled, &state.correlation_id()).into_response();
    };
    let Some(label) = policy.instance_label(&row.instance_id) else {
        return project_error(&AppError::NotFound, &state.correlation_id()).into_response();
    };
    match JobResponse::from_row(&row, label) {
        Ok(job) => (status, Json(json!({ "job": job }))).into_response(),
        Err(_) => project_error(
            &AppError::Unavailable {
                reason_code: "STORE_UNAVAILABLE",
            },
            &state.correlation_id(),
        )
        .into_response(),
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ListQuery {
    limit: Option<u32>,
    cursor: Option<String>,
}

fn encode_cursor(row: &JobRow) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(format!(
        "{}|{}",
        row.created_at.to_rfc3339(),
        row.id
    ))
}

fn decode_cursor(value: &str) -> Option<JobCursor> {
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(value)
        .ok()?;
    let text = std::str::from_utf8(&bytes).ok()?;
    let (created_at, id) = text.split_once('|')?;
    Some(JobCursor {
        created_at: DateTime::parse_from_rfc3339(created_at)
            .ok()?
            .with_timezone(&Utc),
        id: Uuid::parse_str(id).ok()?,
    })
}

fn validation(reason_code: &'static str, correlation_id: &str) -> Response {
    project_error(&AppError::Validation { reason_code }, correlation_id).into_response()
}

async fn list_jobs(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(_actor): Extension<AuthenticatedActor>,
    query: Result<Query<ListQuery>, axum::extract::rejection::QueryRejection>,
) -> Response {
    if let Some(error) = state.operational_error() {
        return project_error(&error, &state.correlation_id()).into_response();
    }
    let correlation_id = state.correlation_id();
    let Query(query) = match query {
        Ok(query) => query,
        Err(_) => return validation("MALFORMED_REQUEST", &correlation_id),
    };
    let limit = query.limit.unwrap_or(DEFAULT_PAGE_SIZE);
    if limit == 0 || limit > MAX_PAGE_SIZE {
        return validation("MALFORMED_REQUEST", &correlation_id);
    }
    let cursor = match query.cursor.as_deref() {
        Some(value) => match decode_cursor(value) {
            Some(c) => Some(c),
            None => return validation("MALFORMED_REQUEST", &correlation_id),
        },
        None => None,
    };
    let config = state.config_enabled().expect("enabled config checked");
    let mut rows = match state
        .store
        .list(config.instance_id(), limit + 1, cursor)
        .await
    {
        Ok(rows) => rows,
        Err(error) => return store_error(error, &correlation_id),
    };
    let has_more = rows.len() > limit as usize;
    if has_more {
        rows.truncate(limit as usize);
    }
    let Some(policy) = state.policy() else {
        return project_error(&AppError::BackupDisabled, &correlation_id).into_response();
    };
    let mut projected = Vec::with_capacity(rows.len());
    let mut last_row = None;
    for row in rows {
        if policy.authorize_resource(&row.instance_id).is_err() {
            continue;
        }
        let Some(label) = policy.instance_label(&row.instance_id) else {
            continue;
        };
        last_row = Some(row.clone());
        match JobResponse::from_row(&row, label) {
            Ok(job) => projected.push(job),
            Err(_) => {
                return project_error(
                    &AppError::Unavailable {
                        reason_code: "STORE_UNAVAILABLE",
                    },
                    &correlation_id,
                )
                .into_response()
            }
        }
    }
    let next_cursor = if has_more {
        last_row.as_ref().map(encode_cursor)
    } else {
        None
    };
    (
        StatusCode::OK,
        Json(json!({ "jobs": projected, "next_cursor": next_cursor })),
    )
        .into_response()
}

async fn get_job(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(job_id): Path<String>,
) -> Response {
    if let Some(error) = state.operational_error() {
        return project_error(&error, &state.correlation_id()).into_response();
    }
    let correlation_id = state.correlation_id();
    let id = match Uuid::parse_str(&job_id) {
        Ok(id) => id,
        Err(_) => return project_error(&AppError::NotFound, &correlation_id).into_response(),
    };
    match state.store.get(id).await {
        Ok(Some(row)) => {
            let Some(policy) = state.policy() else {
                return project_error(&AppError::BackupDisabled, &correlation_id).into_response();
            };
            if policy.authorize_resource(&row.instance_id).is_err() {
                return project_error(&AppError::NotFound, &correlation_id).into_response();
            }
            let label = policy
                .instance_label(&row.instance_id)
                .expect("authorized instance");
            match JobResponse::from_row(&row, label) {
                Ok(job) => (StatusCode::OK, Json(json!({ "job": job }))).into_response(),
                Err(_) => project_error(
                    &AppError::Unavailable {
                        reason_code: "STORE_UNAVAILABLE",
                    },
                    &correlation_id,
                )
                .into_response(),
            }
        }
        Ok(None) => project_error(&AppError::NotFound, &correlation_id).into_response(),
        Err(error) => store_error(error, &correlation_id),
    }
}

pub(crate) fn store_error(error: StoreError, correlation_id: &str) -> Response {
    let app = match error {
        StoreError::NotFound => AppError::NotFound,
        StoreError::AuditUnavailable => AppError::Unavailable {
            reason_code: "AUDIT_UNAVAILABLE",
        },
        _ => AppError::StoreUnavailable,
    };
    project_error(&app, correlation_id).into_response()
}

async fn status(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(_actor): Extension<AuthenticatedActor>,
) -> Response {
    let mut response: StatusResponse =
        project_status(&state.config, state.runtime.as_deref(), state.cleanup);
    if let Some(config) = state.config_enabled() {
        if state.db_healthy {
            match durable_cleanup_status(&state.conn, config.instance_id()).await {
                Ok(cleanup) => {
                    response.escalation.visible |=
                        cleanup.exhausted > 0 || cleanup.ownership_incidents > 0;
                    response.cleanup = Some(cleanup);
                }
                Err(_) => {
                    response.state = "degraded".to_owned();
                    response.db_healthy = false;
                    response.admissions_open = false;
                    response.reconciliation = None;
                    response.cleanup = None;
                    response.escalation.visible = true;
                }
            }
        }
    }
    (StatusCode::OK, Json(response)).into_response()
}

async fn durable_cleanup_status(
    conn: &sea_orm::DatabaseConnection,
    instance_id: &str,
) -> Result<CleanupStatus, sea_orm::DbErr> {
    let sql = Statement::from_sql_and_values(
        DbBackend::Postgres,
        "SELECT \
            (SELECT COUNT(*) FROM backup_allocations a WHERE a.state IN ('release_pending','orphan_discovered','deleting') AND (a.job_id IS NULL OR EXISTS (SELECT 1 FROM backup_jobs j WHERE j.id = a.job_id AND j.instance_id = $1)))::bigint AS pending, \
            (SELECT COUNT(*) FROM backup_allocations a WHERE a.state = 'cleanup_failed' AND (a.job_id IS NULL OR EXISTS (SELECT 1 FROM backup_jobs j WHERE j.id = a.job_id AND j.instance_id = $1)))::bigint AS exhausted, \
            (SELECT COUNT(*) FROM backup_allocations a WHERE a.state = 'orphan_discovered' AND (a.job_id IS NULL OR EXISTS (SELECT 1 FROM backup_jobs j WHERE j.id = a.job_id AND j.instance_id = $1)))::bigint AS orphan_discovered, \
            (SELECT COUNT(*) FROM backup_audit WHERE event = 'ownership.ambiguous' AND instance_id = $1)::bigint AS ownership_incidents",
        vec![instance_id.to_owned().into()],
    );
    let row = conn
        .query_one(sql)
        .await?
        .expect("aggregate query returns one row");
    Ok(CleanupStatus {
        pending: nonnegative_count(row.try_get_by("pending")?),
        exhausted: nonnegative_count(row.try_get_by("exhausted")?),
        orphan_discovered: nonnegative_count(row.try_get_by("orphan_discovered")?),
        ownership_incidents: nonnegative_count(row.try_get_by("ownership_incidents")?),
    })
}

pub async fn json_rejection(
    correlation_id: &str,
    error: axum::extract::rejection::JsonRejection,
) -> Response {
    let app = match error.status() {
        StatusCode::PAYLOAD_TOO_LARGE => AppError::PayloadTooLarge,
        StatusCode::UNSUPPORTED_MEDIA_TYPE => AppError::UnsupportedMediaType,
        _ => AppError::Validation {
            reason_code: "MALFORMED_REQUEST",
        },
    };
    project_error(&app, correlation_id).into_response()
}
