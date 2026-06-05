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
            AppError::ConcurrencyOptimistic(_) => None,
            AppError::NotFound => None,
            AppError::Unknown => None,
            AppError::StorageError(_) => None,
            AppError::OpenAIError(_) => None,
        }
    }
}

impl Into<AppError> for DbErr {
    fn into(self) -> AppError {
        AppError::Db(self)
    }
}

impl Into<AppError> for TransactionError<DbErr> {
    fn into(self) -> AppError {
        AppError::DbTx(self)
    }
}

impl Into<AppError> for TransactionError<AppError> {
    fn into(self) -> AppError {
        match self {
            TransactionError::<AppError>::Connection(err) => AppError::Db(err),
            TransactionError::<AppError>::Transaction(err) => err,
        }
    }
}
