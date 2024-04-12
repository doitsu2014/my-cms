use chrono::Utc;
use entity::{post, prelude::*};
use sea_orm::ActiveValue;

pub struct RequestCreatePost {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
}

impl RequestCreatePost {
    pub fn into_model(self) -> post::Model {
        post::Model {
            id: 0,
            title: self.title,
            content: self.content,
            slug: self.slug,
            published: self.published,
            created_at: Utc::now().naive_utc(),
            created_by: "System".to_string(),
            last_modified_at: Utc::now().naive_utc(),
            last_modified_by: "System".to_string(),
        }
    }
}
