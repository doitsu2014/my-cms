# Image + Storage Migration Plan

> **For agentic workers:** These are two independent sub-plans that can be executed in parallel. Use superpowers:subagent-driven-development.
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace S3 (Contabo) media storage with Supabase Storage, and replace on-the-fly image resizing with Supabase Image Transformation.

**Architecture:** The MediaCommand handlers switch from `rust-s3` Bucket operations to Supabase Storage REST API calls via `reqwest` (already a dependency). Image delivery URLs are replaced with Supabase Storage render URLs for image transformation. The `S3MediaStorage` struct becomes `SupabaseStorage`. The `S3Error` error variant is replaced with a `StorageError`.

**Tech Stack:** Rust (reqwest, serde, moka cache), Supabase Storage REST API, Supabase Image Transformation (wraps imgproxy internally).

**Files to Create:**
- `services/application_core/src/commands/media/supabase_storage.rs` — new SupabaseStorage client using reqwest

**Files to Modify:**
- `services/application_core/src/commands/media/mod.rs` — replace S3MediaStorage with SupabaseStorage, update response models
- `services/application_core/src/commands/media/create/create_handler.rs` — upload via Supabase Storage API
- `services/application_core/src/commands/media/read/read_handler.rs` — fetch via Supabase Storage URL, remove resize in favor of render URLs
- `services/application_core/src/commands/media/read/metadata_handler.rs` — metadata via Supabase API
- `services/application_core/src/commands/media/list/list_handler.rs` — list via Supabase API
- `services/application_core/src/commands/media/delete/delete_handler.rs` — delete via Supabase API
- `services/application_core/src/commands/media/read/read_response.rs` — update response types
- `services/application_core/Cargo.toml` — remove rust-s3, image (resize lib)
- `services/Cargo.toml` — remove rust-s3, image from workspace
- `services/src/bin/my-cms-api.rs` — update AppState and config construction
- `services/src/lib.rs` — update AppState if needed
- `services/.env` — update env vars from S3 to Supabase Storage
- `services/application_core/src/common/app_error.rs` — replace S3Error with StorageError
- `services/src/presentation_models/api_response.rs` — update error mapping

**Files to Delete (eventually):**
- None reused by the curl/reqwest approach, but `rust-s3` and `image` crate deps are removed

---

## Part C: S3 → Supabase Storage Migration

### Task C1: Create SupabaseStorage Client

**Files:**
- Create: `services/application_core/src/commands/media/supabase_storage.rs`
- Modify: `services/application_core/src/commands/media/mod.rs`

- [ ] **Step 1: Create SupabaseStorage**

