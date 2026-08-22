use sea_orm_migration::prelude::*;

pub mod m20260822_000001_seo_head_assets;

pub const SEO_MIGRATION_IDS: &[&str] = &["m20260822_000001_seo_head_assets"];

#[derive(Debug)]
pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260822_000001_seo_head_assets::Migration)]
    }
}
pub fn migration_descriptors() -> Vec<domain_interface::MigrationDescriptor> {
    SEO_MIGRATION_IDS
        .iter()
        .map(|id| domain_interface::MigrationDescriptor {
            id,
            depends_on: &[],
        })
        .collect()
}
