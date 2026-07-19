# Design: Lift Media Default Bucket onto MediaConfig

## Context

`SupabaseStorage` currently owns five pieces of state:

- Supabase base URL
- Anonymous key
- Optional service-role key
- HTTP client
- Bucket name

The first four describe a reusable Supabase Storage client. The bucket name does not: it is request/application policy that determines the target of one operation.

This creates an asymmetric API:

```text
Object operations                         Bucket management
──────────────────────────────────────    ─────────────────────────────
download(path)                            get_bucket(name)
upload(path, ...)                         update_bucket(name, payload)
list_objects(prefix)                      empty_bucket(name)
delete(path)                              delete_bucket(name, purge)
bucket comes from self.bucket             bucket comes from argument
```

Per-request overrides are implemented by cloning the storage object with `with_bucket(name)`. Forgetting that call does not fail compilation; it silently targets the storage object's existing bucket.

The default `"media"` also appears in more than one policy location:

- The `SupabaseStorage` constructor in the composition root
- Public read fallback logic
- Reserved-name constants in bucket create/delete handlers
- Test fixtures

The refactor makes storage bucket-neutral and moves the required application default to `MediaConfig`.

### Current integration points

- Storage implementation:
  `apps/api/application_core/src/commands/media/supabase_storage.rs`
- Media configuration:
  `apps/api/application_core/src/commands/media/mod.rs`
- Application handlers:
  `apps/api/application_core/src/commands/media/{create,list,read,delete}/`
- Bucket handlers:
  `apps/api/application_core/src/commands/media/bucket/`
- HTTP handlers:
  `apps/api/src/api/media/`
- Composition root:
  `apps/api/src/bin/my-cms-api.rs`
- Shared state:
  `apps/api/src/lib.rs`

`media_cache` and `bucket_visibility_cache` currently belong to `AppState`. They remain there.

## Goals

- Remove `SupabaseStorage.bucket`.
- Remove `SupabaseStorage::with_bucket`.
- Make every object-operation target visible at the call site.
- Store the required default bucket on `MediaConfig`.
- Centralize override-versus-default resolution.
- Preserve all existing HTTP routes and query parameters.
- Preserve current authentication, authorization, caching, and error mapping.
- Preserve bucket-management method signatures.
- Preserve existing object-operation parameter types other than the new bucket argument.

## Non-Goals

- Relocating caches from `AppState` to `MediaConfig`.
- Adding an environment variable for the default bucket.
- Renaming or removing `bucket_override`.
- Deprecating `with_bucket`; it is removed.
- Renaming bucket-management methods.
- Changing bucket-management payload DTOs.
- Changing Supabase Storage endpoint patterns.
- Changing media cache key behavior.
- Changing public/private bucket authorization behavior.
- Changing database schema or entities.
- Changing frontend behavior.

## Decisions

### 1. `MediaConfig.default_bucket` is a required `String`

`default_bucket` SHALL be `String`, not `Option<String>`.

There is always a usable default. The prior removal of `SUPABASE_STORAGE_BUCKET` established `"media"` as an application default rather than optional environment configuration.

Advantages:

- Every no-override request has a target.
- Callers do not need to handle a missing-default error.
- Reserved-name validation can refer to the configured default.
- Tests may use a non-`"media"` default to prove the code does not depend on a literal.

Trade-off:

- Every `MediaConfig` fixture must initialize one additional field.

### 2. Bucket resolution is a shared application-core helper

The helper SHALL live in:

`apps/api/application_core/src/commands/media/mod.rs`

```rust
pub fn effective_bucket<'a>(
    config: &'a MediaConfig,
    override_: Option<&'a str>,
) -> &'a str {
    override_.unwrap_or(config.default_bucket.as_str())
}
```

This location is preferred over a private helper in the public read API because create, list, metadata, delete, and read operations all need the same rule.

Usage:

```rust
let bucket_name = effective_bucket(
    &self.media_config,
    self.media_config.bucket_override.as_deref(),
);
```

Public read endpoints use the validated query value:

```rust
let bucket_name = effective_bucket(&state.media_config, bucket.as_deref());
```

The public API uses the effective name for the visibility gate. The application command handler resolves the same value before invoking storage.

Trade-off:

- The public read flow resolves the same reference once for authorization and once inside the command handler. This is a negligible `Option::unwrap_or` and avoids moving business logic into the API layer.

