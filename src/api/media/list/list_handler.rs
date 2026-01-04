use application_core::commands::media::list::list_handler::{
    ListMediaHandler, ListMediaHandlerTrait,
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension,
};
use axum_keycloak_auth::decode::KeycloakToken;
use serde::Deserialize;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

#[derive(Debug, Deserialize)]
pub struct ListQueryParams {
    pub prefix: Option<String>,
}

#[instrument(skip(state))]
pub async fn api_list_media(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<KeycloakToken<String>>,
    Query(params): Query<ListQueryParams>,
) -> impl IntoResponse {
    let handler = ListMediaHandler {
        media_config: state.media_config.clone(),
    };

    match handler.list_media(params.prefix).await {
        Ok(media_list) => ApiResponseWith::new(media_list).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
