//! `tag_helper` — local tag operations owned by the post domain.
//!
//! Per design Decision 2 / Migration Plan step 4.4, the
//! `PostCreateHandler -> TagCreateHandler` cross-domain call is resolved
//! by lifting the tag create + read handlers into this module so the
//! post domain does not depend on `application_core::commands::tag::*`.
//! The tag delete handler is also lifted here by the
//! `split-media-and-user-domains-merge-tags-into-posts` change, so the
//! post-domain tag CRUD lifecycle is fully self-contained.
//!
//! The helper is `pub(crate)` and is not exported from any other crate.
//! Future domain extraction (`domain_tags`) will consume this module as
//! the canonical tag-domain source of truth.

pub mod create;
pub mod delete;
pub mod read;

pub use create::{CreateTagsResponse, TagCreateHandler, TagCreateHandlerTrait};
pub use delete::{TagDeleteHandler, TagDeleteHandlerTrait};
pub use read::{GetAndClassifyTagCommandResponse, TagReadHandler, TagReadHandlerTrait};
