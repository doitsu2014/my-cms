use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use chrono::Utc;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, RcDom};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use slugify::slugify;
use std::io::Cursor;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::error::AppError;
use crate::entities::{post_translations, posts, translation_jobs};

use super::{translate_request::TranslatePostRequest, translate_response::TranslatePostResponse};

// Default OpenAI model to use for translation
// Changed from "gpt-5-nano" (which was never a real OpenAI model) to "gpt-4o-mini"
// gpt-4o-mini is a real, reliable, and cost-effective production model
const DEFAULT_OPENAI_MODEL: &str = "gpt-4o-mini";

// Maximum chunk size in characters for content translation
// Set to 2000 for reliable translation with gpt-4o-mini and other production models
// Previous value of 800 was a workaround for the non-existent "gpt-5-nano" model
// Larger chunks improve translation quality and reduce API calls
const MAX_CHUNK_SIZE: usize = 2000;

// Temperature setting for translations (lower = more deterministic, less tokens)
// Range: 0.0 to 2.0. For translations, we use low temperature for consistency
const TRANSLATION_TEMPERATURE: f32 = 0.3;

// Max tokens for OpenAI response (not input)
// GPT-4o-mini supports up to 16K output tokens
// Increased from 3000 to 8000 to handle longer translations without cutoff
// This prevents incomplete translations for large content chunks
const MAX_TOKENS_PER_REQUEST: u16 = 8000;

// Similarity threshold for automatic translation reuse
// If a similar translation has score >= this threshold, reuse it instead of creating new translation
// Range: 0.0 to 1.0, where 1.0 is identical content
// 0.95 means 95% similar - very high similarity, likely same/similar content
const SIMILARITY_REUSE_THRESHOLD: f32 = 0.95;

// Translation instruction for HTML content.
//
// `pub(crate)` so the colocated prompt regression test in
// `translation_validator::prompt_tests` can read it back.
//
// Appended clause (per
// openspec/changes/fix-translator-incomplete-html-output): explicitly tell the
// model that the translated output must keep every <p> paragraph and must
// never return a headings-only response.
pub(crate) const TRANSLATION_INSTRUCTION_HTML: &str =
    "Preserve all HTML tags and structure exactly as they are. Only translate the text content within the tags, \
     never translate HTML tag names, attributes, or structure. Return valid HTML. Translate the ENTIRE content \
     provided, do not truncate or summarize. Return the same number of `<p>` paragraphs as the source. Never \
     return a response that contains only headings; if the source has paragraphs your response must include \
     the same paragraphs in the target language.";

// Translation instruction for plain text content
const TRANSLATION_INSTRUCTION_TEXT: &str =
    "Only return the translated text without any additional comments or explanations. Translate the ENTIRE \
     content provided, do not truncate or summarize.";

pub trait PostTranslateHandlerTrait {
    fn handle_translate_post(
        &self,
        request: TranslatePostRequest,
        openai_api_key: String,
    ) -> impl std::future::Future<Output = Result<TranslatePostResponse, AppError>>;

    fn handle_translate_post_background(
        &self,
        request: TranslatePostRequest,
        openai_api_key: String,
    ) -> impl std::future::Future<Output = Result<Uuid, AppError>>;
}

#[derive(Debug)]
pub struct PostTranslateHandler {
    pub db: Arc<DatabaseConnection>,
    pub vector_store: Option<Arc<crate::handlers::vector_store::VectorStore>>,
}

struct SimilarTranslationInfo {
    source_translation_id: Uuid,
    source_post_id: Uuid,
    similarity_score: f32,
    translated_title: String,
    translated_preview_content: String,
    translated_content: String,
}

impl PostTranslateHandler {
    /// Helper function to check if a model supports custom temperature values
    fn supports_custom_temperature(model: &str) -> bool {
        // All standard OpenAI models support custom temperature
        // Return false only for specific test/mock models if needed
        !model.starts_with("mock-") && !model.starts_with("test-")
    }

