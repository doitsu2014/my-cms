//! `GET /ai/models` — returns the curated OpenAI model catalogue.
//!
//! Moved from `cms::src::api::ai::models::models_handler` per Decision 2 of
//! the `consolidate-category-ai-translate-into-domain-posts` change. The
//! handler signature was migrated from `Extension<AppState>` to
//! `State<DomainContext>` to match the gateway composition contract.

use axum::{extract::State, Json};
use domain_interface::DomainContext;
use std::env;

use crate::domain::response::{ApiResponseError, ApiResponseWith, ErrorCode};
use crate::handlers::ai::models::{ModelsHandler, ModelsHandlerTrait, ModelsListResponse};

pub async fn api_get_openai_models(
    State(_ctx): State<DomainContext>,
) -> Result<Json<ApiResponseWith<ModelsListResponse>>, Json<ApiResponseError>> {
    let _ = _ctx;
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
