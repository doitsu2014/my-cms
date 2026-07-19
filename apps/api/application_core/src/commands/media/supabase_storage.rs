use crate::commands::media::bucket::dto::Bucket;
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
            .finish()
    }
}

impl SupabaseStorage {
    pub fn new(
        supabase_url: impl Into<String>,
        anon_key: impl Into<String>,
        service_role_key: Option<String>,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("reqwest::Client should build with default config");
        Self {
            supabase_url: supabase_url.into(),
            anon_key: anon_key.into(),
            service_role_key,
            client,
        }
    }

    pub fn auth_key(&self) -> &str {
        self.service_role_key
            .as_deref()
            .unwrap_or(self.anon_key.as_str())
    }

    pub fn public_url(&self, bucket: &str, path: &str) -> String {
        format!(
            "{}/storage/v1/object/public/{}/{}",
            self.supabase_url, bucket, path
        )
    }

    // Image rendering requires the legacy `/public/` segment in Supabase
    // Storage v1.60.2; the single-segment pattern returns 404 here. Re-test
    // this endpoint when upgrading storage-api.
    pub fn render_image_url(
        &self,
        bucket: &str,
        path: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> String {
        let base = format!(
            "{}/storage/v1/render/image/public/{}/{}",
            self.supabase_url, bucket, path
        );
        match (width, height) {
            (Some(w), Some(h)) => format!("{}?width={}&height={}", base, w, h),
            (Some(w), None) => format!("{}?width={}", base, w),
            (None, Some(h)) => format!("{}?height={}", base, h),
            (None, None) => base,
        }
    }

    pub async fn download_render(
        &self,
        bucket: &str,
        path: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<(Vec<u8>, String), AppError> {
        let url = self.render_image_url(bucket, path, width, height);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Render request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            if is_bucket_not_found(status, &body) {
                return Err(AppError::NotFound);
            }
            return Err(AppError::StorageError(format!(
                "Render failed ({}): {}",
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
            .map_err(|e| AppError::StorageError(format!("Failed to read render body: {}", e)))?;
        Ok((bytes.to_vec(), content_type))
    }

    pub async fn upload(
        &self,
        bucket: &str,
        file_path: &str,
        data: &[u8],
        content_type: &str,
        cache_control: Option<&str>,
    ) -> Result<(), AppError> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            self.supabase_url, bucket, file_path
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
            if is_bucket_not_found(status, &body) {
                return Err(AppError::NotFound);
            }
            return Err(AppError::StorageError(format!(
                "Upload failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    pub async fn download(&self, bucket: &str, path: &str) -> Result<(Vec<u8>, String), AppError> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            self.supabase_url, bucket, path
        );
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Download request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            if is_bucket_not_found(status, &body) {
                return Err(AppError::NotFound);
            }
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

    pub async fn get_info(
        &self,
        bucket: &str,
        path: &str,
    ) -> Result<StorageObjectMetadata, AppError> {
        let url = format!(
            "{}/storage/v1/object/info/{}/{}",
            self.supabase_url, bucket, path
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
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            if is_bucket_not_found(status, &body) {
                return Err(AppError::NotFound);
            }
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

    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<StorageObject>, AppError> {
        let url = format!("{}/storage/v1/object/list/{}", self.supabase_url, bucket);
        let req_body = serde_json::json!({
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
            .body(req_body.to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("List request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let resp_body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            if is_bucket_not_found(status, &resp_body) {
                return Err(AppError::NotFound);
            }
            return Err(AppError::StorageError(format!(
                "List failed ({}): {}",
                status, resp_body
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

    pub async fn delete(&self, bucket: &str, path: &str) -> Result<(), AppError> {
        let url = format!(
            "{}/storage/v1/object/{}/{}",
            self.supabase_url, bucket, path
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
            if is_bucket_not_found(status, &body) {
                return Err(AppError::NotFound);
            }
            return Err(AppError::StorageError(format!(
                "Delete failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    pub async fn delete_batch(
        &self,
        bucket: &str,
        paths: &[String],
    ) -> Result<Vec<DeletedObject>, AppError> {
        let url = format!("{}/storage/v1/object/{}/delete", self.supabase_url, bucket);
        let req_body = serde_json::json!({ "prefixes": paths });
        let response = self
            .client
            .delete(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(req_body.to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Batch delete request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let resp_body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            if is_bucket_not_found(status, &resp_body) {
                return Err(AppError::NotFound);
            }
            return Err(AppError::StorageError(format!(
                "Batch delete failed ({}): {}",
                status, resp_body
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

    pub async fn list_buckets(&self) -> Result<Vec<Bucket>, AppError> {
        let url = format!("{}/storage/v1/bucket", self.supabase_url);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("List buckets request failed: {}", e)))?;
        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::StorageError(format!(
                "List buckets failed ({}): {}",
                status, body
            )));
        }
        let raw: Vec<serde_json::Value> = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse list buckets response: {}", e))
        })?;
        Ok(raw.into_iter().map(bucket_from_value).collect())
    }

    pub async fn get_bucket(&self, name: &str) -> Result<Bucket, AppError> {
        let url = format!("{}/storage/v1/bucket/{}", self.supabase_url, name);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Get bucket request failed: {}", e)))?;
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
                "Get bucket failed ({}): {}",
                status, body
            )));
        }
        let raw: serde_json::Value = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse get bucket response: {}", e))
        })?;
        Ok(bucket_from_value(raw))
    }

    pub async fn create_bucket(&self, payload: serde_json::Value) -> Result<Bucket, AppError> {
        let url = format!("{}/storage/v1/bucket", self.supabase_url);
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload.to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Create bucket request failed: {}", e)))?;
        let status = response.status();
        if status.as_u16() == 409 {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::Conflict(format!(
                "Bucket already exists: {}",
                body
            )));
        }
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "<no body>".to_string());
            return Err(AppError::StorageError(format!(
                "Create bucket failed ({}): {}",
                status, body
            )));
        }
        let raw: serde_json::Value = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse create bucket response: {}", e))
        })?;
        Ok(bucket_from_value(raw))
    }

    pub async fn update_bucket(
        &self,
        name: &str,
        payload: serde_json::Value,
    ) -> Result<Bucket, AppError> {
        let url = format!("{}/storage/v1/bucket/{}", self.supabase_url, name);
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload.to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Update bucket request failed: {}", e)))?;
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
                "Update bucket failed ({}): {}",
                status, body
            )));
        }
        let raw: serde_json::Value = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse update bucket response: {}", e))
        })?;
        Ok(bucket_from_value(raw))
    }

    pub async fn empty_bucket(&self, name: &str) -> Result<(), AppError> {
        let url = format!("{}/storage/v1/bucket/{}/empty", self.supabase_url, name);
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Empty bucket request failed: {}", e)))?;
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
                "Empty bucket failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }

    pub async fn delete_bucket(&self, name: &str, purge: bool) -> Result<(), AppError> {
        let url = format!("{}/storage/v1/bucket/{}", self.supabase_url, name);
        let body = serde_json::json!({ "purge": purge });
        let response = self
            .client
            .delete(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Delete bucket request failed: {}", e)))?;
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
                "Delete bucket failed ({}): {}",
                status, body
            )));
        }
        Ok(())
    }
}

fn is_bucket_not_found(status: reqwest::StatusCode, body: &str) -> bool {
    if status.as_u16() == 404 {
        return true;
    }

    let Ok(value) = serde_json::from_str::<serde_json::Value>(body) else {
        return false;
    };
    let status_code = value.get("statusCode");
    let has_not_found_status = status_code.and_then(serde_json::Value::as_u64) == Some(404)
        || status_code.and_then(serde_json::Value::as_str) == Some("404");
    if has_not_found_status {
        return true;
    }

    const BUCKET_NOT_FOUND_PATTERNS: &[&str] = &[
        "does not exist",
        "bucket missing",
        "bucket not found",
        "bucketnotfound",
        "notfound",
        "bucket_not_found",
    ];

    ["error", "message"]
        .iter()
        .filter_map(|field| value.get(*field).and_then(serde_json::Value::as_str))
        .any(|text| {
            let normalized = text.to_lowercase();
            BUCKET_NOT_FOUND_PATTERNS
                .iter()
                .any(|p| normalized.contains(p))
        })
}

fn bucket_from_value(v: serde_json::Value) -> Bucket {
    Bucket {
        id: v
            .get("id")
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string(),
        name: v
            .get("name")
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string(),
        public: v.get("public").and_then(|x| x.as_bool()).unwrap_or(false),
        file_size_limit: v.get("file_size_limit").and_then(|x| x.as_u64()),
        allowed_mime_types: v.get("allowed_mime_types").and_then(|x| {
            x.as_array().map(|arr| {
                arr.iter()
                    .filter_map(|i| i.as_str().map(String::from))
                    .collect()
            })
        }),
        owner: v.get("owner").and_then(|x| x.as_str()).map(String::from),
        bucket_type: v
            .get("type")
            .and_then(|x| x.as_str())
            .unwrap_or("STANDARD")
            .to_string(),
        created_at: v
            .get("created_at")
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string(),
        updated_at: v
            .get("updated_at")
            .and_then(|x| x.as_str())
            .unwrap_or_default()
            .to_string(),
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

    const BUCKET_NOT_FOUND_BODY: &str =
        r#"{"statusCode":"404","error":"Bucket not found","message":"Bucket not found"}"#;

    fn status(s: u16) -> reqwest::StatusCode {
        reqwest::StatusCode::from_u16(s).unwrap()
    }

    #[test]
    fn is_bucket_not_found_matches_does_not_exist_with_statuscode_404() {
        let body = r#"{"statusCode":"404","error":"not_found","message":"Bucket does not exist"}"#;
        assert!(is_bucket_not_found(status(400), body));
    }

    #[test]
    fn is_bucket_not_found_matches_classic_bucket_not_found_with_statuscode_404() {
        let body = r#"{"statusCode":"404","error":"not_found","message":"The specified bucket was not found"}"#;
        assert!(is_bucket_not_found(status(400), body));
    }

    #[test]
    fn is_bucket_not_found_matches_bucket_missing_and_bucket_not_found_codes() {
        let body = r#"{"statusCode":"404","error":"bucket_not_found","message":"bucket missing"}"#;
        assert!(is_bucket_not_found(status(400), body));
    }

    #[test]
    fn is_bucket_not_found_matches_bucket_missing_alone() {
        let body = r#"{"statusCode":"404","message":"bucket missing"}"#;
        assert!(is_bucket_not_found(status(400), body));
    }

    #[test]
    fn is_bucket_not_found_rejects_non_404_status_codes_with_unrelated_messages() {
        let body = r#"{"statusCode":"500","message":"server error"}"#;
        assert!(!is_bucket_not_found(status(400), body));
    }

    #[test]
    fn is_bucket_not_found_rejects_non_json_body() {
        assert!(!is_bucket_not_found(
            status(400),
            "<html><body>not json</body></html>"
        ));
    }

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
            s.public_url("media", "foo/bar.png"),
            "http://localhost:8000/storage/v1/object/public/media/foo/bar.png"
        );
    }

