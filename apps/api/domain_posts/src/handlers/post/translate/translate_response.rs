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
    /// Indicates whether this translation was reused from a similar existing translation
    /// instead of calling OpenAI API. True = reused (cost savings), False = new translation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reused_from_similar: Option<ReusedTranslationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReusedTranslationInfo {
    /// UUID of the source translation that was reused
    pub source_translation_id: Uuid,
    /// Similarity score (0.0 to 1.0) that triggered the reuse
    pub similarity_score: f32,
    /// Post ID of the source post
    pub source_post_id: Uuid,
}
