use application_core::commands::category::read::category_read_handler::{
    CategoryReadHandler, CategoryReadHandlerTrait,
};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use sea_orm::sqlx::types::Uuid;
use std::sync::Arc;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse, ErrorCode};

#[instrument]
pub async fn api_get_all_categories(state: State<AppState>) -> impl IntoResponse {
    let handler = CategoryReadHandler {
        db: Arc::new(state.conn.clone()),
    };

    let result = handler.handle_get_all_categories().await;

    match result {
        Ok(categories) => ApiResponseWith::new(categories).to_axum_response(),
        Err(e) => ApiResponseError::new()
            .with_error_code(ErrorCode::UnknownError)
            .add_error(e.to_string())
            .to_axum_response(),
    }
}

#[instrument]
pub async fn api_get_category(
    state: State<AppState>,
    Path(category_id): Path<Uuid>,
) -> impl IntoResponse {
    let handler = CategoryReadHandler {
        db: Arc::new(state.conn.clone()),
    };
    let result = handler.handle_get_category(category_id).await;

    match result {
        Ok(categories) => ApiResponseWith::new(categories).to_axum_response(),
        Err(e) => ApiResponseError::new()
            .with_error_code(ErrorCode::UnknownError)
            .add_error(e.to_string())
            .to_axum_response(),
    }
}
