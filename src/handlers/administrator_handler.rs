use axum::{extract::State, Extension};
use axum_keycloak_auth::decode::KeycloakToken;
use migration::{Migrator, MigratorTrait};

use crate::AppState;

#[tracing::instrument]
pub async fn administrator_database_migration(
    Extension(token): Extension<KeycloakToken<String>>,
    state: State<AppState>,
) -> &'static str {
    // expect_role!(&token, "administrator");
    tracing::info!("Token payload is {token:#?}");
    Migrator::up(&state.conn, None).await.unwrap();
    "CMS is running successfully!"
}
