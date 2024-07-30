use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;
use sea_query::extension::postgres::Type;

use crate::{NAME_LENGTH, TITLE_LENGTH, USER_EMAIL_LENGTH};

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
                    .col(
                        ColumnDef::new(Categories::DisplayName)
                            .string_len(NAME_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Categories::Slug)
                            .string_len(NAME_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Categories::CategoryType)
                            .custom(Categories::CategoryType)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Categories::CreatedBy)
                            .string_len(USER_EMAIL_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Categories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Categories::LastModifiedBy)
                            .string_len(USER_EMAIL_LENGTH)
                            .null(),
                    )
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
                    .index(
                        Index::create()
                            .name("index_unique_category_slug")
                            .col(Categories::Slug)
                            .unique(),
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
                    .col(
                        ColumnDef::new(Posts::Title)
                            .string_len(TITLE_LENGTH)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Posts::PreviewContent).text())
                    .col(ColumnDef::new(Posts::Content).text().not_null())
                    .col(
                        ColumnDef::new(Posts::Slug)
                            .string_len(TITLE_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Posts::Published)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Posts::CreatedBy)
                            .string_len(USER_EMAIL_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Posts::LastModifiedBy)
                            .string_len(USER_EMAIL_LENGTH)
                            .null(),
                    )
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
                    .index(
                        Index::create()
                            .name("index_unique_post_slug")
                            .col(Posts::Slug)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // Tags
        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tags::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Tags::Name)
                            .string_len(NAME_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tags::Slug)
                            .string_len(NAME_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tags::CreatedBy)
                            .string_len(USER_EMAIL_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tags::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Tags::LastModifiedBy)
                            .string_len(USER_EMAIL_LENGTH)
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Tags::LastModifiedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .index(
                        Index::create()
                            .name("index_unique_tag_slug")
                            .col(Tags::Slug)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("index_unique_tag_name")
                            .col(Tags::Name)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        // Post Tags
        manager
            .create_table(
                Table::create()
                    .table(PostTags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PostTags::PostId).uuid().not_null())
                    .col(ColumnDef::new(PostTags::TagId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tags_post_id")
                            .from(PostTags::Table, PostTags::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_tags_tag_id")
                            .from(PostTags::Table, PostTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .table(PostTags::Table)
                            .col(PostTags::PostId)
                            .col(PostTags::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        // Category Tags
        manager
            .create_table(
                Table::create()
                    .table(CategoryTags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CategoryTags::CategoryId).uuid().not_null())
                    .col(ColumnDef::new(CategoryTags::TagId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_tags_category_id")
                            .from(CategoryTags::Table, CategoryTags::CategoryId)
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_tags_tag_id")
                            .from(CategoryTags::Table, CategoryTags::TagId)
                            .to(Tags::Table, Tags::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .table(CategoryTags::Table)
                            .col(CategoryTags::CategoryId)
                            .col(CategoryTags::TagId),
                    )
                    .to_owned(),
            )
            .await?;

        return Ok(());
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Then Drop CategoryTags
        manager
            .drop_table(Table::drop().table(CategoryTags::Table).to_owned())
            .await?;

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
    PreviewContent,
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
pub enum Tags {
    Table,
    Id,
    Name,
    Slug,
    CreatedAt,
    CreatedBy,
    LastModifiedAt,
    LastModifiedBy,
}

#[derive(DeriveIden)]
pub enum PostTags {
    Table,
    PostId,
    TagId,
}

#[derive(DeriveIden)]
pub enum CategoryTags {
    Table,
    CategoryId,
    TagId,
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
