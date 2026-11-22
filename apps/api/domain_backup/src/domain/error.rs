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
}

/// Internal application error; mapped to a safe body by `response::project_error`.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("backup is disabled")]
    BackupDisabled,
    #[error("job not found")]
    JobNotFound,
    #[error("store unavailable")]
    StoreUnavailable,
}
