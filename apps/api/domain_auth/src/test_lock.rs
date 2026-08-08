//! Test-only helpers shared across `domain_auth` modules.
//!
//! Exposes a single global `Mutex` (`ENV_LOCK`) that env-mutating tests must
//! acquire before touching `std::env::set_var` / `std::env::remove_var`,
//! and a `with_env_var` helper that snapshots the previous value of an env
//! var, runs a closure, and restores the previous value. This serializes
//! env-mutating tests so the default multi-threaded `cargo test` run does
//! not race the global process env.

#![allow(dead_code)]

use std::sync::Mutex;

/// Snapshot the current value of `var`, set or remove it per `value`, run
/// the closure `f`, then restore the previous value. Used by env-var
/// manipulation tests across `domain_auth`.
pub fn with_env_var<F, R>(var: &str, value: Option<&str>, f: F) -> R
where
    F: FnOnce() -> R,
{
    let previous = std::env::var(var).ok();
    match value {
        Some(v) => std::env::set_var(var, v),
        None => std::env::remove_var(var),
    }
    let result = f();
    match previous {
        Some(v) => std::env::set_var(var, v),
        None => std::env::remove_var(var),
    }
    result
}

/// Global lock for env-var manipulation in tests.
pub static ENV_LOCK: Mutex<()> = Mutex::new(());
