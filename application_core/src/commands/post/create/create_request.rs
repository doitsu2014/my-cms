use crate::{common::datetime_generator::generate_vietname_now, entities::posts, StringExtension};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreatePostRequest {
    pub title: String,
    pub preview_content: Option<String>,
    pub content: String,
    pub published: bool,
    pub tag_names: Option<Vec<String>>,
    pub category_id: Uuid,
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
            created_at: generate_vietname_now(),
            created_by: "System".to_string(),
            last_modified_at: None,
            last_modified_by: None,
            row_version: 1,
            category_id: self.category_id.to_owned(),
        }
    }
}
