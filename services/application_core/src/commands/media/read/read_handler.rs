use crate::{commands::media::SupabaseStorage, common::app_error::AppError};
use moka::future::Cache;
use std::{sync::Arc, time::Duration};
use tracing::{debug, info};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaCacheKey {
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub type ImageCacheKey = MediaCacheKey;

#[derive(Clone, Debug)]
pub struct CachedMedia {
    pub data: Vec<u8>,
    pub content_type: String,
}

pub type CachedImage = CachedMedia;

pub struct ReadMediaHandler {
    pub storage: Arc<SupabaseStorage>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
}

impl ReadMediaHandler {
    pub fn new(
        storage: Arc<SupabaseStorage>,
        cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    ) -> Self {
        Self {
            storage,
            media_cache: cache,
        }
    }

    pub fn with_image_cache(
        storage: Arc<SupabaseStorage>,
        image_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    ) -> Self {
        Self {
            storage,
            media_cache: image_cache,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResizeParams {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl ResizeParams {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self { width, height }
    }

    pub fn needs_resize(&self) -> bool {
        self.width.is_some() || self.height.is_some()
    }
}

pub fn create_media_cache() -> Cache<MediaCacheKey, CachedMedia> {
    Cache::builder()
        .max_capacity(500)
        .time_to_live(Duration::from_secs(3600))
        .time_to_idle(Duration::from_secs(1800))
        .build()
}

pub fn create_image_cache() -> Cache<MediaCacheKey, CachedMedia> {
    create_media_cache()
}

pub trait ReadMediaHandlerTrait {
    fn get_image(
        &self,
        path: String,
        resize_params: ResizeParams,
    ) -> impl std::future::Future<Output = Result<CachedMedia, AppError>>;

    fn get_media(
        &self,
        path: String,
    ) -> impl std::future::Future<Output = Result<CachedMedia, AppError>>;
}

impl ReadMediaHandlerTrait for ReadMediaHandler {
    async fn get_image(
        &self,
        path: String,
        resize_params: ResizeParams,
    ) -> Result<CachedMedia, AppError> {
        if resize_params.needs_resize() {
            return Err(AppError::Logical(format!(
                "Server-side resize is deprecated; the API redirect handler should serve a 302 for w/h params (path={})",
                path
            )));
        }
        self.get_media(path).await
    }

    async fn get_media(&self, path: String) -> Result<CachedMedia, AppError> {
        let cache_key = MediaCacheKey {
            path: path.clone(),
            width: None,
            height: None,
        };

        if let Some(cached) = self.media_cache.get(&cache_key).await {
            debug!("Cache hit for media: {}", path);
            return Ok(cached);
        }

        debug!(
            "Cache miss for media: {}, fetching from Supabase Storage",
            path
        );

        let (original_data, content_type) = self.storage.download(&path).await?;

        let result = CachedMedia {
            data: original_data,
            content_type,
        };

        self.media_cache.insert(cache_key, result.clone()).await;
        info!("Cached media: {}", path);

        Ok(result)
    }
}
