//! `POST /posts` — create a post with tag attach.

use axum::{extract::State, response::IntoResponse, Extension, Json};
use tracing::instrument;

use domain_interface::DomainContext;
use domain_posts::handlers::post::create::create_handler::{PostCreateHandler, PostCreateHandlerTrait};
use domain_posts::handlers::post::create::create_request::CreatePostRequest;

use crate::domain::auth::SupabaseToken;
use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};

#[instrument]
pub async fn api_create_post(
    State(ctx): State<DomainContext>,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let handler = PostCreateHandler {
        db: ctx.conn.clone(),
    };

    let result = handler
        .handle_create_post(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}