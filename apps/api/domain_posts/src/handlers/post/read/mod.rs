//! `PostReadHandler` — application-layer command handler for reading posts.
//!
//! Moved from `application_core::commands::post::read::read_handler` per
//! design Decision 2. The canonical implementation now lives in this crate.

pub mod read_handler;
pub use read_handler::*;
