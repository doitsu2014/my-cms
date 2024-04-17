use axum::{extract::State, http::Result, response::IntoResponse, Json};

use crate::{request_create_post::RequestCreatePost, ApiResponseError, AppState, ErrorCode};

pub async fn handle_get_list(state: State<AppState>) {}

pub async fn handle_post(state: State<AppState>) -> impl IntoResponse {
    // let active_model = post.into_model().into_active_model();
    // post::Entity::insert(active_model).exec(&state.conn).await;
    //

    ApiResponseError::new(
        ErrorCode::ValidationError,
        "User is unauthorized".to_string(),
    )
    .to_axum_response()
}
