use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslatePostResponse {
    pub post_translation_id: Uuid,
    pub post_id: Uuid,
    pub language_code: String,
    pub translated_title: String,
    pub translated_preview_content: String,
    pub translated_content: String,
}