### 3. `ReadMediaHandler` receives `MediaConfig`, not bucket-mutated storage

`ReadMediaHandler` currently owns `Arc<SupabaseStorage>` and clones it when an override exists. It SHALL instead own `Arc<MediaConfig>` alongside the existing cache.

```rust
pub struct ReadMediaHandler {
    pub media_config: Arc<MediaConfig>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
}
```

This gives the command handler access to:

- Bucket-neutral storage
- Required default bucket
- Existing request override resolution policy

The API remains thin:

1. Validate query input.
2. Resolve the bucket for the visibility gate.
3. Invoke `ReadMediaHandler`.
4. Convert the result to an HTTP response.

### 4. `with_bucket` is removed rather than deprecated

Every current caller already knows either:

- The explicit request override, or
- The configured default.

A deprecated alias would preserve the implicit state model and allow new code to continue depending on it. Because this is an internal Rust API and all callers are in the repository, a compile-time migration is preferred.

`SupabaseStorage::new` also drops its bucket argument.

### 5. Reserved-name checks read `MediaConfig.default_bucket`

These constants are removed:

```rust
const RESERVED_BUCKET_NAME: &str = "media";
```

Bucket create/delete handlers instead compare with:

```rust
self.media_config.default_bucket.as_str()
```

Error messages interpolate the configured value. This prevents reserved-name policy from diverging if the application default changes in a fixture or future composition root.

### 6. Existing non-bucket parameter types are preserved

The illustrative target signatures are normalized to the current implementation types. Only the explicit bucket argument and constructor shape change.

```rust
impl SupabaseStorage {
    pub fn public_url(&self, bucket: &str, path: &str) -> String;

    pub fn render_image_url(
        &self,
        bucket: &str,
        path: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> String;

    pub async fn download_render(
        &self,
        bucket: &str,
        path: &str,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<(Vec<u8>, String), AppError>;

    pub async fn upload(
        &self,
        bucket: &str,
        file_path: &str,
        data: &[u8],
        content_type: &str,
        cache_control: Option<&str>,
    ) -> Result<(), AppError>;

    pub async fn download(
        &self,
        bucket: &str,
        path: &str,
    ) -> Result<(Vec<u8>, String), AppError>;

    pub async fn get_info(
        &self,
        bucket: &str,
        path: &str,
    ) -> Result<StorageObjectMetadata, AppError>;

    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: Option<&str>,
    ) -> Result<Vec<StorageObject>, AppError>;

    pub async fn delete(
        &self,
        bucket: &str,
        path: &str,
    ) -> Result<(), AppError>;

    pub async fn delete_batch(
        &self,
        bucket: &str,
        paths: &[String],
    ) -> Result<Vec<DeletedObject>, AppError>;
}
```

This avoids unrelated ownership and API changes to file bytes, dimensions, prefix handling, cache control, and batch paths.

### 7. Tests assert method arguments through request behavior

Tests SHALL no longer inspect a storage bucket field.

Wiremock path assertions already provide a stronger contract. Tests will call methods with explicit buckets and assert that the expected URL is requested.

The former `with_bucket_returns_clone_with_replaced_bucket` test is replaced by a test proving one `SupabaseStorage` instance can target two bucket names through method arguments without mutation.

## Architecture

### Configuration and storage types

```rust
#[derive(Clone, Debug)]
pub struct MediaConfig {
    pub storage: SupabaseStorage,
    pub default_bucket: String,
    pub media_base_url: String,
    pub bucket_override: Option<String>,
}

#[derive(Clone)]
pub struct SupabaseStorage {
    pub supabase_url: String,
    pub anon_key: String,
    pub service_role_key: Option<String>,
    pub client: reqwest::Client,
}

impl SupabaseStorage {
    pub fn new(
        supabase_url: impl Into<String>,
        anon_key: impl Into<String>,
        service_role_key: Option<String>,
    ) -> Self {
        // Existing client construction; no bucket state.
    }
}
```

`AppState` remains:

```rust
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    pub bucket_visibility_cache: Arc<Cache<String, bool>>,
    // Existing GraphQL and administrator fields.
}
```

### Request flow

```text
HTTP query ?bucket=
        │
        ▼
validate bucket syntax
        │
        ▼
effective_bucket(MediaConfig, override)
        │
        ├──────────────▶ bucket visibility gate
        │                  get_bucket(explicit_name)
        │
        ▼
application command handler
        │
        ▼
effective_bucket(MediaConfig, request override)
        │
        ▼
SupabaseStorage.operation(explicit_bucket, ...)
        │
        ▼
Supabase Storage REST URL
```

