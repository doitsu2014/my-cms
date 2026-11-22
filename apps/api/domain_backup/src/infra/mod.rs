//! Infrastructure adapters over PostgreSQL.

pub mod db_audit_sink;
pub mod db_job_store;
pub(crate) mod pg_error;

#[cfg(test)]
mod tests;
