//! WU-03 red tests: `DbJobStore` (TS-14a, CAS and claim race tests) and
//! `DbAuditSink` (TS-69). DB-backed tests need Docker and are opt-in
//! (`-- --ignored`).

mod audit_sink;
mod job_store;
mod support;
