//! Tag-delete handler — owned by the post domain per design Decision 2.
//!
//! The post-domain tag CRUD lifecycle (`create`, `read`, `delete`) is
//! fully self-contained. Originally the delete logic lived in
//! `application_core::commands::tag::delete`; that module is removed
//! by the `split-media-and-user-domains-merge-tags-into-posts` change
//! in favor of this canonical post-domain implementation.

pub mod delete_handler;

pub use delete_handler::{TagDeleteHandler, TagDeleteHandlerTrait};
