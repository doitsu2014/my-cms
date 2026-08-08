//! `api` — HTTP adapter layer for the media domain.
//!
//! All adapters in this module are thin: they extract Axum state, perform
//! request-level validation (bucket names, content types, multipart), and
//! delegate to the application-layer handlers in `crate::handlers::*`.
//! Business logic, storage interaction, and cache policy live there.

pub mod bucket;
pub mod media;
pub mod routes;
pub mod state;

pub use crate::service::DomainMediaService;
pub use state::MediaApiState;
