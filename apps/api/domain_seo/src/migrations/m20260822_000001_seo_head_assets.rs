use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName, Debug)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                r#"CREATE TABLE IF NOT EXISTS seo_head_assets (
                id UUID PRIMARY KEY,
                label VARCHAR(128) NOT NULL UNIQUE CHECK (length(btrim(label)) > 0),
                html TEXT NOT NULL CHECK (length(html) > 0 AND octet_length(html) <= 32768),
                enabled BOOLEAN NOT NULL DEFAULT TRUE,
                sort_order INTEGER NOT NULL CHECK (sort_order > 0),
                row_version INTEGER NOT NULL DEFAULT 1 CHECK (row_version > 0),
                created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
                created_by TEXT NOT NULL,
                updated_by TEXT NOT NULL
            )"#,
            )
            .await?;
        manager.get_connection().execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_seo_head_assets_public_order ON seo_head_assets (sort_order ASC, id ASC) WHERE enabled = TRUE",
        ).await?;
        Ok(())
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP INDEX IF EXISTS idx_seo_head_assets_public_order")
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS seo_head_assets")
            .await?;
        Ok(())
    }
}
