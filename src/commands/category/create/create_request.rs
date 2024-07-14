use entity::category::{self, CategoryTypeEnum};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCategoryRequest {
    pub display_name: String,
    pub category_type: CategoryTypeEnum,
    pub parent_id: Option<Uuid>,
}

impl CreateCategoryRequest {
    pub fn into_model(&self) -> category::Model {
        category::Model {
            id: Uuid::new_v4(),
            display_name: self.display_name.to_owned(),
            category_type: self.category_type.to_owned(),
            created_at: chrono::Utc::now(),
            created_by: "System".to_string(),
            last_modified_at: None,
            last_modified_by: None,
            parent_id: self.parent_id.to_owned(),
        }
    }
}
