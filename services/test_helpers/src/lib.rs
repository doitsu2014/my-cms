use std::future::Future;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
use testcontainers_modules::postgres::Postgres;

pub use supabase_testcontainers_modules::Auth;

pub struct TestSpace {
    pub postgres: ContainerAsync<Postgres>,
}

pub async fn setup_test_space() -> TestSpace {
    TestSpace {
        postgres: Postgres::default().start().await.unwrap(),
    }
}

pub struct TestSpaceWithPgVector {
    pub postgres: ContainerAsync<GenericImage>,
}

pub async fn setup_test_space_with_pgvector() -> TestSpaceWithPgVector {
    TestSpaceWithPgVector {
        postgres: GenericImage::new("pgvector/pgvector", "pg15")
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stdout(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", "postgres")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres")
            .start()
            .await
            .unwrap(),
    }
}

pub trait ContainerAsyncPostgresEx {
    fn get_database_connection(&self) -> impl Future<Output = DatabaseConnection>;
}

impl ContainerAsyncPostgresEx for ContainerAsync<Postgres> {
    async fn get_database_connection(&self) -> DatabaseConnection {
        get_db_connection(self).await
    }
}

impl ContainerAsyncPostgresEx for ContainerAsync<GenericImage> {
    async fn get_database_connection(&self) -> DatabaseConnection {
        get_db_connection(self).await
    }
}

async fn get_db_connection<C: testcontainers::Image>(
    container: &ContainerAsync<C>,
) -> DatabaseConnection {
    let connection_string = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        container.get_host_port_ipv4(5432).await.unwrap()
    );
    let conn = Database::connect(&connection_string).await.unwrap();
    Migrator::refresh(&conn.clone()).await.unwrap();
    conn
}
