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
        include_bucket_query: bool,
    ) -> impl std::future::Future<Output = Result<Vec<MediaMetadata>, AppError>>;
}

impl ListMediaHandlerTrait for ListMediaHandler {
    async fn list_media(
        &self,
        prefix: Option<String>,
        include_bucket_query: bool,
    ) -> Result<Vec<MediaMetadata>, AppError> {
        let prefix_str = prefix.unwrap_or_default();
        debug!("Listing media with prefix: {}", prefix_str);

        let objects = self
            .media_config
            .storage
            .list_objects(
                self.media_config.bucket.as_str(),
                if prefix_str.is_empty() {
                    None
                } else {
                    Some(prefix_str.as_str())
                },
            )
            .await?;

        let media_list = objects
            .into_iter()
            .map(|obj| {
                let url = if include_bucket_query {
                    format!(
                        "{}/media/{}?bucket={}",
                        self.media_config.media_base_url, obj.name, self.media_config.bucket
                    )
                } else {
                    format!("{}/media/{}", self.media_config.media_base_url, obj.name)
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

    fn mock_list_body() -> serde_json::Value {
        json!([
            {
                "name": "anything.png",
                "id": "1",
                "updated_at": "2026-01-01T00:00:00Z",
                "metadata": { "size": "100", "mimetype": "image/png" }
            }
        ])
    }

    #[async_std::test]
    async fn response_url_uses_media_base_url_when_bucket_query_included() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/storage/v1/object/list/hi29831"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_list_body()))
            .mount(&server)
            .await;

        let config = make_config(&server.uri(), "http://localhost:8989", "hi29831");
        let handler = ListMediaHandler {
            media_config: config,
        };

        let items = handler.list_media(None, true).await.expect("list ok");
        assert_eq!(items.len(), 1);
        assert!(
            items[0].url.starts_with("http://localhost:8989/media/")
                && items[0].url.contains("?bucket=hi29831"),
            "url should use the media proxy with the requested bucket, got {}",
            items[0].url
        );
    }

    #[async_std::test]
    async fn response_url_uses_media_base_url_when_bucket_query_omitted() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/storage/v1/object/list/media"))
            .respond_with(ResponseTemplate::new(200).set_body_json(mock_list_body()))
            .mount(&server)
            .await;

        let config = make_default_config(&server.uri(), "http://localhost:8989");
        let handler = ListMediaHandler {
            media_config: config,
        };

        let items = handler.list_media(None, false).await.expect("list ok");
        assert_eq!(items.len(), 1);
        assert!(
            items[0].url.starts_with("http://localhost:8989/media/"),
            "url should use media_base_url + /media/, got {}",
            items[0].url
        );
    }
}
