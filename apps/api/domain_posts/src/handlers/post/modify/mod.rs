//! `PostModifyHandler` — application-layer command handler for modifying
//! posts.
//!
//! Moved from `application_core::commands::post::modify::modify_handler` per
//! design Decision 2. The handler continues to reference the legacy
//! `crate::entities` during the transition; the entity set moves
//! to `domain_posts::entities` in Task 4.6.

pub mod modify_handler;
pub mod modify_request;

pub use modify_handler::{PostModifyHandler, PostModifyHandlerTrait};
pub use modify_request::ModifyPostRequest;
