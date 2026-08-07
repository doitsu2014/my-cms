//! Observability hooks for the user domain.
//!
//! The user domain inherits its tracing pipeline from the top-level
//! `legacy_bootstrap.rs` of `domain_auth`/`cms`; this stub mirrors the
//! shape used by `domain_auth::observability` and reserves the namespace
//! for any future `init_tracing()` helper needed by user-specific
//! standalone binaries.
