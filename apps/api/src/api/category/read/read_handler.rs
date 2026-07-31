use application_core::commands::category::read::category_read_handler::{
    CategoryReadHandler, CategoryReadHandlerTrait,
};
use application_core::entities::sea_orm_active_enums::CategoryType;
use axum::extract::{Extension, Path, Query};
use axum::response::IntoResponse;
use sea_orm::sqlx::types::Uuid;
use serde::Deserialize;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub category_type: Option<CategoryType>,
}

#[instrument]
pub async fn api_get_categories_with_filtering(
    state: Extension<AppState>,
    query: Query<QueryParams>,
) -> impl IntoResponse {
    let handler = CategoryReadHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_get_with_filtering(query.category_type.to_owned())
        .await;

    match result {
        Ok(categories) => ApiResponseWith::new(categories).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

#[instrument]
pub async fn api_get_category(
    state: Extension<AppState>,
    Path(category_id): Path<Uuid>,
) -> impl IntoResponse {
    let handler = CategoryReadHandler {
        db: state.conn.clone(),
    };
    let result = handler.handle_get_category(category_id).await;

    match result {
        Ok(categories) => ApiResponseWith::new(categories).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
