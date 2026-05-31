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
