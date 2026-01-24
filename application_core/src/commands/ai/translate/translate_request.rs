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
}

impl TranslatePostRequest {
    pub fn new(post_id: Uuid, target_language_code: String) -> Self {
        Self {
            post_id,
            target_language_code,
            force_retranslate: false,
        }
    }
    
    pub fn with_force_retranslate(mut self, force: bool) -> Self {
        self.force_retranslate = force;
        self
    }
}
