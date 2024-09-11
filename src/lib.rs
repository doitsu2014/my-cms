pub mod api;
pub mod common;
pub mod presentation_models;

pub use api::*;
use application_core::commands::media::MediaConfig;
use async_graphql::dynamic::*;
pub use common::*;
pub use presentation_models::*;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub graphql_immutable_schema: Arc<Schema>,
    pub graphql_mutable_schema: Arc<Schema>,
}
