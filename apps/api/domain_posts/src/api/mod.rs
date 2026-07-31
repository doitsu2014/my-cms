//! HTTP adapters for the post domain.
//!
//! These are thin Axum handlers that extract state, call the corresponding
//! application-layer command handler in `domain_posts::handlers::post::*`,
//! and return the existing `ApiResponseWith` / `ApiResponseError` envelope.
//!
//! `routes(ctx)` is the single entry point used by
//! `DomainPostService::register_routes` to build the post-domain's
//! `Vec<RouteRegistration>`.

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use domain_interface::{DomainContext, Mount, RouteRegistration};

pub mod post;

/// Build the post domain's public router — currently empty.
///
/// Task 6 wires the post-domain `/health` endpoint here.
fn public_router(_ctx: &DomainContext) -> Router<DomainContext> {
    Router::new()
}

/// Build the post domain's protected router — `/posts/**`,
/// `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`,
/// `/posts/{post_id}/translate/jobs`, `/posts/{post_id}/translate/jobs/{job_id}`.
fn protected_router(_ctx: &DomainContext) -> Router<DomainContext> {
    Router::new()
        .route(
            "/posts",
            get(post::read::read_handler::api_get_posts_with_filtering)
                .post(post::create::create_handler::api_create_post)
                .put(post::modify::modify_handler::api_modify_post)
                .delete(post::delete::delete_handler::api_delete_posts),
        )
        .route(
            "/posts/{post_id}",
            get(post::read::read_handler::api_get_post),
        )
        .route(
            "/posts/{post_id}/translate",
            post(post::translate::translate_handler::api_translate_post),
        )
        .route(
            "/posts/{post_id}/translate/background",
            post(post::translate::translate_handler::api_translate_post_background),
        )
        .route(
            "/posts/{post_id}/translate/jobs/{job_id}",
            get(post::translate::job_handler::api_get_job_status),
        )
        .route(
            "/posts/{post_id}/translate/jobs",
            get(post::translate::job_handler::api_get_active_jobs),
        )
}

/// Build the post domain's administrator router — currently empty.
///
/// The legacy `/administrator/database/migration` route is owned by the
/// legacy `cms` crate during the transition and migrates to `domain_posts`
/// once the migration orchestrator is wired in Task 8.
fn administrator_router(_ctx: &DomainContext) -> Router<DomainContext> {
    Router::new().layer(DefaultBodyLimit::max(
        std::env::var("MAX_BODY_LENGTH")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10 * 1024 * 1024),
    ))
}

/// Build the post domain's `RouteRegistration`s — public, protected,
/// administrator. The routers are bare (no auth/CORS/cookie/Otel layers).
pub fn routes(ctx: &DomainContext) -> Vec<RouteRegistration> {
    vec![
        RouteRegistration {
            mount: Mount::Public,
            router: public_router(ctx),
            prefix: "/",
        },
        RouteRegistration {
            mount: Mount::Protected,
            router: protected_router(ctx),
            prefix: "/posts",
        },
        RouteRegistration {
            mount: Mount::Administrator,
            router: administrator_router(ctx),
            prefix: "/posts-admin",
        },
    ]
}