```rust
// services/application_core/src/commands/media/supabase_storage.rs

use reqwest::{multipart, Client, StatusCode};
use serde::Deserialize;
use std::time::Duration;

use crate::common::app_error::AppError;

#[derive(Clone)]
pub struct SupabaseStorage {
    pub supabase_url: String,
    pub anon_key: String,
    pub service_role_key: Option<String>,
    pub bucket_name: String,
    pub client: Client,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageObject {
    pub name: String,
    pub id: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "last_accessed_at")]
    pub last_accessed_at: Option<String>,
    pub metadata: Option<StorageObjectMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageObjectMetadata {
    #[serde(rename = "size")]
    pub size: Option<u64>,
    #[serde(rename = "mimetype")]
    pub mimetype: Option<String>,
    #[serde(rename = "cacheControl")]
    pub cache_control: Option<String>,
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,
    #[serde(rename = "contentLength")]
    pub content_length: Option<u64>,
    #[serde(rename = "httpStatusCode")]
    pub http_status_code: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
struct StorageErrorResponse {
    message: String,
    error: Option<String>,
    status_code: Option<String>,
}

impl SupabaseStorage {
    pub fn new(
        supabase_url: String,
        anon_key: String,
        service_role_key: Option<String>,
        bucket_name: String,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            supabase_url: supabase_url.trim_end_matches('/').to_string(),
            anon_key,
            service_role_key,
            bucket_name,
            client,
        }
    }

    /// Returns the API key to use. Prefers service_role_key for server-side operations.
    fn auth_key(&self) -> &str {
        self.service_role_key.as_deref().unwrap_or(&self.anon_key)
    }

    fn storage_base_url(&self) -> String {
        format!("{}/storage/v1", self.supabase_url)
    }

    fn object_url(&self, path: &str) -> String {
        format!(
            "{}/object/public/{}/{}",
            self.storage_base_url(),
            self.bucket_name,
            path
        )
    }

    /// Generate a public URL for a stored file.
    pub fn public_url(&self, path: &str) -> String {
        self.object_url(path)
    }

    /// Generate an image transformation URL via Supabase Image Transformation.
    pub fn render_image_url(
        &self,
        path: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> String {
        let mut url = format!(
            "{}/render/image/public/{}/{}",
            self.storage_base_url(),
            self.bucket_name,
            path
        );

        let mut params = Vec::new();
        if let Some(w) = width {
            params.push(format!("width={}", w));
        }
        if let Some(h) = height {
            params.push(format!("height={}", h));
        }
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }

    /// Upload a file to Supabase Storage.
    /// Returns the object path (key) in the bucket.
    pub async fn upload(
        &self,
        file_path: &str,
        data: Vec<u8>,
        content_type: &str,
        cache_control: Option<&str>,
    ) -> Result<(), AppError> {
        let url = format!(
            "{}/object/{}/{}",
            self.storage_base_url(),
            self.bucket_name,
            file_path
        );

        let part = multipart::Part::bytes(data)
            .file_name(file_path.to_string())
            .mime_str(content_type)
            .map_err(|e| AppError::StorageError(format!("Invalid content type: {}", e)))?;

        let form = multipart::Form::new().part("file", part);

        let mut request = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.auth_key()))
            .header("x-upsert", "true");

        if let Some(cc) = cache_control {
            request = request.header("Cache-Control", cc);
        }

        let response = request.multipart(form).send().await.map_err(|e| {
            AppError::StorageError(format!("Upload request failed: {}", e))
        })?;

        if !response.status().is_success() {
            let error_body = response.text().await.unwrap_or_default();
            return Err(AppError::StorageError(format!(
                "Upload failed ({}): {}",
                response.status(),
                error_body
            )));
        }

        Ok(())
    }

    /// Download a file from Supabase Storage.
    pub async fn download(&self, path: &str) -> Result<(Vec<u8>, String), AppError> {
        let url = self.object_url(path);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Download request failed: {}", e)))?;

        match response.status() {
            StatusCode::OK => {
                let content_type = response
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("application/octet-stream")
                    .to_string();
                let bytes = response
                    .bytes()
                    .await
                    .map_err(|e| AppError::StorageError(format!("Read body failed: {}", e)))?;
                Ok((bytes.to_vec(), content_type))
            }
            StatusCode::NOT_FOUND => Err(AppError::NotFound),
            status => Err(AppError::StorageError(format!(
                "Download failed: HTTP {}",
                status
            ))),
        }
    }

    /// Get object metadata (HEAD request equivalent).
    pub async fn get_info(
        &self,
        path: &str,
    ) -> Result<StorageObjectMetadata, AppError> {
        let url = format!(
            "{}/object/info/public/{}/{}",
            self.storage_base_url(),
            self.bucket_name,
            path
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.auth_key()))
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Info request failed: {}", e)))?;

        if response.status() == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound);
        }

        if !response.status().is_success() {
            return Err(AppError::StorageError(format!(
                "Info request failed: HTTP {}",
                response.status()
            )));
        }

        response.json::<StorageObjectMetadata>().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse info response: {}", e))
        })
    }

    /// List objects in the bucket with optional prefix filter.
    pub async fn list_objects(
        &self,
        prefix: Option<&str>,
    ) -> Result<Vec<StorageObject>, AppError> {
        let url = format!(
            "{}/object/list/public/{}",
            self.storage_base_url(),
            self.bucket_name
        );

        let mut request = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.auth_key()))
            .json(&serde_json::json!({
                "prefix": prefix.unwrap_or(""),
                "limit": 1000,
                "offset": 0,
                "sortBy": { "column": "name", "order": "asc" }
            }));

        let response = request.send().await.map_err(|e| {
            AppError::StorageError(format!("List request failed: {}", e))
        })?;

        if !response.status().is_success() {
            return Err(AppError::StorageError(format!(
                "List failed: HTTP {}",
                response.status()
            )));
        }

        response.json::<Vec<StorageObject>>().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse list response: {}", e))
        })
    }

    /// Delete a single object.
    pub async fn delete(&self, path: &str) -> Result<(), AppError> {
        let url = format!(
            "{}/object/{}/{}",
            self.storage_base_url(),
            self.bucket_name,
            path
        );

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.auth_key()))
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Delete request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::StorageError(format!(
                "Delete failed: HTTP {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Delete multiple objects.
    pub async fn delete_batch(&self, paths: &[String]) -> Result<usize, AppError> {
        let url = format!(
            "{}/object/{}/{}",
            self.storage_base_url(),
            self.bucket_name,
            "delete"
        );

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.auth_key()))
            .json(&serde_json::json!({ "prefixes": paths }))
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("Delete batch request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::StorageError(format!(
                "Delete batch failed: HTTP {}",
                response.status()
            )));
        }

        // Supabase batch delete returns an array of deleted objects
        #[derive(Deserialize)]
        struct DeletedObject {
            name: String,
        }

        let deleted: Vec<DeletedObject> = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse delete response: {}", e))
        })?;

        Ok(deleted.len())
    }
}
```