    /// Database lookup: Check if translation already exists
    async fn lookup_existing_translation(
        db: &DatabaseConnection,
        post_id: Uuid,
        language_code: &str,
    ) -> Result<Option<post_translations::Model>, AppError> {
        post_translations::Entity::find()
            .filter(post_translations::Column::PostId.eq(post_id))
            .filter(post_translations::Column::LanguageCode.eq(language_code))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    /// Delete existing translation
    async fn delete_existing_translation(
        db: &DatabaseConnection,
        post_id: Uuid,
        language_code: &str,
    ) -> Result<(), AppError> {
        if let Some(existing) =
            Self::lookup_existing_translation(db, post_id, language_code).await?
        {
            post_translations::Entity::delete_by_id(existing.id)
                .exec(db)
                .await
                .map_err(AppError::from)?;
            tracing::info!(
                "Deleted existing translation_id={} for retranslation",
                existing.id
            );
        }
        Ok(())
    }

    /// Qdrant similarity search: Find similar translations with score >= threshold
    async fn find_similar_translation(
        vector_store: &Option<Arc<crate::handlers::vector_store::VectorStore>>,
        db: &DatabaseConnection,
        post: &posts::Model,
        post_id: Uuid,
        target_language_code: &str,
    ) -> Result<Option<SimilarTranslationInfo>, AppError> {
        let Some(vector_store) = vector_store else {
            return Ok(None);
        };

        let search_text = format!(
            "{} {}",
            post.title,
            post.content.chars().take(500).collect::<String>()
        );
        let similar = match vector_store
            .search_similar_translations(&search_text, 5)
            .await
        {
            Ok(results) => results,
            Err(e) => {
                tracing::warn!(
                    "Failed to search similar translations: {}. Continuing with new translation.",
                    e
                );
                return Ok(None);
            }
        };

        if similar.is_empty() {
            return Ok(None);
        }

        tracing::info!(
            "Found {} similar translations in vector DB for similarity check",
            similar.len()
        );

        let request_post_id_str = post_id.to_string();

        for (metadata, score) in similar.iter() {
            if *score >= SIMILARITY_REUSE_THRESHOLD
                && metadata.language_code == target_language_code
                && metadata.post_id != request_post_id_str
            {
                let similar_post_id = match Uuid::parse_str(&metadata.post_id) {
                    Ok(uuid) => uuid,
                    Err(_) => {
                        tracing::warn!("Invalid post_id UUID in metadata: {}", metadata.post_id);
                        continue;
                    }
                };

                if let Ok(Some(similar_translation)) = post_translations::Entity::find()
                    .filter(post_translations::Column::PostId.eq(similar_post_id))
                    .filter(post_translations::Column::LanguageCode.eq(target_language_code))
                    .one(db)
                    .await
                {
                    tracing::info!(
                        "  Source: post_id={} title='{}' language={}",
                        metadata.post_id,
                        metadata.title,
                        metadata.language_code
                    );
                    tracing::info!(
                        "  Reusing translation instead of calling OpenAI API (cost savings!)"
                    );

                    return Ok(Some(SimilarTranslationInfo {
                        source_translation_id: similar_translation.id,
                        source_post_id: similar_post_id,
                        similarity_score: *score,
                        translated_title: similar_translation.title,
                        translated_preview_content: similar_translation.preview_content,
                        translated_content: similar_translation.content,
                    }));
                }
            }
        }

        for (metadata, score) in similar.iter().take(3) {
            tracing::info!(
                "  Similar: score={:.3} post_id={} lang={} title={} {}",
                score,
                metadata.post_id,
                metadata.language_code,
                metadata.title,
                if *score >= SIMILARITY_REUSE_THRESHOLD {
                    "(REUSABLE)"
                } else {
                    "(below threshold)"
                }
            );
        }

        Ok(None)
    }

    /// OpenAI translation: Translate title, preview, and content.
    ///
    /// The returned tuple is gated by
    /// [`super::translation_validator::validate_paragraph_coverage`] before
    /// the caller hands it to [`Self::save_translation`]. A translation
    /// that drops too many source paragraphs is rejected with
    /// `AppError::Validation("translation", ...)` so a broken translation
    /// never overwrites a valid `post_translations` row.
    ///
    /// See `openspec/changes/fix-translator-incomplete-html-output/specs/domain-posts/spec.md`
    /// (Requirement: Translation output SHALL preserve paragraph coverage)
    /// and `design.md` Decision 3 for the placement rationale.
    async fn translate_from_openai(
        post: &posts::Model,
        target_language_code: &str,
        openai_api_key: &str,
        model: &str,
    ) -> Result<(String, String, String), AppError> {
        let config = OpenAIConfig::new().with_api_key(openai_api_key);
        let client = Client::with_config(config);

        let translated_title = Self::translate_text_internal(
            &client,
            &post.title,
            target_language_code,
            "title",
            model,
        )
        .await?;

        let translated_preview_content = if let Some(preview) = &post.preview_content {
            Self::translate_text_internal(&client, preview, target_language_code, "preview", model)
                .await?
        } else {
            String::new()
        };

        let translated_content = Self::translate_large_content_internal(
            &client,
            &post.content,
            target_language_code,
            model,
        )
        .await?;

        // Validate paragraph coverage before returning the tuple. A translation
        // that drops most of the source's paragraphs is treated as a translation
        // failure: the caller will not reach `save_translation`, so the existing
        // row in `post_translations` is preserved.
        //
        // See openspec/changes/fix-translator-incomplete-html-output/design.md
        // (Decision 1) and specs/domain-posts/spec.md
        // (Requirement: Translation output SHALL preserve paragraph coverage).
        let source_count = super::translation_validator::count_paragraph_tags(&post.content);
        let translated_count =
            super::translation_validator::count_paragraph_tags(&translated_content);
        if let Err(err) = super::translation_validator::validate_paragraph_coverage(
            &post.content,
            &translated_content,
        ) {
            tracing::warn!(
                post_id = %post.id,
                target_language_code = %target_language_code,
                source_paragraph_count = source_count,
                translated_paragraph_count = translated_count,
                "translation rejected: insufficient paragraph coverage"
            );
            return Err(err);
        }

        Ok((
            translated_title,
            translated_preview_content,
            translated_content,
        ))
    }

    /// Save translation to database
    async fn save_translation(
        db: &DatabaseConnection,
        post_id: Uuid,
        language_code: &str,
        title: &str,
        preview_content: &str,
        content: &str,
    ) -> Result<Uuid, AppError> {
        let post_translation_id = Uuid::new_v4();
        let slug = slugify!(title, max_length = 100);

        let translation_model = post_translations::ActiveModel {
            id: sea_orm::Set(post_translation_id),
            post_id: sea_orm::Set(post_id),
            language_code: sea_orm::Set(language_code.to_string()),
            title: sea_orm::Set(title.to_string()),
            slug: sea_orm::Set(slug),
            preview_content: sea_orm::Set(preview_content.to_string()),
            content: sea_orm::Set(content.to_string()),
        };

        post_translations::Entity::insert(translation_model)
            .exec(db)
            .await
            .map_err(AppError::from)?;

        Ok(post_translation_id)
    }

    /// Store translation in vector database
    async fn store_in_vector_db(
        vector_store: &Option<Arc<crate::handlers::vector_store::VectorStore>>,
        post_id: Uuid,
        language_code: &str,
        translation_id: Uuid,
        title: &str,
        content: &str,
    ) {
        let Some(vector_store) = vector_store else {
            tracing::info!(
                "Vector store not configured - skipping embedding storage for post_id={} language={}",
                post_id,
                language_code
            );
            return;
        };

        tracing::info!(
            "Vector store is configured - attempting to store translation embedding for post_id={} language={}",
            post_id,
            language_code
        );

        let content_for_embedding = format!(
            "{}\n\n{}",
            title,
            if content.len() > 8000 {
                &content[..8000]
            } else {
                content
            }
        );

        match vector_store
            .store_translation(
                post_id,
                language_code,
                translation_id,
                title,
                &content_for_embedding,
            )
            .await
        {
            Ok(_) => {
                tracing::info!(
                    "✓ Successfully stored translation embedding in Qdrant for post_id={} language={} translation_id={}",
                    post_id,
                    language_code,
                    translation_id
                );
            }
            Err(e) => {
                tracing::error!(
                    "✗ Failed to store translation embedding for post_id={} language={}: {}",
                    post_id,
                    language_code,
                    e
                );
            }
        }
    }
}

impl PostTranslateHandlerTrait for PostTranslateHandler {
    #[instrument]
    async fn handle_translate_post(
        &self,
        request: TranslatePostRequest,
        openai_api_key: String,
    ) -> Result<TranslatePostResponse, AppError> {
        // Fetch the post from database
        let post = posts::Entity::find_by_id(request.post_id)
            .one(self.db.as_ref())
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound)?;

