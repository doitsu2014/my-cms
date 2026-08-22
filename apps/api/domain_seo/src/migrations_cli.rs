use crate::migrations::{Migrator, SEO_MIGRATION_IDS};
use sea_orm_migration::MigratorTrait;

pub async fn run(conn: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    Migrator::up(conn, None).await
}
pub async fn revert(conn: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    Migrator::down(conn, None).await
}
pub fn list_identities() -> Vec<&'static str> {
    SEO_MIGRATION_IDS.to_vec()
}
