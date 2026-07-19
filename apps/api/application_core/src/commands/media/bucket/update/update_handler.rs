use crate::{
    commands::media::{
        bucket::{
            dto::{is_valid_bucket_name, Bucket},
            update::update_request::UpdateBucketRequest,
        },
        MediaConfig,
    },
    common::app_error::AppError,
};
use serde_json::json;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug)]
pub struct UpdateBucketHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait UpdateBucketHandlerTrait {
    fn update_bucket(
        &self,
        name: &str,
        req: UpdateBucketRequest,
    ) -> impl std::future::Future<Output = Result<Bucket, AppError>>;
}

impl UpdateBucketHandlerTrait for UpdateBucketHandler {
    #[instrument(skip(self, req))]
    async fn update_bucket(
        &self,
        name: &str,
        req: UpdateBucketRequest,
    ) -> Result<Bucket, AppError> {
        if !is_valid_bucket_name(name) {
            return Err(AppError::Validation(
                "name".to_string(),
                "must start with a lowercase letter; only [a-z0-9_-] allowed".to_string(),
            ));
        }

        if req.public.is_none() && req.file_size_limit.is_none() && req.allowed_mime_types.is_none()
        {
            return Err(AppError::Validation(
                "body".to_string(),
                "at least one field must be present".to_string(),
            ));
        }

        let mut payload = json!({});
        if let Some(public) = req.public {
            payload["public"] = json!(public);
        }
        if let Some(limit) = req.file_size_limit {
            payload["file_size_limit"] = match limit {
                Some(v) => json!(v),
                None => json!(null),
            };
        }
        if let Some(mime_types) = req.allowed_mime_types {
            payload["allowed_mime_types"] = match mime_types {
                Some(v) => json!(v),
                None => json!(null),
            };
        }

        self.media_config.storage.update_bucket(name, payload).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn make_config() -> Arc<MediaConfig> {
        Arc::new(MediaConfig {
            storage: crate::commands::media::SupabaseStorage::new(
                "http://example.com",
                "anon",
                None,
            ),
            bucket: "media".to_string(),
            media_base_url: "http://example.com".to_string(),
        })
    }

    #[async_std::test]
    async fn rejects_empty_body() {
        let handler = UpdateBucketHandler {
            media_config: make_config(),
        };
        let req = UpdateBucketRequest {
            public: None,
            file_size_limit: None,
            allowed_mime_types: None,
        };
        let result = handler.update_bucket("media", req).await;
        match result {
            Err(AppError::Validation(field, msg)) => {
                assert_eq!(field, "body");
                assert!(msg.contains("at least one field"));
            }
            other => panic!("expected Validation, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn rejects_invalid_name() {
        let handler = UpdateBucketHandler {
            media_config: make_config(),
        };
        let req = UpdateBucketRequest {
            public: Some(true),
            file_size_limit: None,
            allowed_mime_types: None,
        };
        let result = handler.update_bucket("BadName", req).await;
        match result {
            Err(AppError::Validation(field, _)) => assert_eq!(field, "name"),
            other => panic!("expected Validation, got {:?}", other),
        }
    }
}
