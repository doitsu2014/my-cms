use async_openai::{
    config::OpenAIConfig,
    types::{CreateEmbeddingRequestArgs, EmbeddingInput},
    Client,
};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, QueryResult, Statement};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::common::app_error::AppError;

pub(crate) const TRANSLATION_COLLECTION: &str = "translations";
pub(crate) const EMBEDDING_MODEL: &str = "text-embedding-3-small";
pub(crate) const EMBEDDING_DIMENSION: u64 = 1536;
pub(crate) const SIMILARITY_REUSE_THRESHOLD: f32 = 0.95;
pub(crate) const MAX_SEARCH_TEXT_LENGTH: usize = 8000;
pub(crate) const CONTENT_PREVIEW_LENGTH: usize = 2000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMetadata {
    pub post_id: String,
    pub language_code: String,
    pub translation_id: String,
    pub title: String,
    pub content_preview: String,
}

pub struct VectorStore {
    db: Arc<DatabaseConnection>,
    openai_client: Arc<Client<OpenAIConfig>>,
}

impl std::fmt::Debug for VectorStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VectorStore")
            .field("db", &"DatabaseConnection")
            .field("openai_client", &"OpenAIClient")
            .finish()
    }
}

impl VectorStore {
    pub async fn new(db: Arc<DatabaseConnection>, openai_api_key: String) -> Result<Self, AppError> {
        let config = OpenAIConfig::new().with_api_key(openai_api_key);
        let openai_client = Client::with_config(config);

        Ok(Self {
            db,
            openai_client: Arc::new(openai_client),
        })
    }

    pub async fn initialize_collection(&self) -> Result<(), AppError> {
        tracing::info!("Initializing pgvector embeddings table");

        self.db
            .execute(Statement::from_string(
                DbBackend::Postgres,
                "SELECT 1 FROM pg_extension WHERE extname = 'vector'".to_string(),
            ))
            .await
            .map_err(|e| {
                AppError::OpenAIError(format!(
                    "pgvector extension not found: {}. Run migration first.",
                    e
                ))
            })?;

        tracing::info!("pgvector extension verified for embeddings table");
        Ok(())
    }

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

    pub fn create_content_preview(content: &str, max_length: usize) -> String {
        if content.len() <= max_length {
            return content.to_string();
        }

        let truncated = &content[..max_length];

        if let Some(pos) = truncated.rfind("\n\n") {
            if pos > max_length / 2 {
                return truncated[..pos].trim().to_string();
            }
        }

        let sentence_endings = [". ", ".\n", "! ", "!\n", "? ", "?\n"];
        for ending in &sentence_endings {
            if let Some(pos) = truncated.rfind(ending) {
                if pos > max_length / 2 {
                    return truncated[..pos + 1].trim().to_string();
                }
            }
        }

        if let Some(pos) = truncated.rfind(' ') {
            return truncated[..pos].trim().to_string();
        }

        truncated.to_string()
    }

    fn format_embedding_for_pg(embedding: &[f32]) -> String {
        let values: Vec<String> = embedding.iter().map(|v| v.to_string()).collect();
        format!("[{}]", values.join(","))
    }

    #[instrument(skip(self, content))]
    pub async fn store_translation(
        &self,
        post_id: Uuid,
        language_code: &str,
        translation_id: Uuid,
        title: &str,
        content: &str,
    ) -> Result<(), AppError> {
        let text_for_embedding = format!("{} {}", title, content);
        let truncated_text = if text_for_embedding.len() > 8000 {
            &text_for_embedding[..8000]
        } else {
            &text_for_embedding
        };

        tracing::debug!("Generating embedding for {} characters", truncated_text.len());

        let embedding = self.generate_embedding(truncated_text).await?;

        tracing::debug!(
            "Generated embedding with {} dimensions",
            embedding.len()
        );

        let embedding_str = Self::format_embedding_for_pg(&embedding);
        let content_preview = Self::create_content_preview(content, 2000);
        let id = Uuid::new_v4();

        let safe_title = title.replace('\'', "''");
        let safe_preview = content_preview.replace('\'', "''");

        let sql = format!(
            r#"INSERT INTO embeddings (id, post_id, language_code, translation_id, embedding, title, content_preview, created_at, updated_at)
            VALUES ('{}'::uuid, '{}'::uuid, '{}', '{}'::uuid, '{}'::vector, '{}', '{}', NOW(), NOW())
            ON CONFLICT (post_id, language_code) DO UPDATE SET
                translation_id = EXCLUDED.translation_id,
                embedding = EXCLUDED.embedding,
                title = EXCLUDED.title,
                content_preview = EXCLUDED.content_preview,
                updated_at = NOW()"#,
            id.to_string(),
            post_id.to_string(),
            language_code,
            translation_id.to_string(),
            embedding_str,
            safe_title,
            safe_preview,
        );

        self.db
            .execute(Statement::from_string(DbBackend::Postgres, sql))
            .await
            .map_err(|e| {
                AppError::OpenAIError(format!("Failed to store embedding: {}", e))
            })?;

        tracing::info!(
            "Successfully stored translation embedding: post_id={} language={} translation_id={}",
            post_id,
            language_code,
            translation_id
        );

        Ok(())
    }

