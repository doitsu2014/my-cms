use sea_orm::{Database, DatabaseConnection};
use std::sync::Arc;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

pub async fn setup_test_space() -> Arc<DatabaseConnection> {
    let postgres = Postgres::default().start().await.unwrap();

    let connection_string: String = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        postgres.get_host_port_ipv4(5432).await.unwrap()
    );
    let conn = Database::connect(&connection_string).await.unwrap();
    Arc::new(conn)
}
