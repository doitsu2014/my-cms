use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};
use crate::entities::sea_orm_active_enums::CategoryType;
use crate::handlers::category::read::category_read_handler::{
    CategoryReadHandler, CategoryReadHandlerTrait,
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use domain_interface::DomainContext;
use sea_orm::sqlx::types::Uuid;
use serde::Deserialize;
use tracing::instrument;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub category_type: Option<CategoryType>,
}

#[instrument]
pub async fn api_get_categories_with_filtering(
    State(ctx): State<DomainContext>,
    query: Query<QueryParams>,
) -> impl IntoResponse {
    let handler = CategoryReadHandler {
        db: ctx.conn.clone(),
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
    State(ctx): State<DomainContext>,
    Path(category_id): Path<Uuid>,
) -> impl IntoResponse {
    let handler = CategoryReadHandler {
        db: ctx.conn.clone(),
    };
    let result = handler.handle_get_category(category_id).await;

    match result {
        Ok(categories) => ApiResponseWith::new(categories).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
