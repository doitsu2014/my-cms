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

pub(crate) const EMBEDDING_MODEL: &str = "text-embedding-3-small";
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
    pub async fn new(
        db: Arc<DatabaseConnection>,
        openai_api_key: String,
    ) -> Result<Self, AppError> {
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
        let truncated_text = if text_for_embedding.len() > MAX_SEARCH_TEXT_LENGTH {
            &text_for_embedding[..MAX_SEARCH_TEXT_LENGTH]
        } else {
            &text_for_embedding
        };

        tracing::debug!(
            "Generating embedding for {} characters",
            truncated_text.len()
        );

        let embedding = self.generate_embedding(truncated_text).await?;

        tracing::debug!("Generated embedding with {} dimensions", embedding.len());

        let embedding_str = Self::format_embedding_for_pg(&embedding);
        let content_preview = Self::create_content_preview(content, CONTENT_PREVIEW_LENGTH);
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
            .map_err(|e| AppError::OpenAIError(format!("Failed to store embedding: {}", e)))?;

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
            .map_err(|e| AppError::OpenAIError(format!("Failed to search embeddings: {}", e)))?;

        let mut similar = Vec::new();
        for row in &results {
            let post_id: String = row.try_get_by_index::<String>(0usize).unwrap_or_default();
            let language_code: String = row.try_get_by_index::<String>(1usize).unwrap_or_default();
            let translation_id: String = row.try_get_by_index::<String>(2usize).unwrap_or_default();
            let title: String = row.try_get_by_index::<String>(3usize).unwrap_or_default();
            let content_preview: String =
                row.try_get_by_index::<String>(4usize).unwrap_or_default();
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

        Ok(results
            .first()
            .map(|row: &QueryResult| TranslationMetadata {
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
        let content =
            "First paragraph.\n\nSecond paragraph. This is some extra text that makes it longer.";
        let result = VectorStore::create_content_preview(content, 50);
        assert!(!result.contains("extra text"));
        assert!(result.len() <= 50);
    }

    #[test]
    fn test_create_content_preview_truncates_at_sentence() {
        let content = "First sentence. Second sentence. Third sentence that continues.";
        let result = VectorStore::create_content_preview(content, 30);
        assert!(result.len() <= 30);
        assert!(!result.contains("Third sentence"));
    }

    #[test]
    fn test_format_embedding_for_pg() {
        let embedding = vec![1.0_f32, 2.5, 3.0];
        let result = VectorStore::format_embedding_for_pg(&embedding);
        assert_eq!(result, "[1,2.5,3]");
    }

    fn require_openai_key() -> Option<String> {
        std::env::var("OPENAI_API_KEY").ok()
    }

    use test_helpers::ContainerAsyncPostgresEx;

    /// Integration test: store and retrieve a translation embedding
    #[async_std::test]
    async fn test_store_and_find_translation() {
        let api_key = match require_openai_key() {
            Some(k) => k,
            None => {
                eprintln!("Skipping: OPENAI_API_KEY not set");
                return;
            }
        };

        let test_space = test_helpers::setup_test_space_with_pgvector().await;
        let db: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let db = std::sync::Arc::new(db);

        let store = VectorStore::new(db.clone(), api_key)
            .await
            .expect("should create VectorStore");
        store
            .initialize_collection()
            .await
            .expect("pgvector available");

        let post_id = uuid::Uuid::new_v4();
        let translation_id = uuid::Uuid::new_v4();

        store
            .store_translation(
                post_id,
                "en",
                translation_id,
                "Hello World",
                "Some content here for embedding",
            )
            .await
            .expect("should store embedding");

        let found = store
            .find_translation(post_id, "en")
            .await
            .expect("should find embedding")
            .expect("should return Some");

        assert_eq!(found.post_id, post_id.to_string());
        assert_eq!(found.language_code, "en");
        assert_eq!(found.translation_id, translation_id.to_string());
        assert_eq!(found.title, "Hello World");
    }

    /// Integration test: search for similar translations
    #[async_std::test]
    async fn test_search_similar_translations() {
        let api_key = match require_openai_key() {
            Some(k) => k,
            None => {
                eprintln!("Skipping: OPENAI_API_KEY not set");
                return;
            }
        };

        let test_space = test_helpers::setup_test_space_with_pgvector().await;
        let db: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let db = std::sync::Arc::new(db);

        let store = VectorStore::new(db.clone(), api_key)
            .await
            .expect("should create VectorStore");
        store
            .initialize_collection()
            .await
            .expect("pgvector available");

        let post_a_id = uuid::Uuid::new_v4();
        let post_b_id = uuid::Uuid::new_v4();

        store
            .store_translation(
                post_a_id,
                "en",
                uuid::Uuid::new_v4(),
                "Rust Programming Guide",
                "Rust is a systems programming language focused on safety and performance.",
            )
            .await
            .expect("store post a");

        store
            .store_translation(
                post_b_id,
                "en",
                uuid::Uuid::new_v4(),
                "Italian Cooking",
                "Pasta carbonara is a classic Italian dish made with eggs, cheese, and bacon.",
            )
            .await
            .expect("store post b");

        let results = store
            .search_similar_translations("Rust programming language tutorial", 3)
            .await
            .expect("should search");

        assert!(!results.is_empty(), "should find at least one result");

        let best_match = &results[0].0;
        assert!(
            best_match.title.contains("Rust"),
            "best match should be about Rust, got: {}",
            best_match.title
        );
    }

    /// Integration test: upsert on conflict
    #[async_std::test]
    async fn test_store_translation_upsert() {
        let api_key = match require_openai_key() {
            Some(k) => k,
            None => {
                eprintln!("Skipping: OPENAI_API_KEY not set");
                return;
            }
        };

        let test_space = test_helpers::setup_test_space_with_pgvector().await;
        let db: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let db = std::sync::Arc::new(db);

        let store = VectorStore::new(db.clone(), api_key)
            .await
            .expect("should create VectorStore");
        store
            .initialize_collection()
            .await
            .expect("pgvector available");

        let post_id = uuid::Uuid::new_v4();
        let tid1 = uuid::Uuid::new_v4();
        let tid2 = uuid::Uuid::new_v4();

        store
            .store_translation(
                post_id,
                "fr",
                tid1,
                "Version 1",
                "Première version du contenu.",
            )
            .await
            .expect("store initial");

        store
            .store_translation(
                post_id,
                "fr",
                tid2,
                "Version 2",
                "Deuxième version mise à jour.",
            )
            .await
            .expect("store update");

        let found = store
            .find_translation(post_id, "fr")
            .await
            .expect("should find")
            .expect("should exist");

        assert_eq!(
            found.translation_id,
            tid2.to_string(),
            "should reflect updated translation_id"
        );
        assert_eq!(found.title, "Version 2", "should reflect updated title");
    }

    /// Integration test: find_translation returns None for non-existent entry
    #[async_std::test]
    async fn test_find_translation_not_found() {
        let api_key = match require_openai_key() {
            Some(k) => k,
            None => {
                eprintln!("Skipping: OPENAI_API_KEY not set");
                return;
            }
        };

        let test_space = test_helpers::setup_test_space_with_pgvector().await;
        let db: DatabaseConnection = test_space.postgres.get_database_connection().await;
        let db = std::sync::Arc::new(db);

        let store = VectorStore::new(db.clone(), api_key)
            .await
            .expect("should create VectorStore");
        store
            .initialize_collection()
            .await
            .expect("pgvector available");

        let result = store
            .find_translation(uuid::Uuid::new_v4(), "xx")
            .await
            .expect("should not error");
        assert!(
            result.is_none(),
            "should return None for non-existent entry"
        );
    }
}
