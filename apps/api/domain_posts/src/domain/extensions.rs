//! `StringExtension::to_slug`, `generate_vietnam_now`, and small string/date
//! helpers used by the post handlers.
//!
//! Moved from `application_core::common::{extensions, datetime_generator}`
//! per design Decision 2.

pub use super::datetime_generator::generate_vietnam_now;
pub use super::extensions_impl::StringExtension;