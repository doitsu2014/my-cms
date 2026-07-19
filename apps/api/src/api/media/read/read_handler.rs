use application_core::commands::media::bucket::access::access_handler::{
    BucketAccessPolicy, BucketAccessPolicyTrait,
};
use application_core::commands::media::bucket::dto::{bucket_name_error, is_valid_bucket_name};
use application_core::commands::media::read::read_handler::{
    ReadMediaHandler, ReadMediaHandlerTrait, ResizeParams,
};
use application_core::commands::media::{MediaConfig, SupabaseStorage};
use application_core::common::app_error::AppError;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::{
    env,
    sync::{Arc, LazyLock},
};
use tracing::instrument;

use crate::{common::supabase_auth::SupabaseClaims, AppState};

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

fn media_response(data: Vec<u8>, content_type: String) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap());
    headers.insert(
        header::CACHE_CONTROL,
        "public, max-age=31536000, immutable".parse().unwrap(),
    );

    (StatusCode::OK, headers, Body::from(data)).into_response()
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
    let Ok(token_data) = decode::<SupabaseClaims>(token, &verifier.decoding_key, &validation)
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

pub(crate) async fn enforce_bucket_visibility_gate(
    storage: SupabaseStorage,
    cache: Arc<moka::future::Cache<String, bool>>,
    bucket_name: &str,
    headers: &HeaderMap,
) -> Result<(), AppError> {
    let policy = BucketAccessPolicy { storage, cache };
    policy
        .ensure_public_or_admin(bucket_name, is_admin_jwt_present(headers))
        .await
}

#[instrument(skip(state, headers))]
pub async fn api_get_media_image(
    state: State<AppState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    Query(params): Query<ImageQueryParams>,
) -> Response {
    let bucket = match validate_bucket(params.bucket.as_deref()) {
        Ok(b) => b,
        Err(resp) => return *resp,
    };

    let storage = state.media_config.storage.clone();
    let media_config = Arc::new(MediaConfig {
        storage,
        bucket: bucket.unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });

    if let Err(e) = enforce_bucket_visibility_gate(
        media_config.storage.clone(),
        state.bucket_visibility_cache.clone(),
        media_config.bucket.as_str(),
        &headers,
    )
    .await
    {
        return error_response(e);
    }

    let handler = ReadMediaHandler::new(media_config, state.media_cache.clone());
    let resize_params = ResizeParams::new(params.w, params.h);

    match handler.get_rendered_image(path, resize_params).await {
        Ok(cached_media) => media_response(cached_media.data, cached_media.content_type),
        Err(e) => error_response(e),
    }
}

#[instrument(skip(state, headers))]
pub async fn api_get_media(
    state: State<AppState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    Query(params): Query<ReadQueryParams>,
) -> Response {
    let bucket = match validate_bucket(params.bucket.as_deref()) {
        Ok(b) => b,
        Err(resp) => return *resp,
    };

    let storage = state.media_config.storage.clone();
    let media_config = Arc::new(MediaConfig {
        storage,
        bucket: bucket.unwrap_or_else(|| state.media_config.bucket.clone()),
        media_base_url: state.media_config.media_base_url.clone(),
    });

    if let Err(e) = enforce_bucket_visibility_gate(
        media_config.storage.clone(),
        state.bucket_visibility_cache.clone(),
        media_config.bucket.as_str(),
        &headers,
    )
    .await
    {
        return error_response(e);
    }

    let handler = ReadMediaHandler::new(media_config, state.media_cache.clone());

    match handler.get_media_for_bucket(path).await {
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

    use axum::http::HeaderValue;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio::sync::Mutex;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    const TEST_JWT_SECRET: &str = "test-secret-key-at-least-32-characters-long!!";
    const TEST_AUTHORIZATION_AUDIENCE: &str = "authenticated";
    static TEST_AUTH_LOCK: Mutex<()> = Mutex::const_new(());

    fn configure_test_auth() {
        std::env::set_var("SUPABASE_JWT_SECRET", TEST_JWT_SECRET);
        std::env::set_var("AUTHORIZATION_AUDIENCE", TEST_AUTHORIZATION_AUDIENCE);
    }

    fn admin_headers() -> HeaderMap {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_secs() as i64;
        let claims = json!({
            "sub": "admin-user",
            "aud": TEST_AUTHORIZATION_AUDIENCE,
            "role": "authenticated",
            "exp": now + 3600,
            "iat": now,
            "app_metadata": {
                "roles": ["my-headless-cms-administrator"]
            }
        });
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(TEST_JWT_SECRET.as_bytes()),
        )
        .expect("test JWT should encode");
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .expect("test authorization header should be valid"),
        );
        headers
    }

    fn make_storage(base_url: &str) -> SupabaseStorage {
        SupabaseStorage {
            supabase_url: base_url.to_string(),
            anon_key: "anon-test-key".to_string(),
            service_role_key: Some("service-role-test-key".to_string()),
            client: reqwest::Client::new(),
        }
    }

    fn bucket_body(name: &str, public: bool) -> serde_json::Value {
        json!({
            "id": name,
            "name": name,
            "public": public,
            "file_size_limit": null,
            "allowed_mime_types": null,
            "owner": null,
            "type": "STANDARD",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-02T00:00:00Z"
        })
    }

    #[async_std::test]
    async fn enforce_bucket_visibility_gate_returns_404_when_bucket_is_private_and_no_admin_jwt() {
        let _guard = TEST_AUTH_LOCK.lock().await;
        configure_test_auth();
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket/private-docs"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(bucket_body("private-docs", false)),
            )
            .mount(&server)
            .await;

        let result = enforce_bucket_visibility_gate(
            make_storage(&server.uri()),
            Arc::new(application_core::commands::media::bucket::access::access_cache::create_bucket_visibility_cache()),
            "private-docs",
            &HeaderMap::new(),
        )
        .await;

        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn enforce_bucket_visibility_gate_returns_ok_when_bucket_is_private_and_admin_jwt() {
        let _guard = TEST_AUTH_LOCK.lock().await;
        configure_test_auth();
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket/private-docs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let result = enforce_bucket_visibility_gate(
            make_storage(&server.uri()),
            Arc::new(application_core::commands::media::bucket::access::access_cache::create_bucket_visibility_cache()),
            "private-docs",
            &admin_headers(),
        )
        .await;

        assert!(matches!(result, Ok(())));
    }

    #[async_std::test]
    async fn enforce_bucket_visibility_gate_serves_public_bucket_anonymous() {
        let _guard = TEST_AUTH_LOCK.lock().await;
        configure_test_auth();
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket/media"))
            .respond_with(ResponseTemplate::new(200).set_body_json(bucket_body("media", true)))
            .mount(&server)
            .await;

        let result = enforce_bucket_visibility_gate(
            make_storage(&server.uri()),
            Arc::new(application_core::commands::media::bucket::access::access_cache::create_bucket_visibility_cache()),
            "media",
            &HeaderMap::new(),
        )
        .await;

        assert!(matches!(result, Ok(())));
    }
}
