pub use sea_orm_migration::prelude::*;

mod m20240409_151952_release_100;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240409_151952_release_100::Migration)]
    }
}
