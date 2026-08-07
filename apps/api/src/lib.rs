pub mod api;
pub mod common;
pub mod presentation_models;

pub use api::*;
use async_graphql::dynamic::*;
use domain_media::handlers::{CachedMedia, MediaCacheKey, MediaConfig};
use domain_user::handlers::supabase_admin_client::SupabaseAdminClient;
use moka::future::Cache;
pub use presentation_models::*;

use sea_orm::DatabaseConnection;
use std::{fmt::Debug, sync::Arc};

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    pub bucket_visibility_cache: Arc<Cache<String, bool>>,
    pub graphql_immutable_schema: Arc<Schema>,
    pub graphql_mutable_schema: Arc<Schema>,
    pub supabase_admin_client: Arc<SupabaseAdminClient>,
}

impl Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("media_config", &self.media_config)
            .field("media_cache", &"<Cache>")
            .field("bucket_visibility_cache", &"<Cache>")
            .field("supabase_admin_client", &"<SupabaseAdminClient>")
            .finish_non_exhaustive()
    }
}
