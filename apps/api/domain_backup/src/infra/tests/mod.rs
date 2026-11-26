//! WU-03 and WU-04 red tests. WU-03: `DbJobStore` (TS-14a, CAS and claim race tests)
//! and `DbAuditSink` (TS-69). WU-04: `DbAllocationStore` (budgets, register-before-write,
//! ledger audit), `FsFixtureStorage` (TS-19, TS-31, TS-56) and `FsNamespaceLock`
//! (TS-32). DB-backed tests need Docker and are opt-in (`-- --ignored`); the
//! file-system tests run by default.

mod alloc_support;
mod allocation_store;
mod audit_sink;
mod fs_fixture_storage;
mod fs_namespace_lock;
mod fs_support;
mod job_store;
mod reconciliation;
mod retention;
mod support;
mod worker;
