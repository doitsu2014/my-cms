use s3::{creds::Credentials, Bucket, Region};

pub mod create;
pub mod delete;
pub mod read;

pub struct S3MediaStorage {
    pub s3_region: Region,
    pub s3_credentials: Credentials,
    pub s3_bucket_name: String,
}

impl S3MediaStorage {
    fn spawn_bucket() -> Bucket {
        let bucket = Bucket::new(self.s3_bucket_name, self.s3_region, self.s3_credentials).unwrap();
        bucket
    }
}
