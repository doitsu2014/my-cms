use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use axum::response::{self, IntoResponse};
use tracing::instrument;

pub async fn graphiql() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
