use crate::{commands::media::SupabaseStorage, common::app_error::AppError};
use moka::future::Cache;
use std::{sync::Arc, time::Duration};
use tracing::{debug, info};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaCacheKey {
    pub bucket: Option<String>,
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

    fn get_rendered_image(
        &self,
        path: String,
        resize_params: ResizeParams,
        bucket: Option<String>,
    ) -> impl std::future::Future<Output = Result<CachedMedia, AppError>>;

    fn get_media_for_bucket(
        &self,
        path: String,
        bucket: Option<String>,
    ) -> impl std::future::Future<Output = Result<CachedMedia, AppError>>;
}

impl ReadMediaHandlerTrait for ReadMediaHandler {
    async fn get_image(
        &self,
        path: String,
        resize_params: ResizeParams,
    ) -> Result<CachedMedia, AppError> {
        if resize_params.needs_resize() {
            return self.get_rendered_image(path, resize_params, None).await;
        }
        self.get_media(path).await
    }

    async fn get_media(&self, path: String) -> Result<CachedMedia, AppError> {
        self.get_media_for_bucket(path, None).await
    }

    async fn get_rendered_image(
        &self,
        path: String,
        resize_params: ResizeParams,
        bucket: Option<String>,
    ) -> Result<CachedMedia, AppError> {
        let cache_key = MediaCacheKey {
            bucket: bucket.clone(),
            path: path.clone(),
            width: resize_params.width,
            height: resize_params.height,
        };

        if let Some(cached) = self.media_cache.get(&cache_key).await {
            debug!(
                "Cache hit for rendered image: bucket={:?} path={} w={:?} h={:?}",
                bucket, path, resize_params.width, resize_params.height
            );
            return Ok(cached);
        }

        debug!(
            "Cache miss for rendered image: bucket={:?} path={} w={:?} h={:?}",
            bucket, path, resize_params.width, resize_params.height
        );

        let storage = match bucket.as_deref() {
            Some(name) => Arc::new(self.storage.with_bucket(name)),
            None => self.storage.clone(),
        };

        let (data, content_type) = storage
            .download_render(&path, resize_params.width, resize_params.height)
            .await?;

        let result = CachedMedia { data, content_type };

        self.media_cache.insert(cache_key, result.clone()).await;
        info!(
            "Cached rendered image: bucket={:?} path={} w={:?} h={:?}",
            bucket, path, resize_params.width, resize_params.height
        );

        Ok(result)
    }

