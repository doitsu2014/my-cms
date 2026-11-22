//! Safe error projection. Interfaces-and-contracts section 10.2.

use crate::domain::error::{AppError, CoarseErrorCode};

/// Client-visible error body: fixed safe message, no internals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeErrorBody {
    pub error_code: CoarseErrorCode,
    pub reason_code: String,
    pub message: String,
    pub correlation_id: String,
}

/// Map an internal error to its safe projection.
pub fn project_error(err: &AppError, correlation_id: &str) -> SafeErrorBody {
    let (error_code, reason_code, message) = match err {
        AppError::BackupDisabled => (
            CoarseErrorCode::Unavailable,
            "BACKUP_DISABLED",
            "Backup is disabled.",
        ),
        AppError::JobNotFound => (CoarseErrorCode::NotFound, "JOB_NOT_FOUND", "Job not found."),
        AppError::StoreUnavailable => (
            CoarseErrorCode::Unavailable,
            "STORE_UNAVAILABLE",
            "The service is temporarily unavailable.",
        ),
    };
    SafeErrorBody {
        error_code,
        reason_code: reason_code.to_string(),
        message: message.to_string(),
        correlation_id: correlation_id.to_string(),
    }
}
