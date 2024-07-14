use chrono::Utc;
use entity::post;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestCreatePost {
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
    pub category_id: Uuid,
}

impl RequestCreatePost {
    pub fn into_model(&self) -> post::Model {
        post::Model {
            id: Default::default(),
            title: self.title.to_owned(),
            content: self.content.to_owned(),
            slug: self.slug.to_owned(),
            published: self.published.to_owned(),
            created_at: Utc::now().naive_utc(),
            created_by: "System".to_string(),
            last_modified_at: Utc::now().naive_utc(),
            last_modified_by: "System".to_string(),
            category_id: self.category_id.to_owned(),
        }
    }
}
