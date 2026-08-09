//! `PostDeleteHandler` — application-layer command handler for post deletion.
//!
//! Moved from `application_core::commands::post::delete::delete_handler`
//! per the `consolidate-category-ai-translate-into-domain-posts` change.

pub mod delete_handler;

pub use delete_handler::{PostDeleteHandler, PostDeleteHandlerTrait};
