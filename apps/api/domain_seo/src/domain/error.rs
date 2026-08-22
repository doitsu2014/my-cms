use core::fmt;
use std::{error::Error, fmt::Display};

use sea_orm::{DbErr, TransactionError};

#[derive(Debug)]
pub enum AppError {
    Db(DbErr),
    DbTx(TransactionError<DbErr>),
    Validation(String, String),
    Conflict(String),
    ConcurrencyOptimistic(String),
    NotFound,
    Unknown,
}

impl Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Db(error) => write!(f, "Database error: {error}"),
            Self::DbTx(error) => write!(f, "Database transaction error: {error}"),
            Self::Validation(field, message) => write!(f, "Validation error: {field}: {message}"),
            Self::Conflict(message) => write!(f, "Conflict error: {message}"),
            Self::ConcurrencyOptimistic(message) => {
                write!(f, "ConcurrencyOptimistic error: {message}")
            }
            Self::NotFound => write!(f, "Not found"),
            Self::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl Error for AppError {}
impl From<DbErr> for AppError {
    fn from(error: DbErr) -> Self {
        Self::Db(error)
    }
}
impl From<TransactionError<DbErr>> for AppError {
    fn from(error: TransactionError<DbErr>) -> Self {
        Self::DbTx(error)
    }
}
