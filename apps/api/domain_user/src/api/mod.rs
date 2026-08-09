//! `api` — HTTP adapter layer for the user domain.
//!
//! Mirrors `domain_media::api` shape. The `routes` aggregator returns the
//! `Vec<RouteRegistration>` consumed by the gateway composition root, and the
//! `state` type wraps the per-process `SupabaseAdminClient` shared with every
//! per-request handler.

pub mod routes;
pub mod state;

pub use state::UserApiState;