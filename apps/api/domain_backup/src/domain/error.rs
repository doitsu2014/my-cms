//! Error taxonomy. Interfaces-and-contracts section 10.1.

use thiserror::Error;

/// Coarse error code, serialized as name strings (not the numeric posts/media scheme).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoarseErrorCode {
    ValidationError,
    Forbidden,
    NotFound,
    Conflict,
    RequestTimeout,
    HeaderTooLarge,
    PayloadTooLarge,
    UnsupportedMediaType,
    Unavailable,
    Unauthorized,
}

/// Internal application error; mapped to a safe body by `response::project_error`.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("backup is disabled")]
    BackupDisabled,
    #[error("backup capability is not ready")]
    BackupNotReady,
    #[error("backup operation is forbidden")]
    Forbidden { reason_code: &'static str },
    #[error("request validation failed")]
    Validation { reason_code: &'static str },
    #[error("backup service is unavailable")]
    Unavailable { reason_code: &'static str },
    #[error("backup request conflicts with current state")]
    Conflict { reason_code: &'static str },
    #[error("backup request is too large")]
    PayloadTooLarge,
    #[error("backup request media type is unsupported")]
    UnsupportedMediaType,
    #[error("backup job is not found")]
    NotFound,
    #[error("administrator role is required")]
    Unauthorized,
    #[error("job not found")]
    JobNotFound,
    #[error("store unavailable")]
    StoreUnavailable,
}
