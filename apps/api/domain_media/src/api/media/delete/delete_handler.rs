//! `DELETE /media/delete/{*path}` and `DELETE /media` (batch).
//!
//! Adapter-layer concern: bucket-name validation, `MediaConfig` construction,
//! and Axum envelope mapping. Single and batch delete are both delegated to
//! `DeleteMediaHandler`.

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use domain_interface::AuthenticatedActor;
use serde::Deserialize;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{
    api::state::MediaApiState,
    domain::{error::AppError, response::ApiResponseError, response::ApiResponseWith, response::AxumResponse, response::ErrorCode},
    handlers::{
        bucket::dto::bucket_name_error,
        delete::delete_handler::{DeleteMediaHandler, DeleteMediaHandlerTrait},
        MediaConfig,
    },
};

#[derive(Debug, Deserialize)]
pub struct DeleteMediaQueryParams {
    pub bucket: Option<String>,
}

/// `DELETE /media/delete/{*path}` — single-object delete.
#[instrument(skip(state, _cookies))]
pub async fn api_delete_media(
    State(state): State<MediaApiState>,
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
    let media_config = std::sync::Arc::new(MediaConfig {
        storage: state.media_config.storage.clone(),
        bucket: params
            .bucket
            .unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });
    let handler = DeleteMediaHandler { media_config };

    match handler.delete_media(path).await {
        Ok(_) => ApiResponseWith::new("Deleted successfully").to_axum_response(),
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}

/// `DELETE /media` — batch delete (JSON body of paths).
#[instrument(skip(state, _cookies, paths))]
pub async fn api_delete_media_batch(
    State(state): State<MediaApiState>,
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
    let media_config = std::sync::Arc::new(MediaConfig {
        storage: state.media_config.storage.clone(),
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
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
