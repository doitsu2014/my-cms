use std::sync::Arc;

use s3::Bucket;
use tracing::info;

use crate::{commands::media::S3MediaStorage, common::app_error::AppError};

pub struct CreateMediaHandler {
    pub s3_media_storage: Arc<S3MediaStorage>,
}

pub trait CreateMediaHandlerTrait {
    fn create_image_media(
        &self,
        media: Box<[u8]>,
    ) -> impl std::future::Future<Output = Result<(), AppError>>;
}

impl CreateMediaHandlerTrait for CreateMediaHandler {
    async fn create_image_media(&self, media: Box<[u8]>) -> Result<(), AppError> {
        let bucket = self.s3_media_storage.spawn_bucket()?;
        let response = bucket.put_object("a", &media).await.map_err(|e| e.into())?;
        info!("{:?}", response);
        Ok(())
    }
}
