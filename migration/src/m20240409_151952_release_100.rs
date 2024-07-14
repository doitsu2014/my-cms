use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Categories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Categories::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Categories::DisplayName).string().not_null())
                    .col(ColumnDef::new(Categories::CategoryType).string().not_null())
                    .col(ColumnDef::new(Categories::CreatedBy).string().not_null())
                    .col(
                        ColumnDef::new(Categories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Categories::LastModifiedBy).string().null())
                    .col(
                        ColumnDef::new(Categories::LastModifiedAt)
                            .timestamp_with_time_zone()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Categories::ParentId).uuid().null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_categories_parent_id")
                            .from(Categories::Table, Categories::ParentId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Posts::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Posts::Title).string().not_null())
                    .col(ColumnDef::new(Posts::Content).string().not_null())
                    .col(ColumnDef::new(Posts::Slug).string().not_null())
                    .col(
                        ColumnDef::new(Posts::Published)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Posts::CreatedBy).string().not_null())
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Posts::LastModifiedBy).string().null())
                    .col(
                        ColumnDef::new(Posts::LastModifiedAt)
                            .timestamp_with_time_zone()
                            .null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Posts::CategoryId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-posts-category_id")
                            .from(Posts::Table, Posts::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the posts table first to remove the foreign key constraint
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await?;
        // Then drop the categories table
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Categories {
    Table,
    Id,
    DisplayName,
    CategoryType,
    CreatedAt,
    CreatedBy,
    LastModifiedAt,
    LastModifiedBy,

    ParentId,
}

#[derive(Iden)]
pub enum Posts {
    Table,
    Id,
    Title,
    Content,
    Slug,
    Published,
    CategoryId,
    CreatedAt,
    CreatedBy,
    LastModifiedAt,
    LastModifiedBy,
}