    #[test]
    fn render_image_url_with_both_dimensions() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.render_image_url("media", "foo.webp", Some(300), Some(200)),
            "http://localhost:8000/storage/v1/render/image/public/media/foo.webp?width=300&height=200"
        );
    }

    #[test]
    fn render_image_url_with_only_width() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.render_image_url("media", "foo.webp", Some(300), None),
            "http://localhost:8000/storage/v1/render/image/public/media/foo.webp?width=300"
        );
    }

    #[test]
    fn render_image_url_with_only_height() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.render_image_url("media", "foo.webp", None, Some(200)),
            "http://localhost:8000/storage/v1/render/image/public/media/foo.webp?height=200"
        );
    }

    #[test]
    fn render_image_url_with_no_dimensions() {
        let s = make_storage("http://localhost:8000", false);
        assert_eq!(
            s.render_image_url("media", "foo.webp", None, None),
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
            .upload("media", "some/path.png", b"binary-data", "image/png", None)
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
            .upload("media", "some/path.png", b"binary-data", "image/png", None)
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
            .and(path("/storage/v1/object/media/foo.png"))
            .and(header("authorization", "Bearer anon-test-key"))
            .and(header("apikey", "anon-test-key"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"hello-bytes".to_vec()),
            )
            .mount(&server)
            .await;

        let (bytes, content_type) = storage
            .download("media", "foo.png")
            .await
            .expect("download ok");
        assert_eq!(bytes, b"hello-bytes");
        assert_eq!(content_type, "image/png");
    }

    #[async_std::test]
    async fn download_returns_not_found_on_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/media/missing.png"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let result = storage.download("media", "missing.png").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn download_returns_not_found_on_supabase_400_with_statuscode_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/xx/foo.png"))
            .respond_with(ResponseTemplate::new(400).set_body_string(BUCKET_NOT_FOUND_BODY))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage.download("xx", "foo.png").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn download_returns_storage_error_on_500() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/media/oops.png"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal"))
            .mount(&server)
            .await;

        let result = storage.download("media", "oops.png").await;
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
            .and(path("/storage/v1/object/info/media/foo.png"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let info = storage
            .get_info("media", "foo.png")
            .await
            .expect("get_info ok");
        assert_eq!(info.content_type, "image/png");
        assert_eq!(info.size, 1234);
        assert!(info.last_modified.is_some());
    }

    #[async_std::test]
    async fn get_info_returns_not_found_on_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/info/media/missing.png"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let result = storage.get_info("media", "missing.png").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn get_info_returns_not_found_on_supabase_400_with_statuscode_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/info/xx/foo.png"))
            .respond_with(ResponseTemplate::new(400).set_body_string(BUCKET_NOT_FOUND_BODY))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage.get_info("xx", "foo.png").await;
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
            .and(path("/storage/v1/object/list/media"))
            .and(header("content-type", "application/json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let items = storage
            .list_objects("media", Some("images/"))
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

        let result = storage.delete("media", "foo.png").await;
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
        let result = storage
            .delete_batch("media", &paths)
            .await
            .expect("batch delete ok");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].name, "a.png");
        assert_eq!(result[1].name, "b.png");
    }

    #[async_std::test]
    async fn list_objects_returns_not_found_on_supabase_400_with_statuscode_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/list/xx"))
            .respond_with(ResponseTemplate::new(400).set_body_string(BUCKET_NOT_FOUND_BODY))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage.list_objects("xx", None).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn list_objects_returns_not_found_on_does_not_exist_body() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        let body = r#"{"statusCode":"404","error":"not_found","message":"Bucket does not exist"}"#;
        Mock::given(method("POST"))
            .and(path("/storage/v1/object/list/xx"))
            .respond_with(ResponseTemplate::new(400).set_body_string(body))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage.list_objects("xx", None).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn list_objects_returns_not_found_on_http_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/list/xx"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage.list_objects("xx", None).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn url_patterns_match_spec_single_segment() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/object/contract-bucket/foo.png"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"png-bytes".to_vec()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let info_body = json!({
            "name": "foo.png",
            "size": 8,
            "mimetype": "image/png",
            "last_modified": "2026-01-01T00:00:00Z",
            "http_metadata": { "contentType": "image/png" }
        });
        Mock::given(method("GET"))
            .and(path("/storage/v1/object/info/contract-bucket/foo.png"))
            .respond_with(ResponseTemplate::new(200).set_body_json(info_body))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/list/contract-bucket"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
            .expect(1)
            .mount(&server)
            .await;

        let (bytes, content_type) = storage
            .download("contract-bucket", "foo.png")
            .await
            .expect("download should hit single-segment /object/{bucket}/{path}");
        assert_eq!(bytes, b"png-bytes");
        assert_eq!(content_type, "image/png");

        let info = storage
            .get_info("contract-bucket", "foo.png")
            .await
            .expect("get_info should hit single-segment /object/info/{bucket}/{path}");
        assert_eq!(info.content_type, "image/png");
        assert_eq!(info.size, 8);

        let listed = storage
            .list_objects("contract-bucket", None)
            .await
            .expect("list_objects should hit single-segment /object/list/{bucket}");
        assert!(listed.is_empty());

        assert_eq!(
            storage.public_url("contract-bucket", "foo.png"),
            format!(
                "{}/storage/v1/object/public/contract-bucket/foo.png",
                server.uri()
            )
        );
        assert_eq!(
            storage.render_image_url("contract-bucket", "foo.png", Some(300), None),
            format!(
                "{}/storage/v1/render/image/public/contract-bucket/foo.png?width=300",
                server.uri()
            )
        );
    }

    #[async_std::test]
    async fn upload_returns_not_found_on_supabase_400_with_statuscode_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/xx/foo.png"))
            .respond_with(ResponseTemplate::new(400).set_body_string(BUCKET_NOT_FOUND_BODY))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage
            .upload("xx", "foo.png", b"binary-data", "image/png", None)
            .await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn upload_returns_not_found_on_http_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/xx/foo.png"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage
            .upload("xx", "foo.png", b"binary-data", "image/png", None)
            .await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn delete_returns_not_found_on_supabase_400_with_statuscode_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/object/xx/foo.png"))
            .respond_with(ResponseTemplate::new(400).set_body_string(BUCKET_NOT_FOUND_BODY))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage.delete("xx", "foo.png").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn delete_returns_not_found_on_http_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/object/xx/foo.png"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage.delete("xx", "foo.png").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn delete_batch_returns_not_found_on_supabase_400_with_statuscode_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/object/xx/delete"))
            .respond_with(ResponseTemplate::new(400).set_body_string(BUCKET_NOT_FOUND_BODY))
            .expect(1)
            .mount(&server)
            .await;

        let paths = vec!["foo.png".to_string()];
        let result = storage.delete_batch("xx", &paths).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn delete_batch_returns_not_found_on_http_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/object/xx/delete"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;

        let paths = vec!["foo.png".to_string()];
        let result = storage.delete_batch("xx", &paths).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn same_storage_targets_two_buckets_via_method_argument() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/media/foo.png"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/storage/v1/object/avatars/foo.png"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .expect(1)
            .mount(&server)
            .await;

        storage
            .upload("media", "foo.png", b"bytes", "image/png", None)
            .await
            .expect("first upload ok");
        storage
            .upload("avatars", "foo.png", b"bytes", "image/png", None)
            .await
            .expect("second upload ok");
    }

    #[async_std::test]
    async fn list_buckets_returns_array() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        let body = json!([
            {
                "id": "media",
                "name": "media",
                "public": true,
                "file_size_limit": null,
                "allowed_mime_types": null,
                "owner": null,
                "type": "STANDARD",
                "created_at": "2026-01-01T00:00:00Z",
                "updated_at": "2026-01-02T00:00:00Z"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;

        let buckets = storage.list_buckets().await.expect("list_buckets ok");
        assert_eq!(buckets.len(), 1);
        assert_eq!(buckets[0].name, "media");
        assert!(buckets[0].public);
        assert_eq!(buckets[0].bucket_type, "STANDARD");
        assert_eq!(buckets[0].created_at, "2026-01-01T00:00:00Z");
    }

    #[async_std::test]
    async fn get_bucket_returns_404_as_not_found() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket/missing"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let result = storage.get_bucket("missing").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn create_bucket_posts_body() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        let response_body = json!({
            "id": "private-docs",
            "name": "private-docs",
            "public": false,
            "file_size_limit": 5242880u64,
            "allowed_mime_types": ["application/pdf"],
            "owner": null,
            "type": "STANDARD",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z"
        });

        Mock::given(method("POST"))
            .and(path("/storage/v1/bucket"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .and(header("content-type", "application/json"))
            .and(wiremock::matchers::body_string(
                "{\"allowed_mime_types\":[\"application/pdf\"],\"file_size_limit\":5242880,\"name\":\"private-docs\",\"public\":false}".to_string(),
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&server)
            .await;

        let payload = json!({
            "name": "private-docs",
            "public": false,
            "file_size_limit": 5242880u64,
            "allowed_mime_types": ["application/pdf"],
        });
        let bucket = storage
            .create_bucket(payload)
            .await
            .expect("create_bucket ok");
        assert_eq!(bucket.name, "private-docs");
        assert!(!bucket.public);
        assert_eq!(bucket.file_size_limit, Some(5242880));
    }

    #[async_std::test]
    async fn create_bucket_returns_conflict_on_409() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("POST"))
            .and(path("/storage/v1/bucket"))
            .respond_with(ResponseTemplate::new(409).set_body_string("duplicate"))
            .mount(&server)
            .await;

        let payload = json!({ "name": "private-docs", "public": false });
        let result = storage.create_bucket(payload).await;
        assert!(matches!(result, Err(AppError::Conflict(_))));
    }

    #[async_std::test]
    async fn update_bucket_posts_to_path() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        let response_body = json!({
            "id": "private-docs",
            "name": "private-docs",
            "public": true,
            "file_size_limit": null,
            "allowed_mime_types": null,
            "owner": null,
            "type": "STANDARD",
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-02T00:00:00Z"
        });

        Mock::given(method("POST"))
            .and(path("/storage/v1/bucket/private-docs"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .and(wiremock::matchers::body_string("{\"public\":true}"))
            .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
            .mount(&server)
            .await;

        let payload = json!({ "public": true });
        let bucket = storage
            .update_bucket("private-docs", payload)
            .await
            .expect("update_bucket ok");
        assert_eq!(bucket.name, "private-docs");
        assert!(bucket.public);
    }

    #[async_std::test]
    async fn empty_bucket_posts_to_empty_path() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("POST"))
            .and(path("/storage/v1/bucket/private-docs/empty"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let result = storage.empty_bucket("private-docs").await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[async_std::test]
    async fn empty_bucket_returns_not_found_on_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("POST"))
            .and(path("/storage/v1/bucket/missing/empty"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let result = storage.empty_bucket("missing").await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn delete_bucket_purge_true_sends_purge_in_body() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/bucket/old-test"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .and(wiremock::matchers::body_string("{\"purge\":true}"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let result = storage.delete_bucket("old-test", true).await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[async_std::test]
    async fn delete_bucket_purge_false_sends_purge_false_in_body() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/bucket/old-test"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(wiremock::matchers::body_string("{\"purge\":false}"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let result = storage.delete_bucket("old-test", false).await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[async_std::test]
    async fn delete_bucket_returns_not_found_on_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/bucket/missing"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let result = storage.delete_bucket("missing", true).await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn delete_bucket_returns_storage_error_on_400() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("DELETE"))
            .and(path("/storage/v1/bucket/non-empty"))
            .respond_with(ResponseTemplate::new(400).set_body_string("not empty"))
            .mount(&server)
            .await;

        let result = storage.delete_bucket("non-empty", false).await;
        match result {
            Err(AppError::StorageError(msg)) => {
                assert!(msg.contains("400"));
                assert!(msg.contains("not empty"));
            }
            other => panic!("expected StorageError, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn list_buckets_returns_storage_error_on_500() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket"))
            .respond_with(ResponseTemplate::new(500).set_body_string("oops"))
            .mount(&server)
            .await;

        let result = storage.list_buckets().await;
        match result {
            Err(AppError::StorageError(msg)) => assert!(msg.contains("500")),
            other => panic!("expected StorageError, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn list_buckets_returns_storage_error_on_401() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket"))
            .respond_with(ResponseTemplate::new(401).set_body_string("unauth"))
            .mount(&server)
            .await;

        let result = storage.list_buckets().await;
        match result {
            Err(AppError::StorageError(msg)) => assert!(msg.contains("401")),
            other => panic!("expected StorageError, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn error_messages_never_include_service_role_key() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/bucket"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal"))
            .mount(&server)
            .await;

        let result = storage.list_buckets().await;
        match result {
            Err(AppError::StorageError(msg)) => {
                assert!(!msg.contains("service-role-test-key"));
                assert!(!msg.contains("service_role_test_key"));
            }
            other => panic!("expected StorageError, got {:?}", other),
        }
    }

    #[async_std::test]
    async fn download_render_returns_bytes_and_content_type_on_200_with_both_dimensions() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/foo.png"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .and(wiremock::matchers::query_param("width", "300"))
            .and(wiremock::matchers::query_param("height", "200"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/webp")
                    .set_body_bytes(b"rendered-bytes".to_vec()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let (bytes, content_type) = storage
            .download_render("media", "foo.png", Some(300), Some(200))
            .await
            .expect("download_render ok");
        assert_eq!(bytes, b"rendered-bytes");
        assert_eq!(content_type, "image/webp");
    }

    #[async_std::test]
    async fn download_render_with_only_width_sends_only_width_query() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/foo.png"))
            .and(wiremock::matchers::query_param("width", "300"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"resized".to_vec()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let (bytes, _) = storage
            .download_render("media", "foo.png", Some(300), None)
            .await
            .expect("download_render ok");
        assert_eq!(bytes, b"resized");
    }

    #[async_std::test]
    async fn download_render_with_only_height_sends_only_height_query() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/foo.png"))
            .and(wiremock::matchers::query_param("height", "200"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"resized".to_vec()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let (bytes, _) = storage
            .download_render("media", "foo.png", None, Some(200))
            .await
            .expect("download_render ok");
        assert_eq!(bytes, b"resized");
    }

    #[async_std::test]
    async fn download_render_falls_back_to_anonymous_key_when_service_role_absent() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), false);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/foo.png"))
            .and(header("authorization", "Bearer anon-test-key"))
            .and(header("apikey", "anon-test-key"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"ok".to_vec()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let result = storage
            .download_render("media", "foo.png", Some(300), None)
            .await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[async_std::test]
    async fn download_render_uses_explicit_bucket_argument() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/avatars/foo.png"))
            .and(wiremock::matchers::query_param("width", "300"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "image/png")
                    .set_body_bytes(b"avatars-bytes".to_vec()),
            )
            .expect(1)
            .mount(&server)
            .await;

        let (bytes, _) = storage
            .download_render("avatars", "foo.png", Some(300), None)
            .await
            .expect("download_render ok");
        assert_eq!(bytes, b"avatars-bytes");
    }

    #[async_std::test]
    async fn download_render_guesses_content_type_when_response_omits_it() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/foo.png"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"ok".to_vec()))
            .expect(1)
            .mount(&server)
            .await;

        let (_, content_type) = storage
            .download_render("media", "foo.png", Some(300), None)
            .await
            .expect("download_render ok");
        assert_eq!(content_type, "image/png");
    }

    #[async_std::test]
    async fn download_render_returns_not_found_on_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/missing.png"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let result = storage
            .download_render("media", "missing.png", Some(300), None)
            .await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn download_render_returns_not_found_on_supabase_400_with_statuscode_404() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/xx/foo.png"))
            .respond_with(ResponseTemplate::new(400).set_body_string(BUCKET_NOT_FOUND_BODY))
            .expect(1)
            .mount(&server)
            .await;

        let result = storage
            .download_render("xx", "foo.png", Some(300), None)
            .await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn download_render_returns_storage_error_on_500() {
        let server = MockServer::start().await;
        let storage = make_storage(&server.uri(), true);

        Mock::given(method("GET"))
            .and(path("/storage/v1/render/image/public/media/oops.png"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal"))
            .mount(&server)
            .await;

        let result = storage
            .download_render("media", "oops.png", Some(300), None)
            .await;
        match result {
            Err(AppError::StorageError(msg)) => assert!(msg.contains("500")),
            other => panic!("expected StorageError, got {:?}", other),
        }
    }
}
