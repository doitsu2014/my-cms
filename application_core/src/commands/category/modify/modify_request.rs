use crate::{
    common::datetime_generator::generate_vietname_now,
    entities::{categories, sea_orm_active_enums::CategoryType},
};
use sea_orm::{prelude::Uuid, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ModifyCategoryRequest {
    pub id: Uuid,
    pub display_name: String,
    pub slug: String,
    pub category_type: CategoryType,
    pub parent_id: Option<Uuid>,
    pub row_version: i32,
}

impl ModifyCategoryRequest {
    pub fn into_active_model(&self) -> categories::ActiveModel {
        categories::ActiveModel {
            display_name: Set(self.display_name.to_owned()),
            category_type: Set(self.category_type.to_owned()),
            slug: Set(self.slug.to_owned()),
            last_modified_at: Set(Some(generate_vietname_now())),
            last_modified_by: Set(Some("System".to_owned())),
            row_version: Set(self.row_version + 1),
            parent_id: Set(self.parent_id.to_owned()),
            ..Default::default()
        }
    }
}
