use sea_orm_migration::prelude::*;

use crate::{
    m20240409_151952_release_100::{Categories, Posts},
    NAME_LENGTH, TITLE_LENGTH,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CategoryTranslations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CategoryTranslations::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslations::CategoryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslations::LanguageCode)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslations::DisplayName)
                            .string_len(NAME_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CategoryTranslations::Slug)
                            .string_len(NAME_LENGTH)
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_category_translations_category_id")
                            .from(
                                CategoryTranslations::Table,
                                CategoryTranslations::CategoryId,
                            )
                            .to(Categories::Table, Categories::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("index_unique_category_translations_slug")
                            .col(CategoryTranslations::Slug)
                            .unique(),
                    )
                    .index(
                        Index::create()
                            .name("index_unique_category_translations_language")
                            .col(CategoryTranslations::CategoryId)
                            .col(CategoryTranslations::LanguageCode)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PostTranslations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PostTranslations::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PostTranslations::PostId).uuid().not_null())
                    .col(
                        ColumnDef::new(PostTranslations::LanguageCode)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PostTranslations::Title)
                            .string_len(TITLE_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PostTranslations::Slug)
                            .string_len(TITLE_LENGTH)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PostTranslations::PreviewContent)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(PostTranslations::Content).text().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_post_translations_post_id")
                            .from(PostTranslations::Table, PostTranslations::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("index_unique_post_translations_language")
                            .col(PostTranslations::PostId)
                            .col(PostTranslations::LanguageCode)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CategoryTranslations::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(PostTranslations::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum CategoryTranslations {
    Table,
    Id,
    CategoryId,
    LanguageCode,
    DisplayName,
    Slug,
}

#[derive(DeriveIden)]
pub enum PostTranslations {
    Table,
    Id,
    PostId,
    LanguageCode,
    Title,
    Slug,
    PreviewContent,
    Content,
}
