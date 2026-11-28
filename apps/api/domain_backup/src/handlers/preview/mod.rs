//! IF-09 and IF-10: labelled synthetic previews (F-13, F-14; TS-43, TS-53, TS-64).
//!
//! Stateless on purpose: the handlers read only the configured instance and the compiled-in
//! fixture catalog (`domain::fixtures`). They hold no mutation port, enqueue nothing, write no
//! audit record and touch no storage. A preview never accepts an upload or a path: the only
//! way to name a package is a trusted fixture id, and the target fields of the response come
//! from the configured instance, never from the request.
//!
//! Order after the WU-08 authorization middleware (IFC 2.1): 503 `BACKUP_DISABLED` /
//! `STORE_UNAVAILABLE` / `BACKUP_NOT_READY`, 415, 413, then 400 `MALFORMED_REQUEST`.

use std::sync::Arc;

use axum::body::{to_bytes, Body, Bytes};
use axum::extract::Extension;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain_interface::AuthenticatedActor;
use serde::Deserialize;
use serde_json::json;

use crate::domain::error::AppError;
use crate::domain::fixtures::lookup;
use crate::domain::instance::BackupInstance;
use crate::domain::metrics::{MetricErrorCode, MetricOutcome};
use crate::domain::response::project_error;
use crate::handlers::create_job::request::is_json_content_type;
use crate::handlers::create_job::MAX_BODY_BYTES;
use crate::handlers::http::BackupApiState;

#[cfg(test)]
mod tests;

pub const SYNTHETIC_IMPACT_SUMMARY: &str = "Synthetic preview only. No file was uploaded or read.";
pub const SYNTHETIC_CONFIRMATION_MESSAGE: &str =
    "Synthetic preview only. Nothing was recorded and no restore was performed.";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PreflightRequest {
    fixture_id: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Decision {
    Confirm,
    Cancel,
}

impl Decision {
    fn as_str(self) -> &'static str {
        match self {
            Self::Confirm => "confirm",
            Self::Cancel => "cancel",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfirmationRequest {
    fixture_id: String,
    decision: Decision,
}

fn refuse(state: &BackupApiState, error: AppError, metric: MetricErrorCode) -> Response {
    state.record_metric(MetricOutcome::Refused, metric);
    project_error(&error, &state.correlation_id()).into_response()
}

fn malformed(state: &BackupApiState) -> Response {
    refuse(
        state,
        AppError::Validation {
            reason_code: "MALFORMED_REQUEST",
        },
        MetricErrorCode::ValidationError,
    )
}

/// Readiness, media type and the 16 KiB body cap, in the IFC 2.1 order. Returns the body
/// bytes, or the refusal. The body is polled only after all earlier checks pass.
async fn admitted_body(
    state: &BackupApiState,
    headers: &HeaderMap,
    body: Body,
) -> Result<Bytes, Box<Response>> {
    if let Some(error) = state.operational_error() {
        state.record_refusal(&error);
        return Err(Box::new(
            project_error(&error, &state.correlation_id()).into_response(),
        ));
    }
    let content_type = headers
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok());
    if !is_json_content_type(content_type) {
        return Err(Box::new(refuse(
            state,
            AppError::UnsupportedMediaType,
            MetricErrorCode::UnsupportedMediaType,
        )));
    }
    let declared_too_large = headers
        .get(header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|length| length > MAX_BODY_BYTES);
    if declared_too_large {
        return Err(Box::new(refuse(
            state,
            AppError::PayloadTooLarge,
            MetricErrorCode::PayloadTooLarge,
        )));
    }
    to_bytes(body, MAX_BODY_BYTES).await.map_err(|_| {
        Box::new(refuse(
            state,
            AppError::PayloadTooLarge,
            MetricErrorCode::PayloadTooLarge,
        ))
    })
}

/// The configured instance, whose id, label and environment name the target (TS-64).
fn target(state: &BackupApiState) -> Option<BackupInstance> {
    state
        .registry()
        .map(|registry| registry.configured().clone())
}

/// IF-09 `POST /backup/previews/preflight`.
pub(crate) async fn preview_preflight(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(_actor): Extension<AuthenticatedActor>,
    headers: HeaderMap,
    body: Body,
) -> Response {
    let bytes = match admitted_body(&state, &headers, body).await {
        Ok(bytes) => bytes,
        Err(response) => return *response,
    };
    let Ok(request) = serde_json::from_slice::<PreflightRequest>(&bytes) else {
        return malformed(&state);
    };
    let (Some(fixture), Some(instance)) = (lookup(&request.fixture_id), target(&state)) else {
        return malformed(&state);
    };
    (
        StatusCode::OK,
        Json(json!({
            "synthetic": true,
            "result": fixture.result.as_str(),
            "fixture_label": fixture.label,
            "target_instance_id": instance.id,
            "target_instance_label": instance.label,
            "target_environment": instance.environment,
            "data_scope": fixture.data_scope,
            "impact_summary": SYNTHETIC_IMPACT_SUMMARY,
        })),
    )
        .into_response()
}

/// IF-10 `POST /backup/previews/restore-confirmation`. `confirm` and `cancel` answer the same
/// shape; neither runs a restore, records anything or enqueues anything.
pub(crate) async fn preview_restore_confirmation(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(_actor): Extension<AuthenticatedActor>,
    headers: HeaderMap,
    body: Body,
) -> Response {
    let bytes = match admitted_body(&state, &headers, body).await {
        Ok(bytes) => bytes,
        Err(response) => return *response,
    };
    let Ok(request) = serde_json::from_slice::<ConfirmationRequest>(&bytes) else {
        return malformed(&state);
    };
    let (Some(fixture), Some(instance)) = (lookup(&request.fixture_id), target(&state)) else {
        return malformed(&state);
    };
    (
        StatusCode::OK,
        Json(json!({
            "synthetic": true,
            "executed": false,
            "decision": request.decision.as_str(),
            "fixture_label": fixture.label,
            "target_instance_id": instance.id,
            "target_instance_label": instance.label,
            "target_environment": instance.environment,
            "data_scope": fixture.data_scope,
            "message": SYNTHETIC_CONFIRMATION_MESSAGE,
        })),
    )
        .into_response()
}
