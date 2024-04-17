use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use entity::post;
use sea_orm::EntityTrait;

use crate::{request_create_post::RequestCreatePost, AppState};

pub async fn handle_get_list(state: State<AppState>) {}

pub async fn handle_post(
    state: State<AppState>,
    Json(post): Json<RequestCreatePost>,
) -> impl IntoResponse {
    let active_model: post::ActiveModel = post.into_model().into();
    post::Entity::insert(active_model).exec(&state.conn).await;

    (StatusCode::CREATED, "Post created").into_response()
}
