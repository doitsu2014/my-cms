//! `PUT /posts` — modify an existing post.

use axum::{extract::State, response::IntoResponse, Extension, Json};
use tracing::instrument;

use crate::handlers::post::modify::modify_handler::{PostModifyHandler, PostModifyHandlerTrait};
use crate::handlers::post::modify::modify_request::ModifyPostRequest;
use domain_interface::{AuthenticatedActor, DomainContext};

use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};

#[instrument]
pub async fn api_modify_post(
    State(ctx): State<DomainContext>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<ModifyPostRequest>,
) -> impl IntoResponse {
    let handler = PostModifyHandler {
        db: ctx.conn.clone(),
    };

    let result = handler.handle_modify_post(body, actor.email.clone()).await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
