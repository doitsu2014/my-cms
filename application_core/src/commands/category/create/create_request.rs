use crate::{
    common::datetime_generator::generate_vietname_now,
    entities::{categories, sea_orm_active_enums::CategoryType},
    StringExtension,
};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCategoryRequest {
    pub display_name: String,
    pub category_type: CategoryType,
    pub parent_id: Option<Uuid>,
    pub tag_names: Option<Vec<String>>,
}

impl CreateCategoryRequest {
    pub fn into_model(&self) -> categories::Model {
        categories::Model {
            id: Uuid::new_v4(),
            slug: self.display_name.clone().to_slug(),
            display_name: self.display_name.to_owned(),
            category_type: self.category_type.to_owned(),
            created_at: generate_vietname_now(),
            created_by: "System".to_string(),
            last_modified_at: None,
            last_modified_by: None,
            row_version: 1,
            parent_id: self.parent_id.to_owned(),
        }
    }
}
