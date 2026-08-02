//! SeaORM-generated entities owned by the post domain.
//!
//! The `consolidate-category-ai-translate-into-domain-posts` change physically
//! moved the category entities (`categories`, `category_tags`,
//! `category_translations`, `CategoryType` enum) into this module alongside
//! the post-aggregate entities (`posts`, `post_tags`, `post_translations`,
//! `translation_jobs`, `tags`, `test_fulltext`). The canonical entity set is
//! the one declared here; `application_core::entities` is now a re-export
//! shim that forwards to this module.

pub mod prelude;

pub use prelude::*;

pub mod categories;
pub mod category_tags;
pub mod category_translations;
pub mod post_tags;
pub mod post_translations;
pub mod posts;
pub mod sea_orm_active_enums;
pub mod tags;
pub mod test_fulltext;
pub mod translation_jobs;