- [ ] **Step 2: Update module declaration**

```rust
// In services/application_core/src/commands/media/mod.rs, add:
pub mod supabase_storage;
```

- [ ] **Step 3: Commit**

```bash
git add services/application_core/src/commands/media/supabase_storage.rs services/application_core/src/commands/media/mod.rs
git commit -m "feat: add SupabaseStorage client using REST API"
```

---

### Task C2: Update AppError with StorageError

**Files:**
- Modify: `services/application_core/src/common/app_error.rs`
- Modify: `services/src/presentation_models/api_response.rs`

- [ ] **Step 1: Add StorageError variant**

```rust
// In services/application_core/src/common/app_error.rs

#[derive(Debug)]
pub enum AppError {
    Db(DbErr),
    DbTx(TransactionError<DbErr>),
    S3Error(S3Error),          // KEEP for backwards compat during transition
    StorageError(String),        // ADD THIS for Supabase Storage errors
    Validation(String, String),
    Logical(String),
    ConcurrencyOptimistic(String),
    NotFound,
    Unknown,
    OpenAIError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // ... existing arms ...
            Self::StorageError(msg) => write!(f, "Storage error: {}", msg),
            // ... rest ...
        }
    }
}

impl std::error::Error for AppError {}
```

- [ ] **Step 2: Update API error mapping**

```rust
// In services/src/presentation_models/api_response.rs
// Add mapping for StorageError alongside S3Error:

// In the From<AppError> for ApiResponseError implementation:
AppError::StorageError(msg) => Self {
    code: ErrorCode::ConnectionError,  // reuse existing code
    message: msg,
},
```

- [ ] **Step 3: Commit**

```bash
git add services/application_core/src/common/app_error.rs services/src/presentation_models/api_response.rs
git commit -m "feat: add StorageError for Supabase storage operations"
```

---

### Task C3: Update MediaConfig and AppState

**Files:**
- Modify: `services/application_core/src/commands/media/mod.rs`
- Modify: `services/src/bin/my-cms-api.rs`
- Modify: `services/src/lib.rs`
- Modify: `services/.env`

- [ ] **Step 1: Redefine MediaConfig**

