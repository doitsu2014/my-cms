use crate::commands::media::SupabaseStorage;
use crate::common::app_error::AppError;
use moka::future::Cache;
use std::sync::Arc;

pub trait BucketAccessPolicyTrait {
    fn ensure_public_or_admin(
        &self,
        bucket_name: &str,
        is_admin: bool,
    ) -> impl std::future::Future<Output = Result<(), AppError>>;
}

pub struct BucketAccessPolicy {
    pub storage: SupabaseStorage,
    pub cache: Arc<Cache<String, bool>>,
}

impl BucketAccessPolicyTrait for BucketAccessPolicy {
    async fn ensure_public_or_admin(
        &self,
        bucket_name: &str,
        is_admin: bool,
    ) -> Result<(), AppError> {
        if is_admin {
            return Ok(());
        }
        let key = bucket_name.to_string();
        let is_public = if let Some(cached) = self.cache.get(&key).await {
            cached
        } else {
            let bucket = self.storage.get_bucket(bucket_name).await?;
            let public = bucket.public;
            self.cache.insert(key, public).await;
            public
        };
        if is_public {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::media::bucket::access::access_cache::create_bucket_visibility_cache;
    use reqwest::Client;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn make_storage(base_url: &str, with_service_role: bool) -> SupabaseStorage {
        SupabaseStorage {
            supabase_url: base_url.to_string(),
            anon_key: "anon-test-key".to_string(),
            service_role_key: if with_service_role {
                Some("service-role-test-key".to_string())
            } else {
                None
            },
            client: Client::new(),
        }
    }

    fn public_bucket_body(name: &str) -> serde_json::Value {
        json!({
            "id": name,
            "name": name,
            "public": true,
            "file_size_limit": null,
            "allowed_mime_types": null,
            "owner": null,
            "type": "STANDARD",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-02T00:00:00Z"
        })
    }

    fn private_bucket_body(name: &str) -> serde_json::Value {
        json!({
            "id": name,
            "name": name,
            "public": false,
            "file_size_limit": null,
            "allowed_mime_types": null,
            "owner": null,
            "type": "STANDARD",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-02T00:00:00Z"
        })
    }

    fn make_cache() -> Arc<Cache<String, bool>> {
        Arc::new(create_bucket_visibility_cache())
    }

    #[async_std::test]
    async fn ensure_public_or_admin_passes_when_bucket_is_public_and_caller_is_anon() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket/avatars"))
            .respond_with(ResponseTemplate::new(200).set_body_json(public_bucket_body("avatars")))
            .mount(&server)
            .await;

        let policy = BucketAccessPolicy {
            storage,
            cache: make_cache(),
        };

        let result = policy.ensure_public_or_admin("avatars", false).await;
        assert!(matches!(result, Ok(())));
    }

    #[async_std::test]
    async fn ensure_public_or_admin_rejects_with_not_found_when_bucket_is_private_and_caller_is_anon(
    ) {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket/private-docs"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(private_bucket_body("private-docs")),
            )
            .mount(&server)
            .await;

        let policy = BucketAccessPolicy {
            storage,
            cache: make_cache(),
        };

        let result = policy.ensure_public_or_admin("private-docs", false).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn ensure_public_or_admin_skips_supabase_when_is_admin_true() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket/anything"))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&server)
            .await;

        let policy = BucketAccessPolicy {
            storage,
            cache: make_cache(),
        };

        let result = policy.ensure_public_or_admin("anything", true).await;
        assert!(matches!(result, Ok(())));
    }

    #[async_std::test]
    async fn ensure_public_or_admin_caches_public_flag_across_calls() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket/avatars"))
            .respond_with(ResponseTemplate::new(200).set_body_json(public_bucket_body("avatars")))
            .expect(1)
            .mount(&server)
            .await;

        let policy = BucketAccessPolicy {
            storage,
            cache: make_cache(),
        };

        let first = policy.ensure_public_or_admin("avatars", false).await;
        assert!(matches!(first, Ok(())));

        let second = policy.ensure_public_or_admin("avatars", false).await;
        assert!(matches!(second, Ok(())));
    }
}
