//! SeaORM prelude — canonical re-exports of every entity module so that
//! `use domain_posts::entities::prelude::*;` brings `Entity as Posts`,
//! `Categories`, etc. into scope. Originally part of
//! `application_core::entities::prelude`; moved here per the
//! `consolidate-category-ai-translate-into-domain-posts` change.

pub use super::categories::Entity as Categories;
pub use super::category_tags::Entity as CategoryTags;
pub use super::category_translations::Entity as CategoryTranslations;
pub use super::post_tags::Entity as PostTags;
pub use super::post_translations::Entity as PostTranslations;
pub use super::posts::Entity as Posts;
pub use super::tags::Entity as Tags;
pub use super::translation_jobs::Entity as TranslationJobs;
