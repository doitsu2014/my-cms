pub mod api;
pub mod common;
pub mod presentation_models;

pub use api::*;
pub use common::*;
pub use presentation_models::*;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
}
