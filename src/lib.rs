pub mod commands;
pub mod common;
pub mod presentation_models;

pub use commands::*;
pub use common::*;
pub use presentation_models::*;

use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
