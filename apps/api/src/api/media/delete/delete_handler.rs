use application_core::commands::media::bucket::dto::bucket_name_error;
use application_core::commands::media::delete::delete_handler::{
    DeleteMediaHandler, DeleteMediaHandlerTrait,
};
use application_core::commands::media::MediaConfig;
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    Extension, Json,
};
use domain_interface::AuthenticatedActor;
use serde::Deserialize;
use std::sync::Arc;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse, ErrorCode};

#[derive(Debug, Deserialize)]
pub struct DeleteMediaQueryParams {
    pub bucket: Option<String>,
}

/// Delete a single media file by path
#[instrument(skip(state))]
pub async fn api_delete_media(
    state: Extension<AppState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(path): Path<String>,
    Query(params): Query<DeleteMediaQueryParams>,
) -> impl IntoResponse {
    if let Some(name) = &params.bucket {
        if let Some(reason) = bucket_name_error(name) {
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ValidationError)
                .add_error(format!("bucket: {}", reason))
                .to_axum_response();
        }
    }
    let storage = state.media_config.storage.clone();
    let media_config = Arc::new(MediaConfig {
        storage,
        bucket: params
            .bucket
            .unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });
    let handler = DeleteMediaHandler { media_config };

    match handler.delete_media(path).await {
        Ok(_) => ApiResponseWith::new("Deleted successfully").to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// Delete multiple media files by paths (batch delete)
#[instrument(skip(state))]
pub async fn api_delete_media_batch(
    state: Extension<AppState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Query(params): Query<DeleteMediaQueryParams>,
    Json(paths): Json<Vec<String>>,
) -> impl IntoResponse {
    if let Some(name) = &params.bucket {
        if let Some(reason) = bucket_name_error(name) {
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ValidationError)
                .add_error(format!("bucket: {}", reason))
                .to_axum_response();
        }
    }
    let storage = state.media_config.storage.clone();
    let media_config = Arc::new(MediaConfig {
        storage,
        bucket: params
            .bucket
            .unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });
    let handler = DeleteMediaHandler { media_config };

    match handler.delete_media_batch(paths).await {
        Ok(deleted_count) => ApiResponseWith::new(serde_json::json!({
            "deletedCount": deleted_count
        }))
        .to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
