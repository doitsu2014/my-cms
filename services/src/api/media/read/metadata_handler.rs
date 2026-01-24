use application_core::commands::media::read::metadata_handler::{
    MetadataMediaHandler, MetadataMediaHandlerTrait,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension,
};
use axum_keycloak_auth::decode::KeycloakToken;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

#[instrument(skip(state))]
pub async fn api_get_media_metadata(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<KeycloakToken<String>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let handler = MetadataMediaHandler {
        media_config: state.media_config.clone(),
    };

    match handler.get_metadata(path).await {
        Ok(metadata) => ApiResponseWith::new(metadata).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
