//! Safe error projection. Interfaces-and-contracts section 10.2.

use axum::http::StatusCode;
use serde::Serialize;

use crate::domain::error::{AppError, CoarseErrorCode};

/// Client-visible error body: fixed safe message, no internals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SafeErrorBody {
    pub error_code: CoarseErrorCode,
    pub errors: Vec<String>,
    pub reason_code: String,
    pub correlation_id: String,
}

impl CoarseErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ValidationError => "ValidationError",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "NotFound",
            Self::Conflict => "Conflict",
            Self::RequestTimeout => "RequestTimeout",
            Self::HeaderTooLarge => "HeaderTooLarge",
            Self::PayloadTooLarge => "PayloadTooLarge",
            Self::UnsupportedMediaType => "UnsupportedMediaType",
            Self::Unavailable => "Unavailable",
            Self::Unauthorized => "Unauthorized",
        }
    }
}

impl Serialize for CoarseErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl axum::response::IntoResponse for SafeErrorBody {
    fn into_response(self) -> axum::response::Response {
        let status = match self.error_code {
            CoarseErrorCode::ValidationError => StatusCode::BAD_REQUEST,
            CoarseErrorCode::Forbidden => StatusCode::FORBIDDEN,
            CoarseErrorCode::NotFound => StatusCode::NOT_FOUND,
            CoarseErrorCode::Conflict => StatusCode::CONFLICT,
            CoarseErrorCode::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            CoarseErrorCode::HeaderTooLarge => StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
            CoarseErrorCode::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            CoarseErrorCode::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            CoarseErrorCode::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            CoarseErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
        };
        axum::response::IntoResponse::into_response((status, axum::Json(self)))
    }
}

/// Map an internal error to its safe projection.
pub fn project_error(err: &AppError, correlation_id: &str) -> SafeErrorBody {
    let (error_code, reason_code, message) = match err {
        AppError::BackupDisabled => (
            CoarseErrorCode::Unavailable,
            "BACKUP_DISABLED",
            "Backup is disabled.",
        ),
        AppError::BackupNotReady => (
            CoarseErrorCode::Unavailable,
            "BACKUP_NOT_READY",
            "Backup is not ready.",
        ),
        AppError::Forbidden { reason_code } => (
            CoarseErrorCode::Forbidden,
            *reason_code,
            "Request is forbidden.",
        ),
        AppError::Validation { reason_code } => (
            CoarseErrorCode::ValidationError,
            *reason_code,
            "Request is invalid.",
        ),
        AppError::Unavailable { reason_code } => (
            CoarseErrorCode::Unavailable,
            *reason_code,
            "The service is temporarily unavailable.",
        ),
        AppError::Conflict { reason_code } => (
            CoarseErrorCode::Conflict,
            *reason_code,
            "Request conflicts with current state.",
        ),
        AppError::PayloadTooLarge => (
            CoarseErrorCode::PayloadTooLarge,
            "PAYLOAD_TOO_LARGE",
            "Request body is too large.",
        ),
        AppError::UnsupportedMediaType => (
            CoarseErrorCode::UnsupportedMediaType,
            "UNSUPPORTED_MEDIA_TYPE",
            "Content-Type must be application/json.",
        ),
        AppError::JobNotFound | AppError::NotFound => {
            (CoarseErrorCode::NotFound, "JOB_NOT_FOUND", "Job not found.")
        }
        AppError::Unauthorized => (
            CoarseErrorCode::Unauthorized,
            "ROLE_REQUIRED",
            "Administrator role is required.",
        ),
        AppError::StoreUnavailable => (
            CoarseErrorCode::Unavailable,
            "STORE_UNAVAILABLE",
            "The service is temporarily unavailable.",
        ),
    };
    SafeErrorBody {
        error_code,
        errors: vec![message.to_string()],
        reason_code: reason_code.to_string(),
        correlation_id: correlation_id.to_string(),
    }
}

pub fn project_create_error(
    err: crate::handlers::create_job::CreateJobError,
    correlation_id: &str,
) -> SafeErrorBody {
    use crate::handlers::create_job::CreateJobError as E;
    let (code, message) = match err {
        E::UnsupportedMediaType => (
            CoarseErrorCode::UnsupportedMediaType,
            "Content-Type must be application/json.",
        ),
        E::PayloadTooLarge => (
            CoarseErrorCode::PayloadTooLarge,
            "Request body is too large.",
        ),
        E::MalformedRequest => (CoarseErrorCode::ValidationError, "Request body is invalid."),
        E::IdempotencyKeyRequired => (
            CoarseErrorCode::ValidationError,
            "Idempotency-Key header is required.",
        ),
        E::IdempotencyKeyInvalid => (
            CoarseErrorCode::ValidationError,
            "Idempotency-Key header is invalid.",
        ),
        E::InstanceNotAllowed => (CoarseErrorCode::ValidationError, "Instance is not allowed."),
        E::IdempotencyKeyReused => (
            CoarseErrorCode::Conflict,
            "Idempotency key was used for a different request.",
        ),
        E::ConcurrencyLimit => (
            CoarseErrorCode::Conflict,
            "A job is already queued or running for this instance.",
        ),
        E::AuditUnavailable => (
            CoarseErrorCode::Unavailable,
            "Audit service is temporarily unavailable.",
        ),
        E::StoreUnavailable => (
            CoarseErrorCode::Unavailable,
            "The service is temporarily unavailable.",
        ),
    };
    SafeErrorBody {
        error_code: code,
        errors: vec![message.to_owned()],
        reason_code: err.reason_code().to_owned(),
        correlation_id: correlation_id.to_owned(),
    }
}
