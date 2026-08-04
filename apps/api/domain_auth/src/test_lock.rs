//! Test-only helpers shared across `domain_auth` modules.
//!
//! Currently exposes a single global `Mutex` that env-mutating tests must
//! acquire before touching `std::env::set_var` / `std::env::remove_var`.
//! This serializes those tests so the default multi-threaded `cargo test`
//! run does not race the global process env.

#![allow(dead_code)]

use std::sync::Mutex;

/// Global lock for env-var manipulation in tests.
pub static ENV_LOCK: Mutex<()> = Mutex::new(());
