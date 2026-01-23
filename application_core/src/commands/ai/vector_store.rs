use async_openai::{
    config::OpenAIConfig,
    types::{CreateEmbeddingRequestArgs, EmbeddingInput},
    Client,
};
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    vectors_config::Config, CreateCollectionBuilder, Distance, PointStruct, 
    VectorParamsBuilder, VectorsConfig,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::common::app_error::AppError;

// Qdrant collection name for translations
const TRANSLATION_COLLECTION: &str = "translations";

// OpenAI embedding model
const EMBEDDING_MODEL: &str = "text-embedding-3-small";

// Embedding dimension for text-embedding-3-small
const EMBEDDING_DIMENSION: u64 = 1536;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMetadata {
    pub post_id: String,
    pub language_code: String,
    pub translation_id: String,
    pub title: String,
    pub content_preview: String,
}

pub struct VectorStore {
    qdrant: Arc<Qdrant>,
    openai_client: Arc<Client<OpenAIConfig>>,
}

impl std::fmt::Debug for VectorStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VectorStore")
            .field("qdrant", &"Qdrant")
            .field("openai_client", &"OpenAIClient")
            .finish()
    }
}

impl VectorStore {
    /// Creates a new VectorStore instance
    pub async fn new(qdrant_url: &str, openai_api_key: String) -> Result<Self, AppError> {
        let qdrant = Qdrant::from_url(qdrant_url)
            .build()
            .map_err(|e| AppError::OpenAIError(format!("Failed to connect to Qdrant: {}", e)))?;

        let config = OpenAIConfig::new().with_api_key(openai_api_key);
        let openai_client = Client::with_config(config);

        Ok(Self {
            qdrant: Arc::new(qdrant),
            openai_client: Arc::new(openai_client),
        })
    }

    /// Initializes the translation collection in Qdrant
    #[instrument(skip(self))]
    pub async fn initialize_collection(&self) -> Result<(), AppError> {
        // Check if collection exists
        let collections = self
            .qdrant
            .list_collections()
            .await
            .map_err(|e| AppError::OpenAIError(format!("Failed to list collections: {}", e)))?;

        let collection_exists = collections
            .collections
            .iter()
            .any(|c| c.name == TRANSLATION_COLLECTION);

        if !collection_exists {
            // Create collection
            self.qdrant
                .create_collection(
                    CreateCollectionBuilder::new(TRANSLATION_COLLECTION)
                        .vectors_config(VectorsConfig {
                            config: Some(Config::Params(
                                VectorParamsBuilder::new(EMBEDDING_DIMENSION, Distance::Cosine)
                                    .build(),
                            )),
                        })
                        .build(),
                )
                .await
                .map_err(|e| {
                    AppError::OpenAIError(format!("Failed to create collection: {}", e))
                })?;

            tracing::info!("Created Qdrant collection: {}", TRANSLATION_COLLECTION);
        }

        Ok(())
    }

    /// Generates embeddings for text using OpenAI
    #[instrument(skip(self, text))]
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(EMBEDDING_MODEL)
            .input(EmbeddingInput::String(text.to_string()))
            .build()
            .map_err(|e| AppError::OpenAIError(e.to_string()))?;

        let response = self
            .openai_client
            .embeddings()
            .create(request)
            .await
            .map_err(|e| AppError::OpenAIError(e.to_string()))?;

        let embedding = response
            .data
            .first()
            .ok_or_else(|| AppError::OpenAIError("No embedding returned".to_string()))?
            .embedding
            .clone();

