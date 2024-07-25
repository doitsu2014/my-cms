use application_core::{
    common::datetime_generator::generate_vietname_now, entities::posts::ActiveModel,
};
use sea_orm::{prelude::Uuid, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ModifyPostRequest {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
    pub category_id: Uuid,
    pub row_version: i32,
}

impl ModifyPostRequest {
    pub fn into_active_model(&self) -> ActiveModel {
        ActiveModel {
            title: Set(self.title.to_owned()),
            content: Set(self.content.to_owned()),
            slug: Set(self.slug.to_owned()),
            last_modified_by: Set(Some("System".to_owned())),
            last_modified_at: Set(Some(generate_vietname_now())),
            category_id: Set(self.category_id.to_owned()),
            row_version: Set(self.row_version + 1),
            ..Default::default()
        }
    }
}
