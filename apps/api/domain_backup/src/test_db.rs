//! Crate-local PostgreSQL fixture. Starts its own testcontainer (like
//! `domain_seo`) and never touches posts migrations or `test_helpers`.

use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use std::time::Duration;
use testcontainers::{runners::AsyncRunner, ContainerAsync, ImageExt};
use testcontainers_modules::postgres::Postgres;

#[derive(Debug)]
pub(crate) struct BackupTestDb {
    _container: ContainerAsync<Postgres>,
    pub conn: DatabaseConnection,
}

/// Empty database, no migrations applied. Retries the connection because the
/// image can report ready during PostgreSQL's init restart.
pub(crate) async fn untouched_db() -> BackupTestDb {
    // Production runs supabase/postgres 15.x; prove the migrations on 15.
    let container = Postgres::default()
        .with_tag("15-alpine")
        .start()
        .await
        .expect("postgres container");
    let port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("postgres port");
    let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
    let mut last_error = String::new();
    for _ in 0..60 {
        match Database::connect(&url).await {
            Ok(conn) => match conn
                .execute(Statement::from_string(
                    DbBackend::Postgres,
                    "SELECT 1".to_owned(),
                ))
                .await
            {
                Ok(_) => {
                    return BackupTestDb {
                        _container: container,
                        conn,
                    }
                }
                Err(e) => last_error = e.to_string(),
            },
            Err(e) => last_error = e.to_string(),
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    panic!("postgres never became ready: {last_error}");
}

/// Database with only the backup `Migrator::up` applied.
pub(crate) async fn backup_test_db() -> BackupTestDb {
    use sea_orm_migration::MigratorTrait;
    let db = untouched_db().await;
    crate::migrations::Migrator::up(&db.conn, None)
        .await
        .expect("backup migrations apply on a fresh database");
    db
}
