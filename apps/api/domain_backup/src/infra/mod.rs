//! Infrastructure adapters over PostgreSQL and the file system.

pub mod db_allocation_store;
pub mod db_audit_sink;
pub mod db_job_store;
pub mod db_retention_store;
pub mod fs_fixture_storage;
pub mod fs_namespace_lock;
pub(crate) mod pg_error;

#[cfg(test)]
mod tests;
