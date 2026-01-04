use application_core::commands::media::{
    create::create_handler::{CreateMediaHandler, CreateMediaHandlerTrait},
    is_supported_content_type,
};
use axum::{
    extract::{Multipart, State},
    response::IntoResponse,
    Extension,
};
use axum_keycloak_auth::decode::KeycloakToken;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse, ErrorCode};

#[instrument(skip(state, multipart))]
pub async fn api_create_media(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<KeycloakToken<String>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let handler = CreateMediaHandler {
        media_config: state.media_config.clone(),
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap_or_default();
        if field_name == "file" || field_name == "image" {
            let filename = field.file_name().unwrap_or("unknown").to_string();
            let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
            let data = field.bytes().await.unwrap().to_owned();

            if is_supported_content_type(&content_type) {
                let result = handler
                    .create_media(filename.to_string(), data.as_ref(), content_type)
                    .await;

                return match result {
                    Ok(m) => ApiResponseWith::new(m).to_axum_response(),
                    Err(e) => ApiResponseError::from(e).to_axum_response(),
                };
            } else {
                return ApiResponseError::new()
                    .with_error_code(ErrorCode::ValidationError)
                    .add_error(format!("Unsupported content type: {}", content_type))
                    .to_axum_response();
            }
        }
    }

    ApiResponseError::new()
        .with_error_code(ErrorCode::ValidationError)
        .add_error("No file found in request. Use 'file' or 'image' field name.".to_string())
        .to_axum_response()
}
