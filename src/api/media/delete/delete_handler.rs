use application_core::commands::media::delete::delete_handler::{
    DeleteMediaHandler, DeleteMediaHandlerTrait,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use axum_keycloak_auth::decode::KeycloakToken;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

/// Delete a single media file by path
#[instrument(skip(state))]
pub async fn api_delete_media(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<KeycloakToken<String>>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    let handler = DeleteMediaHandler {
        media_config: state.media_config.clone(),
    };

    match handler.delete_media(path).await {
        Ok(_) => ApiResponseWith::new("Deleted successfully").to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// Delete multiple media files by paths (batch delete)
#[instrument(skip(state))]
pub async fn api_delete_media_batch(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<KeycloakToken<String>>,
    Json(paths): Json<Vec<String>>,
) -> impl IntoResponse {
    let handler = DeleteMediaHandler {
        media_config: state.media_config.clone(),
    };

    match handler.delete_media_batch(paths).await {
        Ok(deleted_count) => {
            ApiResponseWith::new(serde_json::json!({
                "deletedCount": deleted_count
            }))
            .to_axum_response()
        }
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
