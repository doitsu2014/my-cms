//! Seaography contribution for the post domain.
//!
//! The gateway owns the two `Arc<Schema>` instances; the post domain
//! contributes the entity registration via
//! [`contribute_post_schema`]. This keeps the gateway from importing
//! post entities directly.
//!
//! During the transition the entity set is the same historical set the
//! legacy `application_core::graphql::query_root::schema` registers.

use sea_orm::DatabaseConnection;

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

/// Build the post-domain's contribution to a Seaography schema.
///
/// This is a thin shim over the legacy
/// `application_core::graphql::query_root::schema` that hands the gateway
/// back a schema seeded with the post-domain entities. The gateway builds
/// both immutable and mutable schemas from this contribution.
pub fn contribute_post_schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
    is_mutation_supported: bool,
) -> Result<async_graphql::dynamic::Schema, async_graphql::dynamic::SchemaError> {
    application_core::graphql::query_root::schema(database, depth, complexity, is_mutation_supported)
}