//! Observability hooks for the user domain.
//!
//! The user domain inherits its tracing pipeline from the top-level
//! `domain_auth::factory::auth_layer_from_env` factory; this stub mirrors
//! the shape used by `domain_auth::observability` and reserves the
//! namespace for any future `init_tracing()` helper needed by
//! user-specific standalone binaries.
