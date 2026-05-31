pub use sea_orm_migration::prelude::*;

mod m20240409_151952_release_100;
mod m20250330_151455_release_110;
mod m20260126_040610_release_300;
mod m20260531_000001_pgvector;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240409_151952_release_100::Migration),
            Box::new(m20250330_151455_release_110::Migration),
            Box::new(m20260126_040610_release_300::Migration),
            Box::new(m20260531_000001_pgvector::Migration),
        ]
    }
}

pub mod constants;
pub use constants::*;
