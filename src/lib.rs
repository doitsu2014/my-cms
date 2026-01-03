pub mod api;
pub mod common;
pub mod presentation_models;

pub use api::*;
use application_core::commands::media::{
    read::read_handler::{CachedImage, ImageCacheKey},
    MediaConfig,
};
use async_graphql::dynamic::*;
pub use common::*;
use moka::future::Cache;
pub use presentation_models::*;

use sea_orm::DatabaseConnection;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub image_cache: Arc<Cache<ImageCacheKey, CachedImage>>,
    pub graphql_immutable_schema: Arc<Schema>,
    pub graphql_mutable_schema: Arc<Schema>,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("media_config", &self.media_config)
            .field("image_cache", &"<Cache>")
            .finish_non_exhaustive()
    }
}
