pub mod common;
pub mod handlers;
pub mod presentation_models;

pub use handlers::*;
pub use presentation_models::*;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
