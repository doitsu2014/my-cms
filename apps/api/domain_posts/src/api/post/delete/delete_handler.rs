//! `DELETE /posts` — bulk delete posts by id.

use axum::{extract::State, response::IntoResponse, Extension, Json};
use sea_orm::sqlx::types::Uuid;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::handlers::post::delete::delete_handler::{PostDeleteHandler, PostDeleteHandlerTrait};
use domain_interface::DomainContext;

use crate::domain::auth::SupabaseToken;
use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};

#[instrument]
pub async fn api_delete_posts(
    State(ctx): State<DomainContext>,
    _cookies: Cookies,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<Vec<Uuid>>,
) -> impl IntoResponse {
    let handler = PostDeleteHandler {
        db: ctx.conn.clone(),
    };

    let result = handler
        .handle_delete_posts(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
