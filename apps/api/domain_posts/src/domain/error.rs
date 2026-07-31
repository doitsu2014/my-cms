//! `AppError` — the single error type returned by every handler in the
//! post domain.
//!
//! Moved from `application_core::common::app_error.rs` per design Decision 2.
//! The legacy `crate::domain::error::AppError` is now a
//! re-export of `domain_posts::domain::error::AppError` so existing call
//! sites keep working during the transition.

use core::fmt;
use std::{error::Error, fmt::Display};

use sea_orm::{DbErr, TransactionError};

#[derive(Debug)]
pub enum AppError {
    Db(DbErr),
    DbTx(TransactionError<DbErr>),
    StorageError(String),
    Validation(String, String),
    Logical(String),
    Conflict(String),
    ConcurrencyOptimistic(String),
    NotFound,
    Unknown,
    OpenAIError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Db(err) => write!(f, "Database error: {}", err),
            AppError::DbTx(err) => write!(f, "Database transaction error: {}", err),
            AppError::Validation(field, message) => {
                write!(f, "Validation error: {}: {}", field, message)
            }
            AppError::Logical(message) => write!(f, "Logical error: {}", message),
            AppError::Conflict(message) => write!(f, "Conflict error: {}", message),
            AppError::ConcurrencyOptimistic(message) => {
                write!(f, "ConcurrencyOptimistic error: {}", message)
            }
            AppError::NotFound => write!(f, "Not found"),
            AppError::Unknown => write!(f, "Unknown error"),
            AppError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            AppError::OpenAIError(err) => write!(f, "OpenAI error: {}", err),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Db(err) => Some(err),
            AppError::DbTx(err) => Some(err),
            AppError::Validation(_, _) => None,
            AppError::Logical(_) => None,
            AppError::Conflict(_) => None,
            AppError::ConcurrencyOptimistic(_) => None,
            AppError::NotFound => None,
            AppError::Unknown => None,
            AppError::StorageError(_) => None,
            AppError::OpenAIError(_) => None,
        }
    }
}

impl From<AppError> for DbErr {
    fn from(err: AppError) -> DbErr {
        sea_orm::DbErr::Custom(err.to_string())
    }
}

impl From<TransactionError<DbErr>> for AppError {
    fn from(err: TransactionError<DbErr>) -> AppError {
        AppError::DbTx(err)
    }
}

impl From<TransactionError<AppError>> for AppError {
    fn from(err: TransactionError<AppError>) -> AppError {
        match err {
            TransactionError::<AppError>::Connection(err) => AppError::Db(err),
            TransactionError::<AppError>::Transaction(err) => err,
        }
    }
}

/// Transitional bridge from the legacy `application_core::common::app_error::AppError`
/// to the canonical `domain_posts::domain::error::AppError`. Both types share the
/// same variant structure; the bridge will be removed once the legacy crate
/// becomes a re-export shim.
impl From<application_core::common::app_error::AppError> for AppError {
    fn from(legacy: application_core::common::app_error::AppError) -> AppError {
        match legacy {
            application_core::common::app_error::AppError::Db(err) => AppError::Db(err),
            application_core::common::app_error::AppError::DbTx(err) => AppError::DbTx(err),
            application_core::common::app_error::AppError::StorageError(err) => {
                AppError::StorageError(err)
            }
            application_core::common::app_error::AppError::Validation(f, m) => {
                AppError::Validation(f, m)
            }
            application_core::common::app_error::AppError::Logical(m) => AppError::Logical(m),
            application_core::common::app_error::AppError::Conflict(m) => AppError::Conflict(m),
            application_core::common::app_error::AppError::ConcurrencyOptimistic(m) => {
                AppError::ConcurrencyOptimistic(m)
            }
            application_core::common::app_error::AppError::NotFound => AppError::NotFound,
            application_core::common::app_error::AppError::Unknown => AppError::Unknown,
            application_core::common::app_error::AppError::OpenAIError(err) => {
                AppError::OpenAIError(err)
            }
        }
    }
}
