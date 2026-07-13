use axum::{extract::State, Json};
use std::env;

use crate::{
    presentation_models::api_response::{ApiResponseError, ApiResponseWith, ErrorCode},
    AppState,
};
use application_core::commands::ai::models::{ModelsHandler, ModelsHandlerTrait};

pub async fn api_get_openai_models(
    State(_state): State<AppState>,
) -> Result<
    Json<ApiResponseWith<application_core::commands::ai::models::ModelsListResponse>>,
    Json<ApiResponseError>,
> {
    let openai_api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return Err(Json(
                ApiResponseError::new()
                    .with_error_code(ErrorCode::ConnectionError)
                    .add_error("OpenAI API key not configured".to_string()),
            ));
        }
    };

    let handler = ModelsHandler::new();

    match handler.get_available_models(openai_api_key).await {
        Ok(response) => Ok(Json(ApiResponseWith::new(response))),
        Err(e) => Err(Json(ApiResponseError::from(e))),
    }
}
