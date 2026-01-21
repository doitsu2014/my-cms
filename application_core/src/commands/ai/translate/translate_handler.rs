use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom::{Handle, NodeData, RcDom};
use sea_orm::{DatabaseConnection, EntityTrait};
use slugify::slugify;
use std::io::Cursor;
use std::sync::Arc;
use tokio::task::JoinSet;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    common::app_error::AppError,
    entities::{post_translations, posts},
};

use super::{translate_request::TranslatePostRequest, translate_response::TranslatePostResponse};

// Default OpenAI model to use for translation
const DEFAULT_OPENAI_MODEL: &str = "gpt-4o-mini";

// Maximum chunk size in characters for content translation
// Large content will be split into chunks to avoid token limits
const MAX_CHUNK_SIZE: usize = 2000;

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
            .map_err(|e| e.into())?
            .ok_or(AppError::NotFound)?;

        // Initialize OpenAI client with API key
        let config = OpenAIConfig::new().with_api_key(openai_api_key);
        let client = Client::with_config(config);

        // Translate title
        let translated_title = self
            .translate_text(
                &client,
                &post.title,
                &request.target_language_code,
                "title",
            )
            .await?;

        // Translate preview content
        // Note: If original preview_content is None, we set it to empty string
        // as the database schema requires a non-nullable String
        let translated_preview_content = if let Some(preview) = &post.preview_content {
            self.translate_text(&client, preview, &request.target_language_code, "preview")
                .await?
        } else {
            String::new()
        };

        // Translate content with chunking for large content
        let translated_content = self
            .translate_large_content(
                &client,
                &post.content,
                &request.target_language_code,
            )
            .await?;

        // Generate a slug from the translated title
        let translated_slug = slugify!(&translated_title);

        // Save translation to database
        let post_translation_id = Uuid::new_v4();
        let translation_model = post_translations::ActiveModel {
            id: sea_orm::Set(post_translation_id),
            post_id: sea_orm::Set(request.post_id),
            language_code: sea_orm::Set(request.target_language_code.clone()),
            title: sea_orm::Set(translated_title.clone()),
            slug: sea_orm::Set(translated_slug),
            preview_content: sea_orm::Set(translated_preview_content.clone()),
            content: sea_orm::Set(translated_content.clone()),
        };

        post_translations::Entity::insert(translation_model)
            .exec(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        Ok(TranslatePostResponse {
            post_translation_id,
            post_id: request.post_id,
            language_code: request.target_language_code,
            translated_title,
            translated_preview_content,
            translated_content,
        })
    }

    #[instrument]
    async fn handle_translate_post_background(
        &self,
        request: TranslatePostRequest,
        openai_api_key: String,
    ) -> Result<Uuid, AppError> {
        // Generate translation ID upfront
        let post_translation_id = Uuid::new_v4();
        
        // Clone necessary data for background task
        let db = self.db.clone();
        let post_id = request.post_id;
        let language_code = request.target_language_code.clone();
        
        // Spawn background task for translation
        tokio::spawn(async move {
            let handler = PostTranslateHandler { db };
            match handler.handle_translate_post(request, openai_api_key).await {
                Ok(_) => {
                    tracing::info!(
                        "Background translation completed successfully for post_id={} to language={}",
                        post_id,
                        language_code
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Background translation failed for post_id={} to language={}: {}",
                        post_id,
                        language_code,
                        e
                    );
                }
            }
        });
        
        Ok(post_translation_id)
    }
}

impl PostTranslateHandler {
    /// Checks if content appears to be HTML
    fn is_html_content(&self, text: &str) -> bool {
        // Simple heuristic: check for common HTML tags
        text.contains('<') && text.contains('>') && 
        (text.contains("<p") || text.contains("<div") || text.contains("<span") || 
         text.contains("<h") || text.contains("<br") || text.contains("<li") ||
         text.contains("<ul") || text.contains("<ol") || text.contains("<a"))
    }

    /// Serializes HTML node to string
    fn serialize_node(&self, handle: &Handle) -> String {
        use html5ever::serialize::{serialize, SerializeOpts, TraversalScope};
        use markup5ever_rcdom::SerializableHandle;
        
        let mut bytes = Vec::new();
        let serializable = SerializableHandle::from(handle.clone());
        serialize(&mut bytes, &serializable, SerializeOpts {
            traversal_scope: TraversalScope::IncludeNode,
            ..Default::default()
        }).ok();
        String::from_utf8_lossy(&bytes).to_string()
    }

    /// Chunks HTML content by block-level elements to preserve structure
    fn chunk_html_content(&self, html: &str, max_chunk_size: usize) -> Vec<String> {
        // Parse HTML
        let dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut Cursor::new(html.as_bytes()))
            .unwrap();

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();

