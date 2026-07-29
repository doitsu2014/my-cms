//! Supabase storage adapter and media config used by the post domain's media
//! delivery routes (when they are eventually moved). The legacy source lives
//! in `application_core::commands::media::{supabase_storage, read, bucket}`.

pub use application_core::commands::media::{
    bucket::{
        access::access_cache::{create_bucket_visibility_cache, BucketVisibilityCacheKey},
        dto::BucketDto,
    },
    read::read_handler::{
        create_media_cache, CachedMedia, MediaCacheKey,
    },
    MediaConfig, SupabaseStorage,
};