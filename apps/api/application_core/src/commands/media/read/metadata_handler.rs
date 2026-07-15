use crate::{
    commands::media::{MediaConfig, MediaMetadata},
    common::app_error::AppError,
};
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
        debug!("Getting metadata for: {}", path);

        let info = self.media_config.storage.get_info(&path).await?;

        let url = match &self.media_config.bucket_override {
            Some(bucket) => format!(
                "{}/storage/v1/object/{}/{}",
                self.media_config.storage.supabase_url, bucket, path
            ),
            None => format!("{}/media/{}", self.media_config.media_base_url, path),
        };

        Ok(MediaMetadata {
            url,
            path,
            content_type: info.content_type,
            size: info.size,
            last_modified: info.last_modified,
        })
    }
}
