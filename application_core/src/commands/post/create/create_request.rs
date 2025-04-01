use crate::{
    common::datetime_generator::generate_vietnam_now,
    entities::{post_translations, posts},
    StringExtension,
};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostRequest {
    pub title: String,
    pub preview_content: Option<String>,
    pub content: String,
    pub published: bool,
    pub tag_names: Option<Vec<String>>,
    pub thumbnail_paths: Vec<String>,
    pub category_id: Uuid,
    pub translations: Option<Vec<CreatePostTranslationRequest>>,
}

impl CreatePostRequest {
    pub fn into_model(&self) -> posts::Model {
        posts::Model {
            id: Uuid::new_v4(),
            title: self.title.to_owned(),
            preview_content: self.preview_content.to_owned(),
            content: self.content.to_owned(),
            slug: self.title.to_slug(),
            published: self.published.to_owned(),
            category_id: self.category_id.to_owned(),
            thumbnail_paths: self.thumbnail_paths.to_owned(),
            created_at: generate_vietnam_now(),
            created_by: "System".to_string(),
            last_modified_at: None,
            last_modified_by: None,
            row_version: 1,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostTranslationRequest {
    pub title: String,
    pub language_code: String,
    pub preview_content: Option<String>,
    pub content: String,
}

impl CreatePostTranslationRequest {
    pub fn into_model(&self) -> post_translations::Model {
        post_translations::Model {
            id: Uuid::new_v4(),
            slug: self.title.clone().to_slug(),
            title: self.title.to_owned(),
            language_code: self.language_code.to_owned(),
            post_id: Uuid::new_v4(), // This will be set later when the post is created
            preview_content: match self.preview_content {
                Some(ref preview_content_unwrapped) => preview_content_unwrapped.to_owned(),
                None => "".to_string(),
            },
            content: self.content.to_owned(),
        }
    }
}
