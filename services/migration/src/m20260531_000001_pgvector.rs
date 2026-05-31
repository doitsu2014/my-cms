use sea_orm_migration::prelude::*;

use crate::m20240409_151952_release_100::Posts;
use crate::m20250330_151455_release_110::PostTranslations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS vector")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Embeddings::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Embeddings::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Embeddings::PostId).uuid().not_null())
                    .col(ColumnDef::new(Embeddings::LanguageCode).string_len(50).not_null())
                    .col(ColumnDef::new(Embeddings::TranslationId).uuid())
                    .col(ColumnDef::new(Embeddings::Embedding).custom("vector(1536)").not_null())
                    .col(ColumnDef::new(Embeddings::Title).string_len(512).not_null())
                    .col(ColumnDef::new(Embeddings::ContentPreview).text().not_null())
                    .col(ColumnDef::new(Embeddings::CreatedAt)
                        .timestamp_with_time_zone().not_null()
                        .default(Expr::current_timestamp()))
                    .col(ColumnDef::new(Embeddings::UpdatedAt)
                        .timestamp_with_time_zone().not_null()
                        .default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_embeddings_post_id")
                            .from(Embeddings::Table, Embeddings::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_embeddings_translation_id")
                            .from(Embeddings::Table, Embeddings::TranslationId)
                            .to(PostTranslations::Table, PostTranslations::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_embeddings_vector ON embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100)",
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_embeddings_post_lang")
                    .table(Embeddings::Table)
                    .col(Embeddings::PostId)
                    .col(Embeddings::LanguageCode)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_embeddings_post_lang")
                    .table(Embeddings::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_embeddings_vector")
                    .table(Embeddings::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Embeddings::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Embeddings {
    Table,
    Id,
    PostId,
    LanguageCode,
    TranslationId,
    Embedding,
    Title,
    ContentPreview,
    CreatedAt,
    UpdatedAt,
}
