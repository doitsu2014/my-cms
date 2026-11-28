//! Classification of driver errors into outage, database rejection and other.
//! Carries no SQL text or driver message: only SQLSTATE and constraint name.

use sea_orm::{sqlx, DbErr, RuntimeErr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PgFailure {
    /// Connection loss, pool exhaustion, statement timeout or shutdown: the store is
    /// unavailable.
    Outage,
    /// The server rejected the statement on a healthy connection.
    Database {
        code: String,
        constraint: Option<String>,
    },
    /// Anything else (type, decode or custom errors).
    Other,
}

/// SQLSTATE classes that mean a statement rejection on a healthy connection: 22 data
/// exception, 23 integrity, 42 syntax or access rule (schema rejection), P0 PL/pgSQL
/// RAISE (triggers), and 25P02 in-failed-transaction.
fn is_statement_rejection(code: &str) -> bool {
    code.starts_with("22")
        || code.starts_with("23")
        || code.starts_with("42")
        || code.starts_with("P0")
        || code == "25P02"
}

fn from_sqlx(e: &sqlx::Error) -> PgFailure {
    match e {
        sqlx::Error::Database(db) => {
            let code = db.code().map(|c| c.into_owned()).unwrap_or_default();
            if code.is_empty() {
                PgFailure::Other
            } else if is_statement_rejection(&code) {
                PgFailure::Database {
                    code,
                    constraint: db.constraint().map(str::to_owned),
                }
            } else {
                // Allow-list: anything that is not clearly a rejection of the statement on a
                // healthy connection (57014 timeout, 57P*, 08*, 53*, 58*, XX*, 25006, 25P03,
                // 55P03, 40*, ...) fails closed as an outage.
                PgFailure::Outage
            }
        }
        sqlx::Error::Io(_)
        | sqlx::Error::Tls(_)
        | sqlx::Error::PoolTimedOut
        | sqlx::Error::PoolClosed
        | sqlx::Error::WorkerCrashed
        | sqlx::Error::Protocol(_) => PgFailure::Outage,
        _ => PgFailure::Other,
    }
}

pub(crate) fn classify(err: &DbErr) -> PgFailure {
    match err {
        DbErr::ConnectionAcquire(_) | DbErr::Conn(_) => PgFailure::Outage,
        DbErr::Exec(RuntimeErr::SqlxError(e)) | DbErr::Query(RuntimeErr::SqlxError(e)) => {
            from_sqlx(e)
        }
        _ => PgFailure::Other,
    }
}
