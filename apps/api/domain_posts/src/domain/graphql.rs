//! Seaography contribution for the post domain.
//!
//! The gateway owns the two `Arc<Schema>` instances; the post domain
//! contributes the entity registration via [`contribute_post_schema`]. This
//! keeps the gateway from importing post entities directly.
//!
//! The canonical schema builder previously lived in
//! `application_core::graphql::query_root::schema`; it is now inlined here
//! because `domain_posts` no longer depends on `application_core`.

use crate::entities::*;
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext};

lazy_static::lazy_static! {
    static ref CONTEXT : BuilderContext = {
        let context = BuilderContext::default();
        BuilderContext {
            ..context
        }
    };
}

/// Build the post-domain's contribution to a Seaography schema.
pub fn contribute_post_schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
    is_mutation_supported: bool,
) -> Result<async_graphql::dynamic::Schema, async_graphql::dynamic::SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(
        builder,
        [
            categories,
            category_tags,
            posts,
            post_tags,
            tags,
            category_translations,
            post_translations
        ]
    );
    builder.register_enumeration::<crate::entities::sea_orm_active_enums::CategoryType>();
    if !is_mutation_supported {
        builder.mutations = vec![];
    }
    let schema = builder.schema_builder();
    let schema = if let Some(depth) = depth {
        schema.limit_depth(depth)
    } else {
        schema
    };
    let schema = if let Some(complexity) = complexity {
        schema.limit_complexity(complexity)
    } else {
        schema
    };
    schema.data(database).finish()
}

/// Historical Seaography entity set for the post domain — mirrors
/// the prior `application_core::graphql::query_root::schema` entity list.
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
