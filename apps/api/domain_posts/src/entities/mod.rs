//! SeaORM-generated entities owned by the post domain.
//!
//! During the transition the entities live in `application_core::entities`
//! because they were generated once against the single migration set.
//! The historical entity set (categories, category_tags,
//! category_translations, posts, post_tags, post_translations, tags,
//! translation_jobs) is preserved exactly so the migration identity and
//! column names match. Each non-post domain is extracted in a follow-up
//! change and brings its own entity subset.

pub use application_core::entities::*;