//! Domain-owned infrastructure for the post service.
//!
//! Each module here is the canonical owner of that infrastructure type
//! (for example `error`, `response`, and `layers`). See
//! `docs/architecture/software-architecture.md`.

pub mod datetime_generator;
pub mod env;
pub mod error;
pub mod extensions;
pub mod extensions_impl;
pub mod graphql;
pub mod layers;
pub mod postgres;
pub mod response;
pub mod storage;
pub mod vector_store;
