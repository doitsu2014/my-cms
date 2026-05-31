use application_core::commands::ai::translate::{
    translate_handler::{PostTranslateHandler, PostTranslateHandlerTrait},
    translate_request::TranslatePostRequest,
};
use axum::{extract::{Path, State}, response::IntoResponse, Extension, Json};
use crate::common::supabase_auth::SupabaseToken;
use sea_orm::sqlx::types::Uuid;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tracing::instrument;

use crate::{
    presentation_models::api_response::ErrorCode, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatePostRequestBody {
    pub target_language: String,
    /// Force re-translation even if translation already exists
    /// When true, will check Qdrant for similar translations before proceeding
    #[serde(default)]
    pub force_retranslate: bool,
    /// OpenAI model to use for translation (e.g., "gpt-4o-mini", "gpt-4o")
    /// If not specified, defaults to "gpt-5-nano"
    pub model: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatePostResponse {
    pub translation_id: String,
    pub status: String,
}

/// Initialize pgvector-backed vector store for AI translation embeddings
async fn initialize_vector_store(
    db: Arc<DatabaseConnection>,
    openai_api_key: &str,
) -> Option<Arc<application_core::commands::ai::vector_store_pg::VectorStore>> {
    match application_core::commands::ai::vector_store_pg::VectorStore::new(
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

/// Translate a post synchronously (waits for completion)
/// 
/// POST /posts/{post_id}/translate
/// 
/// Body: { "targetLanguage": "VI" }
#[instrument]
pub async fn api_translate_post(
    state: State<AppState>,
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

    let vector_store = initialize_vector_store(state.conn.clone(), &openai_api_key).await;

    let handler = PostTranslateHandler {
        db: state.conn.clone(),
        vector_store,
    };

    let mut request = TranslatePostRequest::new(post_id, body.target_language)
        .with_force_retranslate(body.force_retranslate);
    
    if let Some(model) = body.model {
        request = request.with_model(model);
    }

    let result = handler
        .handle_translate_post(request, openai_api_key)
        .await;

    match result {
        Ok(response) => ApiResponseWith::new(TranslatePostResponse {
            translation_id: response.post_translation_id.to_string(),
            status: "completed".to_string(),
        })
        .to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// Translate a post in background (returns immediately)
/// 
/// POST /posts/{post_id}/translate/background
/// 
/// Body: { "targetLanguage": "VI" }
#[instrument]
pub async fn api_translate_post_background(
    state: State<AppState>,
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

    let vector_store = initialize_vector_store(state.conn.clone(), &openai_api_key).await;

    let handler = PostTranslateHandler {
        db: state.conn.clone(),
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
