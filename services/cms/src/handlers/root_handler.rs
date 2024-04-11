use axum::extract::State;
use migration::{Migrator, MigratorTrait};

use crate::AppState;

pub async fn handle() -> &'static str {
    "CMS is running successfully!"
}

pub async fn check_health() -> &'static str {
    "CMS is running successfully!"
}

pub async fn admin_database_migration(state: State<AppState>) -> &'static str {
    Migrator::up(&state.conn, None).await.unwrap();
    "CMS is running successfully!"
}