```rust
// In services/application_core/src/commands/media/mod.rs
// Replace S3MediaStorage with SupabaseStorage:

use crate::commands::media::supabase_storage::SupabaseStorage;

#[derive(Clone)]
pub struct MediaConfig {
    pub storage: SupabaseStorage,
    pub media_base_url: String,
}

pub struct MediaModel {
    pub path: String,
    pub url: String,
}

pub struct MediaMetadata {
    pub path: String,
    pub url: String,
    pub content_type: String,
    pub size: u64,
    pub last_modified: Option<DateTime<Utc>>,
}
```

- [ ] **Step 2: Update AppState construction**

```rust
// In services/src/bin/my-cms-api.rs

// Replace S3 config block with Supabase config:
async fn construct_app_state() -> AppState {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_anon_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set");
    let supabase_service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").ok();
    let storage_bucket = env::var("SUPABASE_STORAGE_BUCKET").unwrap_or("media".to_string());
    let media_base_url = env::var("MEDIA_BASE_URL")
        .unwrap_or(format!("http://{}:{}", host, port));

    let storage = SupabaseStorage::new(
        supabase_url,
        supabase_anon_key,
        supabase_service_role_key,
        storage_bucket,
    );

    AppState {
        conn: Arc::new(conn),
        media_config: Arc::new(MediaConfig {
            storage,
            media_base_url,
        }),
        media_cache: Arc::new(create_media_cache()),
        graphql_immutable_schema: Arc::new(immutable_schema),
        graphql_mutable_schema: Arc::new(mutable_schema),
    }
}
```

- [ ] **Step 3: Update AppState in lib.rs**

```rust
// services/src/lib.rs — update use statements:
use application_core::commands::media::MediaConfig;

pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    pub graphql_immutable_schema: Arc<Schema>,
    pub graphql_mutable_schema: Arc<Schema>,
}
```

And add `use moka::future::Cache;` and MediaCacheKey, CachedMedia imports.

- [ ] **Step 4: Update .env**

```env
# Replace S3 config:
# S3_ENDPOINT=https://sin1.contabostorage.com
# S3_BUCKET_NAME=doitsu-technology
# AWS_ACCESS_KEY_ID=...
# AWS_SECRET_ACCESS_KEY=...

# With Supabase Storage config:
SUPABASE_URL=http://localhost:8000
SUPABASE_ANON_KEY=eyJhbGciOiJI...  # From Supabase project settings
SUPABASE_SERVICE_ROLE_KEY=eyJhbGciOiJI...  # For server-side operations
SUPABASE_STORAGE_BUCKET=media
MEDIA_BASE_URL=http://localhost:8989
```

- [ ] **Step 5: Commit**

```bash
git add services/application_core/src/commands/media/mod.rs services/src/bin/my-cms-api.rs services/src/lib.rs services/.env
git commit -m "refactor: replace S3MediaStorage with SupabaseStorage in config"
```

---

### Task C4: Update Media Handlers (Create, Read, Delete, List, Metadata)

**Files:**
- Modify: `services/application_core/src/commands/media/create/create_handler.rs`
- Modify: `services/application_core/src/commands/media/read/read_handler.rs`
- Modify: `services/application_core/src/commands/media/read/metadata_handler.rs`
- Modify: `services/application_core/src/commands/media/list/list_handler.rs`
- Modify: `services/application_core/src/commands/media/delete/delete_handler.rs`

- [ ] **Step 1: Update CreateMediaHandler (upload)**

```rust
// services/application_core/src/commands/media/create/create_handler.rs

use crate::commands::media::MediaConfig;
use crate::common::app_error::AppError;

pub trait CreateMediaHandlerTrait {
    fn handle_create_media(
        &self,
        media: Vec<u8>,
        media_name: String,
        content_type: String,
        extension: String,
    ) -> impl std::future::Future<Output = Result<super::MediaModel, AppError>>;
}

#[derive(Debug)]
pub struct CreateMediaHandler {
    pub media_config: Arc<MediaConfig>,
}

impl CreateMediaHandlerTrait for CreateMediaHandler {
    async fn handle_create_media(
        &self,
        media: Vec<u8>,
        media_name: String,
        content_type: String,
        extension: String,
    ) -> Result<super::MediaModel, AppError> {
        let media_name_with_nano = format!("{} {}", nanoid!(10), media_name.clone()).to_slug();
        let beautiful_media_name = format!("{}.{}", media_name_with_nano, extension);

        let cache_control = "public, max-age=31536000, immutable";

        self.media_config
            .storage
            .upload(&beautiful_media_name, media, &content_type, Some(cache_control))
            .await?;

        let url_path = if is_image_content_type(&content_type) {
            format!(
                "{}/media/images/{}",
                self.media_config.media_base_url, beautiful_media_name
            )
        } else {
            format!(
                "{}/media/{}",
                self.media_config.media_base_url, beautiful_media_name
            )
        };

        Ok(super::MediaModel {
            path: beautiful_media_name,
            url: url_path,
        })
    }
}
```

