use crate::{
    domain::response::{ApiResponseError, ApiResponseWith, AxumResponse},
    handlers::head_assets::{CreateHeadAssetRequest, HeadAssetHandler, UpdateHeadAssetRequest},
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use domain_interface::{AuthenticatedActor, DomainContext, Mount, RouteRegistration};
use uuid::Uuid;

async fn list(
    State(ctx): State<DomainContext>,
    Extension(_actor): Extension<AuthenticatedActor>,
) -> impl IntoResponse {
    match (HeadAssetHandler { db: ctx.conn }).list().await {
        Ok(data) => ApiResponseWith::new(data).to_axum_response(),
        Err(e) => ApiResponseError::from_error(e).to_axum_response(),
    }
}
async fn get_one(
    State(ctx): State<DomainContext>,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match (HeadAssetHandler { db: ctx.conn }).get(id).await {
        Ok(data) => ApiResponseWith::new(data).to_axum_response(),
        Err(e) => ApiResponseError::from_error(e).to_axum_response(),
    }
}
async fn create(
    State(ctx): State<DomainContext>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<CreateHeadAssetRequest>,
) -> impl IntoResponse {
    match (HeadAssetHandler { db: ctx.conn })
        .create(body, &actor)
        .await
    {
        Ok(data) => ApiResponseWith::new(data).to_status(StatusCode::CREATED),
        Err(e) => ApiResponseError::from_error(e).to_axum_response(),
    }
}
async fn update(
    State(ctx): State<DomainContext>,
    Extension(actor): Extension<AuthenticatedActor>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateHeadAssetRequest>,
) -> impl IntoResponse {
    match (HeadAssetHandler { db: ctx.conn })
        .update(id, body, &actor)
        .await
    {
        Ok(data) => ApiResponseWith::new(data).to_axum_response(),
        Err(e) => ApiResponseError::from_error(e).to_axum_response(),
    }
}
async fn delete_one(
    State(ctx): State<DomainContext>,
    Extension(actor): Extension<AuthenticatedActor>,
    Path(id): Path<Uuid>,
) -> Response<Body> {
    match (HeadAssetHandler { db: ctx.conn }).delete(id, &actor).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => ApiResponseError::from_error(e)
            .to_axum_response()
            .map(Body::from),
    }
}
async fn public(State(ctx): State<DomainContext>) -> impl IntoResponse {
    match (HeadAssetHandler { db: ctx.conn }).public_list().await {
        Ok(data) => ApiResponseWith::new(data).to_axum_response(),
        Err(e) => ApiResponseError::from_error(e).to_axum_response(),
    }
}

fn administrator_router() -> Router<DomainContext> {
    Router::new()
        .route("/seo/head-assets", get(list).post(create))
        .route(
            "/seo/head-assets/{id}",
            get(get_one).put(update).delete(delete_one),
        )
}
fn public_router() -> Router<DomainContext> {
    Router::new().route("/seo/head-assets/ducth-dev", get(public))
}
pub fn routes(_ctx: &DomainContext) -> Vec<RouteRegistration> {
    vec![
        RouteRegistration {
            mount: Mount::Administrator,
            router: administrator_router(),
            prefix: "/seo/head-assets",
        },
        RouteRegistration {
            mount: Mount::Public,
            router: public_router(),
            prefix: "/seo/head-assets/ducth-dev",
        },
    ]
}
