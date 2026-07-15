use crate::{
    commands::media::{MediaConfig, MediaMetadata},
    common::app_error::AppError,
};
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
    async fn list_media(&self, prefix: Option<String>) -> Result<Vec<MediaMetadata>, AppError> {
        let prefix_str = prefix.unwrap_or_default();
        debug!("Listing media with prefix: {}", prefix_str);

        let objects = self
            .media_config
            .storage
            .list_objects(if prefix_str.is_empty() {
                None
            } else {
                Some(prefix_str.as_str())
            })
            .await?;

        let media_list = objects
            .into_iter()
            .map(|obj| {
                let url = match &self.media_config.bucket_override {
                    Some(bucket) => format!(
                        "{}/storage/v1/object/{}/{}",
                        self.media_config.storage.supabase_url, bucket, obj.name
                    ),
                    None => format!("{}/media/{}", self.media_config.media_base_url, obj.name),
                };
                MediaMetadata {
                    url,
                    path: obj.name,
                    content_type: obj.content_type,
                    size: obj.size,
                    last_modified: obj.last_modified,
                }
            })
            .collect();

        Ok(media_list)
    }
}
