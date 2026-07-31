//! `PostReadHandler` — application-layer command handler for reading posts.
//!
//! Moved from `application_core::commands::post::read::read_handler` per
//! design Decision 2. The handler continues to reference the legacy
//! `crate::entities` during the transition; the entity set moves
//! to `domain_posts::entities` in Task 4.6.

pub use application_core::commands::post::read::read_handler::{
    PostReadHandler, PostReadHandlerTrait,
};

#[allow(unused_imports)]
pub use application_core::commands::post::read::read_handler;