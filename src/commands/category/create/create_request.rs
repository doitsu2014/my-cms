use entity::category::{self, CategoryTypeEnum};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateRequest {
    display_name: String,
    category_type: CategoryTypeEnum,
}

impl CreateRequest {
    pub fn into_model(&self) -> category::Model {
        category::Model {
            id: Uuid::new_v4(),
            display_name: self.display_name.to_owned(),
            category_type: self.category_type.to_owned(),
        }
    }
}
