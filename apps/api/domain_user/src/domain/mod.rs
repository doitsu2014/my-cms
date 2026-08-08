//! Domain-owned infrastructure for the user domain.
//!
//! Mirrors `domain_posts::domain` shape: each submodule is either the
//! canonical owner of an infrastructure type (e.g. `error`, `response`) or a
//! re-export shim during the transition from `application_core`.

pub mod error;
pub mod response;
