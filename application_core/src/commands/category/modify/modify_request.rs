use crate::{
    common::datetime_generator::generate_vietnam_now,
    entities::{categories, category_translations, sea_orm_active_enums::CategoryType},
    StringExtension,
};
use sea_orm::{prelude::Uuid, Set};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModifyCategoryTranslationRequest {
    pub id: Option<Uuid>,
    pub display_name: String,
    pub language_code: String,
    pub slug: String,
}

impl ModifyCategoryTranslationRequest {
    pub fn into_active_model(&self) -> category_translations::ActiveModel {
        category_translations::ActiveModel {
            id: Set(match self.id {
                Some(id) => id,
                None => Uuid::new_v4(),
            }),
            display_name: Set(self.display_name.to_owned()),
            slug: Set(self.display_name.to_slug()),
            language_code: Set(self.language_code.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModifyCategoryRequest {
    pub id: Uuid,
    pub display_name: String,
    pub category_type: CategoryType,
    pub parent_id: Option<Uuid>,
    pub row_version: i32,
    pub tag_names: Option<Vec<String>>,
    pub translations: Option<Vec<ModifyCategoryTranslationRequest>>,
}

impl ModifyCategoryRequest {
    pub fn into_active_model(&self) -> categories::ActiveModel {
        categories::ActiveModel {
            display_name: Set(self.display_name.to_owned()),
            category_type: Set(self.category_type.to_owned()),
            slug: Set(self.display_name.to_slug()),
            last_modified_at: Set(Some(generate_vietnam_now())),
            last_modified_by: Set(Some("System".to_owned())),
            row_version: Set(self.row_version + 1),
            parent_id: Set(self.parent_id.to_owned()),
            ..Default::default()
        }
    }
}