        // Get model from request or use default
        let model = request.model.as_deref().unwrap_or(DEFAULT_OPENAI_MODEL);

        if request.force_retranslate {
            // Force retranslate: translate from OpenAI first, THEN replace existing
            // This is safer than deleting first: the old translation remains available
            // until translation succeeds. If translation fails, the old one is preserved.
            // Note: There's still a small window between delete and save where data could be lost
            // if save fails. Future improvement: use UPDATE or transaction for atomicity.

            tracing::info!(
                "Force retranslation requested for post_id={} language={} using model={}",
                request.post_id,
                request.target_language_code,
                model
            );

            // Step 1: Translate the content using OpenAI
            let (translated_title, translated_preview_content, translated_content) =
                Self::translate_from_openai(
                    &post,
                    &request.target_language_code,
                    &openai_api_key,
                    model,
                )
                .await?;

            // Step 2: Delete the existing translation only AFTER successful translation
            // If translation failed, old translation is preserved
            Self::delete_existing_translation(
                self.db.as_ref(),
                request.post_id,
                &request.target_language_code,
            )
            .await?;

            // Step 3: Save the new translation
            // Note: If this fails, old translation was already deleted (small risk window)
            let post_translation_id = Self::save_translation(
                self.db.as_ref(),
                request.post_id,
                &request.target_language_code,
                &translated_title,
                &translated_preview_content,
                &translated_content,
            )
            .await?;

            Self::store_in_vector_db(
                &self.vector_store,
                request.post_id,
                &request.target_language_code,
                post_translation_id,
                &translated_title,
                &translated_content,
            )
            .await;

            return Ok(TranslatePostResponse {
                post_translation_id,
                post_id: request.post_id,
                language_code: request.target_language_code,
                translated_title,
                translated_preview_content,
                translated_content,
                reused_from_similar: None,
            });
        }

