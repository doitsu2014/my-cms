use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod bucket;
pub mod create;
pub mod delete;
pub mod list;
pub mod read;
pub mod supabase_storage;

pub use bucket::dto::{Bucket, CreateBucketRequest, UpdateBucketRequest};
pub use supabase_storage::{DeletedObject, StorageObject, StorageObjectMetadata, SupabaseStorage};

#[derive(Clone, Debug)]
pub struct MediaConfig {
    pub storage: SupabaseStorage,
    pub media_base_url: String,
    pub bucket_override: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaModel {
    pub path: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    pub path: String,
    pub url: String,
    pub content_type: String,
    pub size: u64,
    pub last_modified: Option<DateTime<Utc>>,
}

pub fn is_supported_content_type(content_type: &str) -> bool {
    content_type.starts_with("image/")
        || content_type == "application/pdf"
        || content_type == "application/msword"
        || content_type.starts_with("application/vnd.openxmlformats-officedocument.")
        || content_type.starts_with("application/vnd.ms-")
        || content_type.starts_with("text/")
}

pub fn is_image_content_type(content_type: &str) -> bool {
    content_type.starts_with("image/")
}
