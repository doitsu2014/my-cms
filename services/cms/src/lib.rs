use diesel::PgConnection;

pub mod domains;
pub mod handlers;
pub mod infrastructure;
pub mod schema;

pub use handlers::*;

pub struct AppState {
    pub db_connection: PgConnection,
}
