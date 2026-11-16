//! `domain_posts` — self-contained Blog Post Service.
//!
//! Owns posts, categories, tags, translations, and the post GraphQL HTTP
//! surface. The gateway is the only composition root. There is no standalone
//! posts binary and no `application_core` shim. See
//! `docs/architecture/software-architecture.md`.

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