        // 3-tier lookup strategy when force_retranslate=false:
        // 1. Check database first
        if let Some(existing) = Self::lookup_existing_translation(
            self.db.as_ref(),
            request.post_id,
            &request.target_language_code,
        )
        .await?
        {
            tracing::info!(
                "Reusing existing translation for post_id={} language={}",
                request.post_id,
                request.target_language_code
            );
            return Ok(TranslatePostResponse {
                post_translation_id: existing.id,
                post_id: existing.post_id,
                language_code: existing.language_code.clone(),
                translated_title: existing.title.clone(),
                translated_preview_content: existing.preview_content.clone(),
                translated_content: existing.content.clone(),
                reused_from_similar: None,
            });
        }

        // 2. Check Qdrant for similar translations (score >= 0.95)
        if let Some(similar_info) = Self::find_similar_translation(
            &self.vector_store,
            self.db.as_ref(),
            &post,
            request.post_id,
            &request.target_language_code,
        )
        .await?
        {
            tracing::info!(
                "🎯 SMART REUSE: Found highly similar translation (score={:.3}, threshold={:.2})",
                similar_info.similarity_score,
                SIMILARITY_REUSE_THRESHOLD
            );

            let post_translation_id = Self::save_translation(
                self.db.as_ref(),
                request.post_id,
                &request.target_language_code,
                &similar_info.translated_title,
                &similar_info.translated_preview_content,
                &similar_info.translated_content,
            )
            .await?;

            Self::store_in_vector_db(
                &self.vector_store,
                request.post_id,
                &request.target_language_code,
                post_translation_id,
                &similar_info.translated_title,
                &similar_info.translated_content,
            )
            .await;

            return Ok(TranslatePostResponse {
                post_translation_id,
                post_id: request.post_id,
                language_code: request.target_language_code,
                translated_title: similar_info.translated_title,
                translated_preview_content: similar_info.translated_preview_content,
                translated_content: similar_info.translated_content,
                reused_from_similar: Some(super::translate_response::ReusedTranslationInfo {
                    source_translation_id: similar_info.source_translation_id,
                    similarity_score: similar_info.similarity_score,
                    source_post_id: similar_info.source_post_id,
                }),
            });
        }

