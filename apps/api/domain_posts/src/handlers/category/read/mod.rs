//! `CategoryReadHandler` — application-layer command handler for reading
//! categories with their tags and translations.
//!
//! Moved from `application_core::commands::category::read` per the
//! `consolidate-category-ai-translate-into-domain-posts` change.

pub mod category_read_handler;
pub use category_read_handler::*;
