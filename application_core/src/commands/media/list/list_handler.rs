use crate::{
    commands::media::{MediaConfig, MediaMetadata},
    common::app_error::AppError,
};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::debug;

pub struct ListMediaHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait ListMediaHandlerTrait {
    fn list_media(
        &self,
        prefix: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<MediaMetadata>, AppError>>;
}

impl ListMediaHandlerTrait for ListMediaHandler {
    async fn list_media(
        &self,
        prefix: Option<String>,
    ) -> Result<Vec<MediaMetadata>, AppError> {
        let bucket = self.media_config.s3_media_storage.spawn_bucket()?;
        let prefix_str = prefix.unwrap_or_default();

        debug!("Listing media with prefix: {}", prefix_str);

        let list_results = bucket
            .list(prefix_str, None)
            .await
            .map_err(|e| AppError::Logical(format!("Failed to list S3 objects: {}", e)))?;

        let mut media_list: Vec<MediaMetadata> = Vec::new();

        for result in list_results {
            for object in result.contents {
                let path = object.key.clone();
                let content_type = guess_content_type(&path);
                let size = object.size;
                let last_modified = DateTime::parse_from_rfc3339(&object.last_modified)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc));

                media_list.push(MediaMetadata {
                    url: format!(
                        "{}/media/{}",
                        self.media_config.media_base_url, path
                    ),
                    path,
                    content_type,
                    size,
                    last_modified,
                });
            }
        }

        Ok(media_list)
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
