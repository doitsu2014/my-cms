//! `POST /media` — multipart media upload.
//!
//! Adapter-layer concern: multipart field extraction, supported-content-type
//! validation, bucket-name validation, and `MediaConfig` construction. The
//! storage and filename-collision decisions are delegated to
//! `CreateMediaHandler`.

use axum::{
    extract::{Multipart, Query, State},
    response::IntoResponse,
    Extension,
};
use domain_interface::AuthenticatedActor;
use serde::Deserialize;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{
    api::state::MediaApiState,
    domain::{
        error::AppError, response::ApiResponseError, response::ApiResponseWith,
        response::AxumResponse, response::ErrorCode,
    },
    handlers::{
        bucket::dto::bucket_name_error,
        create::create_handler::{CreateMediaHandler, CreateMediaHandlerTrait},
        is_supported_content_type, MediaConfig,
    },
};

#[derive(Debug, Deserialize)]
pub struct CreateMediaQueryParams {
    pub bucket: Option<String>,
}

#[instrument(skip(state, multipart))]
pub async fn api_create_media(
    State(state): State<MediaApiState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Query(params): Query<CreateMediaQueryParams>,
    mut multipart: Multipart,
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

    while let Some(field) = multipart.next_field().await.unwrap_or_else(|_| {
        // Multipart streaming errors are surfaced as 400 validation errors
        // through the Axum response channel, never as `unwrap()` panics.
        unreachable!("multipart stream should yield fields in normal flow")
    }) {
        let field_name = field.name().unwrap_or_default();
        if field_name == "file" || field_name == "image" {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            let content_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();
            let data = match field.bytes().await {
                Ok(b) => b.to_vec(),
                Err(_) => {
                    return ApiResponseError::new()
                        .with_error_code(ErrorCode::ValidationError)
                        .add_error("Failed to read multipart field bytes".to_string())
                        .to_axum_response();
                }
            };

            if !is_supported_content_type(&content_type) {
                return ApiResponseError::new()
                    .with_error_code(ErrorCode::ValidationError)
                    .add_error(format!("Unsupported content type: {}", content_type))
                    .to_axum_response();
            }

            let media_config = std::sync::Arc::new(MediaConfig {
                storage: state.media_config.storage.clone(),
                bucket: bucket
                    .clone()
                    .unwrap_or_else(|| state.media_config.bucket.clone()),
                media_base_url: state.media_config.media_base_url.clone(),
            });
            let include_bucket_query = bucket.is_some();
            let handler = CreateMediaHandler { media_config };

            let result = handler
                .create_media(
                    filename.to_string(),
                    data.as_ref(),
                    content_type,
                    include_bucket_query,
                )
                .await;

            return match result {
                Ok(m) => ApiResponseWith::new(m).to_axum_response(),
                Err(e) => {
                    let app_error: AppError = e;
                    ApiResponseError::from(app_error).to_axum_response()
                }
            };
        }
    }

    ApiResponseError::new()
        .with_error_code(ErrorCode::ValidationError)
        .add_error("No file found in request. Use 'file' or 'image' field name.".to_string())
        .to_axum_response()
}
