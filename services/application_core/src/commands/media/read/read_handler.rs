use crate::{commands::media::S3MediaStorage, common::app_error::AppError};
use image::{imageops::FilterType, DynamicImage, ImageFormat, ImageReader};
use moka::future::Cache;
use std::{io::Cursor, sync::Arc, time::Duration};
use tracing::{debug, info};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaCacheKey {
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

// Backward compatibility alias
pub type ImageCacheKey = MediaCacheKey;

#[derive(Clone, Debug)]
pub struct CachedMedia {
    pub data: Vec<u8>,
    pub content_type: String,
}

// Backward compatibility alias
pub type CachedImage = CachedMedia;

pub struct ReadMediaHandler {
    pub s3_media_storage: Arc<S3MediaStorage>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
}

// Backward compatibility: allow using image_cache field name
impl ReadMediaHandler {
    pub fn new(
        s3_media_storage: Arc<S3MediaStorage>,
        cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    ) -> Self {
        Self {
            s3_media_storage,
            media_cache: cache,
        }
    }

    pub fn with_image_cache(
        s3_media_storage: Arc<S3MediaStorage>,
        image_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    ) -> Self {
        Self {
            s3_media_storage,
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
        .max_capacity(500) // max 500 files
        .time_to_live(Duration::from_secs(3600)) // 1 hour TTL
        .time_to_idle(Duration::from_secs(1800)) // 30 minutes idle
        .build()
}

// Backward compatibility alias
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
        let cache_key = MediaCacheKey {
            path: path.clone(),
            width: resize_params.width,
            height: resize_params.height,
        };

        // Try to get from cache first
        if let Some(cached) = self.media_cache.get(&cache_key).await {
            debug!("Cache hit for image: {}", path);
            return Ok(cached);
        }

        debug!("Cache miss for image: {}, fetching from S3", path);

        // Fetch from S3
        let bucket = self.s3_media_storage.spawn_bucket()?;
        let response = bucket
            .get_object(&path)
            .await
            .map_err(|e| AppError::Logical(format!("Failed to fetch image from S3: {}", e)))?;

        let content_type = response
            .headers()
            .get("content-type")
            .map(|s| s.to_string())
            .unwrap_or_else(|| guess_content_type(&path));
        let original_data = response.to_vec();

        let result = if resize_params.needs_resize() {
            resize_image(&original_data, &content_type, &resize_params)?
        } else {
            CachedMedia {
                data: original_data,
                content_type,
            }
        };

        // Store in cache
        self.media_cache.insert(cache_key, result.clone()).await;
        info!("Cached image: {}", path);

        Ok(result)
    }

    async fn get_media(&self, path: String) -> Result<CachedMedia, AppError> {
        let cache_key = MediaCacheKey {
            path: path.clone(),
            width: None,
            height: None,
        };

        // Try to get from cache first
        if let Some(cached) = self.media_cache.get(&cache_key).await {
            debug!("Cache hit for media: {}", path);
            return Ok(cached);
        }

        debug!("Cache miss for media: {}, fetching from S3", path);

        // Fetch from S3
        let bucket = self.s3_media_storage.spawn_bucket()?;
        let response = bucket
            .get_object(&path)
            .await
            .map_err(|e| AppError::Logical(format!("Failed to fetch media from S3: {}", e)))?;

        let content_type = response
            .headers()
            .get("content-type")
            .map(|s| s.to_string())
            .unwrap_or_else(|| guess_content_type(&path));
        let original_data = response.to_vec();

        let result = CachedMedia {
            data: original_data,
            content_type,
        };

        // Store in cache
        self.media_cache.insert(cache_key, result.clone()).await;
        info!("Cached media: {}", path);

        Ok(result)
    }
}

fn guess_content_type(path: &str) -> String {
    let ext = path.split('.').last().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        "webp" => "image/webp".to_string(),
        "bmp" => "image/bmp".to_string(),
        "ico" => "image/x-icon".to_string(),
        "tiff" | "tif" => "image/tiff".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

fn resize_image(
    data: &[u8],
    content_type: &str,
    params: &ResizeParams,
) -> Result<CachedImage, AppError> {
    let img = ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|e| AppError::Logical(format!("Failed to detect image format: {}", e)))?
        .decode()
        .map_err(|e| AppError::Logical(format!("Failed to decode image: {}", e)))?;

    let (orig_width, orig_height) = (img.width(), img.height());

    let (new_width, new_height) = match (params.width, params.height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => {
            let ratio = w as f64 / orig_width as f64;
            (w, (orig_height as f64 * ratio) as u32)
        }
        (None, Some(h)) => {
            let ratio = h as f64 / orig_height as f64;
            ((orig_width as f64 * ratio) as u32, h)
        }
        (None, None) => (orig_width, orig_height),
    };

    // Cap resize to reasonable bounds to prevent abuse
    let max_dimension = 4096u32;
    let new_width = new_width.min(max_dimension);
    let new_height = new_height.min(max_dimension);

    let resized: DynamicImage = img.resize(new_width, new_height, FilterType::Lanczos3);

    let output_format = match content_type {
        "image/png" => ImageFormat::Png,
        "image/gif" => ImageFormat::Gif,
        "image/webp" => ImageFormat::WebP,
        "image/bmp" => ImageFormat::Bmp,
        _ => ImageFormat::Jpeg,
    };

    let mut output_data = Cursor::new(Vec::new());
    resized
        .write_to(&mut output_data, output_format)
        .map_err(|e| AppError::Logical(format!("Failed to encode resized image: {}", e)))?;

    let final_content_type = match output_format {
        ImageFormat::Png => "image/png",
        ImageFormat::Gif => "image/gif",
        ImageFormat::WebP => "image/webp",
        ImageFormat::Bmp => "image/bmp",
        _ => "image/jpeg",
    };

    Ok(CachedImage {
        data: output_data.into_inner(),
        content_type: final_content_type.to_string(),
    })
}