- [ ] **Step 2: Update ReadMediaHandler (fetch + image transformation)**

```rust
// services/application_core/src/commands/media/read/read_handler.rs

use crate::commands::media::supabase_storage::SupabaseStorage;
use crate::common::app_error::AppError;
use moka::future::Cache;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MediaCacheKey {
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct CachedMedia {
    pub data: Vec<u8>,
    pub content_type: String,
}

pub fn create_media_cache() -> Cache<MediaCacheKey, CachedMedia> {
    Cache::builder()
        .max_capacity(500)
        .time_to_live(Duration::from_secs(3600))
        .time_to_idle(Duration::from_secs(1800))
        .build()
}

#[derive(Debug)]
pub struct ReadMediaHandler {
    pub storage: SupabaseStorage,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
}

impl ReadMediaHandler {
    /// Fetch media with optional resize dimensions.
    /// Uses cache for repeat requests. Image resizing delegates to
    /// Supabase Image Transformation (render endpoint).
    pub async fn fetch_media(
        &self,
        path: String,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<CachedMedia, AppError> {
        let cache_key = MediaCacheKey {
            path: path.clone(),
            width,
            height,
        };

        // Check cache first
        if let Some(cached) = self.media_cache.get(&cache_key).await {
            return Ok(cached);
        }

        let (bytes, content_type) = self.storage.download(&path).await?;

        let is_image = content_type.starts_with("image/");
        let has_resize = width.is_some() || height.is_some();

        let result = if is_image && has_resize {
            // For image resizing, the frontend will use Supabase render URLs
            // The API just returns the original; client-side handles resize
            // via the rendered URL in the response
            bytes
        } else {
            bytes
        };

        let cached = CachedMedia {
            data: result.clone(),
            content_type: content_type.clone(),
        };

        self.media_cache.insert(cache_key, cached.clone()).await;

        Ok(cached)
    }
}
```

Note: The image resize logic moves from server-side (using `image` crate) to client-side via Supabase render URLs. The API endpoints continue to serve images, but the `read_handler.rs` no longer performs CPU-intensive resize operations. The `/media/images/{*path}?w=&h=` endpoint can instead return a redirect or the Supabase render URL.

- [ ] **Step 3: Update API layer for image endpoints**

```rust
// services/src/api/media/read/read_handler.rs

// Update api_get_media_image to use render URLs for resizing:

pub async fn api_get_media_image(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Query(params): Query<ResizeParams>,
) -> impl IntoResponse {
    // For resize requests, redirect to Supabase Image Transformation
    if params.width.is_some() || params.height.is_some() {
        let render_url = state.media_config.storage.render_image_url(
            &path,
            params.width,
            params.height,
        );
        return Redirect::temporary(&render_url);
    }

    // For non-resized images, serve from Supabase Storage
    let handler = ReadMediaHandler {
        storage: state.media_config.storage.clone(),
        media_cache: state.media_cache.clone(),
    };

    match handler.fetch_media(path, None, None).await {
        Ok(cached) => {
            let mime: mime::Mime = cached.content_type.parse().unwrap_or(mime::APPLICATION_OCTET_STREAM);
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, mime.to_string()),
                    (header::CACHE_CONTROL, "public, max-age=31536000, immutable".to_string()),
                ],
                cached.data,
            )
                .into_response()
        }
        Err(AppError::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
```

