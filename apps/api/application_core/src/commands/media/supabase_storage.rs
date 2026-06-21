use crate::common::app_error::AppError;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageObject {
    pub name: String,
    pub content_type: String,
    pub size: u64,
    pub last_modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageObjectMetadata {
    pub content_type: String,
    pub size: u64,
    pub last_modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedObject {
    pub name: String,
}

#[derive(Clone)]
pub struct SupabaseStorage {
    pub supabase_url: String,
    pub anon_key: String,
    pub service_role_key: Option<String>,
    pub bucket: String,
    pub client: Client,
}

impl std::fmt::Debug for SupabaseStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SupabaseStorage")
            .field("supabase_url", &self.supabase_url)
            .field("anon_key", &"<redacted>")
            .field(
                "service_role_key",
                &self.service_role_key.as_ref().map(|_| "<redacted>"),
            )
            .field("bucket", &self.bucket)
            .finish()
    }
}

impl SupabaseStorage {
    pub fn new(
        supabase_url: impl Into<String>,
        anon_key: impl Into<String>,
        service_role_key: Option<String>,
        bucket: impl Into<String>,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("reqwest::Client should build with default config");
        Self {
            supabase_url: supabase_url.into(),
            anon_key: anon_key.into(),
            service_role_key,
            bucket: bucket.into(),
            client,
        }
    }

    pub fn auth_key(&self) -> &str {
        self.service_role_key
            .as_deref()
            .unwrap_or(self.anon_key.as_str())
    }

    pub fn public_url(&self, path: &str) -> String {
        format!(
            "{}/storage/v1/object/public/{}/{}",
            self.supabase_url, self.bucket, path
        )
    }

    pub fn render_image_url(&self, path: &str, width: Option<u32>, height: Option<u32>) -> String {
        let base = format!(
            "{}/storage/v1/render/image/public/{}/{}",
            self.supabase_url, self.bucket, path
        );
        match (width, height) {
            (Some(w), Some(h)) => format!("{}?width={}&height={}", base, w, h),
            (Some(w), None) => format!("{}?width={}", base, w),
            (None, Some(h)) => format!("{}?height={}", base, h),
            (None, None) => base,
        }
    }

