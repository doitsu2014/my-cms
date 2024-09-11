use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::response::{self, IntoResponse};
use tracing::instrument;

#[instrument]
pub async fn graphql_mutable() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(
        "/graphql/mutable",
    )))
}

#[instrument]
pub async fn graphql_immutable() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new(
        "/graphql/immutable",
    )))
}
