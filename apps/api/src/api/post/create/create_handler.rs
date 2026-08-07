use axum::{extract::State, response::IntoResponse, Extension, Json};
use domain_interface::AuthenticatedActor;
use domain_posts::handlers::post::create::{
    create_handler::{PostCreateHandler, PostCreateHandlerTrait},
    create_request::CreatePostRequest,
};
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

#[instrument]
pub async fn api_create_post(
    state: State<AppState>,
    cookies: Cookies,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let handler = PostCreateHandler {
        db: state.conn.clone(),
    };

    let result = handler.handle_create_post(body, actor.email.clone()).await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
