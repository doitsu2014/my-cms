use crate::{
    keycloak_extension::ExtractKeyCloakToken, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse, ErrorCode,
};
use application_core::commands::post::modify::{
    modify_handler::{PostModifyHandler, PostModifyHandlerTrait},
    modify_request::ModifyPostRequest,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use std::sync::Arc;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_modify_post(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<ModifyPostRequest>,
) -> impl IntoResponse {
    let handler = PostModifyHandler {
        db: Arc::new(state.conn.clone()),
    };

    let result = handler
        .handle_modify_post(body, Some(token.extract_email().email))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::new()
            .with_error_code(ErrorCode::UnknownError)
            .add_error(e.to_string())
            .to_axum_response(),
    }
}
