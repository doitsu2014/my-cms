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
                "{}/media/{}?bucket={}",
                self.media_config.media_base_url, path, bucket
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::media::SupabaseStorage;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn make_config(server_uri: &str, public_url: &str, bucket: &str) -> Arc<MediaConfig> {
        let storage = SupabaseStorage::new(server_uri, "anon", None, bucket);
        Arc::new(MediaConfig {
            storage,
            media_base_url: public_url.to_string(),
            bucket_override: Some(bucket.to_string()),
        })
    }

    fn make_default_config(server_uri: &str, public_url: &str) -> Arc<MediaConfig> {
        let storage = SupabaseStorage::new(server_uri, "anon", None, "media");
        Arc::new(MediaConfig {
            storage,
            media_base_url: public_url.to_string(),
            bucket_override: None,
        })
    }

    fn mock_info_body() -> serde_json::Value {
        json!({
            "name": "test.png",
            "size": 1234,
            "mimetype": "image/png",
            "last_modified": "2026-01-01T00:00:00Z",
            "http_metadata": { "contentType": "image/png" }
        })
    }

    #[async_std::test]
    async fn response_url_uses_media_base_url_when_bucket_override_set() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/storage/v1/object/info/hi29831/test.png"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_info_body()))
            .mount(&server)
            .await;

        let config = make_config(&server.uri(), "http://localhost:8989", "hi29831");
        let handler = MetadataMediaHandler {
            media_config: config,
        };

        let meta = handler
            .get_metadata("test.png".to_string())
            .await
            .expect("get_metadata ok");
        assert!(
            meta.url.starts_with("http://localhost:8989/media/")
                && meta.url.contains("?bucket=hi29831"),
            "url should use the media proxy with the requested bucket, got {}",
            meta.url
        );
    }

    #[async_std::test]
    async fn response_url_uses_media_base_url_when_no_bucket_override() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/storage/v1/object/info/media/test.png"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_info_body()))
            .mount(&server)
            .await;

        let config = make_default_config(&server.uri(), "http://localhost:8989");
        let handler = MetadataMediaHandler {
            media_config: config,
        };

        let meta = handler
            .get_metadata("test.png".to_string())
            .await
            .expect("get_metadata ok");
        assert!(
            meta.url.starts_with("http://localhost:8989/media/"),
            "url should use media_base_url + /media/, got {}",
            meta.url
        );
    }
}
