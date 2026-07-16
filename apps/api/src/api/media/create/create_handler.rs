use crate::common::supabase_auth::SupabaseToken;
use application_core::commands::media::bucket::dto::bucket_name_error;
use application_core::commands::media::{
    create::create_handler::{CreateMediaHandler, CreateMediaHandlerTrait},
    is_supported_content_type, MediaConfig,
};
use axum::{
    extract::{Multipart, Query, State},
    response::IntoResponse,
    Extension,
};
use serde::Deserialize;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse, ErrorCode};

#[derive(Debug, Deserialize)]
pub struct CreateMediaQueryParams {
    pub bucket: Option<String>,
}

#[instrument(skip(state, multipart))]
pub async fn api_create_media(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
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

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap_or_default();
        if field_name == "file" || field_name == "image" {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            let content_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();
            let data = field.bytes().await.unwrap().to_owned();

            if !is_supported_content_type(&content_type) {
                return ApiResponseError::new()
                    .with_error_code(ErrorCode::ValidationError)
                    .add_error(format!("Unsupported content type: {}", content_type))
                    .to_axum_response();
            }

            let storage = match &bucket {
                Some(name) => state.media_config.storage.with_bucket(name),
                None => state.media_config.storage.clone(),
            };
            let media_config = std::sync::Arc::new(MediaConfig {
                storage,
                media_base_url: state.media_config.media_base_url.clone(),
                bucket_override: bucket.clone(),
            });
            let handler = CreateMediaHandler { media_config };

            let result = handler
                .create_media(filename.to_string(), data.as_ref(), content_type)
                .await;

            return match result {
                Ok(m) => ApiResponseWith::new(m).to_axum_response(),
                Err(e) => ApiResponseError::from(e).to_axum_response(),
            };
        }
    }

    ApiResponseError::new()
        .with_error_code(ErrorCode::ValidationError)
        .add_error("No file found in request. Use 'file' or 'image' field name.".to_string())
        .to_axum_response()
}
