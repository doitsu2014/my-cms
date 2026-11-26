//! Job lifecycle (WU-05): the worker, the synthetic fixture capture adapter, the
//! terminal rules and the deadline.

pub mod cleaner;
pub mod fixture_capture;
pub mod reconciler;
pub mod runtime;
pub mod sweeper;
pub mod worker;

#[cfg(test)]
mod lifecycle_tests;

#[cfg(test)]
pub(crate) mod test_support;
