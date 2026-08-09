//! HTTP adapters for the post domain.
//!
//! These are thin Axum handlers that extract state, call the corresponding
//! application-layer command handler in `domain_posts::handlers::*`, and
//! return the existing `ApiResponseWith` / `ApiResponseError` envelope.
//!
//! `routes(ctx)` is the single entry point used by
//! `DomainPostService::register_routes` to build the post-domain's
//! `Vec<RouteRegistration>`. The post domain owns the GraphQL HTTP surface
//! — see `post::graphql` for the playground handlers and `routes()` for the
//! `Mount::Public` / `Mount::Protected` registrations of
//! `/posts/graphql/{immutable,mutable}`. See
//! `openspec/changes/merge-graphql-into-posts-domain/design.md` Decision 1.

use async_graphql_axum::GraphQL;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use domain_interface::{DomainContext, Mount, RouteRegistration};

pub mod ai;
pub mod category;
pub mod post;

/// Build the post domain's public router — `/posts/graphql/immutable`
/// playground (GET) and POST endpoint. No auth layer (immutable mount is
/// public). The `Arc<Schema>` is read from the shared `DomainContext` so
/// the schema is built once at startup, not per-request.
fn public_router(ctx: &DomainContext) -> Router<DomainContext> {
    Router::new().route(
        "/posts/graphql/immutable",
        get(post::graphql::playground_immutable)
            .post_service(GraphQL::new(ctx.graphql_immutable.as_ref().clone())),
    )
}

/// Build the post domain's protected router — `/posts/**`,
/// `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`,
/// `/posts/{post_id}/translate/jobs`, `/posts/{post_id}/translate/jobs/{job_id}`.
/// The auth layer is applied at the gateway or standalone-binary boundary
/// with the role vector
/// `["my-headless-cms-writer", "my-headless-cms-administrator"]`.
///
/// Note: the GraphQL mutable mount is NOT bundled into this router — it is
/// its own `RouteRegistration` (see `graphql_mutable_router`) so the
/// `RouteRegistration::prefix` is `/posts/graphql` (matching the design +
/// spec invariant `posts-graphql-mount` → "Post domain registers GraphQL
/// routes via `DomainService`").
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

/// Build the GraphQL mutable-mount router — `GET/POST /posts/graphql/mutable`
/// (playground GET + GraphQL POST endpoint). Registered as a separate
/// `Mount::Protected` `RouteRegistration` with `prefix: "/posts/graphql"`
/// so the registration metadata mirrors the design + spec invariant.
fn graphql_mutable_router(ctx: &DomainContext) -> Router<DomainContext> {
    Router::new().route(
        "/posts/graphql/mutable",
        get(post::graphql::playground_mutable)
            .post_service(GraphQL::new(ctx.graphql_mutable.as_ref().clone())),
    )
}

/// Build the category router — `GET/POST/PUT/DELETE /categories` and
/// `GET /categories/{category_id}`. Lives under `Mount::Protected` so the
/// gateway applies the same Supabase auth layer as the post CRUD endpoints.
fn category_router(_ctx: &DomainContext) -> Router<DomainContext> {
    Router::new()
        .route(
            "/categories",
            get(category::read::read_handler::api_get_categories_with_filtering)
                .post(category::create::create_handler::api_create_category_with_tags)
                .put(category::modify::modify_handler::api_modify_category)
                .delete(category::delete::delete_handler::api_delete_categories),
        )
        .route(
            "/categories/{category_id}",
            get(category::read::read_handler::api_get_category),
        )
}

/// Build the AI router — `GET /ai/models`. Returns the curated OpenAI model
/// catalogue. Lives under `Mount::Protected`.
fn ai_router(_ctx: &DomainContext) -> Router<DomainContext> {
    Router::new().route(
        "/ai/models",
        get(ai::models::models_handler::api_get_openai_models),
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
///
/// The GraphQL endpoints live under the post domain: the immutable mount
/// (`/posts/graphql/immutable`) is registered as `Mount::Public`; the
/// mutable mount (`/posts/graphql/mutable`) is registered as
/// `Mount::Protected`. The gateway and standalone binary apply the
/// `["my-headless-cms-writer", "my-headless-cms-administrator"]` role gate
/// to the protected registrations so writers and administrators can both
/// issue mutations via GraphQL.
pub fn routes(ctx: &DomainContext) -> Vec<RouteRegistration> {
    vec![
        RouteRegistration {
            mount: Mount::Public,
            router: public_router(ctx),
            prefix: "/posts/graphql",
        },
        RouteRegistration {
            mount: Mount::Protected,
            router: graphql_mutable_router(ctx),
            prefix: "/posts/graphql",
        },
        RouteRegistration {
            mount: Mount::Protected,
            router: protected_router(ctx),
            prefix: "/posts",
        },
        RouteRegistration {
            mount: Mount::Protected,
            router: category_router(ctx),
            prefix: "/categories",
        },
        RouteRegistration {
            mount: Mount::Protected,
            router: ai_router(ctx),
            prefix: "/ai",
        },
        RouteRegistration {
            mount: Mount::Administrator,
            router: administrator_router(ctx),
            prefix: "/posts-admin",
        },
    ]
}
