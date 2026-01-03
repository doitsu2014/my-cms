use s3::{creds::Credentials, Bucket, Region};
use serde::{Deserialize, Serialize};

use crate::common::app_error::AppError;

pub mod create;
pub mod delete;
pub mod read;

#[derive(Clone, Debug)]
pub struct MediaConfig {
    pub s3_media_storage: S3MediaStorage,
    pub media_base_url: String,
}

#[derive(Clone, Debug)]
pub struct S3MediaStorage {
    pub s3_endpoint: String,
    pub s3_credentials: Credentials,
    pub s3_bucket_name: String,
}

impl S3MediaStorage {
    pub fn spawn_bucket(&self) -> Result<Box<Bucket>, AppError> {
        let region = Region::Custom {
            region: "".to_string(),
            endpoint: self.s3_endpoint.clone(),
        };
        Bucket::new(&self.s3_bucket_name, region, self.s3_credentials.to_owned())
            .map(|b| b.with_path_style())
            .map_err(|e| e.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaModel {
    pub path: String,
    pub url: String,
}