    pub async fn upload(
        &self,
        file_path: &str,
        data: &[u8],
        content_type: &str,
        cache_control: Option<&str>,
    ) -> Result<(), AppError> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            self.supabase_url, self.bucket, file_path
        );
        let mut part = reqwest::multipart::Part::bytes(data.to_vec())
            .file_name(file_path.to_string())
            .mime_str(content_type)
            .map_err(|e| {
                AppError::Logical(format!("Invalid content type '{}': {}", content_type, e))
            })?;
        if let Some(cc) = cache_control {
            part = part.headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::CACHE_CONTROL,
                    reqwest::header::HeaderValue::from_str(cc)
                        .map_err(|e| AppError::Logical(format!("Invalid cache-control: {}", e)))?,
                );
                headers
            });
        }
        let form = reqwest::multipart::Form::new().part("file", part);
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header("x-upsert", "true")
            .multipart(form)
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Upload request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::StorageError(format!(
                "Upload failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    pub async fn download(&self, path: &str) -> Result<(Vec<u8>, String), AppError> {
        let url = self.public_url(path);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Download request failed: {}", e)))?;
        let status = response.status();
        if status.as_u16() == 404 {
            return Err(AppError::NotFound);
        }
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::StorageError(format!(
                "Download failed ({}): {}",
                status, body
            )));
        }
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| guess_content_type(path));
        let bytes = response
            .bytes()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to read download body: {}", e)))?;
        Ok((bytes.to_vec(), content_type))
    }

    pub async fn get_info(&self, path: &str) -> Result<StorageObjectMetadata, AppError> {
        let url = format!(
            "{}/storage/v1/object/info/public/{}/{}",
            self.supabase_url, self.bucket, path
        );
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Get info request failed: {}", e)))?;
        let status = response.status();
        if status.as_u16() == 404 {
            return Err(AppError::NotFound);
        }
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::StorageError(format!(
                "Get info failed ({}): {}",
                status, body
            )));
        }
        let raw: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to parse info response: {}", e)))?;

        let content_type = raw
            .pointer("/http_metadata/contentType")
            .and_then(|v| v.as_str())
            .or_else(|| raw.get("mimetype").and_then(|v| v.as_str()))
            .map(|s| s.to_string())
            .unwrap_or_else(|| guess_content_type(path));

        let size = raw
            .get("size")
            .and_then(|v| {
                v.as_u64()
                    .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
            })
            .unwrap_or(0);

        let last_modified = raw
            .get("last_modified")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        Ok(StorageObjectMetadata {
            content_type,
            size,
            last_modified,
        })
    }

    pub async fn list_objects(&self, prefix: Option<&str>) -> Result<Vec<StorageObject>, AppError> {
        let url = format!(
            "{}/storage/v1/object/list/public/{}",
            self.supabase_url, self.bucket
        );
        let body = serde_json::json!({
            "prefix": prefix.unwrap_or(""),
            "limit": 1000,
            "offset": 0,
            "sortBy": { "column": "name", "order": "asc" }
        });
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("List request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::StorageError(format!(
                "List failed ({}): {}",
                status, body
            )));
        }
        let raw: Vec<serde_json::Value> = response
            .json()
            .await
            .map_err(|e| AppError::StorageError(format!("Failed to parse list response: {}", e)))?;

        let objects = raw
            .into_iter()
            .map(|item| {
                let name = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let content_type = item
                    .pointer("/metadata/mimetype")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| guess_content_type(&name));
                let size = item
                    .pointer("/metadata/size")
                    .and_then(|v| {
                        v.as_u64()
                            .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                    })
                    .unwrap_or(0);
                let last_modified = item
                    .get("updated_at")
                    .and_then(|v| v.as_str())
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc));
                StorageObject {
                    name,
                    content_type,
                    size,
                    last_modified,
                }
            })
            .collect();

        Ok(objects)
    }

    pub async fn delete(&self, path: &str) -> Result<(), AppError> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            self.supabase_url, self.bucket, path
        );
        let response = self
            .client
            .delete(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Delete request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::StorageError(format!(
                "Delete failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    pub async fn delete_batch(&self, paths: &[String]) -> Result<Vec<DeletedObject>, AppError> {
        let url = format!(
            "{}/storage/v1/object/{}/delete",
            self.supabase_url, self.bucket
        );
        let body = serde_json::json!({ "prefixes": paths });
        let response = self
            .client
            .delete(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Batch delete request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::StorageError(format!(
                "Batch delete failed ({}): {}",
                status, body
            )));
        }
        let raw: Vec<serde_json::Value> = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse batch delete response: {}", e))
        })?;
        let deleted = raw
            .into_iter()
            .map(|item| DeletedObject {
                name: item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string(),
            })
            .collect();
        Ok(deleted)
    }
}

