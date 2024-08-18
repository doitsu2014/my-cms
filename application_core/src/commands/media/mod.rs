use s3::{creds::Credentials, Bucket, Region};

use crate::common::app_error::AppError;

pub mod create;
pub mod delete;
pub mod read;

#[derive(Clone, Debug)]
pub struct S3MediaStorage {
    pub s3_region: Region,
    pub s3_credentials: Credentials,
    pub s3_bucket_name: String,
}

impl S3MediaStorage {
    pub fn spawn_bucket(&self) -> Result<Bucket, AppError> {
        Bucket::new(
            &self.s3_bucket_name,
            self.s3_region.to_owned(),
            self.s3_credentials.to_owned(),
        )
        .map_err(|e| e.into())
    }
}
