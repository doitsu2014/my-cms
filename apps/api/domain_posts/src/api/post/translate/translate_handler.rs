//! `POST /posts/{post_id}/translate` and
//! `POST /posts/{post_id}/translate/background` — translate a post synchronously
//! or fire-and-forget.

use std::env;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::DatabaseConnection;
use sea_orm::sqlx::types::Uuid;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use domain_interface::DomainContext;
use domain_posts::handlers::post::translate::translate_handler::{
    PostTranslateHandler, PostTranslateHandlerTrait,
};
use domain_posts::handlers::post::translate::translate_request::TranslatePostRequest;

use crate::domain::auth::SupabaseToken;
use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse, ErrorCode};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatePostRequestBody {
    pub target_language: String,
    /// Force re-translation even if translation already exists.
    #[serde(default)]
    pub force_retranslate: bool,
    /// OpenAI model to use for translation (e.g. `gpt-4o-mini`).
    pub model: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatePostResponse {
    pub translation_id: String,
    pub status: String,
}

/// Build a pgvector-backed `VectorStore` if `OPENAI_API_KEY` is configured.
async fn initialize_vector_store(
    db: Arc<DatabaseConnection>,
    openai_api_key: &str,
) -> Option<Arc<domain_posts::handlers::vector_store::VectorStore>> {
    match domain_posts::handlers::vector_store::VectorStore::new(
        db,
        openai_api_key.to_string(),
    )
    .await
    {
        Ok(vs) => {
            if let Err(e) = vs.initialize_collection().await {
                tracing::error!("Failed to initialize pgvector embeddings table: {}", e);
                None
            } else {
                tracing::info!("pgvector embeddings store ready for use");
                Some(Arc::new(vs))
            }
        }
        Err(e) => {
            tracing::error!("Failed to create pgvector VectorStore: {}", e);
            None
        }
    }
}

#[instrument]
pub async fn api_translate_post(
    State(ctx): State<DomainContext>,
    Extension(_token): Extension<SupabaseToken>,
    Path(post_id): Path<Uuid>,
    Json(body): Json<TranslatePostRequestBody>,
) -> impl IntoResponse {
    let openai_api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ConnectionError)
                .add_error("OPENAI_API_KEY environment variable not set".to_string())
                .to_axum_response();
        }
    };

    let vector_store = initialize_vector_store(ctx.conn.clone(), &openai_api_key).await;

    let handler = PostTranslateHandler {
        db: ctx.conn.clone(),
        vector_store,
    };

    let mut request = TranslatePostRequest::new(post_id, body.target_language)
        .with_force_retranslate(body.force_retranslate);

    if let Some(model) = body.model {
        request = request.with_model(model);
    }

    let result = handler.handle_translate_post(request, openai_api_key).await;

    match result {
        Ok(response) => ApiResponseWith::new(TranslatePostResponse {
            translation_id: response.post_translation_id.to_string(),
            status: "completed".to_string(),
        })
        .to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

#[instrument]
pub async fn api_translate_post_background(
    State(ctx): State<DomainContext>,
    Extension(_token): Extension<SupabaseToken>,
    Path(post_id): Path<Uuid>,
    Json(body): Json<TranslatePostRequestBody>,
) -> impl IntoResponse {
    let openai_api_key = match env::var("OPENAI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return ApiResponseError::new()
                .with_error_code(ErrorCode::ConnectionError)
                .add_error("OPENAI_API_KEY environment variable not set".to_string())
                .to_axum_response();
        }
    };

    let vector_store = initialize_vector_store(ctx.conn.clone(), &openai_api_key).await;

    let handler = PostTranslateHandler {
        db: ctx.conn.clone(),
        vector_store,
    };

    let mut request = TranslatePostRequest::new(post_id, body.target_language)
        .with_force_retranslate(body.force_retranslate);

    if let Some(model) = body.model {
        request = request.with_model(model);
    }

    let result = handler
        .handle_translate_post_background(request, openai_api_key)
        .await;

    match result {
        Ok(translation_id) => ApiResponseWith::new(TranslatePostResponse {
            translation_id: translation_id.to_string(),
            status: "processing".to_string(),
        })
        .to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}