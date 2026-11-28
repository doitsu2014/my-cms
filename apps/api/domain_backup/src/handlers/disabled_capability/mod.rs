//! IF-06..IF-08: real upload, preflight and restore are refused (F-12; TS-09).
//!
//! The handlers take no body extractor, so the request body is never polled whatever
//! `Content-Length` declares. They sit behind the WU-08 authorization middleware (role, Host,
//! Origin), then answer 403 `CAPABILITY_DISABLED` regardless of `BACKUP_ENABLED` and readiness
//! (IFC 2.1 precedence). They touch no store, ledger or storage, and enqueue nothing. The only
//! side effect is the attempted `capability.denied` audit record (IFC 6.3: attempt-only, the
//! denial is returned even when the write fails).

use std::sync::Arc;
use std::time::Duration;

use axum::extract::Extension;
use axum::response::{IntoResponse, Response};
use domain_interface::AuthenticatedActor;
use uuid::Uuid;

use crate::domain::clock::{Clock, SystemClock};
use crate::domain::error::CoarseErrorCode;
use crate::domain::metrics::{MetricErrorCode, MetricOutcome};
use crate::domain::response::SafeErrorBody;
use crate::handlers::http::BackupApiState;
use crate::ports::audit::{AuditEvent, AuditEventKind, AuditOutcome};

#[cfg(test)]
pub(crate) mod test_support;
#[cfg(test)]
mod tests;

/// Stable reason code of IF-06..IF-08 (IFC 10.1).
pub const CAPABILITY_DISABLED: &str = "CAPABILITY_DISABLED";

const CAPABILITY_MESSAGE: &str = "This backup capability is not enabled.";

/// A denial audit write is attempted for at most this long; the denial never waits longer.
const AUDIT_ATTEMPT_LIMIT: Duration = Duration::from_secs(2);

/// The three real capabilities U1 refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Capability {
    Upload,
    Preflight,
    Restore,
}

impl Capability {
    /// Audit `operation` value.
    fn operation(self) -> &'static str {
        match self {
            Self::Upload => "upload",
            Self::Preflight => "preflight",
            Self::Restore => "restore",
        }
    }
}

/// Attempt one denial audit record. Failure, rejection and timeout are all ignored: the
/// caller returns its denial either way. Skipped when no instance is configured (backup is
/// disabled), because `backup_audit.instance_id` is NOT NULL and there is nothing to name.
pub(crate) async fn record_denial(
    state: &BackupApiState,
    actor: &AuthenticatedActor,
    event: AuditEventKind,
    operation: &str,
    job_id: Option<Uuid>,
    reason_code: &str,
    correlation_id: &str,
) {
    let Some(config) = state.config_enabled() else {
        return;
    };
    let record = AuditEvent {
        occurred_at: SystemClock.now(),
        actor_sub: Some(actor.user_id.clone()),
        instance_id: config.instance_id().to_owned(),
        job_id,
        allocation_id: None,
        operation: operation.to_owned(),
        event,
        outcome: AuditOutcome::Denied,
        reason_code: Some(reason_code.to_owned()),
        digest: None,
        correlation_id: correlation_id.to_owned(),
    };
    let _ = tokio::time::timeout(AUDIT_ATTEMPT_LIMIT, state.audit.record(None, record)).await;
}

async fn reject(
    state: Arc<BackupApiState>,
    actor: AuthenticatedActor,
    capability: Capability,
) -> Response {
    let correlation_id = state.correlation_id();
    state.record_metric(MetricOutcome::Denied, MetricErrorCode::Forbidden);
    record_denial(
        &state,
        &actor,
        AuditEventKind::CapabilityDenied,
        capability.operation(),
        None,
        CAPABILITY_DISABLED,
        &correlation_id,
    )
    .await;
    SafeErrorBody {
        error_code: CoarseErrorCode::Forbidden,
        errors: vec![CAPABILITY_MESSAGE.to_owned()],
        reason_code: CAPABILITY_DISABLED.to_owned(),
        correlation_id,
    }
    .into_response()
}

/// IF-06 `POST /backup/packages`.
pub(crate) async fn upload(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(actor): Extension<AuthenticatedActor>,
) -> Response {
    reject(state, actor, Capability::Upload).await
}

/// IF-07 `POST /backup/preflight`.
pub(crate) async fn preflight(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(actor): Extension<AuthenticatedActor>,
) -> Response {
    reject(state, actor, Capability::Preflight).await
}

/// IF-08 `POST /backup/restore`.
pub(crate) async fn restore(
    Extension(state): Extension<Arc<BackupApiState>>,
    Extension(actor): Extension<AuthenticatedActor>,
) -> Response {
    reject(state, actor, Capability::Restore).await
}
