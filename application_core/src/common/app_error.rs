use core::fmt;
use std::{error::Error, fmt::Display};

use s3::error::S3Error;
use sea_orm::{DbErr, TransactionError};

#[derive(Debug)]
pub enum AppError {
    Db(DbErr),
    DbTx(TransactionError<DbErr>),
    S3Error(S3Error),
    Validation(String, String),
    Logical(String),
    ConcurrencyOptimistic(String),
    NotFound,
    Unknown,
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
            AppError::S3Error(err) => write!(f, "S3 error: {}", err),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Db(err) => Some(err),
            AppError::DbTx(err) => Some(err),
            AppError::S3Error(err) => Some(err),
            AppError::Validation(_, _) => None,
            AppError::Logical(_) => None,
            AppError::ConcurrencyOptimistic(_) => None,
            AppError::NotFound => None,
            AppError::Unknown => None,
        }
    }
}

impl Into<AppError> for s3::error::S3Error {
    fn into(self) -> AppError {
        AppError::S3Error(self)
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
