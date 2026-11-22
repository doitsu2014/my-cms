//! Backup migrations. The backup domain keeps its own migration table
//! (`seaql_migrations_backup`); posts owns sea-orm's default `seaql_migrations`.

use sea_orm_migration::prelude::*;

pub mod m20261122_000001_backup_core;
pub mod m20261122_000002_backup_terminal_guards;

#[cfg(test)]
mod tests;

pub const BACKUP_MIGRATION_IDS: &[&str] = &[
    "m20261122_000001_backup_core",
    "m20261122_000002_backup_terminal_guards",
];

#[derive(Debug)]
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> DynIden {
        Alias::new("seaql_migrations_backup").into_iden()
    }

    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261122_000001_backup_core::Migration),
            Box::new(m20261122_000002_backup_terminal_guards::Migration),
        ]
    }
}

pub fn migration_descriptors() -> Vec<domain_interface::MigrationDescriptor> {
    BACKUP_MIGRATION_IDS
        .iter()
        .map(|id| domain_interface::MigrationDescriptor {
            id,
            depends_on: &[],
        })
        .collect()
}

/// Shared `down` precondition. Refuse while `backup_audit` holds rows (audit
/// retention is an open owner decision, RK-13) or any allocation is not
/// `deleted` (the ledger still owns a file). Both `down` steps call this first,
/// so every revert path, including a one-step revert of 000002, is refused while
/// audit rows or live allocations exist. Atomicity comes from sea-orm-migration
/// running a whole Postgres `down` run in one transaction.
///
/// The check first takes SHARE ROW EXCLUSIVE locks on the three tables (held
/// until that transaction ends). This blocks concurrent writers and waits for
/// in-flight ones, so the later DROPs cannot destroy a row committed after the
/// check. A revert therefore runs with backup admissions blocked.
pub(crate) async fn refuse_unsafe_down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    let conn = manager.get_connection();
    conn.execute_unprepared(
        "LOCK TABLE backup_audit, backup_allocations, backup_jobs IN SHARE ROW EXCLUSIVE MODE",
    )
    .await?;
    for (sql, table, what) in [
        (
            "SELECT EXISTS (SELECT 1 FROM backup_audit) AS present",
            "backup_audit",
            "it still has audit rows, which U1 retains",
        ),
        (
            "SELECT EXISTS (SELECT 1 FROM backup_allocations WHERE state <> 'deleted') AS present",
            "backup_allocations",
            "an allocation is not in state `deleted`",
        ),
    ] {
        let row = conn
            .query_one(Statement::from_string(DbBackend::Postgres, sql.to_owned()))
            .await?;
        let present = match row {
            Some(r) => r.try_get::<bool>("", "present")?,
            None => false,
        };
        if present {
            return Err(DbErr::Migration(format!(
                "refusing to revert the backup migrations: {table}: {what}"
            )));
        }
    }
    Ok(())
}
