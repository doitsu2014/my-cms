use crate::{commands::media::MediaConfig, common::app_error::AppError};
use std::sync::Arc;
use tracing::{debug, info};

pub struct DeleteMediaHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait DeleteMediaHandlerTrait {
    fn delete_media(
        &self,
        path: String,
    ) -> impl std::future::Future<Output = Result<(), AppError>>;

    fn delete_media_batch(
        &self,
        paths: Vec<String>,
    ) -> impl std::future::Future<Output = Result<u64, AppError>>;
}

impl DeleteMediaHandlerTrait for DeleteMediaHandler {
    async fn delete_media(&self, path: String) -> Result<(), AppError> {
        let bucket = self.media_config.s3_media_storage.spawn_bucket()?;

        debug!("Deleting media: {}", path);

        bucket
            .delete_object(&path)
            .await
            .map_err(|e| AppError::Logical(format!("Failed to delete S3 object: {}", e)))?;

        info!("Deleted media: {}", path);
        Ok(())
    }

    async fn delete_media_batch(&self, paths: Vec<String>) -> Result<u64, AppError> {
        let bucket = self.media_config.s3_media_storage.spawn_bucket()?;
        let mut deleted_count: u64 = 0;

        for path in &paths {
            debug!("Deleting media: {}", path);
            match bucket.delete_object(path).await {
                Ok(_) => {
                    deleted_count += 1;
                    info!("Deleted media: {}", path);
                }
                Err(e) => {
                    debug!("Failed to delete {}: {}", path, e);
                }
            }
        }

        Ok(deleted_count)
    }
}
