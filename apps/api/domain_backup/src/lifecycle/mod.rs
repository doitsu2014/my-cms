//! Job lifecycle: the worker, the synthetic fixture capture adapter, the terminal rules
//! and the deadline (WU-05); reconciler (WU-17), cleaner (WU-18), sweeper and runtime
//! (WU-19). The lifecycle tests live in `infra::tests` because they need PostgreSQL.

pub mod cleaner;
pub mod fixture_capture;
pub mod reconciler;
pub mod runtime;
pub mod sweeper;
pub mod worker;

#[cfg(test)]
pub(crate) mod test_support;
