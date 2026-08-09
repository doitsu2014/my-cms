//! SeaORM migrations owned by the post domain.
//!
//! The four migration identities
//! `m20240409_151952_release_100`, `m20250330_151455_release_110`,
//! `m20260126_040610_release_300`, `m20260531_000001_pgvector` are
//! preserved exactly. The migration `up` history in the database is
//! unchanged.
//!
//! Moved from `apps/api/migration/src/*` per design Decision 5. The legacy
//! `migration` crate is now a thin shim that re-exports from this module.

pub use sea_orm_migration::prelude::*;

pub(crate) mod m20240409_151952_release_100;
pub(crate) mod m20250330_151455_release_110;
pub(crate) mod m20260126_040610_release_300;
pub(crate) mod m20260531_000001_pgvector;

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

/// Migration IDs owned by the post domain — preserved for the gateway
/// orchestrator and `DomainService::migrations` implementation.
pub const POST_MIGRATION_IDS: &[&str] = &[
    "m20240409_151952_release_100",
    "m20250330_151455_release_110",
    "m20260126_040610_release_300",
    "m20260531_000001_pgvector",
];

/// Re-exported migration descriptors that the gateway's orchestrator
/// consumes through `DomainService::migrations()`. Each descriptor has
/// `depends_on = &[]` because no foundation dependency exists.
pub fn migration_descriptors() -> Vec<domain_interface::MigrationDescriptor> {
    POST_MIGRATION_IDS
        .iter()
        .map(|id| domain_interface::MigrationDescriptor {
            id,
            depends_on: &[],
        })
        .collect()
}
