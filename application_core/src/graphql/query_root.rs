use crate::entities::*;
use async_graphql::dynamic::*;
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

pub fn schema(
    database: DatabaseConnection,
    depth: Option<usize>,
    complexity: Option<usize>,
    is_mutation_supported: bool,
) -> Result<Schema, SchemaError> {
    let mut builder = Builder::new(&CONTEXT, database.clone());
    seaography::register_entities!(builder, [categories, category_tags, posts, post_tags, tags, category_translations, post_translations]);
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
