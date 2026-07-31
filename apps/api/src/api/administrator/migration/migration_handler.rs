use crate::common::supabase_auth::SupabaseToken;
use axum::{extract::Extension, response::IntoResponse};
use migration::{Migrator, MigratorTrait};
use tracing::instrument;

use crate::{ApiResponseWith, AppState, AxumResponse};

#[instrument]
pub async fn handle_api_database_migration(
    Extension(token): Extension<SupabaseToken>,
    state: Extension<AppState>,
) -> impl IntoResponse {
    Migrator::up(state.conn.as_ref(), None).await.unwrap();
    ApiResponseWith::new("Migrated database to latest version").to_axum_response()
}
