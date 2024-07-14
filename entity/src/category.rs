pub use super::category::Entity as ParentCategory;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(Some(1))",
    enum_name = "category_type"
)]
pub enum CategoryTypeEnum {
    #[sea_orm(string_value = "Blog")]
    Blog,
    #[sea_orm(string_value = "Other")]
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub display_name: String,
    pub category_type: CategoryTypeEnum,
    pub created_at: DateTimeUtc,
    pub created_by: String,
    pub last_modified_at: Option<DateTimeUtc>,
    pub last_modified_by: Option<String>,

    pub parent_id: Option<Uuid>, // Optional parent_id for self-referencing
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "ParentCategory",
        from = "Column::ParentId",
        to = "Column::Id",
        on_delete = "Cascade"
    )]
    Parent,
}

impl ActiveModelBehavior for ActiveModel {}
