use crate::{
    commands::media::{is_image_content_type, MediaConfig, MediaModel},
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
    fn create_media(
        &self,
        media_name: String,
        media: &[u8],
        content_type: String,
    ) -> impl std::future::Future<Output = Result<MediaModel, AppError>>;
}

impl CreateMediaHandlerTrait for CreateMediaHandler {
    async fn create_media(
        &self,
        media_name: String,
        media: &[u8],
        content_type: String,
    ) -> Result<MediaModel, AppError> {
        let media_extension = media_name
            .clone()
            .split('.')
            .last()
            .unwrap_or_default()
            .to_string();
        let media_name_with_nano = format!("{} {}", nanoid!(10), media_name.clone()).to_slug();
        let beautiful_media_name = format!("{}.{}", media_name_with_nano, media_extension);

        self.media_config
            .storage
            .upload(&beautiful_media_name, media, content_type.as_str(), None)
            .await?;
        info!("Uploaded media: {}", beautiful_media_name);

        let url_path = if is_image_content_type(&content_type) {
            format!(
                "{}/media/images/{}",
                self.media_config.media_base_url, beautiful_media_name
            )
        } else {
            format!(
                "{}/media/{}",
                self.media_config.media_base_url, beautiful_media_name
            )
        };

        Ok(MediaModel {
            path: beautiful_media_name,
            url: url_path,
        })
    }
}
