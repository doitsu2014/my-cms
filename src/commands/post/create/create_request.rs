use application_core::{common::datetime_generator::generate_vietname_now, entities::posts};
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
    pub fn into_model(&self) -> posts::Model {
        posts::Model {
            id: Uuid::new_v4(),
            title: self.title.to_owned(),
            content: self.content.to_owned(),
            slug: self.slug.to_owned(),
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
