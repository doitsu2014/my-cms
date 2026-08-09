//! `POST /posts` — create a post with tag attach.

use axum::{extract::State, response::IntoResponse, Extension, Json};
use tracing::instrument;

use crate::handlers::post::create::create_handler::{PostCreateHandler, PostCreateHandlerTrait};
use crate::handlers::post::create::create_request::CreatePostRequest;
use domain_interface::{AuthenticatedActor, DomainContext};

use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};

#[instrument]
pub async fn api_create_post(
    State(ctx): State<DomainContext>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let handler = PostCreateHandler {
        db: ctx.conn.clone(),
    };

    let result = handler.handle_create_post(body, actor.email.clone()).await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
