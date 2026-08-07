//! Media-domain handlers — owns bucket CRUD/access, media
//! create/list/read/delete/metadata, the Supabase storage adapter
//! (`SupabaseStorage`), the cache types (`MediaConfig`, `CachedMedia`,
//! `MediaCacheKey`, `BucketAccessPolicy`, `MediaModel`, `MediaMetadata`),
//! and the content-type helpers (`is_supported_content_type`,
//! `is_image_content_type`).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod bucket;
pub mod create;
pub mod delete;
pub mod list;
pub mod read;
pub mod supabase_storage;

pub use bucket::access::access_handler::{BucketAccessPolicy, BucketAccessPolicyTrait};
pub use bucket::dto::{Bucket, CreateBucketRequest, UpdateBucketRequest};
pub use read::read_handler::{CachedMedia, MediaCacheKey};
pub use supabase_storage::{DeletedObject, StorageObject, StorageObjectMetadata, SupabaseStorage};

pub use create::create_handler::{CreateMediaHandler, CreateMediaHandlerTrait};
pub use delete::delete_handler::{DeleteMediaHandler, DeleteMediaHandlerTrait};
pub use list::list_handler::{ListMediaHandler, ListMediaHandlerTrait};
pub use read::metadata_handler::{MetadataMediaHandler, MetadataMediaHandlerTrait};
pub use read::read_handler::{ReadMediaHandler, ReadMediaHandlerTrait};

#[derive(Clone, Debug)]
pub struct MediaConfig {
    pub storage: SupabaseStorage,
    pub bucket: String,
    pub media_base_url: String,
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
