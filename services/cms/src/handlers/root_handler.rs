use axum::{extract::State, response::IntoResponse};
use migration::{Migrator, MigratorTrait};
use serde_json::json;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

use crate::AppState;

#[tracing::instrument]
pub async fn handle() -> &'static str {
    "CMS is running successfully!"
}

#[tracing::instrument]
pub async fn check_health() -> impl IntoResponse {
    let trace_id = find_current_trace_id();
    dbg!(&trace_id);

    axum::Json(json!({
        "message": "CMS is running healthy",
        "trace_id": trace_id
    }))
}

#[tracing::instrument]
pub async fn admin_database_migration(state: State<AppState>) -> &'static str {
    Migrator::up(&state.conn, None).await.unwrap();
    "CMS is running successfully!"
}
