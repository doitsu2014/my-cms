pub use super::category::Entity as Category;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub published: bool,
    pub category_id: Uuid,
    pub created_at: DateTime,
    pub created_by: String,
    pub last_modified_at: DateTime,
    pub last_modified_by: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Category",
        from = "Column::CategoryId",
        to = "crate::category::Column::Id",
        on_delete = "Cascade"
    )]
    Category,
}

impl ActiveModelBehavior for ActiveModel {}