- [ ] **Step 4: Update metadata handler**

```rust
// services/application_core/src/commands/media/read/metadata_handler.rs

pub async fn get_media_info(
    storage: &SupabaseStorage,
    media_base_url: &str,
    path: &str,
) -> Result<MediaMetadata, AppError> {
    let info = storage.get_info(path).await?;

    Ok(MediaMetadata {
        path: path.to_string(),
        url: format!("{}/media/{}", media_base_url, path),
        content_type: info.mimetype.unwrap_or_else(|| guess_content_type(path)),
        size: info.content_length.unwrap_or(0),
        last_modified: info
            .last_modified
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc2822(s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
    })
}
```

- [ ] **Step 5: Update list handler**

```rust
// services/application_core/src/commands/media/list/list_handler.rs

pub async fn list_media(
    storage: &SupabaseStorage,
    media_base_url: &str,
    prefix: Option<&str>,
) -> Result<Vec<MediaMetadata>, AppError> {
    let objects = storage.list_objects(prefix).await?;

    objects
        .into_iter()
        .map(|obj| {
            let name = obj.name;
            let (mimetype, size) = obj
                .metadata
                .map(|m| (m.mimetype, m.content_length))
                .unwrap_or((None, None));

            Ok(MediaMetadata {
                path: name.clone(),
                url: format!("{}/media/{}", media_base_url, name),
                content_type: mimetype.unwrap_or_else(|| guess_content_type(&name)),
                size: size.unwrap_or(0),
                last_modified: None, // Not available in list response
            })
        })
        .collect()
}
```

- [ ] **Step 6: Update delete handler**

```rust
// services/application_core/src/commands/media/delete/delete_handler.rs

pub async fn delete_media(storage: &SupabaseStorage, path: &str) -> Result<(), AppError> {
    storage.delete(path).await
}

pub async fn delete_media_batch(
    storage: &SupabaseStorage,
    paths: Vec<String>,
) -> Result<usize, AppError> {
    storage.delete_batch(&paths).await
}
```

- [ ] **Step 7: Verify compilation**

```bash
cd services && cargo check
```
Expected: Compilation succeeds with all handlers using SupabaseStorage.

- [ ] **Step 8: Commit**

```bash
git add services/application_core/src/commands/media/
git add services/src/api/media/
git commit -m "refactor: migrate media handlers from S3 to Supabase Storage"
```

---

### Task C5: Clean Up S3 Dependencies

**Files:**
- Modify: `services/application_core/Cargo.toml`
- Modify: `services/Cargo.toml`

- [ ] **Step 1: Remove rust-s3 and image crates**

```toml
# In both services/Cargo.toml and services/application_core/Cargo.toml:
# Remove:
# rust-s3 = "0.37.0"
# image = "0.25.9"
```

- [ ] **Step 2: Remove S3Error from AppError**

```rust
// In services/application_core/src/common/app_error.rs, remove:
// S3Error(S3Error),  // no longer needed
// And the From<S3Error> impl
```

- [ ] **Step 3: Verify full build**

```bash
cd services && cargo build
```
Expected: Successful build with no S3 references.

- [ ] **Step 4: Commit**

```bash
git add services/Cargo.toml services/application_core/Cargo.toml services/application_core/src/common/app_error.rs
git commit -m "chore: remove rust-s3 and image crate dependencies"
```

---

## Part D: Expose Image Via Supabase Image Transformation

### Task D1: Implement Render URL Endpoint Strategy

**Files:**
- Modify: `services/src/api/media/read/read_handler.rs`
- Modify: `services/application_core/src/commands/media/read/read_handler.rs`

- [ ] **Step 1: Design the image delivery flow**

The new image delivery strategy:
1. **No resize params** (plain `/media/images/file.webp`): Serve the original image from Supabase Storage (pass-through)
2. **With resize params** (`/media/images/file.webp?w=300&h=200`): Redirect (302) to Supabase's render endpoint
3. The Supabase render URL takes the form: `{supabase_url}/storage/v1/render/image/public/{bucket}/{path}?width=X&height=Y`

