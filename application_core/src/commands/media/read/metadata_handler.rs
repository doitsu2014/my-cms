use crate::{
    commands::media::{MediaConfig, MediaMetadata},
    common::app_error::AppError,
};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::debug;

pub struct MetadataMediaHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait MetadataMediaHandlerTrait {
    fn get_metadata(
        &self,
        path: String,
    ) -> impl std::future::Future<Output = Result<MediaMetadata, AppError>>;
}

impl MetadataMediaHandlerTrait for MetadataMediaHandler {
    async fn get_metadata(&self, path: String) -> Result<MediaMetadata, AppError> {
        let bucket = self.media_config.s3_media_storage.spawn_bucket()?;

        debug!("Getting metadata for: {}", path);

        let (head, status_code) = bucket
            .head_object(&path)
            .await
            .map_err(|e| AppError::Logical(format!("Failed to get S3 object metadata: {}", e)))?;

        if status_code == 404 {
            return Err(AppError::NotFound);
        }

        let content_type = head
            .content_type
            .unwrap_or_else(|| guess_content_type(&path));

        let size = head.content_length.unwrap_or(0) as u64;

        let last_modified = head
            .last_modified
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc2822(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        Ok(MediaMetadata {
            url: format!("{}/media/{}", self.media_config.media_base_url, path),
            path,
            content_type,
            size,
            last_modified,
        })
    }
}

fn guess_content_type(path: &str) -> String {
    let ext = path.split('.').last().unwrap_or("").to_lowercase();
    match ext.as_str() {
        // Images
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        "webp" => "image/webp".to_string(),
        "bmp" => "image/bmp".to_string(),
        "ico" => "image/x-icon".to_string(),
        "svg" => "image/svg+xml".to_string(),
        // Documents
        "pdf" => "application/pdf".to_string(),
        "doc" => "application/msword".to_string(),
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        "xls" => "application/vnd.ms-excel".to_string(),
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
        "ppt" => "application/vnd.ms-powerpoint".to_string(),
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
        // Text
        "txt" => "text/plain".to_string(),
        "csv" => "text/csv".to_string(),
        "md" => "text/markdown".to_string(),
        "json" => "application/json".to_string(),
        "xml" => "application/xml".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}
