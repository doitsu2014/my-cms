//! Application-layer command handlers for the category domain.
//!
//! Moved from `application_core::commands::category::*` per the
//! `consolidate-category-ai-translate-into-domain-posts` change. All category
//! command handlers (create / read / modify / delete) now live alongside the
//! post handlers and consume the canonical `domain_posts::domain::AppError`
//! and `domain_posts::entities::prelude` symbols.

pub mod create;
pub mod delete;
pub mod modify;
pub mod read;
