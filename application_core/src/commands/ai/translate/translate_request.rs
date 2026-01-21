use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatePostRequest {
    pub post_id: Uuid,
    pub target_language_code: String,
}

impl TranslatePostRequest {
    pub fn new(post_id: Uuid, target_language_code: String) -> Self {
        Self {
            post_id,
            target_language_code,
        }
    }
}
