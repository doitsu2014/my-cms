use crate::{
    commands::media::{MediaConfig, MediaModel},
    common::app_error::AppError,
    StringExtension,
};
use nanoid::nanoid;
use std::sync::Arc;
use tracing::info;

pub struct CreateMediaHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait CreateMediaHandlerTrait {
    fn create_image_media(
        &self,
        media_name: String,
        media: &[u8],
        content_type: String,
    ) -> impl std::future::Future<Output = Result<MediaModel, AppError>>;
}

impl CreateMediaHandlerTrait for CreateMediaHandler {
    async fn create_image_media(
        &self,
        media_name: String,
        media: &[u8],
        content_type: String,
    ) -> Result<MediaModel, AppError> {
        let bucket = self.media_config.s3_media_storage.spawn_bucket()?;
        let media_extension = media_name
            .clone()
            .split('.')
            .last()
            .unwrap_or_default()
            .to_string();
        let media_name_with_nano = format!("{} {}", nanoid!(10), media_name.clone()).to_slug();
        let beautiful_media_name = format!("{}.{}", media_name_with_nano, media_extension);
        let response = bucket
            .put_object_with_content_type(
                beautiful_media_name.clone(),
                media,
                content_type.as_str(),
            )
            .await
            .map_err(|e| e.into())?;
        info!("{:?}", response);
        let bucket_name = &self.media_config.s3_media_storage.s3_bucket_name;
        Ok(MediaModel {
            server: self.media_config.media_imgproxy_server.clone(),
            path: beautiful_media_name.clone(),
            imgproxy_url: format!(
                "{}/insecure/rs:fit/plain/s3://{}/{}",
                self.media_config.media_imgproxy_server, bucket_name, beautiful_media_name
            ),
        })
    }
}
