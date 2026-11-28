//! IF-05: download is always denied (F-11; TS-09).
//!
//! Order after the WU-08 authorization middleware (IFC 2.1): 503 `BACKUP_DISABLED` /
//! `STORE_UNAVAILABLE` / `BACKUP_NOT_READY`, then resource authorization (a malformed, unknown
//! or foreign job id is the same 404 `JOB_NOT_FOUND`), then 403 `DOWNLOAD_DENIED` for every
//! status. No file is read, no URL or redirect is produced, and the only side effect is the
//! attempted `download.denied` audit record.

use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::response::{IntoResponse, Response};
use domain_interface::AuthenticatedActor;
use uuid::Uuid;

use crate::domain::error::{AppError, CoarseErrorCode};
use crate::domain::metrics::{MetricErrorCode, MetricOutcome};
use crate::domain::response::{project_error, SafeErrorBody};
use crate::handlers::disabled_capability::record_denial;
use crate::handlers::http::{store_error, BackupApiState};
use crate::ports::audit::AuditEventKind;

#[cfg(test)]
mod tests;

/// Stable reason code of IF-05 (IFC 10.1).
pub const DOWNLOAD_DENIED: &str = "DOWNLOAD_DENIED";

const DOWNLOAD_MESSAGE: &str = "Download is not permitted for backup jobs.";

pub(crate) async fn deny_download(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(actor): Extension<AuthenticatedActor>,
    Path(job_id): Path<String>,
) -> Response {
    if let Some(error) = state.operational_error() {
        state.record_refusal(&error);
        return project_error(&error, &state.correlation_id()).into_response();
    }
    let correlation_id = state.correlation_id();
    let not_found = |state: &BackupApiState| {
        state.record_metric(MetricOutcome::Refused, MetricErrorCode::NotFound);
        project_error(&AppError::NotFound, &correlation_id).into_response()
    };
    let Ok(id) = Uuid::parse_str(&job_id) else {
        return not_found(&state);
    };
    let row = match state.store.get(id).await {
        Ok(Some(row)) => row,
        Ok(None) => return not_found(&state),
        Err(error) => return store_error(error, &correlation_id),
    };
    let authorized = state
        .policy()
        .is_some_and(|policy| policy.authorize_resource(&row.instance_id).is_ok());
    if !authorized {
        return not_found(&state);
    }
    state.record_metric(MetricOutcome::Denied, MetricErrorCode::Forbidden);
    record_denial(
        &state,
        &actor,
        AuditEventKind::DownloadDenied,
        "download",
        Some(row.id),
        DOWNLOAD_DENIED,
        &correlation_id,
    )
    .await;
    SafeErrorBody {
        error_code: CoarseErrorCode::Forbidden,
        errors: vec![DOWNLOAD_MESSAGE.to_owned()],
        reason_code: DOWNLOAD_DENIED.to_owned(),
        correlation_id,
    }
    .into_response()
}
