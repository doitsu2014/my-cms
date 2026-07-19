use crate::{commands::media::MediaConfig, common::app_error::AppError};
use std::sync::Arc;
use tracing::{debug, info};

pub struct DeleteMediaHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait DeleteMediaHandlerTrait {
    fn delete_media(&self, path: String)
        -> impl std::future::Future<Output = Result<(), AppError>>;

    fn delete_media_batch(
        &self,
        paths: Vec<String>,
    ) -> impl std::future::Future<Output = Result<u64, AppError>>;
}

impl DeleteMediaHandlerTrait for DeleteMediaHandler {
    async fn delete_media(&self, path: String) -> Result<(), AppError> {
        debug!("Deleting media: {}", path);
        self.media_config
            .storage
            .delete(self.media_config.bucket.as_str(), &path)
            .await?;
        info!("Deleted media: {}", path);
        Ok(())
    }

    async fn delete_media_batch(&self, paths: Vec<String>) -> Result<u64, AppError> {
        debug!("Deleting media batch ({} items)", paths.len());
        let result = self
            .media_config
            .storage
            .delete_batch(self.media_config.bucket.as_str(), &paths)
            .await?;
        info!("Batch deleted {} media items", result.len());
        Ok(result.len() as u64)
    }
}
