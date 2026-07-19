use application_core::commands::media::bucket::dto::{bucket_name_error, is_valid_bucket_name};
use application_core::commands::media::read::read_handler::{
    ReadMediaHandler, ReadMediaHandlerTrait, ResizeParams,
};
use application_core::commands::media::SupabaseStorage;
use application_core::common::app_error::AppError;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ImageQueryParams {
    pub w: Option<u32>,
    pub h: Option<u32>,
    pub bucket: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReadQueryParams {
    pub bucket: Option<String>,
}

fn error_status(e: &AppError) -> StatusCode {
    match e {
        AppError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::BAD_GATEWAY,
    }
}

fn error_response(e: AppError) -> Response {
    (
        error_status(&e),
        [(header::CONTENT_TYPE, "application/json")],
        format!(r#"{{"error": "{}"}}"#, e),
    )
        .into_response()
}

fn validate_bucket(name: Option<&str>) -> Result<Option<String>, Box<Response>> {
    let bucket = match name {
        Some(raw) => raw.to_string(),
        None => return Ok(None),
    };
    if !is_valid_bucket_name(&bucket) {
        let reason =
            bucket_name_error(&bucket).unwrap_or_else(|| "invalid bucket name".to_string());
        let resp = (
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            format!(r#"{{"error": "invalid bucket: {}"}}"#, reason),
        )
            .into_response();
        return Err(Box::new(resp));
    }
    Ok(Some(bucket))
}

fn resolve_storage(default_storage: &SupabaseStorage, bucket: Option<String>) -> SupabaseStorage {
    match bucket {
        Some(name) => default_storage.with_bucket(&name),
        None => default_storage.clone(),
    }
}

fn media_response(data: Vec<u8>, content_type: String) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CACHE_CONTROL,
        "public, max-age=31536000, immutable".parse().unwrap(),
    );

    (StatusCode::OK, headers, Body::from(data)).into_response()
}

#[instrument(skip(state))]
pub async fn api_get_media_image(
    state: State<AppState>,
    Path(path): Path<String>,
    Query(params): Query<ImageQueryParams>,
) -> Response {
    let bucket = match validate_bucket(params.bucket.as_deref()) {
        Ok(b) => b,
        Err(resp) => return *resp,
    };

    let storage = resolve_storage(&state.media_config.storage, bucket.clone());
    let handler = ReadMediaHandler::new(Arc::new(storage), state.media_cache.clone());
    let resize_params = ResizeParams::new(params.w, params.h);

    match handler
        .get_rendered_image(path, resize_params, bucket)
        .await
    {
        Ok(cached_media) => media_response(cached_media.data, cached_media.content_type),
        Err(e) => error_response(e),
    }
}

#[instrument(skip(state))]
pub async fn api_get_media(
    state: State<AppState>,
    Path(path): Path<String>,
    Query(params): Query<ReadQueryParams>,
) -> Response {
    let bucket = match validate_bucket(params.bucket.as_deref()) {
        Ok(b) => b,
        Err(resp) => return *resp,
    };

    let storage = resolve_storage(&state.media_config.storage, bucket.clone());
    let handler = ReadMediaHandler::new(Arc::new(storage), state.media_cache.clone());

    match handler.get_media_for_bucket(path, bucket).await {
        Ok(cached_media) => media_response(cached_media.data, cached_media.content_type),
        Err(e) => error_response(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_bucket_returns_none_for_no_bucket() {
        let result = validate_bucket(None);
        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn validate_bucket_accepts_valid_lowercase_bucket_name() {
        let result = validate_bucket(Some("hi29831"));
        assert!(matches!(result, Ok(Some(ref s)) if s == "hi29831"));
    }

    #[test]
    fn validate_bucket_rejects_uppercase_name() {
        let result = validate_bucket(Some("BadName"));
        assert!(result.is_err());
    }

    #[test]
    fn validate_bucket_rejects_short_name() {
        let result = validate_bucket(Some("ab"));
        assert!(result.is_err());
    }

    #[test]
    fn validate_bucket_rejects_name_with_dots() {
        let result = validate_bucket(Some("my.bucket"));
        assert!(result.is_err());
    }

    #[test]
    fn resolve_storage_returns_default_clone_when_bucket_is_none() {
        let default = SupabaseStorage::new("http://example.com", "anon", None, "media");
        let storage = resolve_storage(&default, None);
        assert_eq!(storage.bucket, "media");
        assert_eq!(storage.supabase_url, "http://example.com");
    }

    #[test]
    fn resolve_storage_replaces_bucket_when_bucket_is_some() {
        let default = SupabaseStorage::new("http://example.com", "anon", None, "media");
        let storage = resolve_storage(&default, Some("hi29831".to_string()));
        assert_eq!(storage.bucket, "hi29831");
        assert_eq!(storage.supabase_url, "http://example.com");
    }
}
