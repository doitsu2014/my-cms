use application_core::commands::media::create::create_handler::{
    CreateMediaHandler, CreateMediaHandlerTrait,
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

#[instrument]
pub async fn api_create_media_image(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let handler = CreateMediaHandler {
        media_config: state.media_config.clone(),
    };

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap();
        if field_name == "image" {
            let filename = field.file_name().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            let data = field.bytes().await.unwrap().to_owned();
            if content_type.starts_with("image/") {
                let result = handler
                    .create_image_media(filename.to_string(), data.as_ref(), content_type)
                    .await;

                return match result {
                    Ok(m) => ApiResponseWith::new(m).to_axum_response(),
                    Err(e) => ApiResponseError::from(e).to_axum_response(),
                };
            }
        }
    }

    return ApiResponseError::new()
        .with_error_code(ErrorCode::ValidationError)
        .add_error("Image is not valid".to_string())
        .to_axum_response();
}
