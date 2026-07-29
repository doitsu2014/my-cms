//! Tag-creation helper lifted from the cross-domain call inside
//! `PostCreateHandler` into the post domain per design Decision 2 /
//! Migration Plan. This is the local helper the post domain uses to
//! create tags inside a transaction without depending on a sibling
//! domain's tag handler.
//!
//! During the transition this re-exports the canonical implementation
//! in `application_core::commands::tag::create::create_handler`.
//! When a future `domain_tags` is extracted, this helper is updated to
//! call into that domain or remains a private in-domain helper.

pub use application_core::commands::tag::create::create_handler::{
    CreateTagsResponse, TagCreateHandler, TagCreateHandlerTrait,
};