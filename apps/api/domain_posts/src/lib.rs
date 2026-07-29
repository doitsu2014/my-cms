//! `domain_posts` — self-contained Blog Post Service.
//!
//! See `openspec/changes/refactor-api-into-pluggable-domain-libraries/design.md`
//! for the full architectural context. During the transition each module
//! here either owns its own code or re-exports the post-relevant subset of
//! the legacy `application_core` / `migration` / `src/api` trees. The public
//! surface (`DomainPostService`, `AppError`, `ApiResponseWith`,
//! `SupabaseAuthLayer`, …) is stable and is what the gateway and any
//! standalone deployment depend on.

#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod api;
pub mod domain;
pub mod entities;
pub mod handlers;
pub mod migrations;
pub mod observability;
pub mod service;

pub mod migrations_cli;

pub use domain_interface as interface;
pub use service::DomainPostService;