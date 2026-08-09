use crate::{
    domain::error::AppError,
    handlers::{bucket::dto::is_valid_bucket_name, MediaConfig},
};
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug)]
pub struct EmptyBucketHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait EmptyBucketHandlerTrait {
    fn empty_bucket(&self, name: &str) -> impl std::future::Future<Output = Result<(), AppError>>;
}

impl EmptyBucketHandlerTrait for EmptyBucketHandler {
    #[instrument(skip(self))]
    async fn empty_bucket(&self, name: &str) -> Result<(), AppError> {
        if !is_valid_bucket_name(name) {
            return Err(AppError::Validation(
                "name".to_string(),
                "must start with a lowercase letter; only [a-z0-9_-] allowed".to_string(),
            ));
        }
        self.media_config.storage.empty_bucket(name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn make_config() -> Arc<MediaConfig> {
        Arc::new(MediaConfig {
            storage: crate::handlers::supabase_storage::SupabaseStorage::new(
                "http://example.com",
                "anon",
                None,
            ),
            bucket: "media".to_string(),
            media_base_url: "http://example.com".to_string(),
        })
    }

    #[tokio::test]
    async fn rejects_invalid_name() {
        let handler = EmptyBucketHandler {
            media_config: make_config(),
        };
        let result = handler.empty_bucket("BadName").await;
        match result {
            Err(AppError::Validation(field, _)) => assert_eq!(field, "name"),
            other => panic!("expected Validation, got {:?}", other),
        }
    }
}
