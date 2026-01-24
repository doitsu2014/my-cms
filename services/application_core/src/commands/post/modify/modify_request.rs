use crate::{
    entities::{post_translations, posts::ActiveModel},
    StringExtension,
};
use sea_orm::{prelude::Uuid, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModifyPostTranslationRequest {
    pub id: Option<Uuid>,
    pub title: String,
    pub language_code: String,
    pub preview_content: Option<String>,
    pub content: String,
}

impl ModifyPostTranslationRequest {
    pub fn into_active_model(&self) -> post_translations::ActiveModel {
        post_translations::ActiveModel {
            id: Set(self.id.unwrap_or_else(Uuid::new_v4)),
            title: Set(self.title.to_owned()),
            slug: Set(self.title.to_slug()),
            language_code: Set(self.language_code.to_owned()),
            preview_content: match self.preview_content {
                Some(ref preview_content_unwrapped) => Set(preview_content_unwrapped.to_owned()),
                None => Set("".to_string()),
            },
            content: Set(self.content.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModifyPostRequest {
    pub id: Uuid,
    pub title: String,
    pub preview_content: Option<String>,
    pub content: String,
    pub published: bool,
    pub category_id: Uuid,
    pub row_version: i32,
    pub tag_names: Option<Vec<String>>,
    pub thumbnail_paths: Vec<String>,
    pub translations: Option<Vec<ModifyPostTranslationRequest>>,
}

impl ModifyPostRequest {
    pub fn into_active_model(&self) -> ActiveModel {
        ActiveModel {
            title: Set(self.title.to_owned()),
            slug: Set(self.title.to_slug().to_owned()),
            preview_content: Set(self.preview_content.to_owned()),
            content: Set(self.content.to_owned()),
            category_id: Set(self.category_id.to_owned()),
            row_version: Set(self.row_version + 1),
            thumbnail_paths: Set(self.thumbnail_paths.to_owned()),
            published: Set(self.published),
            ..Default::default()
        }
    }
}
