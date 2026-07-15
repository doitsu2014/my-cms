use crate::common::supabase_auth::SupabaseToken;
use application_core::commands::media::bucket::dto::is_valid_bucket_name;
use application_core::commands::media::read::metadata_handler::{
    MetadataMediaHandler, MetadataMediaHandlerTrait,
};
use application_core::commands::media::MediaConfig;
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension,
};
use serde::Deserialize;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse, ErrorCode};

#[derive(Debug, Deserialize)]
pub struct MetadataQueryParams {
    pub bucket: Option<String>,
}

#[instrument(skip(state))]
pub async fn api_get_media_metadata(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
    Path(path): Path<String>,
    Query(params): Query<MetadataQueryParams>,
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
    let handler = MetadataMediaHandler { media_config };

    match handler.get_metadata(path).await {
        Ok(metadata) => ApiResponseWith::new(metadata).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
