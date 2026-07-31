//! Seaography contribution for the post domain.
//!
//! The gateway owns the two `Arc<Schema>` instances; the post domain
//! contributes the entity registration via [`contribute_post_schema`]. This
//! keeps the gateway from importing post entities directly.
//!
//! During the transition the canonical schema builder lives in
//! `application_core::graphql::query_root::schema` (Task 4.6 will generate
//! the post-domain entities directly). The wrapper here preserves the
//! `domain_posts::domain::graphql::contribute_post_schema` signature so the
//! gateway composition can call it without depending on `application_core`.

use sea_orm::DatabaseConnection;

/// Build the post-domain's contribution to a Seaography schema.
///
/// Wraps `application_core::graphql::query_root::schema` with the post
/// domain's parameters. The gateway can call this for both the immutable
/// (mutations disabled) and mutable schemas.
pub fn contribute_post_schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
    is_mutation_supported: bool,
) -> Result<async_graphql::dynamic::Schema, async_graphql::dynamic::SchemaError> {
    application_core::graphql::query_root::schema(
        database,
        depth,
        complexity,
        is_mutation_supported,
    )
}

/// Historical Seaography entity set for the post domain — mirrors
/// `application_core::graphql::query_root::schema` lines 22–33.
///
/// Once each non-post domain is extracted, this set will shrink to only
/// the post-aggregate entities (`posts`, `post_tags`, `post_translations`,
/// `translation_jobs`). Until then the post domain contributes the full
/// historical set.
pub const POST_GRAPHQL_ENTITIES: &[&str] = &[
    "categories",
    "category_tags",
    "posts",
    "post_tags",
    "tags",
    "category_translations",
    "post_translations",
];
