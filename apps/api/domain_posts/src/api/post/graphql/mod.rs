//! GraphQL HTTP handlers for the post domain.
//!
//! The playground handlers render the GraphiQL HTML pointing at the
//! `/posts/graphql/{immutable,mutable}` endpoints. The actual GraphQL
//! POST service is wired inline in `domain_posts::api::routes` via
//! `async_graphql_axum::GraphQL::new(...)` so it can re-use the
//! `Arc<Schema>` values that the gateway already builds at startup.
//!
//! See `openspec/changes/merge-graphql-into-posts-domain/design.md`
//! Decision 2 for the rationale on keeping the module small (just the
//! two playground HTML handlers) and Decision 7 for the rationale on
//! NOT introducing a dedicated POST handler function.

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::response::{self, IntoResponse};
use tracing::instrument;

/// `GET /posts/graphql/immutable` — GraphiQL playground for the
/// public (read-only) schema. The path string embedded in the
/// playground HTML MUST match the route mounted by
/// `domain_posts::api::routes` and by the gateway composition.
#[instrument]
pub async fn playground_immutable() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(
        "/posts/graphql/immutable",
    )))
}

/// `GET /posts/graphql/mutable` — GraphiQL playground for the
/// authenticated (read-write) schema. The path string embedded in
/// the playground HTML MUST match the route mounted by
/// `domain_posts::api::routes` and by the gateway composition.
#[instrument]
pub async fn playground_mutable() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(
        "/posts/graphql/mutable",
    )))
}

#[cfg(test)]
mod tests;
