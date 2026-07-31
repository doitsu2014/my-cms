//! Application-layer command handlers for the post domain.
//!
//! Skeleton phase (Task 3): empty module. Task 4 moves the post command
//! handlers from `application_core::commands::post::*` into this tree.
//!
//! During the transition each `mod.rs` re-exports the canonical handler from
//! `application_core::commands::post::*`. Once the post commands physically
//! move into `domain_posts::handlers::post::*`, the re-exports are dropped.

// Submodules are intentionally empty in the skeleton phase; Task 4 fills them.
pub mod post;
pub mod tag_helper;
pub mod translation_jobs;
pub mod vector_store;
