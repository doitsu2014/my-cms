use crate::{
    commands::media::{bucket::dto::Bucket, MediaConfig},
    common::app_error::AppError,
};
use std::sync::Arc;
use tracing::instrument;

#[derive(Debug)]
pub struct ListBucketsHandler {
    pub media_config: Arc<MediaConfig>,
}

pub trait ListBucketsHandlerTrait {
    fn list_buckets(&self) -> impl std::future::Future<Output = Result<Vec<Bucket>, AppError>>;
}

impl ListBucketsHandlerTrait for ListBucketsHandler {
    #[instrument]
    async fn list_buckets(&self) -> Result<Vec<Bucket>, AppError> {
        self.media_config.storage.list_buckets().await
    }
}
