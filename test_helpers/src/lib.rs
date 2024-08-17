use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use testcontainers::{runners::AsyncRunner, ContainerAsync};
use testcontainers_modules::postgres::Postgres;

pub struct TestSpace {
    pub postgres: ContainerAsync<Postgres>,
}

pub async fn setup_test_space() -> TestSpace {
    TestSpace {
        postgres: Postgres::default().start().await.unwrap(),
    }
}

pub trait ContainerAsyncPostgresEx {
    fn get_database_connection(&self) -> impl std::future::Future<Output = DatabaseConnection>;
}

impl ContainerAsyncPostgresEx for ContainerAsync<Postgres> {
    async fn get_database_connection(&self) -> DatabaseConnection {
        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            self.get_host_port_ipv4(5432).await.unwrap()
        );
        let conn = Database::connect(&connection_string).await.unwrap();
        Migrator::refresh(&conn.clone()).await.unwrap();
        conn
    }
}
