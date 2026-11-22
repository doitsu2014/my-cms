use sea_orm_migration::prelude::*;

pub mod m20260822_000001_seo_head_assets;

pub const SEO_MIGRATION_IDS: &[&str] = &["m20260822_000001_seo_head_assets"];

#[derive(Debug)]
pub struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    /// SEO keeps its own migration table. Posts owns sea-orm's default
    /// `seaql_migrations`, and `Migrator::up` refuses to run when that table
    /// holds versions this migrator does not know (RK-15).
    fn migration_table_name() -> DynIden {
        Alias::new("seaql_migrations_seo").into_iden()
    }

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

#[cfg(test)]
mod tests {
    use super::Migrator;
    use sea_orm_migration::{prelude::Iden, MigratorTrait};

    /// RK-15: posts already owns sea-orm's default `seaql_migrations` table, and
    /// `Migrator::up` refuses to run when that table holds versions it does not
    /// know. SEO must record its migrations in its own table.
    #[test]
    fn seo_uses_its_own_migration_table() {
        assert_eq!(
            Iden::to_string(&*Migrator::migration_table_name()),
            "seaql_migrations_seo"
        );
    }
}
