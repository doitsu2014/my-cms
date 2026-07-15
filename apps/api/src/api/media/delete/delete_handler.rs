use crate::common::supabase_auth::SupabaseToken;
use application_core::commands::media::bucket::dto::is_valid_bucket_name;
use application_core::commands::media::delete::delete_handler::{
    DeleteMediaHandler, DeleteMediaHandlerTrait,
};
use application_core::commands::media::MediaConfig;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::Deserialize;
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
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
    Path(path): Path<String>,
    Query(params): Query<DeleteMediaQueryParams>,
) -> impl IntoResponse {
    if let Some(name) = &params.bucket {
        if !is_valid_bucket_name(name) {
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ValidationError)
                .add_error(
                    "bucket: must start with a lowercase letter; only [a-z0-9_-] allowed"
                        .to_string(),
                )
                .to_axum_response();
        }
    }
    let storage = match &params.bucket {
        Some(name) => state.media_config.storage.with_bucket(name),
        None => state.media_config.storage.clone(),
    };
    let media_config = std::sync::Arc::new(MediaConfig {
        storage,
        media_base_url: state.media_config.media_base_url.clone(),
        bucket_override: params.bucket.clone(),
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
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
    Query(params): Query<DeleteMediaQueryParams>,
    Json(paths): Json<Vec<String>>,
) -> impl IntoResponse {
    if let Some(name) = &params.bucket {
        if !is_valid_bucket_name(name) {
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ValidationError)
                .add_error(
                    "bucket: must start with a lowercase letter; only [a-z0-9_-] allowed"
                        .to_string(),
                )
                .to_axum_response();
        }
    }
    let storage = match &params.bucket {
        Some(name) => state.media_config.storage.with_bucket(name),
        None => state.media_config.storage.clone(),
    };
    let media_config = std::sync::Arc::new(MediaConfig {
        storage,
        media_base_url: state.media_config.media_base_url.clone(),
        bucket_override: params.bucket.clone(),
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