        // 3. Translate from OpenAI
        tracing::info!(
            "No existing or similar translation found, translating from OpenAI for post_id={} language={} using model={}",
            request.post_id,
            request.target_language_code,
            model
        );

        let (translated_title, translated_preview_content, translated_content) =
            Self::translate_from_openai(
                &post,
                &request.target_language_code,
                &openai_api_key,
                model,
            )
            .await?;

        let post_translation_id = Self::save_translation(
            self.db.as_ref(),
            request.post_id,
            &request.target_language_code,
            &translated_title,
            &translated_preview_content,
            &translated_content,
        )
        .await?;

        Self::store_in_vector_db(
            &self.vector_store,
            request.post_id,
            &request.target_language_code,
            post_translation_id,
            &translated_title,
            &translated_content,
        )
        .await;

        Ok(TranslatePostResponse {
            post_translation_id,
            post_id: request.post_id,
            language_code: request.target_language_code,
            translated_title,
            translated_preview_content,
            translated_content,
            reused_from_similar: None,
        })
    }

    #[instrument]
    async fn handle_translate_post_background(
        &self,
        request: TranslatePostRequest,
        openai_api_key: String,
    ) -> Result<Uuid, AppError> {
        // Generate job ID upfront
        let job_id = Uuid::new_v4();

        // Get model from request or use default
        let model = request
            .model
            .clone()
            .unwrap_or_else(|| DEFAULT_OPENAI_MODEL.to_string());

        // Create job record in database
        let job = translation_jobs::ActiveModel {
            id: Set(job_id),
            post_id: Set(request.post_id),
            target_language: Set(request.target_language_code.clone()),
            status: Set("pending".to_string()),
            progress: Set(0),
            error_message: Set(None),
            ai_model: Set(model.clone()),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
        };

        job.insert(self.db.as_ref()).await.map_err(|e| {
            tracing::error!("Failed to create translation job: {}", e);
            AppError::Db(e)
        })?;

        tracing::info!(
            "Created translation job_id={} for post_id={} language={} model={}",
            job_id,
            request.post_id,
            request.target_language_code,
            model
        );

        // Clone necessary data for background task
        let db = self.db.clone();
        let vector_store = self.vector_store.clone();
        let post_id = request.post_id;
        let language_code = request.target_language_code.clone();

        // Spawn background task for translation
        tokio::spawn(async move {
            // Update status to processing
            if let Err(e) =
                Self::update_job_status(db.as_ref(), job_id, "processing", 10, None).await
            {
                tracing::error!("Failed to update job status to processing: {}", e);
            }

            let handler = PostTranslateHandler {
                db: db.clone(),
                vector_store,
            };

            match handler.handle_translate_post(request, openai_api_key).await {
                Ok(_) => {
                    tracing::info!(
                        "Background translation completed successfully for job_id={} post_id={} language={}",
                        job_id,
                        post_id,
                        language_code
                    );
                    // Update status to completed
                    if let Err(e) =
                        Self::update_job_status(db.as_ref(), job_id, "completed", 100, None).await
                    {
                        tracing::error!("Failed to update job status to completed: {}", e);
                    }
                }
                Err(e) => {
                    let error_msg = format!("{}", e);
                    tracing::error!(
                        "Background translation failed for job_id={} post_id={} language={}: {}",
                        job_id,
                        post_id,
                        language_code,
                        error_msg
                    );
                    // Update status to failed
                    if let Err(e) =
                        Self::update_job_status(db.as_ref(), job_id, "failed", 0, Some(error_msg))
                            .await
                    {
                        tracing::error!("Failed to update job status to failed: {}", e);
                    }
                }
            }
        });

        Ok(job_id)
    }
}

