use entity::post;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
    pub category_id: Uuid,
}

impl CreatePostRequest {
    pub fn into_model(&self) -> post::Model {
        post::Model {
            id: Uuid::new_v4(),
            title: self.title.to_owned(),
            content: self.content.to_owned(),
            slug: self.slug.to_owned(),
            published: self.published.to_owned(),
            created_at: chrono::Utc::now(),
            created_by: "System".to_string(),
            last_modified_at: None,
            last_modified_by: None,

            category_id: self.category_id.to_owned(),
        }
    }
}