    #[instrument(skip(self, query_text))]
    pub async fn search_similar_translations(
        &self,
        query_text: &str,
        limit: u64,
    ) -> Result<Vec<(TranslationMetadata, f32)>, AppError> {
        let query_embedding = self.generate_embedding(query_text).await?;
        let embedding_str = Self::format_embedding_for_pg(&query_embedding);

        let sql = format!(
            r#"SELECT
                e.post_id::text,
                e.language_code,
                e.translation_id::text,
                e.title,
                e.content_preview,
                (1.0 - (e.embedding <=> '{}'::vector))::float4 AS similarity
            FROM embeddings e
            ORDER BY e.embedding <=> '{}'::vector
            LIMIT {}"#,
            embedding_str, embedding_str, limit
        );

        let results: Vec<QueryResult> = self
            .db
            .query_all(Statement::from_string(DbBackend::Postgres, sql))
            .await
            .map_err(|e| {
                AppError::OpenAIError(format!("Failed to search embeddings: {}", e))
            })?;

        let mut similar = Vec::new();
        for row in &results {
            let post_id: String = row.try_get_by_index::<String>(0usize).unwrap_or_default();
            let language_code: String = row.try_get_by_index::<String>(1usize).unwrap_or_default();
            let translation_id: String = row.try_get_by_index::<String>(2usize).unwrap_or_default();
            let title: String = row.try_get_by_index::<String>(3usize).unwrap_or_default();
            let content_preview: String = row.try_get_by_index::<String>(4usize).unwrap_or_default();
            let score: f64 = row.try_get_by_index::<f64>(5usize).unwrap_or(0.0);
            let score = score as f32;

            similar.push((
                TranslationMetadata {
                    post_id,
                    language_code,
                    translation_id,
                    title,
                    content_preview,
                },
                score,
            ));
        }

        Ok(similar)
    }

    #[instrument(skip(self))]
    pub async fn find_translation(
        &self,
        post_id: Uuid,
        language_code: &str,
    ) -> Result<Option<TranslationMetadata>, AppError> {
        let sql = format!(
            r#"SELECT post_id::text, language_code, translation_id::text, title, content_preview
            FROM embeddings
            WHERE post_id = '{}'::uuid AND language_code = '{}'
            LIMIT 1"#,
            post_id, language_code
        );

        let results: Vec<QueryResult> = self
            .db
            .query_all(Statement::from_string(DbBackend::Postgres, sql))
            .await
            .map_err(|e| AppError::OpenAIError(format!("Failed to find embedding: {}", e)))?;

        Ok(results.first().map(|row: &QueryResult| TranslationMetadata {
            post_id: row.try_get_by_index::<String>(0usize).unwrap_or_default(),
            language_code: row.try_get_by_index::<String>(1usize).unwrap_or_default(),
            translation_id: row.try_get_by_index::<String>(2usize).unwrap_or_default(),
            title: row.try_get_by_index::<String>(3usize).unwrap_or_default(),
            content_preview: row.try_get_by_index::<String>(4usize).unwrap_or_default(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_content_preview() {
        let short = "Short text";
        let result = VectorStore::create_content_preview(short, 2000);
        assert_eq!(result, short);
    }

    #[test]
    fn test_create_content_preview_truncates_at_paragraph() {
        let content = "First paragraph.\n\nSecond paragraph. This is some extra text that makes it longer.";
        let result = VectorStore::create_content_preview(content, 50);
        assert!(!result.contains("Second paragraph"));
    }

    #[test]
    fn test_create_content_preview_truncates_at_sentence() {
        let content = "First sentence. Second sentence. Third sentence that continues.";
        let result = VectorStore::create_content_preview(content, 30);
        assert!(result.ends_with('.'));
    }
}
