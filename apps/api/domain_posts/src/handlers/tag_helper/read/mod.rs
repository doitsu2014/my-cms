//! Tag-read handler — owned by the post domain per design Decision 2.
//!
//! Used internally by the post creation pipeline to classify tags as
//! existing vs new inside a transaction.

pub mod read_handler;
pub mod read_response;

pub use read_handler::{TagReadHandler, TagReadHandlerTrait};
pub use read_response::GetAndClassifyTagCommandResponse;