    async fn get_media_for_bucket(
        &self,
        path: String,
        bucket: Option<String>,
    ) -> Result<CachedMedia, AppError> {
        let cache_key = MediaCacheKey {
            bucket: bucket.clone(),
            path: path.clone(),
            width: None,
            height: None,
        };

        if let Some(cached) = self.media_cache.get(&cache_key).await {
            debug!("Cache hit for media: bucket={:?} path={}", bucket, path);
            return Ok(cached);
        }

        debug!(
            "Cache miss for media: bucket={:?} path={}, fetching from Supabase Storage",
            bucket, path
        );

        let storage = match bucket.as_deref() {
            Some(name) => Arc::new(self.storage.with_bucket(name)),
            None => self.storage.clone(),
        };

        let (data, content_type) = storage.download(&path).await?;

        let result = CachedMedia { data, content_type };

        self.media_cache.insert(cache_key, result.clone()).await;
        info!("Cached media: bucket={:?} path={}", bucket, path);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::media::SupabaseStorage;
    use reqwest::Client;
    use std::sync::Arc;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn make_storage(base_url: &str, bucket: &str) -> SupabaseStorage {
        SupabaseStorage {
            supabase_url: base_url.to_string(),
            anon_key: "anon-test-key".to_string(),
            service_role_key: Some("service-role-test-key".to_string()),
            bucket: bucket.to_string(),
            client: Client::new(),
        }
    }

    fn make_handler(
        storage: Arc<SupabaseStorage>,
        cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    ) -> ReadMediaHandler {
        ReadMediaHandler {
            storage,
            media_cache: cache,
        }
    }

    #[test]
    fn cache_key_with_same_path_in_two_buckets_are_distinct() {
        let a = MediaCacheKey {
            bucket: Some("bucket-a".to_string()),
            path: "shared.png".to_string(),
            width: None,
            height: None,
        };
        let b = MediaCacheKey {
            bucket: Some("bucket-b".to_string()),
            path: "shared.png".to_string(),
            width: None,
            height: None,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn cache_key_without_bucket_is_distinct_from_bucket_scoped_key() {
        let none_key = MediaCacheKey {
            bucket: None,
            path: "x.png".to_string(),
            width: None,
            height: None,
        };
        let some_key = MediaCacheKey {
            bucket: Some("media".to_string()),
            path: "x.png".to_string(),
            width: None,
            height: None,
        };
        assert_ne!(none_key, some_key);
    }

    #[test]
    fn cache_key_with_same_bucket_path_and_dimensions_are_equal() {
        let a = MediaCacheKey {
            bucket: Some("media".to_string()),
            path: "x.png".to_string(),
            width: Some(300),
            height: Some(200),
        };
        let b = MediaCacheKey {
            bucket: Some("media".to_string()),
            path: "x.png".to_string(),
            width: Some(300),
            height: Some(200),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn cache_key_with_different_dimensions_are_distinct() {
        let a = MediaCacheKey {
            bucket: Some("media".to_string()),
            path: "x.png".to_string(),
            width: Some(300),
            height: Some(200),
        };
        let b = MediaCacheKey {
            bucket: Some("media".to_string()),
            path: "x.png".to_string(),
            width: Some(400),
            height: Some(200),
        };
        assert_ne!(a, b);
    }

    #[async_std::test]
    async fn get_rendered_image_calls_supabase_render_and_caches_bytes() {
        let server = MockServer::start().await;
        let storage = Arc::new(make_storage(&server.uri(), "media"));

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/foo.png"))
            .and(wiremock::matchers::query_param("width", "300"))
            .and(wiremock::matchers::query_param("height", "200"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/webp")
                    .set_body_bytes(b"rendered".to_vec()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let cache = Arc::new(create_media_cache());
        let handler = make_handler(storage.clone(), cache.clone());

        let first = handler
            .get_rendered_image(
                "foo.png".to_string(),
                ResizeParams::new(Some(300), Some(200)),
                Some("media".to_string()),
            )
            .await
            .expect("first call ok");
        assert_eq!(first.data, b"rendered");
        assert_eq!(first.content_type, "image/webp");

        let second = handler
            .get_rendered_image(
                "foo.png".to_string(),
                ResizeParams::new(Some(300), Some(200)),
                Some("media".to_string()),
            )
            .await
            .expect("second call ok");
        assert_eq!(second.data, b"rendered");
    }

    #[async_std::test]
    async fn get_rendered_image_returns_not_found_on_404() {
        let server = MockServer::start().await;
        let storage = Arc::new(make_storage(&server.uri(), "media"));

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/missing.png"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let cache = Arc::new(create_media_cache());
        let handler = make_handler(storage.clone(), cache.clone());

        let result = handler
            .get_rendered_image(
                "missing.png".to_string(),
                ResizeParams::new(Some(300), None),
                Some("media".to_string()),
            )
            .await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn get_rendered_image_with_different_buckets_does_not_share_cache() {
        let server = MockServer::start().await;
        let storage = Arc::new(make_storage(&server.uri(), "media"));

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/x.png"))
            .and(wiremock::matchers::query_param("width", "300"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"media-bytes".to_vec()),
            )
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/avatars/x.png"))
            .and(wiremock::matchers::query_param("width", "300"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"avatar-bytes".to_vec()),
            )
            .mount(&server)
            .await;

        let cache = Arc::new(create_media_cache());
        let handler = make_handler(storage.clone(), cache.clone());

        let media = handler
            .get_rendered_image(
                "x.png".to_string(),
                ResizeParams::new(Some(300), None),
                Some("media".to_string()),
            )
            .await
            .expect("media bucket ok");
        assert_eq!(media.data, b"media-bytes");

        let avatars = handler
            .get_rendered_image(
                "x.png".to_string(),
                ResizeParams::new(Some(300), None),
                Some("avatars".to_string()),
            )
            .await
            .expect("avatars bucket ok");
        assert_eq!(avatars.data, b"avatar-bytes");
    }

    #[async_std::test]
    async fn get_media_caches_bucket_scoped_keys_separately() {
        let server = MockServer::start().await;
        let storage = Arc::new(make_storage(&server.uri(), "media"));

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/media/y.png"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"media-original".to_vec()),
            )
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/avatars/y.png"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"avatar-original".to_vec()),
            )
            .mount(&server)
            .await;

        let cache = Arc::new(create_media_cache());
        let handler = make_handler(storage.clone(), cache.clone());

        let media = handler
            .get_media_for_bucket("y.png".to_string(), Some("media".to_string()))
            .await
            .expect("media bucket ok");
        assert_eq!(media.data, b"media-original");

        let avatars = handler
            .get_media_for_bucket("y.png".to_string(), Some("avatars".to_string()))
            .await
            .expect("avatars bucket ok");
        assert_eq!(avatars.data, b"avatar-original");
    }
}
