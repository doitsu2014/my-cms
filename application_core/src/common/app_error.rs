use core::fmt;
use std::{error::Error, fmt::Display};

use sea_orm::{DbErr, TransactionError};

#[derive(Debug)]
pub enum AppError {
    Db(DbErr),
    DbTx(TransactionError<DbErr>),
    Validation(String, String),
    Logical(String),
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
            AppError::Unknown => write!(f, "Unknown error"),
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
            AppError::Unknown => None,
        }
    }
}

pub trait DbErrExt {
    fn to_app_error(self) -> AppError;
}

impl DbErrExt for DbErr {
    fn to_app_error(self) -> AppError {
        AppError::Db(self)
    }
}

pub trait TransactionDbErrExt {
    fn to_app_error(self) -> AppError;
}

impl TransactionDbErrExt for TransactionError<DbErr> {
    fn to_app_error(self) -> AppError {
        AppError::DbTx(self)
    }
}

impl TransactionDbErrExt for TransactionError<AppError> {
    fn to_app_error(self) -> AppError {
        match self {
            TransactionError::<AppError>::Connection(err) => AppError::Db(err),
            TransactionError::<AppError>::Transaction(err) => err,
        }
    }
}
