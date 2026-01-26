use sea_orm_migration::prelude::*;

use crate::m20240409_151952_release_100::Posts;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TranslationJobs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TranslationJobs::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TranslationJobs::PostId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TranslationJobs::TargetLanguage)
                            .string_len(10)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TranslationJobs::Status)
                            .string_len(20)
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(TranslationJobs::Progress)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(TranslationJobs::ErrorMessage)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TranslationJobs::AiModel)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TranslationJobs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TranslationJobs::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_translation_jobs_post_id")
                            .from(TranslationJobs::Table, TranslationJobs::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("index_translation_jobs_post_id_status")
                            .col(TranslationJobs::PostId)
                            .col(TranslationJobs::Status),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TranslationJobs::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum TranslationJobs {
    Table,
    Id,
    PostId,
    TargetLanguage,
    Status,
    Progress,
    ErrorMessage,
    AiModel,
    CreatedAt,
    UpdatedAt,
}
