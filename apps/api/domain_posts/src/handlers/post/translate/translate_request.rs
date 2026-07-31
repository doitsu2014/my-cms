use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatePostRequest {
    pub post_id: Uuid,
    pub target_language_code: String,
    /// Force re-translation even if translation already exists
    /// When true, will check Qdrant for similar translations before proceeding
    #[serde(default)]
    pub force_retranslate: bool,
    /// OpenAI model to use for translation (e.g., "gpt-4o-mini", "gpt-4o")
    /// If not specified, defaults to "gpt-4o-mini"
    pub model: Option<String>,
}

impl TranslatePostRequest {
    pub fn new(post_id: Uuid, target_language_code: String) -> Self {
        Self {
            post_id,
            target_language_code,
            force_retranslate: false,
            model: None,
        }
    }

    pub fn with_force_retranslate(mut self, force: bool) -> Self {
        self.force_retranslate = force;
        self
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = Some(model);
        self
    }
}
