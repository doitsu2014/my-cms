//! `domain_media` — self-contained media domain service.
//!
//! Owns bucket CRUD/access handlers, media create/list/read/delete/metadata
//! handlers, the Supabase storage adapter (`SupabaseStorage`), media DTOs
//! (`MediaConfig`, `MediaModel`, `MediaMetadata`), the content-type helpers
//! (`is_supported_content_type`, `is_image_content_type`), the media cache
//! (`MediaCacheKey`, `CachedMedia`), and the bucket visibility cache and
//! `BucketAccessPolicy` policy. The crate depends only on `domain_interface`
//! and SHALL NOT depend on any concrete business domain.
//!
//! See `openspec/changes/split-media-and-user-domains-merge-tags-into-posts/design.md`
//! for the architectural context.

#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod domain;
pub mod handlers;
pub mod observability;

pub use domain::error::AppError;
pub use domain::extensions::StringExtension;
