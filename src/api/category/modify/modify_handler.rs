use crate::{
    keycloak_extension::ExtractKeyCloakToken, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};
use application_core::commands::category::modify::{
    modify_handler::{CategoryModifyHandler, CategoryModifyHandlerTrait},
    modify_request::ModifyCategoryRequest,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_modify_category(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<ModifyCategoryRequest>,
) -> impl IntoResponse {
    let handler = CategoryModifyHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_modify_category(body, Some(token.extract_email().email))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
