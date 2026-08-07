//! SeaORM-generated entities owned by the post domain.
//!
//! After the `consolidate-category-ai-translate-into-domain-posts` change,
//! the canonical entity files live in `domain_posts::entities`. This module
//! is a pure re-export shim so that legacy `cms::api::{media, user,
//! administrator}::*` modules (which still import `application_core::entities`)
//! keep compiling without modification.
//!
//! **Note:** `application_core` retains the `domain_posts` path dependency
//! solely to power this shim. The only callers that flow through it are the
//! legacy `cms::api::{media, user, administrator}::*` modules plus
//! `apps/api/test_helpers`. After the
//! `migrate-legacy-to-domain-posts` change, the legacy
//! `application_core::commands::post::*` and
//! `application_core::commands::ai::*` modules have been deleted, and
//! every post-related code path lives in `domain_posts::*`.

pub use domain_posts::entities::*;
