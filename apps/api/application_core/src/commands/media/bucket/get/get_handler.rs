use crate::{
    commands::media::{
        bucket::dto::{is_valid_bucket_name, Bucket},
        MediaConfig,
    },
    common::app_error::AppError,
};
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug)]
pub struct GetBucketHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait GetBucketHandlerTrait {
    fn get_bucket(&self, name: &str)
        -> impl std::future::Future<Output = Result<Bucket, AppError>>;
}

impl GetBucketHandlerTrait for GetBucketHandler {
    #[instrument(skip(self))]
    async fn get_bucket(&self, name: &str) -> Result<Bucket, AppError> {
        if !is_valid_bucket_name(name) {
            return Err(AppError::Validation(
                "name".to_string(),
                "must start with a lowercase letter; only [a-z0-9_-] allowed".to_string(),
            ));
        }
        self.media_config.storage.get_bucket(name).await
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
    async fn get_bucket_rejects_uppercase_name() {
        let handler = GetBucketHandler {
            media_config: make_config(),
        };
        let result = handler.get_bucket("BadName").await;
        match result {
            Err(AppError::Validation(field, _)) => assert_eq!(field, "name"),
            other => panic!("expected Validation error, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn get_bucket_rejects_name_starting_with_digit() {
        let handler = GetBucketHandler {
            media_config: make_config(),
        };
        let result = handler.get_bucket("3d-models").await;
        assert!(matches!(result, Err(AppError::Validation(_, _))));
    }
}
