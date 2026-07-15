use crate::{
    commands::media::{bucket::dto::is_valid_bucket_name, MediaConfig},
    common::app_error::AppError,
};
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug)]
pub struct DeleteBucketHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait DeleteBucketHandlerTrait {
    fn delete_bucket(
        &self,
        name: &str,
        purge: bool,
    ) -> impl std::future::Future<Output = Result<(), AppError>>;
}

const RESERVED_BUCKET_NAME: &str = "media";

impl DeleteBucketHandlerTrait for DeleteBucketHandler {
    #[instrument(skip(self))]
    async fn delete_bucket(&self, name: &str, purge: bool) -> Result<(), AppError> {
        if !is_valid_bucket_name(name) {
            return Err(AppError::Validation(
                "name".to_string(),
                "must start with a lowercase letter; only [a-z0-9_-] allowed".to_string(),
            ));
        }
        if name == RESERVED_BUCKET_NAME {
            return Err(AppError::Validation(
                "name".to_string(),
                format!(
                    "cannot delete reserved bucket name '{}'",
                    RESERVED_BUCKET_NAME
                ),
            ));
        }

        match self.media_config.storage.delete_bucket(name, purge).await {
            Ok(()) => Ok(()),
            Err(AppError::StorageError(msg)) => {
                if msg.contains("not empty") || msg.contains("not_empty") {
                    Err(AppError::Conflict(format!(
                        "Bucket '{}' is not empty; pass ?purge=true to delete with all objects",
                        name
                    )))
                } else {
                    Err(AppError::StorageError(msg))
                }
            }
            Err(e) => Err(e),
        }
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
    async fn rejects_reserved_name() {
        let handler = DeleteBucketHandler {
            media_config: make_config(),
        };
        let result = handler.delete_bucket("media", false).await;
        match result {
            Err(AppError::Validation(field, msg)) => {
                assert_eq!(field, "name");
                assert!(msg.contains("reserved"));
            }
            other => panic!("expected Validation, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn rejects_invalid_name() {
        let handler = DeleteBucketHandler {
            media_config: make_config(),
        };
        let result = handler.delete_bucket("BadName", false).await;
        assert!(matches!(result, Err(AppError::Validation(_, _))));
    }
}
