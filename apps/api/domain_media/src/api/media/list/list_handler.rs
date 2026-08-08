//! `GET /media` — list media objects (prefix-filtered, optional bucket).
//!
//! Adapter-layer concern: bucket-name validation, `MediaConfig` construction
//! per request, and Axum envelope mapping. The actual listing is delegated
//! to `ListMediaHandler`.

use axum::{
    extract::{Query, State},
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
        list::list_handler::{ListMediaHandler, ListMediaHandlerTrait},
        MediaConfig,
    },
};

#[derive(Debug, Deserialize)]
pub struct ListQueryParams {
    pub prefix: Option<String>,
    pub bucket: Option<String>,
}

#[instrument(skip(state, _cookies))]
pub async fn api_list_media(
    State(state): State<MediaApiState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Query(params): Query<ListQueryParams>,
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
            .clone()
            .unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });

    let include_bucket_query = params.bucket.is_some();
    let handler = ListMediaHandler { media_config };

    match handler
        .list_media(params.prefix, include_bucket_query)
        .await
    {
        Ok(media_list) => ApiResponseWith::new(media_list).to_axum_response(),
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
