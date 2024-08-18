use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use sea_orm::sqlx::types::Uuid;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::AppState;

#[instrument]
pub async fn api_create_media_image(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<Vec<Uuid>>,
) -> impl IntoResponse {
}
