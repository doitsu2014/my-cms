pub mod api;
pub mod common;
pub mod presentation_models;

pub use api::*;
use application_core::commands::media::S3MediaStorage;
pub use common::*;
pub use presentation_models::*;

use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub s3_media_storage: Arc<S3MediaStorage>,
}