fn guess_content_type(path: &str) -> String {
    let ext = path.split('.').last().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        "webp" => "image/webp".to_string(),
        "bmp" => "image/bmp".to_string(),
        "ico" => "image/x-icon".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "pdf" => "application/pdf".to_string(),
        "txt" => "text/plain".to_string(),
        "json" => "application/json".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        matchers::{header, method, path},
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
            bucket: "media".to_string(),
            client: Client::new(),
        }
    }

    #[test]
    fn auth_key_returns_service_role_when_set() {
        let s = make_storage("http://example.com", true);
        assert_eq!(s.auth_key(), "service-role-test-key");
    }

    #[test]
    fn auth_key_falls_back_to_anon_key() {
        let s = make_storage("http://example.com", false);
        assert_eq!(s.auth_key(), "anon-test-key");
    }

    #[test]
    fn public_url_builds_expected_path() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.public_url("foo/bar.png"),
            "http://localhost:8000/storage/v1/object/public/media/foo/bar.png"
        );
    }

    #[test]
    fn render_image_url_with_both_dimensions() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.render_image_url("foo.webp", Some(300), Some(200)),
            "http://localhost:8000/storage/v1/render/image/public/media/foo.webp?width=300&height=200"
        );
    }

    #[test]
    fn render_image_url_with_only_width() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.render_image_url("foo.webp", Some(300), None),
            "http://localhost:8000/storage/v1/render/image/public/media/foo.webp?width=300"
        );
    }

    #[test]
    fn render_image_url_with_only_height() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.render_image_url("foo.webp", None, Some(200)),
            "http://localhost:8000/storage/v1/render/image/public/media/foo.webp?height=200"
        );
    }

    #[test]
    fn render_image_url_with_no_dimensions() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.render_image_url("foo.webp", None, None),
            "http://localhost:8000/storage/v1/render/image/public/media/foo.webp"
        );
    }

    #[async_std::test]
    async fn upload_issues_multipart_post_with_bearer_and_upsert() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/media/some/path.png"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .and(header("x-upsert", "true"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage
            .upload("some/path.png", b"binary-data", "image/png", None)
            .await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[async_std::test]
    async fn upload_returns_storage_error_on_500() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/media/some/path.png"))
            .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
            .mount(&server)
            .await;

        let result = storage
            .upload("some/path.png", b"binary-data", "image/png", None)
            .await;
        match result {
            Err(AppError::StorageError(msg)) => assert!(msg.contains("500")),
            other => panic!("expected StorageError, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn download_returns_bytes_and_content_type_on_200() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/public/media/foo.png"))
            .and(header("authorization", "Bearer anon-test-key"))
            .and(header("apikey", "anon-test-key"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"hello-bytes".to_vec()),
            )
            .mount(&server)
            .await;

        let (bytes, content_type) = storage.download("foo.png").await.expect("download ok");
        assert_eq!(bytes, b"hello-bytes");
        assert_eq!(content_type, "image/png");
    }

    #[async_std::test]
    async fn download_returns_not_found_on_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/public/media/missing.png"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let result = storage.download("missing.png").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn download_returns_storage_error_on_500() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/public/media/oops.png"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal"))
            .mount(&server)
            .await;

        let result = storage.download("oops.png").await;
        match result {
            Err(AppError::StorageError(msg)) => assert!(msg.contains("500")),
            other => panic!("expected StorageError, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn get_info_parses_json_response() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        let body = json!({
            "name": "foo.png",
            "bucket_id": "media",
            "owner": "",
            "owner_id": "",
            "version": "1",
            "size": 1234,
            "mimetype": "image/png",
            "etag": "\"abc\"",
            "cache_control": "max-age=3600",
            "last_modified": "2026-01-01T00:00:00Z"
        });

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/info/public/media/foo.png"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let info = storage.get_info("foo.png").await.expect("get_info ok");
        assert_eq!(info.content_type, "image/png");
        assert_eq!(info.size, 1234);
        assert!(info.last_modified.is_some());
    }

    #[async_std::test]
    async fn get_info_returns_not_found_on_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/info/public/media/missing.png"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let result = storage.get_info("missing.png").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn list_objects_posts_with_correct_body_and_returns_list() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        let body = json!([
            {
                "name": "a.png",
                "id": "1",
                "updated_at": "2026-01-01T00:00:00Z",
                "created_at": "2026-01-01T00:00:00Z",
                "last_accessed_at": "2026-01-01T00:00:00Z",
                "metadata": {
                    "size": "100",
                    "mimetype": "image/png"
                }
            },
            {
                "name": "b.png",
                "id": "2",
                "updated_at": "2026-01-02T00:00:00Z",
                "created_at": "2026-01-02T00:00:00Z",
                "last_accessed_at": "2026-01-02T00:00:00Z",
                "metadata": {
                    "size": 250,
                    "mimetype": "image/jpeg"
                }
            }
        ]);

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/list/public/media"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let items = storage
            .list_objects(Some("images/"))
            .await
            .expect("list ok");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].name, "a.png");
        assert_eq!(items[0].content_type, "image/png");
        assert_eq!(items[0].size, 100);
        assert!(items[0].last_modified.is_some());
        assert_eq!(items[1].name, "b.png");
        assert_eq!(items[1].content_type, "image/jpeg");
        assert_eq!(items[1].size, 250);
    }

    #[async_std::test]
    async fn delete_issues_delete_to_correct_path() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/object/media/foo.png"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage.delete("foo.png").await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[async_std::test]
    async fn delete_batch_issues_delete_with_body() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        let body = json!([
            { "name": "a.png", "bucket_id": "media" },
            { "name": "b.png", "bucket_id": "media" }
        ]);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/object/media/delete"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let paths = vec!["a.png".to_string(), "b.png".to_string()];
        let result = storage.delete_batch(&paths).await.expect("batch delete ok");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "a.png");
        assert_eq!(result[1].name, "b.png");
    }
}
