use sea_orm::{DbErr, TransactionError};

#[derive(Debug)]
pub enum AppError {
    Db(DbErr),
    DbTx(TransactionError<DbErr>),
    Validation(String, String),
    Logical(String),
    Unknown,
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
