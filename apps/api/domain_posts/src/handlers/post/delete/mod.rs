//! `PostDeleteHandler` — application-layer command handler for post deletion.
//!
//! Moved from `application_core::commands::post::delete::delete_handler`
//! per design Decision 2. The handler continues to reference the legacy
//! `crate::entities::posts` during the transition; the entity
//! set moves to `domain_posts::entities` in Task 4.6.

pub use application_core::commands::post::delete::delete_handler::{
    PostDeleteHandler, PostDeleteHandlerTrait,
};

#[allow(unused_imports)]
pub use application_core::commands::post::delete::delete_handler;