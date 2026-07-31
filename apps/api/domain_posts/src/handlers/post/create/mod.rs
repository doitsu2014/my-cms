//! `PostCreateHandler` — application-layer command handler for creating
//! posts.
//!
//! Moved from `application_core::commands::post::create::create_handler` per
//! design Decision 2. The handler continues to reference the legacy
//! `crate::entities` during the transition; the entity set moves
//! to `domain_posts::entities` in Task 4.6.
//!
//! The `PostCreateHandler -> TagCreateHandler` cross-domain call is
//! resolved by the in-domain `crate::handlers::tag_helper::create_tags_in_transaction`
//! helper. The helper is `pub(crate)` and is not exported from any other
//! crate.

pub mod create_handler;
pub mod create_request;

pub use create_handler::{PostCreateHandler, PostCreateHandlerTrait};
pub use create_request::{CreatePostRequest, CreatePostTranslationRequest};