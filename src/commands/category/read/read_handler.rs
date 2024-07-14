use crate::{ApiResponseWith, AppState, AxumResponse};
use axum::{extract::State, response::IntoResponse};
use entity::category;
use sea_orm::EntityTrait;
use tracing::instrument;

#[instrument]
pub async fn handle(state: State<AppState>) -> impl IntoResponse {
    let all = category::Entity::find()
        .all(&state.conn)
        .await
        .expect("Failed to fetch all categories");

    ApiResponseWith::new(all).to_axum_response()
}
