//! Media-domain handlers — owns bucket CRUD/access, media
//! create/list/read/delete/metadata, the Supabase storage adapter
//! (`SupabaseStorage`), the cache types (`MediaConfig`, `CachedMedia`,
//! `MediaCacheKey`, `BucketAccessPolicy`, `MediaModel`, `MediaMetadata`),
//! and the content-type helpers (`is_supported_content_type`,
//! `is_image_content_type`).

use chrono::{DateTime, Utc};
use domain_interface::DomainConfigError;
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

impl MediaConfig {
    /// Build a `MediaConfig` from process environment variables.
    ///
    /// Reads `SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY`, `MEDIA_BUCKET`,
    /// and `MEDIA_BASE_URL` in that order. Returns
    /// [`DomainConfigError::MissingEnv`] for the first variable that is not
    /// set. The factory is pure: it makes no I/O and never panics.
    ///
    /// The `SupabaseStorage` is constructed with the `service_role_key` as
    /// the admin credential; the public `anon_key` slot is filled with a
    /// placeholder because `SupabaseStorage::auth_key()` always prefers the
    /// service role key when one is present.
    pub fn from_env() -> Result<Self, DomainConfigError> {
        let supabase_url = std::env::var("SUPABASE_URL")
            .map_err(|_| DomainConfigError::MissingEnv("SUPABASE_URL"))?;
        let service_role_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY")
            .map_err(|_| DomainConfigError::MissingEnv("SUPABASE_SERVICE_ROLE_KEY"))?;
        let bucket = std::env::var("MEDIA_BUCKET")
            .map_err(|_| DomainConfigError::MissingEnv("MEDIA_BUCKET"))?;
        let media_base_url = std::env::var("MEDIA_BASE_URL")
            .map_err(|_| DomainConfigError::MissingEnv("MEDIA_BASE_URL"))?;

        Ok(Self {
            storage: SupabaseStorage::new(supabase_url, "anon", Some(service_role_key)),
            bucket,
            media_base_url,
        })
    }
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

#[cfg(test)]
mod tests {
    //! Module-level tests for the media-domain handler module. Covers the
    //! `MediaConfig::from_env` factory in isolation — no I/O, no async.
    //!
    //! Note: this `mod tests` shares the lib's test target with the
    //! pre-existing `async_std::test` fixtures in
    //! `apps/api/domain_media/src/handlers/{bucket,create,delete,list,read}/`,
    //! which currently fail to build because `async-std` is not declared
    //! with the `attributes` feature in `Cargo.toml`. That is a
    //! pre-existing failure owned by a separate change; the new tests
    //! here are sync and add no new compile errors.

    use super::*;
    use std::sync::Mutex;

    /// Serialises env-mutating tests so concurrent `cargo test` threads
    /// don't race the global process env.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_env_var<F, R>(var: &str, value: Option<&str>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let previous = std::env::var(var).ok();
        match value {
            Some(v) => std::env::set_var(var, v),
            None => std::env::remove_var(var),
        }
        let result = f();
        match previous {
            Some(v) => std::env::set_var(var, v),
            None => std::env::remove_var(var),
        }
        result
    }

    fn clear_all_media_env() {
        std::env::remove_var("SUPABASE_URL");
        std::env::remove_var("SUPABASE_SERVICE_ROLE_KEY");
        std::env::remove_var("MEDIA_BUCKET");
        std::env::remove_var("MEDIA_BASE_URL");
    }

    #[test]
    fn media_config_from_env_returns_ok_when_all_vars_set() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_all_media_env();

        let result = with_env_var("SUPABASE_URL", Some("http://example.com"), || {
            with_env_var("SUPABASE_SERVICE_ROLE_KEY", Some("svc-key"), || {
                with_env_var("MEDIA_BUCKET", Some("media"), || {
                    with_env_var("MEDIA_BASE_URL", Some("http://example.com/media"), || {
                        MediaConfig::from_env()
                    })
                })
            })
        });

        let cfg = result.expect("expected Ok when all four env vars are set");
        assert_eq!(cfg.bucket, "media");
        assert_eq!(cfg.media_base_url, "http://example.com/media");
        assert_eq!(cfg.storage.supabase_url, "http://example.com");
        assert_eq!(
            cfg.storage.service_role_key.as_deref(),
            Some("svc-key")
        );
    }

    #[test]
    fn media_config_from_env_returns_missing_env_for_supabase_url_when_unset() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        clear_all_media_env();

        let result = with_env_var("SUPABASE_URL", None, || {
            with_env_var("SUPABASE_SERVICE_ROLE_KEY", Some("svc-key"), || {
                with_env_var("MEDIA_BUCKET", Some("media"), || {
                    with_env_var("MEDIA_BASE_URL", Some("http://example.com/media"), || {
                        MediaConfig::from_env()
                    })
                })
            })
        });

        match result {
            Err(DomainConfigError::MissingEnv(v)) => assert_eq!(v, "SUPABASE_URL"),
            other => panic!("expected MissingEnv(SUPABASE_URL), got {:?}", other),
        }
    }
}
