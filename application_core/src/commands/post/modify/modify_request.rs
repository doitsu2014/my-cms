use crate::{entities::posts::ActiveModel, StringExtension};
use sea_orm::{prelude::Uuid, Set};
use serde::{Deserialize, Serialize};

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
            ..Default::default()
        }
    }
}