        // Process each top-level child node
        for child in dom.document.children.borrow().iter() {
            let serialized = self.serialize_node(child);
            
            // Skip if it's just the document type declaration or empty
            if serialized.trim().is_empty() || serialized.starts_with("<!DOCTYPE") {
                continue;
            }

            // If adding this node would exceed size, start new chunk
            if current_chunk.len() + serialized.len() > max_chunk_size && !current_chunk.is_empty() {
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
            let block_tags = ["</p>", "</div>", "</section>", "</article>", "</li>", "</h1>", "</h2>", "</h3>", "</h4>", "</h5>", "</h6>"];
            
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

                        if temp_chunk.len() + segment.len() > max_chunk_size && !temp_chunk.is_empty() {
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
    fn chunk_text(&self, text: &str, max_chunk_size: usize) -> Vec<String> {
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
    async fn translate_large_content(
        &self,
        client: &Client<OpenAIConfig>,
        content: &str,
        target_language: &str,
    ) -> Result<String, AppError> {
        // If content is small enough, translate directly
        if content.len() <= MAX_CHUNK_SIZE {
            return self.translate_text(client, content, target_language, "content").await;
        }
        
        // Determine if content is HTML and chunk accordingly
        let chunks = if self.is_html_content(content) {
            self.chunk_html_content(content, MAX_CHUNK_SIZE)
        } else {
            self.chunk_text(content, MAX_CHUNK_SIZE)
        };
        
        // Translate chunks in parallel using JoinSet
        let mut join_set = JoinSet::new();
        
        for (index, chunk) in chunks.into_iter().enumerate() {
            let client_clone = client.clone();
            let target_language = target_language.to_string();
            let is_html = self.is_html_content(&chunk);
            
            join_set.spawn(async move {
                let config = client_clone.config().clone();
                let new_client = Client::with_config(config);
                
                let system_message = ChatCompletionRequestSystemMessageArgs::default()
                    .content(format!(
                        "You are a professional translator. Translate the following {} to {}. {}",
                        if is_html { "HTML content" } else { "text" },
                        target_language,
                        if is_html {
                            "Preserve all HTML tags and structure exactly as they are. Only translate the text content within the tags, never translate HTML tag names, attributes, or structure. Return valid HTML."
                        } else {
                            "Only return the translated text without any additional comments or explanations."
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

                let request = CreateChatCompletionRequestArgs::default()
                    .model(DEFAULT_OPENAI_MODEL)
                    .messages(messages)
                    .build()
                    .map_err(|e| AppError::OpenAIError(e.to_string()))?;

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

                Ok::<(usize, String), AppError>((index, translated_text))
            });
        }
        
        // Collect results in order
        let mut translated_chunks: Vec<(usize, String)> = Vec::new();
        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(chunk_result)) => translated_chunks.push(chunk_result),
                Ok(Err(e)) => return Err(e),
                Err(e) => return Err(AppError::OpenAIError(format!("Task join error: {}", e))),
            }
        }
        
        // Sort by original index to maintain order
        translated_chunks.sort_by_key(|(index, _)| *index);
        
        // Combine chunks
        let combined = translated_chunks
            .into_iter()
            .map(|(_, text)| text)
            .collect::<Vec<String>>()
            .join("");  // For HTML, no separator needed
        
        Ok(combined)
    }

    async fn translate_text(
        &self,
        client: &Client<OpenAIConfig>,
        text: &str,
        target_language: &str,
        content_type: &str,
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

        let request = CreateChatCompletionRequestArgs::default()
            .model(DEFAULT_OPENAI_MODEL)
            .messages(messages)
            .build()
            .map_err(|e| AppError::OpenAIError(e.to_string()))?;

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
mod tests {
    use super::*;
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::commands::{
        category::{
            create::create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
            test::fake_create_category_request,
        },
        post::{
            create::create_handler::{PostCreateHandler, PostCreateHandlerTrait},
            test::fake_create_post_request,
        },
    };

    #[async_std::test]
    #[ignore] // Ignore by default as it requires OpenAI API key
    async fn handle_translate_post_testcase_successfully() {
        let test_space = setup_test_space().await;
        let database = test_space.postgres.get_database_connection().await;

        let arc_conn = Arc::new(database);

        // Create a category first
        let category_create_handler = CategoryCreateHandler {
            db: arc_conn.clone(),
        };
        let create_category_request = fake_create_category_request(0);
        let created_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_request, None)
            .await
            .unwrap();

        // Create a post
        let post_create_handler = PostCreateHandler {
            db: arc_conn.clone(),
        };
        let create_post_request = fake_create_post_request(created_category_id, 0);
        let created_post_id = post_create_handler
            .handle_create_post(create_post_request, None)
            .await
            .unwrap();

        // Translate the post (requires valid OpenAI API key)
        let _translate_handler = PostTranslateHandler {
            db: arc_conn.clone(),
        };
        let _translate_request = TranslatePostRequest::new(created_post_id, "Vietnamese".to_string());
        
        // This would need a real API key to work
        // let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        // let result = translate_handler
        //     .handle_translate_post(translate_request, api_key)
        //     .await
        //     .unwrap();
        
        // assert_eq!(result.post_id, created_post_id);
        // assert_eq!(result.language_code, "Vietnamese");
        // assert!(!result.translated_title.is_empty());
        // assert!(!result.translated_content.is_empty());
    }
}
