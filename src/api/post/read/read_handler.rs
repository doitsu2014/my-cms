use application_core::commands::post::read::read_handler::{PostReadHandler, PostReadHandlerTrait};
use application_core::entities::sea_orm_active_enums::CategoryType;
use axum::extract::{Path, Query};
use axum::{extract::State, response::IntoResponse};
use sea_orm::sqlx::types::Uuid;
use serde::Deserialize;
use tracing::instrument;

use crate::ApiResponseError;
use crate::AxumResponse;
use crate::{ApiResponseWith, AppState};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub category_type: Option<CategoryType>,
}

#[instrument]
pub async fn api_get_posts_with_filtering(
    state: State<AppState>,
    query: Query<QueryParams>,
) -> impl IntoResponse {
    let handler = PostReadHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_get_posts_with_filtering(query.category_type.to_owned())
        .await;
    match result {
        Ok(posts) => ApiResponseWith::new(posts).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
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
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
