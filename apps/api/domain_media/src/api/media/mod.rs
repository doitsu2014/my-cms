//! Media API adapters — HTTP boundary for the media domain.
//!
//! All adapters in this module are thin: they extract Axum state, perform
//! request-level validation (bucket names, content types, multipart), and
//! delegate to the application-layer handlers in `crate::handlers::*`.
//! Business logic, storage interaction, and cache policy live there.

pub mod create;
pub mod delete;
pub mod list;
pub mod read;
