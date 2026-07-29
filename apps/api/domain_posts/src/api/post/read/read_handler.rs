//! `GET /posts`, `GET /posts/{post_id}` — read posts with optional filtering.

use application_core::entities::sea_orm_active_enums::CategoryType;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use sea_orm::sqlx::types::Uuid;
use serde::Deserialize;
use tracing::instrument;

use domain_interface::DomainContext;
use domain_posts::handlers::post::read::read_handler::{PostReadHandler, PostReadHandlerTrait};

use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub category_type: Option<CategoryType>,
}

#[instrument]
pub async fn api_get_posts_with_filtering(
    State(ctx): State<DomainContext>,
    query: Query<QueryParams>,
) -> impl IntoResponse {
    let handler = PostReadHandler {
        db: ctx.conn.clone(),
    };

    let result = handler
        .handle_get_posts_with_filtering(query.category_type.to_owned(), None)
        .await;
    match result {
        Ok(posts) => ApiResponseWith::new(posts).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

#[instrument]
pub async fn api_get_post(
    State(ctx): State<DomainContext>,
    Path(post_id): Path<Uuid>,
) -> impl IntoResponse {
    let handler = PostReadHandler {
        db: ctx.conn.clone(),
    };
    let result = handler.handle_get_post(post_id).await;

    match result {
        Ok(categories) => ApiResponseWith::new(categories).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}