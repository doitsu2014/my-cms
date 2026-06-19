use axum::{
    extract::Request,
    http::StatusCode,
    response::IntoResponse,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::sync::Arc;
use tower::{Layer, Service};

const JWKS_URI_PATH: &str = "/auth/v1/.well-known/jwks.json";

#[derive(Clone)]
pub struct SupabaseAuthConfig {
    pub supabase_url: String,
    pub jwt_secret: String,
    pub expected_audience: String,
    pub required_roles: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SupabaseClaims {
    pub sub: String,
    pub email: Option<String>,
    pub aud: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub app_metadata: Option<serde_json::Value>,
    pub user_metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct SupabaseToken {
    pub claims: SupabaseClaims,
}

impl SupabaseToken {
    pub fn user_id(&self) -> &str {
        &self.claims.sub
    }

    pub fn email(&self) -> Option<&str> {
        self.claims.email.as_deref()
    }

    pub fn role(&self) -> &str {
        &self.claims.role
    }
}

#[derive(Clone)]
pub struct SupabaseAuthLayer {
    config: Arc<SupabaseAuthConfig>,
}

impl SupabaseAuthLayer {
    pub fn new(config: SupabaseAuthConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}

impl<S> Layer<S> for SupabaseAuthLayer {
    type Service = SupabaseAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SupabaseAuthMiddleware {
            inner,
            config: self.config.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SupabaseAuthMiddleware<S> {
    inner: S,
    config: Arc<SupabaseAuthConfig>,
}

impl<S, B> Service<Request<B>> for SupabaseAuthMiddleware<S>
where
    S: Service<Request<B>, Response = axum::response::Response> + Clone + Send + 'static,
    S::Future: Send,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let config = self.config.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "));

            let token_str = match auth_header {
                Some(t) => t.to_string(),
                None => {
                    return Ok((
                        StatusCode::UNAUTHORIZED,
                        r#"{"error":"Missing Authorization header"}"#,
                    )
                        .into_response());
                }
            };

            let claims = match validate_supabase_token(&token_str, &config).await {
                Ok(c) => c,
                Err(e) => {
                    return Ok((
                        StatusCode::UNAUTHORIZED,
                        format!(r#"{{"error":"{}"}}"#, e),
                    )
                        .into_response());
                }
            };

            if !config.required_roles.is_empty() {
                let has_role = claims
                    .app_metadata
                    .as_ref()
                    .and_then(|meta| meta.get("roles"))
                    .and_then(|roles| roles.as_array())
                    .map(|roles| {
                        roles.iter().any(|r| {
                            r.as_str()
                                .map(|s| config.required_roles.contains(&s.to_string()))
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false);

                if !has_role {
                    return Ok((
                        StatusCode::FORBIDDEN,
                        r#"{"error":"Insufficient permissions"}"#,
                    ).into_response());
                }
            }

            let mut req = req;
            req.extensions_mut().insert(SupabaseToken { claims });

            inner.call(req).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request, response::Response, routing::get, Router};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tower::ServiceExt;

    const TEST_JWT_SECRET: &str = "test-secret-key-at-least-32-characters-long!!";
    const TEST_SUPABASE_URL: &str = "http://localhost:8001";

    fn make_token(app_metadata: serde_json::Value, exp_offset_secs: i64) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let claims = json!({
            "sub": "test-user-id",
            "email": "test@example.com",
            "aud": "authenticated",
            "role": "authenticated",
            "exp": now + exp_offset_secs,
            "iat": now,
            "app_metadata": app_metadata,
            "user_metadata": {},
        });

        let header = Header::new(Algorithm::HS256);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(TEST_JWT_SECRET.as_bytes()),
        )
        .expect("should encode test token")
    }

    fn valid_token_with_role(role: &str) -> String {
        make_token(
            json!({"roles": [role]}),
            3600,
        )
    }

    fn valid_token_with_roles(roles: &[&str]) -> String {
        make_token(json!({"roles": roles}), 3600)
    }

    fn valid_token_without_roles() -> String {
        make_token(json!({}), 3600)
    }

    fn expired_token() -> String {
        make_token(json!({"roles": ["my-headless-cms-writer"]}), -3600)
    }

    fn test_app() -> Router {
        let config = SupabaseAuthConfig {
            supabase_url: TEST_SUPABASE_URL.to_string(),
            jwt_secret: TEST_JWT_SECRET.to_string(),
            expected_audience: "authenticated".to_string(),
            required_roles: vec!["my-headless-cms-writer".to_string()],
        };

        Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(SupabaseAuthLayer::new(config))
    }

    async fn assert_status(response: Response<Body>, expected: StatusCode) {
        assert_eq!(
            response.status(),
            expected,
            "expected {} got {}: body={:?}",
            expected,
            response.status(),
            axum::body::to_bytes(response.into_body(), 1024).await.ok()
        );
    }

    #[tokio::test]
    async fn valid_token_passes_auth() {
        let token = valid_token_with_role("my-headless-cms-writer");
        let app = test_app();

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn missing_auth_header_returns_401() {
        let app = test_app();

        let response = app
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_status(response, StatusCode::UNAUTHORIZED).await;
    }

    #[tokio::test]
    async fn wrong_role_returns_403() {
        let token = valid_token_with_role("my-headless-cms-administrator");
        let app = test_app();

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::FORBIDDEN).await;
    }

    #[tokio::test]
    async fn no_roles_returns_403() {
        let token = valid_token_without_roles();
        let app = test_app();

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::FORBIDDEN).await;
    }

    #[tokio::test]
    async fn expired_token_returns_401() {
        let token = expired_token();
        let app = test_app();

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::UNAUTHORIZED).await;
    }

    #[tokio::test]
    async fn empty_required_roles_allows_any_authenticated_user() {
        let config = SupabaseAuthConfig {
            supabase_url: TEST_SUPABASE_URL.to_string(),
            jwt_secret: TEST_JWT_SECRET.to_string(),
            expected_audience: "authenticated".to_string(),
            required_roles: vec![],
        };

        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(SupabaseAuthLayer::new(config));

        let token = valid_token_without_roles();

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::OK).await;
    }

    fn app_with_required_roles(required_roles: Vec<String>) -> Router {
        let config = SupabaseAuthConfig {
            supabase_url: TEST_SUPABASE_URL.to_string(),
            jwt_secret: TEST_JWT_SECRET.to_string(),
            expected_audience: "authenticated".to_string(),
            required_roles,
        };

        Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(SupabaseAuthLayer::new(config))
    }

    #[tokio::test]
    async fn or_semantics_user_with_first_required_role_passes() {
        let app = app_with_required_roles(vec![
            "my-headless-cms-writer".to_string(),
            "my-headless-cms-administrator".to_string(),
        ]);
        let token = valid_token_with_role("my-headless-cms-writer");

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn or_semantics_user_with_second_required_role_passes() {
        let app = app_with_required_roles(vec![
            "my-headless-cms-writer".to_string(),
            "my-headless-cms-administrator".to_string(),
        ]);
        let token = valid_token_with_role("my-headless-cms-administrator");

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn or_semantics_user_with_both_required_roles_passes() {
        let app = app_with_required_roles(vec![
            "my-headless-cms-writer".to_string(),
            "my-headless-cms-administrator".to_string(),
        ]);
        let token =
            valid_token_with_roles(&["my-headless-cms-writer", "my-headless-cms-administrator"]);

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn or_semantics_user_with_unrelated_role_fails() {
        let app = app_with_required_roles(vec![
            "my-headless-cms-writer".to_string(),
            "my-headless-cms-administrator".to_string(),
        ]);
        let token = valid_token_with_role("my-headless-cms-editor");

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::FORBIDDEN).await;
    }

    #[tokio::test]
    async fn or_semantics_user_with_no_roles_fails() {
        let app = app_with_required_roles(vec![
            "my-headless-cms-writer".to_string(),
            "my-headless-cms-administrator".to_string(),
        ]);
        let token = valid_token_without_roles();

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::FORBIDDEN).await;
    }

    #[tokio::test]
    async fn or_semantics_extra_user_role_does_not_block_when_one_matches() {
        let app = app_with_required_roles(vec![
            "my-headless-cms-writer".to_string(),
            "my-headless-cms-administrator".to_string(),
        ]);
        let token = valid_token_with_roles(&["my-headless-cms-writer", "my-headless-cms-editor"]);

        let response = app
            .oneshot(
                Request::get("/")
                    .header("Authorization", format!("Bearer {}", token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_status(response, StatusCode::OK).await;
    }
}

    async fn validate_supabase_token(
    token: &str,
    config: &SupabaseAuthConfig,
) -> Result<SupabaseClaims, String> {
    let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[&config.expected_audience]);

    match decode::<SupabaseClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => return Ok(token_data.claims),
        Err(_) => {
            let header = decode_header(token).map_err(|e| format!("Invalid token header: {}", e))?;

            if header.alg == Algorithm::HS256 {
                return Err("Invalid JWT signature".to_string());
            }

            let jwks_url = format!("{}{}", config.supabase_url, JWKS_URI_PATH);
            let jwks_response = reqwest::get(&jwks_url)
                .await
                .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

            let jwks: jsonwebtoken::jwk::JwkSet = jwks_response
                .json()
                .await
                .map_err(|e| format!("Failed to parse JWKS: {}", e))?;

            let kid = header.kid.ok_or("Token missing kid header")?;
            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| format!("Key not found for kid: {}", kid))?;

            let decoding_key = DecodingKey::from_jwk(&jwk)
                .map_err(|e| format!("Failed to create decoding key from JWK: {}", e))?;

            let mut validation = Validation::new(header.alg);
            validation.set_audience(&[&config.expected_audience]);

            let token_data = decode::<SupabaseClaims>(token, &decoding_key, &validation)
                .map_err(|e| format!("JWT validation failed: {}", e))?;

            Ok(token_data.claims)
        }
    }
}
