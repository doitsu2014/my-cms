use std::sync::Arc;

use s3::Bucket;

use crate::commands::media::S3MediaStorage;

pub struct CreateMediaHandler {
    pub s3_media_storage: Arc<S3MediaStorage>,
}

pub trait CreateMediaHandlerTrait {
    fn create_media(&self, media: Vec<u8>);
}

impl CreateMediaHandlerTrait for CreateMediaHandler {
    fn create_media(&self, media: Vec<u8>) {
        let bucket = Bucket::new(bucket_name, region, credentials)?;

        todo!()
    }
}
