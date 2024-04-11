pub mod handlers;
pub mod tests;

pub use handlers::*;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