Supabase Image Transformation requirements:
- Width and height params
- Supports: `width`, `height`, `resize` (cover/contain/fill), `quality` (1-100), `format` (auto by default)

- [ ] **Step 2: Update the image read handler**

```rust
// services/src/api/media/read/read_handler.rs

use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::AppState;
use application_core::commands::media::read::read_handler::ReadMediaHandler;
use application_core::common::app_error::AppError;

#[derive(Debug, Deserialize)]
pub struct ResizeParams {
    pub w: Option<u32>,
    pub h: Option<u32>,
    pub quality: Option<u8>,
    pub resize: Option<String>, // cover, contain, fill
}

pub async fn api_get_media_image(
    State(state): State<AppState>,
    Path(path): Path<String>,
    Query(params): Query<ResizeParams>,
) -> Response {
    let has_resize = params.w.is_some() || params.h.is_some();

    if has_resize {
        // Redirect to Supabase Image Transformation
        let render_url = state
            .media_config
            .storage
            .render_image_url(&path, params.w, params.h);
        return Redirect::temporary(&render_url).into_response();
    }

    // No resize — serve original
    let handler = ReadMediaHandler {
        storage: state.media_config.storage.clone(),
        media_cache: state.media_cache.clone(),
    };

    match handler.fetch_media(path, None, None).await {
        Ok(cached) => {
            let mime: mime::Mime = cached
                .content_type
                .parse()
                .unwrap_or(mime::APPLICATION_OCTET_STREAM);
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, mime.to_string()),
                    (
                        header::CACHE_CONTROL,
                        "public, max-age=31536000, immutable".to_string(),
                    ),
                ],
                cached.data,
            )
                .into_response()
        }
        Err(AppError::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
```

- [ ] **Step 3: Remove local image resize code**

In `services/application_core/src/commands/media/read/read_handler.rs`, the `resize_image()` function and `image` crate usage can be removed since resize is now handled by Supabase's imgproxy integration.

The `CachedMedia` struct and cache remain — they still cache original image fetches.

- [ ] **Step 4: Update guess_content_type**

The `guess_content_type()` helper stays for the `/media/{*path}` (non-image) endpoint. It's in `read_handler.rs` and handles document content types (PDF, DOCX, etc.) by file extension.

- [ ] **Step 5: Verify integration test**

```bash
# Start services with docker compose
docker compose up -d

# Upload an image and verify:
curl -X POST http://localhost:8989/api/media \
  -H "Authorization: Bearer $TOKEN" \
  -F "file=@test.jpg"

# Get original image:
curl http://localhost:8989/media/images/test-slug.jpg

# Get resized image (should redirect to Supabase render):
curl -v http://localhost:8989/media/images/test-slug.jpg?w=300
# Expected: HTTP 302 redirect to Supabase render URL
```

- [ ] **Step 6: Commit**

```bash
git add services/src/api/media/read/read_handler.rs services/application_core/src/commands/media/read/read_handler.rs
git commit -m "feat: delegate image resizing to Supabase Image Transformation"
```

---

## Verification Checklist

- [ ] Supabase Storage bucket `media` exists and is public
- [ ] Upload a file → stored in Supabase Storage bucket
- [ ] Upload returns correct URL (`{media_base_url}/media/images/{path}`)
- [ ] GET `/media/images/{path}` returns original image (200)
- [ ] GET `/media/images/{path}?w=300` redirects to Supabase render URL (302)
- [ ] GET `/media/{path}` returns any file (no resize)
- [ ] GET `/media/info/{path}` returns metadata (size, type, last modified)
- [ ] GET `/media` returns list of all files
- [ ] DELETE `/media/delete/{path}` removes file from bucket
- [ ] DELETE `/media` with JSON body deletes batch of files
- [ ] Image resizing produces correctly sized images via Supabase
- [ ] Cache works — second fetch hits cache (faster, no network call)
- [ ] `cargo build` succeeds with no rust-s3 or image crate references
- [ ] All existing tests pass (`cargo test`)
