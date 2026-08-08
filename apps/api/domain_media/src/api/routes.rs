//! `routes` — single entry point that builds the media domain's
//! `Vec<RouteRegistration>` for the gateway composition root.
//!
//! Mirrors the route inventory in `apps/api/src/bin/legacy_bootstrap.rs:86-235`:
//!
//! - `Mount::Public`: `GET /media/images/{*path}`, `GET /media/{*path}`.
//! - `Mount::Protected`: `GET /media` (list), `POST /media` (create),
//!   `DELETE /media` (batch), `GET /media/info/{*path}`,
//!   `DELETE /media/delete/{*path}`.
//! - `Mount::Administrator`: `GET /media/buckets`, `POST /media/buckets`,
//!   `GET /media/buckets/{name}`, `PUT /media/buckets/{name}`,
//!   `DELETE /media/buckets/{name}`, `POST /media/buckets/{name}/empty`.

use axum::{
    routing::{delete, get, post},
    Router,
};
use domain_interface::{DomainContext, Mount, RouteRegistration};

use crate::api::{bucket, media, state::MediaApiState};

/// Build the media domain's public router — image delivery and general media
/// delivery. Public surface, no auth layer (the gateway applies the bare
/// router without auth for `Mount::Public`).
fn public_router(state: MediaApiState) -> Router<DomainContext> {
    Router::new()
        .route(
            "/media/images/{*path}",
            get(media::read::read_handler::api_get_media_image),
        )
        .route(
            "/media/{*path}",
            get(media::read::read_handler::api_get_media),
        )
        .with_state(state)
}

/// Build the media domain's protected router — list, create, batch delete,
/// metadata, and single-path delete. Auth and body-limit layers are applied
/// at the gateway boundary.
fn protected_router(state: MediaApiState) -> Router<DomainContext> {
    Router::new()
        .route(
            "/media",
            get(media::list::list_handler::api_list_media)
                .post(media::create::create_handler::api_create_media)
                .delete(media::delete::delete_handler::api_delete_media_batch),
        )
        .route(
            "/media/info/{*path}",
            get(media::read::metadata_handler::api_get_media_metadata),
        )
        .route(
            "/media/delete/{*path}",
            delete(media::delete::delete_handler::api_delete_media),
        )
        .with_state(state)
}

/// Build the media domain's administrator router — bucket CRUD and emptying.
fn administrator_router(state: MediaApiState) -> Router<DomainContext> {
    Router::new()
        .route(
            "/media/buckets",
            get(bucket::list::list_handler::api_list_buckets)
                .post(bucket::create::create_handler::api_create_bucket),
        )
        .route(
            "/media/buckets/{name}",
            get(bucket::get::get_handler::api_get_bucket)
                .put(bucket::update::update_handler::api_update_bucket)
                .delete(bucket::delete::delete_handler::api_delete_bucket),
        )
        .route(
            "/media/buckets/{name}/empty",
            post(bucket::empty::empty_handler::api_empty_bucket),
        )
        .with_state(state)
}

/// Build the media domain's `RouteRegistration`s. The router builders take
/// ownership of a `MediaApiState` clone (which clones the underlying `Arc`s)
/// so the returned `RouteRegistration`s are `Router<DomainContext>` as the
/// gateway expects.
pub fn routes(state: MediaApiState) -> Vec<RouteRegistration> {
    vec![
        RouteRegistration {
            mount: Mount::Public,
            router: public_router(state.clone()),
            prefix: "/media",
        },
        RouteRegistration {
            mount: Mount::Protected,
            router: protected_router(state.clone()),
            prefix: "/media",
        },
        RouteRegistration {
            mount: Mount::Administrator,
            router: administrator_router(state),
            prefix: "/media/buckets",
        },
    ]
}