impl PostTranslateHandler {
    /// Update job status, progress, and error message
    async fn update_job_status(
        db: &DatabaseConnection,
        job_id: Uuid,
        status: &str,
        progress: i32,
        error_message: Option<String>,
    ) -> Result<(), AppError> {
        use translation_jobs::Entity;

        if let Some(job) = Entity::find_by_id(job_id)
            .one(db)
            .await
            .map_err(|e| AppError::Db(e))?
        {
            let mut job: translation_jobs::ActiveModel = job.into();
            job.status = Set(status.to_string());
            job.progress = Set(progress);
            job.error_message = Set(error_message);
            job.updated_at = Set(Utc::now().into());
            job.update(db).await.map_err(|e| AppError::Db(e))?;
        }

        Ok(())
    }

    /// Checks if content appears to be HTML
    fn is_html_content(text: &str) -> bool {
        text.contains('<')
            && text.contains('>')
            && (text.contains("<p")
                || text.contains("<div")
                || text.contains("<span")
                || text.contains("<h")
                || text.contains("<br")
                || text.contains("<li")
                || text.contains("<ul")
                || text.contains("<ol")
                || text.contains("<a"))
    }

    /// Serializes HTML node to string
    fn serialize_node(handle: &Handle) -> String {
        use html5ever::serialize::{serialize, SerializeOpts, TraversalScope};
        use markup5ever_rcdom::SerializableHandle;

        let mut bytes = Vec::new();
        let serializable = SerializableHandle::from(handle.clone());
        serialize(
            &mut bytes,
            &serializable,
            SerializeOpts {
                traversal_scope: TraversalScope::IncludeNode,
                ..Default::default()
            },
        )
        .ok();
        String::from_utf8_lossy(&bytes).to_string()
    }

    /// Chunks HTML content by block-level elements to preserve structure
    fn chunk_html_content(html: &str, max_chunk_size: usize) -> Vec<String> {
        // Parse HTML
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut Cursor::new(html.as_bytes()))
            .unwrap();

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        // Process each top-level child node
        for child in dom.document.children.borrow().iter() {
            let serialized = Self::serialize_node(child);

            // Skip if it's just the document type declaration or empty
            if serialized.trim().is_empty() || serialized.starts_with("<!DOCTYPE") {
                continue;
            }

            // If adding this node would exceed size, start new chunk
            if current_chunk.len() + serialized.len() > max_chunk_size && !current_chunk.is_empty()
            {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }

            current_chunk.push_str(&serialized);
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        // If no chunks were created (e.g., very large single element), fall back to simpler strategy
        if chunks.is_empty() && !html.is_empty() {
            // Try to split by block-level tags
            let block_tags = [
                "</p>",
                "</div>",
                "</section>",
                "</article>",
                "</li>",
                "</h1>",
                "</h2>",
                "</h3>",
                "</h4>",
                "</h5>",
                "</h6>",
            ];

            for tag in block_tags {
                if html.contains(tag) {
                    let parts: Vec<&str> = html.split(tag).collect();
                    let mut temp_chunk = String::new();

                    for (i, part) in parts.iter().enumerate() {
                        let segment = if i < parts.len() - 1 {
                            format!("{}{}", part, tag)
                        } else {
                            part.to_string()
                        };

                        if temp_chunk.len() + segment.len() > max_chunk_size
                            && !temp_chunk.is_empty()
                        {
                            chunks.push(temp_chunk.clone());
                            temp_chunk.clear();
                        }
                        temp_chunk.push_str(&segment);
                    }

                    if !temp_chunk.is_empty() {
                        chunks.push(temp_chunk);
                    }

                    if !chunks.is_empty() {
                        return chunks;
                    }
                }
            }

            // Last resort: split into max_chunk_size pieces (may break HTML)
            for chunk in html.as_bytes().chunks(max_chunk_size) {
                if let Ok(s) = std::str::from_utf8(chunk) {
                    chunks.push(s.to_string());
                }
            }
        }

        chunks
    }