### Create/list/metadata/delete flow

The existing API handlers construct request-scoped `MediaConfig` values to retain `bucket_override`. After the refactor, these values clone bucket-neutral storage and copy the default:

```rust
let media_config = Arc::new(MediaConfig {
    storage: state.media_config.storage.clone(),
    default_bucket: state.media_config.default_bucket.clone(),
    media_base_url: state.media_config.media_base_url.clone(),
    bucket_override: params.bucket.clone(),
});
```

The command handler resolves the effective bucket and passes it to storage.

No request-scoped storage mutation or `with_bucket` call remains.

### Public read flow

```rust
let bucket = validate_bucket(params.bucket.as_deref())?;
let bucket_name = effective_bucket(&state.media_config, bucket.as_deref());

enforce_bucket_visibility_gate(
    state.media_config.storage.clone(),
    state.bucket_visibility_cache.clone(),
    bucket_name,
    &headers,
)
.await?;

let handler = ReadMediaHandler::new(
    state.media_config.clone(),
    state.media_cache.clone(),
);

handler.get_media_for_bucket(path, bucket).await
```

The visibility gate continues to use `get_bucket(bucket_name)`, whose signature is already explicit.

### Bucket-management flow

Bucket-management storage signatures remain unchanged:

```rust
list_buckets()
get_bucket(name)
create_bucket(payload) // name is in payload
update_bucket(name, payload)
empty_bucket(name)
delete_bucket(name, purge)
```

The only bucket-management behavior change is reserved-name lookup:

```rust
if req.name == self.media_config.default_bucket {
    // reject
}
```

## API Contracts

No external contract changes.

Preserved routes include:

- `POST /api/media?bucket=<name>`
- `GET /api/media?bucket=<name>`
- `GET /api/media/info/{path}?bucket=<name>`
- `DELETE /api/media/delete/{path}?bucket=<name>`
- Batch media delete with optional bucket query
- `GET /media/{path}?bucket=<name>`
- `GET /media/images/{path}?bucket=<name>`
- Existing bucket-management routes

The meaning of an absent `?bucket=` remains "use the default media bucket."

## Data Flow and Database Schema

No database records are read or written by this refactor.

No migration is required. No SeaORM entity regeneration is required. No Supabase Storage bucket migration is required.

## Risks and Mitigations

### Missed call site

**Risk:** A missed caller will fail compilation after the method signature changes.

**Mitigation:** The change plan includes repository-wide searches for each of the nine methods, `with_bucket`, the bucket field, `SupabaseStorage::new`, direct struct literals, and `MediaConfig` literals. Rust compilation provides a second completeness check.

### Test assertion rewrites

**Risk:** Tests currently rely on bucket-bearing fixtures or directly inspect the bucket field.

**Mitigation:** Rewrite them to pass bucket names explicitly and retain wiremock request-path assertions. Replace the clone test with a stateless multi-bucket behavior test.

### Incorrect default propagation in request-scoped configuration

**Risk:** An API handler could create a `MediaConfig` without copying `default_bucket`.

**Mitigation:** The field is non-optional, making missed initialization a compiler error. Handler tests cover override and no-override behavior.

### Reserved-name policy divergence

**Risk:** Bucket create/delete tests could still rely on a hard-coded `"media"` constant.

**Mitigation:** Remove both constants and test with the configured default field.

### Performance

`effective_bucket` adds one `Option::unwrap_or` per resolution. Public reads perform it once for the visibility gate and once inside the command handler. This is negligible and performs no allocation.

### Cache behavior

Cache keys remain bucket-aware and retain the existing `Option<String>` representation. This change does not alter cache capacity, TTL, or invalidation.

## Migration

This is a drop-in code migration:

1. Add `MediaConfig.default_bucket`.
2. Remove storage bucket state and `with_bucket`.
3. Change all nine method signatures.
4. Update all compile-time callers and fixtures.
5. Replace hard-coded reserved-name constants.
6. Run targeted and full Rust verification.
7. Verify OpenSpec conformance.
8. Sync the delta spec and archive the change.

There is no environment, database, storage-object, route, or client migration. Deployment rollback is a normal application binary rollback.