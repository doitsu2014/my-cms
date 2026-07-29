//! Application-layer command handlers for the post domain.
//!
//! Each sub-module corresponds to one CRUD or pipeline capability and
//! re-exports the canonical handler from `application_core::commands` during
//! the transition. The transition plan documents the staged physical move.

pub mod post;
pub mod tag_helper;
pub mod translation_jobs;
pub mod vector_store;