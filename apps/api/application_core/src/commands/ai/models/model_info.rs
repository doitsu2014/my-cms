use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIModelInfo {
    pub id: String,
    pub name: String,
    pub input_price_per_1m: f64,
    pub output_price_per_1m: f64,
    pub context_window: u32,
    pub max_output_tokens: u32,
    pub is_recommended: bool,
    pub recommendation_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelsListResponse {
    pub models: Vec<OpenAIModelInfo>,
}

impl OpenAIModelInfo {
    pub fn new(
        id: String,
        name: String,
        input_price_per_1m: f64,
        output_price_per_1m: f64,
        context_window: u32,
        max_output_tokens: u32,
    ) -> Self {
        Self {
            id,
            name,
            input_price_per_1m,
            output_price_per_1m,
            context_window,
            max_output_tokens,
            is_recommended: false,
            recommendation_reason: None,
        }
    }

    pub fn with_recommendation(mut self, reason: String) -> Self {
        self.is_recommended = true;
        self.recommendation_reason = Some(reason);
        self
    }
}
