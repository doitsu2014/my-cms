//! SeaORM-generated entities owned by the post domain.
//!
//! During the transition the canonical entity definitions live in
//! `application_core::entities` (the legacy location). The post domain
//! owns the post-aggregate subset (`posts`, `post_tags`, `post_translations`,
//! `translation_jobs`) and shares the cross-domain entities (`categories`,
//! `category_tags`, `category_translations`, `tags`) with the legacy crate.
//!
//! Once `application_core` becomes a pure re-export shim (Task 4.6 final
//! step), the entities will be physically moved into this module via
//! `sea-orm generate entity` and `application_core::entities` will
//! re-export from here.

pub use application_core::entities::*;