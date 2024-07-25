use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;
use sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Categories::CategoryType)
                    .values(CategoryTypes::iter())
                    .to_owned(),
            )
            .await?;

        // Migration for Categories
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
                    .col(ColumnDef::new(Categories::Slug).string().not_null())
                    .col(
                        ColumnDef::new(Categories::CategoryType)
                            .custom(Categories::CategoryType)
                            .not_null(),
                    )
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
                    .col(
                        ColumnDef::new(Categories::RowVersion)
                            .integer()
                            .not_null()
                            .default(1),
                    )
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

        // Migration for Posts
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
                    .col(
                        ColumnDef::new(Posts::RowVersion)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_category_id")
                            .from(Posts::Table, Posts::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tags::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Tags::Name).string().not_null())
                    .col(ColumnDef::new(Tags::Description).string().not_null())
                    .col(ColumnDef::new(Tags::Slug).string().not_null())
                    .col(ColumnDef::new(Tags::CreatedBy).string().not_null())
                    .col(
                        ColumnDef::new(Tags::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Tags::LastModifiedBy).string().null())
                    .col(
                        ColumnDef::new(Tags::LastModifiedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostTags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostTags::PostId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostTags::TagId).string().not_null())
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Then Drop PostTags
        manager
            .drop_table(Table::drop().table(PostTags::Table).to_owned())
            .await?;

        // Then Drop Tags
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await?;

        // Drop Posts
        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await?;

        // Drop Categories
        manager
            .drop_table(Table::drop().table(Categories::Table).to_owned())
            .await?;

        // Drop a type
        manager
            .drop_type(Type::drop().name(Categories::CategoryType).to_owned())
            .await?;

        return Ok(());
    }
}

#[derive(EnumIter, Iden)]
pub enum CategoryTypes {
    #[iden = "Blog"]
    Blog,
    #[iden = "Other"]
    Other,
}

#[derive(DeriveIden)]
pub enum Categories {
    Table,
    Id,
    DisplayName,
    Slug,
    CategoryType,
    CreatedAt,
    CreatedBy,
    LastModifiedAt,
    LastModifiedBy,
    RowVersion,

    ParentId,
}

#[derive(DeriveIden)]
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
    #[sea_orm(updated_at)]
    RowVersion,
}

#[derive(DeriveIden)]
pub enum PostTags {
    Table,
    PostId,
    TagId,
}

#[derive(DeriveIden)]
pub enum Tags {
    Table,
    Id,
    Name,
    Description,
    Slug,
    CreatedAt,
    CreatedBy,
    LastModifiedAt,
    LastModifiedBy,
}

#[cfg(test)]
mod tests {
    use sea_orm::Iden;

    use crate::m20240409_151952_release_100::Posts;

    #[async_std::test]
    async fn handle_create_post_testcase_01() {
        assert_eq!(Posts::Table.to_string(), "posts");
    }
}
