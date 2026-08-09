//! Public media read adapters — thin Axum wrappers around the
//! `ReadMediaHandler` application command. Path extraction, resize
//! parameters, bucket-name validation, and private-bucket obscuring all
//! happen here at the adapter boundary; storage and policy decisions are
//! delegated to `domain_media::handlers::read::read_handler` and
//! `domain_media::handlers::bucket::access`.

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::{env, sync::LazyLock};
use tracing::instrument;

use crate::{
    api::state::MediaApiState,
    domain::error::AppError,
    handlers::{
        bucket::access::access_handler::{BucketAccessPolicy, BucketAccessPolicyTrait},
        bucket::dto::{bucket_name_error, is_valid_bucket_name},
        read::read_handler::{ReadMediaHandler, ReadMediaHandlerTrait, ResizeParams},
    },
};

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

/// Internal `Claims` view used for optional admin-JWT inspection on the
/// public read endpoints — kept here (not exported) because the public
/// read paths do not require authenticated actors; we only need to check
/// the admin role for private-bucket visibility.
#[derive(Debug, serde::Deserialize)]
struct AdminRoleClaims {
    app_metadata: Option<serde_json::Value>,
}

struct AdminJwtVerifier {
    decoding_key: DecodingKey,
    audience: String,
}

static ADMIN_JWT_VERIFIER: LazyLock<Option<AdminJwtVerifier>> = LazyLock::new(|| {
    let secret = env::var("SUPABASE_JWT_SECRET").ok()?;
    let audience = env::var("AUTHORIZATION_AUDIENCE").ok()?;
    Some(AdminJwtVerifier {
        decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        audience,
    })
});

fn is_admin_jwt_present(headers: &HeaderMap) -> bool {
    let Some(token) = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
    else {
        return false;
    };
    let Some(verifier) = ADMIN_JWT_VERIFIER.as_ref() else {
        return false;
    };

    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[verifier.audience.as_str()]);
    let Ok(token_data) = decode::<AdminRoleClaims>(token, &verifier.decoding_key, &validation)
    else {
        return false;
    };

    token_data
        .claims
        .app_metadata
        .as_ref()
        .and_then(|metadata| metadata.get("roles"))
        .and_then(|roles| roles.as_array())
        .map(|roles| {
            roles.iter().any(|role| {
                role.as_str()
                    .map(|role| role == "my-headless-cms-administrator")
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

fn validate_bucket(name: Option<&str>) -> Result<Option<String>, Response> {
    let bucket = match name {
        Some(raw) => raw.to_string(),
        None => return Ok(None),
    };
    if !is_valid_bucket_name(&bucket) {
        let reason =
            bucket_name_error(&bucket).unwrap_or_else(|| "invalid bucket name".to_string());
        return Err((
            StatusCode::BAD_REQUEST,
            [(header::CONTENT_TYPE, "application/json")],
            format!(r#"{{"error": "invalid bucket: {}"}}"#, reason),
        )
            .into_response());
    }
    Ok(Some(bucket))
}

fn error_response(e: &AppError) -> Response {
    let status = match e {
        AppError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::BAD_GATEWAY,
    };
    (
        status,
        [(header::CONTENT_TYPE, "application/json")],
        format!(r#"{{"error": "{}"}}"#, e),
    )
        .into_response()
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

#[instrument(skip(state, headers))]
pub async fn api_get_media_image(
    State(state): State<MediaApiState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    Query(params): Query<ImageQueryParams>,
) -> Response {
    let bucket = match validate_bucket(params.bucket.as_deref()) {
        Ok(b) => b,
        Err(resp) => return resp,
    };

    let media_config = std::sync::Arc::new(crate::handlers::MediaConfig {
        storage: state.media_config.storage.clone(),
        bucket: bucket.unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });

    let policy = BucketAccessPolicy {
        storage: media_config.storage.clone(),
        cache: state.bucket_visibility_cache.clone(),
    };
    if let Err(e) = policy
        .ensure_public_or_admin(media_config.bucket.as_str(), is_admin_jwt_present(&headers))
        .await
    {
        return error_response(&e);
    }

    let handler = ReadMediaHandler::new(media_config, state.media_cache.clone());
    let resize_params = ResizeParams::new(params.w, params.h);

    match handler.get_rendered_image(path, resize_params).await {
        Ok(cached_media) => media_response(cached_media.data, cached_media.content_type),
        Err(e) => error_response(&e),
    }
}

#[instrument(skip(state, headers))]
pub async fn api_get_media(
    State(state): State<MediaApiState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    Query(params): Query<ReadQueryParams>,
) -> Response {
    let bucket = match validate_bucket(params.bucket.as_deref()) {
        Ok(b) => b,
        Err(resp) => return resp,
    };

    let media_config = std::sync::Arc::new(crate::handlers::MediaConfig {
        storage: state.media_config.storage.clone(),
        bucket: bucket.unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });

    let policy = BucketAccessPolicy {
        storage: media_config.storage.clone(),
        cache: state.bucket_visibility_cache.clone(),
    };
    if let Err(e) = policy
        .ensure_public_or_admin(media_config.bucket.as_str(), is_admin_jwt_present(&headers))
        .await
    {
        return error_response(&e);
    }

    let handler = ReadMediaHandler::new(media_config, state.media_cache.clone());

    match handler.get_media_for_bucket(path).await {
        Ok(cached_media) => media_response(cached_media.data, cached_media.content_type),
        Err(e) => error_response(&e),
    }
}
