use application_core::commands::post::create::{
    create_handler::{PostCreateHandler, PostCreateHandlerTrait},
    create_request::CreatePostRequest,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use std::sync::Arc;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{
    keycloak_extension::ExtractKeyCloakToken, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse, ErrorCode,
};

#[instrument]
pub async fn api_create_post(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let handler = PostCreateHandler {
        db: Arc::new(state.conn.clone()),
    };

    let result = handler
        .handle_create_post(body, Some(token.extract_email().email))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::new()
            .with_error_code(ErrorCode::UnknownError)
            .add_error(e.to_string())
            .to_axum_response(),
    }
}
