//! Self-contained SEO head-asset domain.
//!
//! The domain owns the durable trusted-source boundary, administrator CRUD,
//! public enabled-asset delivery, and its schema migration. The gateway owns
//! authentication layers and composition.

#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod api;
pub mod domain;
pub mod entities;
pub mod handlers;
pub mod migrations;
pub mod migrations_cli;
pub mod service;

#[cfg(test)]
mod integration_tests;

pub use domain::error::AppError;
pub use service::DomainSeoService;
