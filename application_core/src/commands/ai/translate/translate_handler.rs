use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use sea_orm::{DatabaseConnection, EntityTrait};
use slugify::slugify;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    common::app_error::AppError,
    entities::{post_translations, posts},
};

use super::{translate_request::TranslatePostRequest, translate_response::TranslatePostResponse};

pub trait PostTranslateHandlerTrait {
    fn handle_translate_post(
        &self,
        request: TranslatePostRequest,
        openai_api_key: String,
    ) -> impl std::future::Future<Output = Result<TranslatePostResponse, AppError>>;
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
        let translated_preview_content = if let Some(preview) = &post.preview_content {
            self.translate_text(&client, preview, &request.target_language_code, "preview")
                .await?
        } else {
            String::new()
        };

        // Translate content
        let translated_content = self
            .translate_text(
                &client,
                &post.content,
                &request.target_language_code,
                "content",
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
}

impl PostTranslateHandler {
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
            .model("gpt-4o-mini")
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
