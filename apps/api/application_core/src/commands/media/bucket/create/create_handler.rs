use crate::{
    commands::media::{
        bucket::{
            create::create_request::CreateBucketRequest,
            dto::{is_valid_bucket_name, Bucket},
        },
        MediaConfig,
    },
    common::app_error::AppError,
};
use serde_json::json;
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug)]
pub struct CreateBucketHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait CreateBucketHandlerTrait {
    fn create_bucket(
        &self,
        req: CreateBucketRequest,
    ) -> impl std::future::Future<Output = Result<Bucket, AppError>>;
}

const RESERVED_BUCKET_NAME: &str = "media";

impl CreateBucketHandlerTrait for CreateBucketHandler {
    #[instrument(skip(self))]
    async fn create_bucket(&self, req: CreateBucketRequest) -> Result<Bucket, AppError> {
        if !is_valid_bucket_name(&req.name) {
            return Err(AppError::Validation(
                "name".to_string(),
                "must start with a lowercase letter; only [a-z0-9_-] allowed".to_string(),
            ));
        }
        if req.name == RESERVED_BUCKET_NAME {
            return Err(AppError::Validation(
                "name".to_string(),
                format!("cannot use reserved bucket name '{}'", RESERVED_BUCKET_NAME),
            ));
        }

        let mut payload = json!({
            "name": req.name,
            "public": req.public.unwrap_or(false),
        });
        if let Some(limit) = req.file_size_limit {
            payload["file_size_limit"] = json!(limit);
        }
        if let Some(mime_types) = req.allowed_mime_types {
            payload["allowed_mime_types"] = json!(mime_types);
        }

        self.media_config.storage.create_bucket(payload).await
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
                "media",
            ),
            media_base_url: "http://example.com".to_string(),
            bucket_override: None,
        })
    }

    #[async_std::test]
    async fn rejects_invalid_name() {
        let handler = CreateBucketHandler {
            media_config: make_config(),
        };
        let req = CreateBucketRequest {
            name: "BadName".to_string(),
            public: Some(false),
            file_size_limit: None,
            allowed_mime_types: None,
        };
        let result = handler.create_bucket(req).await;
        match result {
            Err(AppError::Validation(field, _)) => assert_eq!(field, "name"),
            other => panic!("expected Validation, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn rejects_reserved_name() {
        let handler = CreateBucketHandler {
            media_config: make_config(),
        };
        let req = CreateBucketRequest {
            name: "media".to_string(),
            public: Some(false),
            file_size_limit: None,
            allowed_mime_types: None,
        };
        let result = handler.create_bucket(req).await;
        match result {
            Err(AppError::Validation(field, msg)) => {
                assert_eq!(field, "name");
                assert!(msg.contains("reserved"));
            }
            other => panic!("expected Validation, got {:?}", other),
        }
    }
}
