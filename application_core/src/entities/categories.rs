//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use super::sea_orm_active_enums::CategoryType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub display_name: String,
    pub category_type: CategoryType,
    pub created_by: String,
    pub created_at: DateTimeWithTimeZone,
    pub last_modified_by: Option<String>,
    pub last_modified_at: Option<DateTimeWithTimeZone>,
    pub parent_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::posts::Entity")]
    Posts,
}

impl Related<super::posts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
