//! Authenticated media metadata adapter — `GET /media/info/{*path}`.
//!
//! Thin wrapper around `MetadataMediaHandler`. Bucket-name validation and
//! `MediaConfig` construction happen here; storage and metadata formatting
//! are delegated to the application handler.

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension,
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
        read::metadata_handler::{MetadataMediaHandler, MetadataMediaHandlerTrait},
        MediaConfig,
    },
};

#[derive(Debug, Deserialize)]
pub struct MetadataQueryParams {
    pub bucket: Option<String>,
}

#[instrument(skip(state, _cookies))]
pub async fn api_get_media_metadata(
    State(state): State<MediaApiState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(path): Path<String>,
    Query(params): Query<MetadataQueryParams>,
) -> impl IntoResponse {
    if let Some(name) = &params.bucket {
        if let Some(reason) = bucket_name_error(name) {
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ValidationError)
                .add_error(format!("bucket: {}", reason))
                .to_axum_response();
        }
    }
    let bucket = params.bucket;
    let include_bucket_query = bucket.is_some();

    let media_config = std::sync::Arc::new(MediaConfig {
        storage: state.media_config.storage.clone(),
        bucket: bucket.unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });
    let handler = MetadataMediaHandler { media_config };

    match handler.get_metadata(path, include_bucket_query).await {
        Ok(metadata) => ApiResponseWith::new(metadata).to_axum_response(),
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
