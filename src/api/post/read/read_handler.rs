use application_core::commands::post::read::read_handler::{PostReadHandler, PostReadHandlerTrait};
use axum::extract::Path;
use axum::{extract::State, response::IntoResponse};
use sea_orm::sqlx::types::Uuid;
use tracing::instrument;

use crate::ApiResponseError;
use crate::AxumResponse;
use crate::{ApiResponseWith, AppState};

#[instrument]
pub async fn api_get_all_posts(state: State<AppState>) -> impl IntoResponse {
    let handler = PostReadHandler {
        db: state.conn.clone(),
    };

    let result = handler.handle_get_all_posts().await;
    match result {
        Ok(posts) => ApiResponseWith::new(posts).to_axum_response(),
        Err(e) => ApiResponseError::from_app_error(e).to_axum_response(),
    }
}

#[instrument]
pub async fn api_get_post(state: State<AppState>, Path(post_id): Path<Uuid>) -> impl IntoResponse {
    let handler = PostReadHandler {
        db: state.conn.clone(),
    };
    let result = handler.handle_get_post(post_id).await;

    match result {
        Ok(categories) => ApiResponseWith::new(categories).to_axum_response(),
        Err(e) => ApiResponseError::from_app_error(e).to_axum_response(),
    }
}
