use application_core::commands::post::read::read_handler::{PostReadHandler, PostReadHandlerTrait};
use axum::{extract::State, response::IntoResponse};
use std::sync::Arc;
use tracing::instrument;

use crate::ApiResponseError;
use crate::AxumResponse;
use crate::ErrorCode;
use crate::{ApiResponseWith, AppState};

#[instrument]
pub async fn api_get_all_posts(state: State<AppState>) -> impl IntoResponse {
    let handler = PostReadHandler {
        db: Arc::new(state.conn.clone()),
    };

    let result = handler.handle_get_all_posts().await;
    match result {
        Ok(posts) => ApiResponseWith::new(posts).to_axum_response(),
        Err(e) => ApiResponseError::new()
            .with_error_code(ErrorCode::UnknownError)
            .add_error(e.to_string())
            .to_axum_response(),
    }
}
