//! Job lifecycle (WU-05): the worker, the synthetic fixture capture adapter, the
//! terminal rules and the deadline.

pub mod fixture_capture;
pub mod worker;

#[cfg(test)]
pub(crate) mod test_support;
