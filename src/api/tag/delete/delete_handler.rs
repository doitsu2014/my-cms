use crate::{
    keycloak_extension::ExtractKeyCloakToken, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};
use application_core::commands::post::delete::delete_handler::{
    PostDeleteHandler, PostDeleteHandlerTrait,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use sea_orm::sqlx::types::Uuid;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_delete_tags(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<Vec<Uuid>>,
) -> impl IntoResponse {
    let handler = PostDeleteHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_delete_posts(body, Some(token.extract_email().email))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from_app_error(e).to_axum_response(),
    }
}
