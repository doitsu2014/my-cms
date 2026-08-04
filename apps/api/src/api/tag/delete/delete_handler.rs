use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::post::delete::delete_handler::{
    PostDeleteHandler, PostDeleteHandlerTrait,
};
use axum::{extract::Extension, response::IntoResponse, Json};
use domain_interface::AuthenticatedActor;
use sea_orm::sqlx::types::Uuid;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_delete_tags(
    state: Extension<AppState>,
    cookies: Cookies,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<Vec<Uuid>>,
) -> impl IntoResponse {
    let handler = PostDeleteHandler {
        db: state.conn.clone(),
    };

    let result = handler.handle_delete_posts(body, actor.email.clone()).await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
