//! SeaORM migrations owned by the post domain.
//!
//! The four migration identities
//! `m20240409_151952_release_100`, `m20250330_151455_release_110`,
//! `m20260126_040610_release_300`, `m20260531_000001_pgvector` are
//! preserved exactly. The migration `up` history in the database is
//! unchanged.
//!
//! During the transition the canonical implementations live in the
//! legacy `migration` crate; this module re-exports the `Migrator` and
//! its constants. Future domains add their own migration modules and
//! declare `MigrationDescriptor::depends_on` on these.

pub use migration::{constants, Migrator, MigratorTrait};

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