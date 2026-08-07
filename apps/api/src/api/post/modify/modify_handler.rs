use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use domain_interface::AuthenticatedActor;
use domain_posts::handlers::post::modify::{
    modify_handler::{PostModifyHandler, PostModifyHandlerTrait},
    modify_request::ModifyPostRequest,
};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_modify_post(
    state: State<AppState>,
    cookies: Cookies,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<ModifyPostRequest>,
) -> impl IntoResponse {
    let handler = PostModifyHandler {
        db: state.conn.clone(),
    };

    let result = handler.handle_modify_post(body, actor.email.clone()).await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