        Ok(embedding)
    }

    /// Stores a translation in the vector database
    #[instrument(skip(self, content))]
    pub async fn store_translation(
        &self,
        post_id: Uuid,
        language_code: &str,
        translation_id: Uuid,
        title: &str,
        content: &str,
    ) -> Result<(), AppError> {
        // Combine title and content for embedding (limit to first 8000 chars to avoid token limits)
        let text_for_embedding = format!("{} {}", title, content);
        let truncated_text = if text_for_embedding.len() > 8000 {
            &text_for_embedding[..8000]
        } else {
            &text_for_embedding
        };

        // Generate embedding
        let embedding = self.generate_embedding(truncated_text).await?;

        // Create metadata
        let metadata = TranslationMetadata {
            post_id: post_id.to_string(),
            language_code: language_code.to_string(),
            translation_id: translation_id.to_string(),
            title: title.to_string(),
            content_preview: content.chars().take(500).collect(),
        };

        // Create point
        let json_value = serde_json::to_value(metadata)
            .map_err(|e| AppError::OpenAIError(format!("Failed to serialize metadata: {}", e)))?;
        
        let payload: serde_json::Map<String, serde_json::Value> = if let serde_json::Value::Object(map) = json_value {
            map
        } else {
            return Err(AppError::OpenAIError("Metadata must be a JSON object".to_string()));
        };
        
        let point = PointStruct::new(
            translation_id.to_string(),
            embedding,
            payload,
        );

        // Upsert point
        use qdrant_client::qdrant::UpsertPointsBuilder;
        
        self.qdrant
            .upsert_points(
                UpsertPointsBuilder::new(TRANSLATION_COLLECTION, vec![point]).build(),
            )
            .await
            .map_err(|e| AppError::OpenAIError(format!("Failed to upsert point: {}", e)))?;

        tracing::info!(
            "Stored translation in vector DB: post_id={} language={} translation_id={}",
            post_id,
            language_code,
            translation_id
        );

        Ok(())
    }

    /// Searches for similar translations
    #[instrument(skip(self, query_text))]
    pub async fn search_similar_translations(
        &self,
        query_text: &str,
        limit: u64,
    ) -> Result<Vec<(TranslationMetadata, f32)>, AppError> {
        use qdrant_client::qdrant::SearchPointsBuilder;
        
        // Generate embedding for query
        let query_embedding = self.generate_embedding(query_text).await?;

        // Search in Qdrant
        let search_result = self
            .qdrant
            .search_points(
                SearchPointsBuilder::new(TRANSLATION_COLLECTION, query_embedding, limit)
                    .with_payload(true)
                    .build(),
            )
            .await
            .map_err(|e| AppError::OpenAIError(format!("Failed to search points: {}", e)))?;

        // Parse results
        let results: Vec<(TranslationMetadata, f32)> = search_result
            .result
            .into_iter()
            .filter_map(|point| {
                let score = point.score;
                let payload = point.payload;
                serde_json::to_value(payload)
                    .ok()
                    .and_then(|v| serde_json::from_value(v).ok())
                    .map(|metadata| (metadata, score))
            })
            .collect();

        Ok(results)
    }

    /// Finds existing translation by exact match
    #[instrument(skip(self))]
    pub async fn find_translation(
        &self,
        post_id: Uuid,
        language_code: &str,
    ) -> Result<Option<TranslationMetadata>, AppError> {
        use qdrant_client::qdrant::{Condition, Filter, FieldCondition, Match, ScrollPointsBuilder};
        use qdrant_client::qdrant::r#match::MatchValue;
        
        // Use scroll with filter to find exact match
        let filter = Filter {
            must: vec![
                Condition {
                    condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                        FieldCondition {
                            key: "post_id".to_string(),
                            r#match: Some(Match {
                                match_value: Some(MatchValue::Keyword(post_id.to_string())),
                            }),
                            ..Default::default()
                        },
                    )),
                },
                Condition {
                    condition_one_of: Some(qdrant_client::qdrant::condition::ConditionOneOf::Field(
                        FieldCondition {
                            key: "language_code".to_string(),
                            r#match: Some(Match {
                                match_value: Some(MatchValue::Keyword(language_code.to_string())),
                            }),
                            ..Default::default()
                        },
                    )),
                },
            ],
            ..Default::default()
        };

        let scroll_result = self
            .qdrant
            .scroll(
                ScrollPointsBuilder::new(TRANSLATION_COLLECTION)
                    .filter(filter)
                    .limit(1)
                    .with_payload(true)
                    .build(),
            )
            .await
            .map_err(|e| AppError::OpenAIError(format!("Failed to scroll points: {}", e)))?;

        // Parse first result
        let result = scroll_result
            .result
            .into_iter()
            .next()
            .and_then(|point| {
                let payload = point.payload;
                serde_json::to_value(payload)
                    .ok()
                    .and_then(|v| serde_json::from_value(v).ok())
            });

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires Qdrant server and OpenAI API key
    async fn test_vector_store_initialization() {
        let qdrant_url = std::env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:6334".to_string());
        let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

        let vector_store = VectorStore::new(&qdrant_url, openai_api_key)
            .await
            .unwrap();
        let result = vector_store.initialize_collection().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore] // Requires Qdrant server and OpenAI API key
    async fn test_store_and_search_translation() {
        let qdrant_url = std::env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:6334".to_string());
        let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

        let vector_store = VectorStore::new(&qdrant_url, openai_api_key)
            .await
            .unwrap();
        vector_store.initialize_collection().await.unwrap();

        // Store a translation
        let post_id = Uuid::new_v4();
        let translation_id = Uuid::new_v4();
        let result = vector_store
            .store_translation(
                post_id,
                "Vietnamese",
                translation_id,
                "Test Title",
                "Test content for translation",
            )
            .await;

        assert!(result.is_ok());

        // Search for similar translations
        let search_results = vector_store
            .search_similar_translations("Test content", 5)
            .await
            .unwrap();

        assert!(!search_results.is_empty());
    }
}
