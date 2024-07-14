pub mod commands;
pub mod common;
pub mod handlers;
pub mod presentation_models;

pub use common::*;
pub use handlers::*;
pub use presentation_models::*;

use sea_orm::DatabaseConnection;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
