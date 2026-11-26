//! Ports (IFC section 9). WU-03 delivers `JobStore` and `AuditSink`; WU-04 delivers
//! `AllocationStore`, `FixtureStorage` and `NamespaceLock`.

pub mod allocation_store;
pub mod audit;
pub mod capture;
pub mod fixture_storage;
pub mod job_store;
pub mod namespace_lock;
pub mod policy;
pub mod retention_store;
