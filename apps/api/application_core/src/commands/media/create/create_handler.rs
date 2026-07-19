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
        include_bucket_query: bool,
    ) -> impl std::future::Future<Output = Result<MediaModel, AppError>>;
}

impl CreateMediaHandlerTrait for CreateMediaHandler {
    async fn create_media(
        &self,
        media_name: String,
        media: &[u8],
        content_type: String,
        include_bucket_query: bool,
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
            .upload(
                self.media_config.bucket.as_str(),
                &beautiful_media_name,
                media,
                content_type.as_str(),
                None,
            )
            .await?;
        info!("Uploaded media: {}", beautiful_media_name);

        let url_path = if include_bucket_query {
            format!(
                "{}/media/{}?bucket={}",
                self.media_config.media_base_url, beautiful_media_name, self.media_config.bucket
            )
        } else if is_image_content_type(&content_type) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::media::SupabaseStorage;
    use wiremock::{
        matchers::{method, path_regex},
        Mock, MockServer, ResponseTemplate,
    };

    fn make_config(server_uri: &str, public_url: &str, bucket: &str) -> Arc<MediaConfig> {
        let storage = SupabaseStorage::new(server_uri, "anon", None);
        Arc::new(MediaConfig {
            storage,
            bucket: bucket.to_string(),
            media_base_url: public_url.to_string(),
        })
    }

    fn make_default_config(server_uri: &str, public_url: &str) -> Arc<MediaConfig> {
        let storage = SupabaseStorage::new(server_uri, "anon", None);
        Arc::new(MediaConfig {
            storage,
            bucket: "media".to_string(),
            media_base_url: public_url.to_string(),
        })
    }

    #[async_std::test]
    async fn response_url_uses_media_base_url_when_bucket_query_included() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex("/storage/v1/object/hi29831/.*\\.png"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&server)
            .await;

        let config = make_config(&server.uri(), "http://localhost:8989", "hi29831");
        let handler = CreateMediaHandler {
            media_config: config,
        };

        let media = handler
            .create_media(
                "anything.png".to_string(),
                b"data",
                "image/png".to_string(),
                true,
            )
            .await
            .expect("create ok");

        assert!(
            media.url.starts_with("http://localhost:8989/media/")
                && media.url.contains("?bucket=hi29831"),
            "url should use the media proxy with the requested bucket, got {}",
            media.url
        );
    }

    #[async_std::test]
    async fn response_url_uses_media_base_url_when_bucket_query_omitted_for_image() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex("/storage/v1/object/media/.*\\.png"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&server)
            .await;

        let config = make_default_config(&server.uri(), "http://localhost:8989");
        let handler = CreateMediaHandler {
            media_config: config,
        };

        let media = handler
            .create_media(
                "anything.png".to_string(),
                b"data",
                "image/png".to_string(),
                false,
            )
            .await
            .expect("create ok");

        assert!(
            media.url.starts_with("http://localhost:8989/media/images/"),
            "url should use media_base_url + /media/images/, got {}",
            media.url
        );
    }
}