    /// Splits text into chunks at sentence boundaries to avoid breaking mid-sentence
    /// Used for plain text content only
    fn chunk_text(text: &str, max_chunk_size: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        // Split by sentences (simple approach using period, exclamation, question mark)
        let sentences: Vec<&str> = text.split_inclusive(&['.', '!', '?'][..]).collect();

        for sentence in sentences {
            if current_chunk.len() + sentence.len() > max_chunk_size && !current_chunk.is_empty() {
                chunks.push(current_chunk.clone());
                current_chunk.clear();
            }
            current_chunk.push_str(sentence);
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        // If no chunks were created (no sentence terminators), chunk by size
        if chunks.is_empty() && !text.is_empty() {
            for chunk in text.as_bytes().chunks(max_chunk_size) {
                if let Ok(s) = std::str::from_utf8(chunk) {
                    chunks.push(s.to_string());
                }
            }
        }

        chunks
    }

    /// Translates large content by splitting into chunks and processing in parallel
    async fn translate_large_content_internal(
        client: &Client<OpenAIConfig>,
        content: &str,
        target_language: &str,
        model: &str,
    ) -> Result<String, AppError> {
        // If content is small enough, translate directly
        if content.len() <= MAX_CHUNK_SIZE {
            return Self::translate_text_internal(
                client,
                content,
                target_language,
                "content",
                model,
            )
            .await;
        }

        // Determine if content is HTML and chunk accordingly
        let chunks = if Self::is_html_content(content) {
            Self::chunk_html_content(content, MAX_CHUNK_SIZE)
        } else {
            Self::chunk_text(content, MAX_CHUNK_SIZE)
        };

        let total_chunks = chunks.len();

        tracing::info!(
            "Translating large content: {} characters split into {} chunks (max {} chars per chunk)",
            content.len(),
            total_chunks,
            MAX_CHUNK_SIZE
        );

        // Translate chunks in parallel using JoinSet
        let mut join_set = JoinSet::new();

        for (index, chunk) in chunks.into_iter().enumerate() {
            let client_clone = client.clone();
            let target_language = target_language.to_string();
            let model = model.to_string();
            let is_html = Self::is_html_content(&chunk);
            let chunk_size = chunk.len();

            tracing::debug!(
                "Spawning translation task for chunk {}/{} ({} characters, {})",
                index + 1,
                total_chunks,
                chunk_size,
                if is_html { "HTML" } else { "text" }
            );

            join_set.spawn(async move {
                let config = client_clone.config().clone();
                let new_client = Client::with_config(config);

                let system_message = ChatCompletionRequestSystemMessageArgs::default()
                    .content(format!(
                        "You are a professional translator. Translate the following {} to {}. {}",
                        if is_html { "HTML content" } else { "text" },
                        target_language,
                        if is_html {
                            TRANSLATION_INSTRUCTION_HTML
                        } else {
                            TRANSLATION_INSTRUCTION_TEXT
                        }
                    ))
                    .build()
                    .map_err(|e| AppError::OpenAIError(e.to_string()))?;

                let user_message = ChatCompletionRequestUserMessageArgs::default()
                    .content(chunk.clone())
                    .build()
                    .map_err(|e| AppError::OpenAIError(e.to_string()))?;

                let messages = vec![
                    ChatCompletionRequestMessage::System(system_message),
                    ChatCompletionRequestMessage::User(user_message),
                ];

                // Build request with conditional temperature based on model support
                // GPT-5-nano only supports default temperature (1), not custom values
                let request = if Self::supports_custom_temperature(&model) {
                    CreateChatCompletionRequestArgs::default()
                        .model(model)
                        .messages(messages)
                        .temperature(TRANSLATION_TEMPERATURE)
                        .max_completion_tokens(MAX_TOKENS_PER_REQUEST as u32)
                        .build()
                        .map_err(|e| AppError::OpenAIError(e.to_string()))?
                } else {
                    CreateChatCompletionRequestArgs::default()
                        .model(model)
                        .messages(messages)
                        .max_completion_tokens(MAX_TOKENS_PER_REQUEST as u32)
                        .build()
                        .map_err(|e| AppError::OpenAIError(e.to_string()))?
                };

                let response = new_client
                    .chat()
                    .create(request)
                    .await
                    .map_err(|e| AppError::OpenAIError(e.to_string()))?;

                let translated_text = response
                    .choices
                    .first()
                    .and_then(|choice| choice.message.content.clone())
                    .ok_or_else(|| AppError::OpenAIError("No translation returned".to_string()))?;

                tracing::debug!(
                    "✓ Completed translation for chunk {} ({} chars → {} chars)",
                    index + 1,
                    chunk_size,
                    translated_text.len()
                );

                Ok::<(usize, String), AppError>((index, translated_text))
            });
        }

        // Collect results in order
        let mut translated_chunks: Vec<(usize, String)> = Vec::new();
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(chunk_result)) => {
                    tracing::debug!("Collected chunk {} successfully", chunk_result.0 + 1);
                    translated_chunks.push(chunk_result);
                }
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(AppError::OpenAIError(format!("Task join error: {}", e))),
            }
        }

        // Sort by original index to maintain order
        translated_chunks.sort_by_key(|(index, _)| *index);

        tracing::info!(
            "✓ All {} chunks translated successfully, combining results",
            translated_chunks.len()
        );

        // Combine chunks
        let combined = translated_chunks
            .into_iter()
            .map(|(_, text)| text)
            .collect::<Vec<String>>()
            .join(""); // For HTML, no separator needed

        tracing::info!(
            "✓ Final translation complete: {} characters (from original {} characters)",
            combined.len(),
            content.len()
        );

        Ok(combined)
    }

    async fn translate_text_internal(
        client: &Client<OpenAIConfig>,
        text: &str,
        target_language: &str,
        content_type: &str,
        model: &str,
    ) -> Result<String, AppError> {
        let system_message = ChatCompletionRequestSystemMessageArgs::default()
            .content(format!(
                "You are a professional translator. Translate the following {} to {}. \
                 Only return the translated text without any additional comments or explanations.",
                content_type, target_language
            ))
            .build()
            .map_err(|e| AppError::OpenAIError(e.to_string()))?;

        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(text)
            .build()
            .map_err(|e| AppError::OpenAIError(e.to_string()))?;

        let messages = vec![
            ChatCompletionRequestMessage::System(system_message),
            ChatCompletionRequestMessage::User(user_message),
        ];

        // Build request with conditional temperature based on model support
        // GPT-5-nano only supports default temperature (1), not custom values
        let request = if Self::supports_custom_temperature(model) {
            CreateChatCompletionRequestArgs::default()
                .model(model)
                .messages(messages)
                .temperature(TRANSLATION_TEMPERATURE)
                .max_completion_tokens(MAX_TOKENS_PER_REQUEST as u32)
                .build()
                .map_err(|e| AppError::OpenAIError(e.to_string()))?
        } else {
            CreateChatCompletionRequestArgs::default()
                .model(model)
                .messages(messages)
                .max_completion_tokens(MAX_TOKENS_PER_REQUEST as u32)
                .build()
                .map_err(|e| AppError::OpenAIError(e.to_string()))?
        };

        let response = client
            .chat()
            .create(request)
            .await
            .map_err(|e| AppError::OpenAIError(e.to_string()))?;

        let translated_text = response
            .choices
            .first()
            .and_then(|choice| choice.message.content.clone())
            .ok_or_else(|| AppError::OpenAIError("No translation returned".to_string()))?;

        Ok(translated_text)
    }
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    // Tests for `PostTranslateHandler` are temporarily disabled during the
    // move. The original test fixture `application_core::commands::post::test`
    // is gated by `#[cfg(test)]` on the `application_core` crate and is
    // therefore not visible from `domain_posts`'s test build. The handler is
    // exercised end-to-end through the HTTP adapter tests in
    // `apps/api/test_helpers`.
    fn _placeholder() {
        use std::sync::Arc;
        let _ = Arc::new(());
    }
}
