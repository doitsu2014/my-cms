//! `MediaApiState` — per-process state for the media domain's Axum router.
//!
//! Holds the `MediaConfig` (storage client + bucket + base URL), the media
//! delivery cache, and the bucket-visibility cache. Constructed once by
//! `DomainMediaService` at gateway startup, then cloned (`Arc`-wrapped) into
//! every per-request handler. `DomainContext` is intentionally NOT expanded
//! with these fields — they remain domain-local per design Decision 3.

use std::sync::Arc;

use moka::future::Cache;

use crate::handlers::{bucket::access::access_cache::create_bucket_visibility_cache, CachedMedia, MediaCacheKey, MediaConfig};

/// Wrapper struct so the router's `State<MediaApiState>` is a single
/// `Clone`-able value. All inner fields are `Arc`-shared.
#[derive(Clone)]
pub struct MediaApiState {
    pub media_config: Arc<MediaConfig>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    pub bucket_visibility_cache: Arc<Cache<String, bool>>,
}

impl std::fmt::Debug for MediaApiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MediaApiState")
            .field("media_config", &self.media_config)
            .field("media_cache", &"<Cache>")
            .field("bucket_visibility_cache", &"<Cache>")
            .finish_non_exhaustive()
    }
}

impl MediaApiState {
    /// Build a `MediaApiState` from a `MediaConfig`. Both caches are created
    /// with the canonical factories from the media domain so cache policies
    /// stay consistent across runtimes.
    pub fn new(media_config: Arc<MediaConfig>) -> Self {
        Self {
            media_config,
            media_cache: Arc::new(crate::handlers::read::read_handler::create_media_cache()),
            bucket_visibility_cache: Arc::new(create_bucket_visibility_cache()),
        }
    }

    /// Build a `MediaApiState` from an existing pair of caches. Useful for
    /// tests and for the gateway, which wants a single cache pair shared
    /// across the process.
    pub fn with_caches(
        media_config: Arc<MediaConfig>,
        media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
        bucket_visibility_cache: Arc<Cache<String, bool>>,
    ) -> Self {
        Self {
            media_config,
            media_cache,
            bucket_visibility_cache,
        }
    }
}
