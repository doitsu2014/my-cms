use axum::{extract::State, response::IntoResponse};
use entity::post;
use sea_orm::EntityTrait;
use tracing::instrument;

use crate::{ApiResponseWith, AppState, AxumResponse};

#[instrument]
pub async fn handle(state: State<AppState>) -> impl IntoResponse {
    let all = post::Entity::find()
        .all(&state.conn)
        .await
        .expect("Failed to fetch all posts");

    ApiResponseWith::new(all).to_axum_response()
}
