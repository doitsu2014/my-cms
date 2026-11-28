//! RK-15: the posts and SEO domains must both apply their migrations through
//! the gateway orchestrator on one shared database, on every boot.
//!
//! Live PostgreSQL coverage is opt-in because it requires Docker.

use crate::run_orchestrator;
use domain_interface::DomainService;
use domain_posts::migrations::POST_MIGRATION_IDS;
use domain_posts::service::DomainPostService;
use domain_seo::migrations::SEO_MIGRATION_IDS;
use domain_seo::service::DomainSeoService;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement};
use std::time::Duration;
use test_helpers::{setup_test_space_with_pgvector, TestSpaceWithPgVector};

/// Connect without applying any migrations. `ContainerAsyncPostgresEx` would
/// run the posts migrator as a side effect, and the shared helper's readiness
/// check can fire during PostgreSQL's init restart, so retry briefly.
pub(crate) async fn connect_untouched(space: &TestSpaceWithPgVector) -> DatabaseConnection {
    let port = space
        .postgres
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
                Ok(_) => return conn,
                Err(e) => last_error = e.to_string(),
            },
            Err(e) => last_error = e.to_string(),
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    panic!("postgres never became ready: {last_error}");
}

fn posts_and_seo() -> Vec<Box<dyn DomainService>> {
    vec![
        Box::new(DomainPostService::new()),
        Box::new(DomainSeoService::new()),
    ]
}

pub(crate) async fn recorded_versions(conn: &DatabaseConnection, table: &str) -> Vec<String> {
    let rows = conn
        .query_all(Statement::from_string(
            DbBackend::Postgres,
            format!("SELECT version FROM {table} ORDER BY version"),
        ))
        .await
        .unwrap_or_else(|e| panic!("read {table}: {e}"));
    rows.iter()
        .map(|row| {
            row.try_get::<String>("", "version")
                .expect("version column")
        })
        .collect()
}

fn sorted(ids: &[&str]) -> Vec<String> {
    let mut ids: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
    ids.sort();
    ids
}

async fn assert_each_domain_owns_its_records(conn: &DatabaseConnection) {
    assert_eq!(
        recorded_versions(conn, "seaql_migrations").await,
        sorted(POST_MIGRATION_IDS),
        "shared default table must hold posts versions only"
    );
    assert_eq!(
        recorded_versions(conn, "seaql_migrations_seo").await,
        sorted(SEO_MIGRATION_IDS),
        "SEO versions must be recorded in the SEO table"
    );
    conn.execute(Statement::from_string(
        DbBackend::Postgres,
        "SELECT 1 FROM seo_head_assets LIMIT 1".to_owned(),
    ))
    .await
    .expect("seo_head_assets table exists");
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn posts_and_seo_migrations_apply_on_fresh_database_and_second_boot() {
    let space = setup_test_space_with_pgvector().await;
    let conn = connect_untouched(&space).await;
    let services = posts_and_seo();

    run_orchestrator(&services, &conn)
        .await
        .expect("first boot applies posts and SEO migrations");
    run_orchestrator(&services, &conn)
        .await
        .expect("second boot is a no-op");

    assert_each_domain_owns_its_records(&conn).await;
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn database_with_only_posts_applied_upgrades_cleanly() {
    // Every existing database is in this state today: posts applied, SEO
    // never applied because its runner always failed on the shared table.
    let space = setup_test_space_with_pgvector().await;
    let conn = connect_untouched(&space).await;
    domain_posts::migrations_cli::run(&conn)
        .await
        .expect("posts-only database");

    run_orchestrator(&posts_and_seo(), &conn)
        .await
        .expect("boot with posts and SEO on a posts-only database");

    assert_each_domain_owns_its_records(&conn).await;
}
