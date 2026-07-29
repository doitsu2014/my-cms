//! Domain-owned infrastructure for the post service.
//!
//! Each module here is either the canonical owner of an infrastructure
//! type (e.g. `error`, `auth`, `response`, `layers`) or a re-export from
//! the legacy `application_core` / `src/{common,presentation_models}` trees
//! during the transition. The transition plan is documented in
//! `openspec/changes/refactor-api-into-pluggable-domain-libraries/tasks.md`.

pub mod auth;
pub mod ai;
pub mod env;
pub mod error;
pub mod extensions;
pub mod graphql;
pub mod layers;
pub mod postgres;
pub mod response;
pub mod storage;
pub mod vector_store;