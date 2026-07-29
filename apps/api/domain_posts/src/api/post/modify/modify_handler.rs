//! `PUT /posts/{post_id}` — modify a post.

use axum::{extract::State, response::IntoResponse, Extension, Json};
use tracing::instrument;

use domain_interface::DomainContext;
use domain_posts::handlers::post::modify::modify_handler::{PostModifyHandler, PostModifyHandlerTrait};
use domain_posts::handlers::post::modify::modify_request::ModifyPostRequest;

use crate::domain::auth::SupabaseToken;
use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};

#[instrument]
pub async fn api_modify_post(
    State(ctx): State<DomainContext>,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<ModifyPostRequest>,
) -> impl IntoResponse {
    let handler = PostModifyHandler {
        db: ctx.conn.clone(),
    };

    let result = handler
        .handle_modify_post(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}