use crate::common::app_error::AppError;
use super::model_info::{ModelsListResponse, OpenAIModelInfo};
use async_openai::{config::OpenAIConfig, Client};
use tracing::instrument;

pub trait ModelsHandlerTrait {
    fn get_available_models(
        &self,
        openai_api_key: String,
    ) -> impl std::future::Future<Output = Result<ModelsListResponse, AppError>>;
}

#[derive(Debug)]
pub struct ModelsHandler;

impl ModelsHandler {
    pub fn new() -> Self {
        Self
    }

    /// Get hardcoded list of OpenAI models with pricing information
    /// Note: OpenAI's list models API doesn't include pricing, so we maintain
    /// a curated list of commonly used models with their current pricing
    fn get_hardcoded_models() -> Vec<OpenAIModelInfo> {
        vec![
            // GPT-4o models - most capable
            OpenAIModelInfo::new(
                "gpt-4o".to_string(),
                "GPT-4o".to_string(),
                2.50,  // $2.50 per 1M input tokens
                10.00, // $10.00 per 1M output tokens
                128000,
                16384,
            ),
            OpenAIModelInfo::new(
                "gpt-4o-2024-11-20".to_string(),
                "GPT-4o (2024-11-20)".to_string(),
                2.50,
                10.00,
                128000,
                16384,
            ),
            // GPT-4o-mini - best balance (RECOMMENDED)
            OpenAIModelInfo::new(
                "gpt-4o-mini".to_string(),
                "GPT-4o Mini".to_string(),
                0.15,  // $0.15 per 1M input tokens
                0.60,  // $0.60 per 1M output tokens
                128000,
                16384,
            )
            .with_recommendation(
                "Best balance of cost and quality for translations. 15x cheaper than GPT-4o with excellent results.".to_string()
            ),
            OpenAIModelInfo::new(
                "gpt-4o-mini-2024-07-18".to_string(),
                "GPT-4o Mini (2024-07-18)".to_string(),
                0.15,
                0.60,
                128000,
                16384,
            ),
            // GPT-4 Turbo models
            OpenAIModelInfo::new(
                "gpt-4-turbo".to_string(),
                "GPT-4 Turbo".to_string(),
                10.00, // $10.00 per 1M input tokens
                30.00, // $30.00 per 1M output tokens
                128000,
                4096,
            ),
            OpenAIModelInfo::new(
                "gpt-4-turbo-2024-04-09".to_string(),
                "GPT-4 Turbo (2024-04-09)".to_string(),
                10.00,
                30.00,
                128000,
                4096,
            ),
            // GPT-4 models
            OpenAIModelInfo::new(
                "gpt-4".to_string(),
                "GPT-4".to_string(),
                30.00, // $30.00 per 1M input tokens
                60.00, // $60.00 per 1M output tokens
                8192,
                8192,
            ),
            // GPT-3.5 Turbo - most economical
            OpenAIModelInfo::new(
                "gpt-3.5-turbo".to_string(),
                "GPT-3.5 Turbo".to_string(),
                0.50,  // $0.50 per 1M input tokens
                1.50,  // $1.50 per 1M output tokens
                16385,
                4096,
            )
            .with_recommendation(
                "Most economical option. Good for simple translations where cost is a primary concern.".to_string()
            ),
            OpenAIModelInfo::new(
                "gpt-3.5-turbo-0125".to_string(),
                "GPT-3.5 Turbo (0125)".to_string(),
                0.50,
                1.50,
                16385,
                4096,
            ),
        ]
    }
}

impl ModelsHandlerTrait for ModelsHandler {
    #[instrument(name = "get_available_models", skip(self, openai_api_key))]
    async fn get_available_models(
        &self,
        openai_api_key: String,
    ) -> Result<ModelsListResponse, AppError> {
        // Create OpenAI client to verify API key is valid
        let config = OpenAIConfig::new().with_api_key(&openai_api_key);
        let _client = Client::with_config(config);

        // Get hardcoded list with pricing information
        let models = Self::get_hardcoded_models();

        Ok(ModelsListResponse { models })
    }
}
