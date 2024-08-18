use application_core::commands::media::create::create_handler::{
    CreateMediaHandler, CreateMediaHandlerTrait,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use sea_orm::sqlx::types::Uuid;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

#[instrument]
pub async fn api_create_media_image(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<Vec<Uuid>>,
) -> impl IntoResponse {
    let handler = CreateMediaHandler {
        s3_media_storage: state.s3_media_storage.clone(),
    };

    let body_string = body
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    let result = handler
        .create_image_media("rand100.txt".to_string(), body_string.as_bytes())
        .await;

    match result {
        Ok(()) => ApiResponseWith::new("x").to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
