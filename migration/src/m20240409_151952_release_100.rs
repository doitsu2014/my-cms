use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Content).string().not_null())
                    .col(ColumnDef::new(Post::Slug).string().not_null())
                    .col(
                        ColumnDef::new(Post::Published)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(Post::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Post::CreatedBy).string().not_null())
                    .col(ColumnDef::new(Post::LastModifiedAt).timestamp().not_null())
                    .col(ColumnDef::new(Post::LastModifiedBy).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Content,
    Slug,
    Published,
    CreatedAt,
    CreatedBy,
    LastModifiedAt,
    LastModifiedBy,
}
