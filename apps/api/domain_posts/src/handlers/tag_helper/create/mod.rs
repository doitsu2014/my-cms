//! Tag-create handler — owned by the post domain per design Decision 2.
//!
//! The post creation pipeline needs to create tags inside a transaction.
//! Per design, this helper is lifted into the post domain so the post
//! domain does not depend on a sibling domain's tag handler.

pub mod create_handler;

pub use create_handler::{
    CreateTagsResponse, TagCreateHandler, TagCreateHandlerTrait,
};