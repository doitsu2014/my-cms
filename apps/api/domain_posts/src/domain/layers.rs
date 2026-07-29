//! Layer factory functions used by both the gateway-composed and standalone
//! modes. These mirror the legacy `apps/api/src/bin/my-cms-api.rs` lines
//! 188–205 (auth + body limit + cookie + Otel) and lines 320–331 (CORS).

use axum::extract::DefaultBodyLimit;
use hyper::Method;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

/// CORS layer used by every mode. Mirrors the legacy `construct_cors_layer`.
pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_origin(Any)
        .allow_headers(Any)
}

/// Cookie manager layer applied to protected routes.
pub fn cookie_layer() -> CookieManagerLayer {
    CookieManagerLayer::new()
}

/// Body limit layer applied to protected routes. Defaults to 10 MiB.
pub fn body_limit_layer() -> DefaultBodyLimit {
    let limit: usize = std::env::var("MAX_BODY_LENGTH")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10 * 1024 * 1024);
    DefaultBodyLimit::max(limit)
}

/// OpenTelemetry layers for Axum. Mirrors `OtelInResponseLayer` +
/// `OtelAxumLayer::default()`.
pub fn otel_layers() -> (
    axum_tracing_opentelemetry::middleware::OtelInResponseLayer,
    axum_tracing_opentelemetry::middleware::OtelAxumLayer,
) {
    (
        axum_tracing_opentelemetry::middleware::OtelInResponseLayer,
        axum_tracing_opentelemetry::middleware::OtelAxumLayer::default(),
    )
}