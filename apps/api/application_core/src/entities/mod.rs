//! SeaORM-generated entities owned by the post domain.
//!
//! After the `consolidate-category-ai-translate-into-domain-posts` change,
//! the canonical entity files live in `domain_posts::entities`. This module
//! is a pure re-export shim so that legacy `cms::api::{media, user,
//! administrator}::*` modules (which still import `application_core::entities`)
//! keep compiling without modification.

pub use domain_posts::entities::*;
