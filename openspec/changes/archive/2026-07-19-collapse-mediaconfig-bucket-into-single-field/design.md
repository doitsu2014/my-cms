# Design: Collapse MediaConfig Bucket into a Single Field

## Context

`MediaConfig` currently encodes a request-scoped fact (`bucket_override`) inside a value that is supposed to describe application policy. This forces every consumer to:

1. Decide whether the request supplied an override or not.
2. Resolve the effective bucket via `effective_bucket(config, override)`.
3. Use the resolved name to drive storage calls.
4. Re-derive the override-vs-default distinction to build the response URL (since omitting `?bucket=` and explicitly setting `?bucket=media` must produce *different* response URLs).

The two-field shape leaks request facts into the application core and is the reason `effective_bucket` exists at all. After the lift-bucket-onto-mediaconfig change, `MediaConfig.default_bucket` is a required application-level default; combining it with `bucket_override: Option<String>` reproduces the same redundancy, only now with a sibling string field.

The goal is to make `MediaConfig.bucket` the **single source of truth** for the target bucket of a request, move the override-vs-default decision into the API layer where it belongs, and remove the helper that re-encoded it for the application core.

### Current integration points

- `MediaConfig` definition and `effective_bucket` helper:
  `apps/api/application_core/src/commands/media/mod.rs`
- Object-operation application handlers (use `effective_bucket` + `bucket_override` for both storage and response URL):
  `apps/api/application_core/src/commands/media/{create,list,read,delete}/`
- Public read application handler (uses `effective_bucket` for the visibility gate and for `download` / `download_render`):
  `apps/api/application_core/src/commands/media/read/read_handler.rs`
- Object-operation API handlers (construct request-scoped `MediaConfig` from validated query):
  `apps/api/src/api/media/{create,list,read,delete}/`
- Bucket-management command handlers (reserved-name check uses `default_bucket`):
  `apps/api/application_core/src/commands/media/bucket/{create,delete}/`
- Bucket-management API handlers (pass the global `MediaConfig` unchanged):
  `apps/api/src/api/media/bucket/{create,delete}/`
- Composition root:
  `apps/api/src/bin/my-cms-api.rs`

## Goals

- Collapse `MediaConfig.default_bucket` and `MediaConfig.bucket_override` into a single required `bucket: String` field.
- Remove `effective_bucket(config, override_)` entirely.
- Make the API layer the single decision point for "did the request supply `?bucket=`?" and pass that fact into the response URL builder as `include_bucket_query: bool`.
- Preserve every external HTTP route, query parameter, request body, and response body.
- Preserve all existing authentication, authorization, caching, and error mapping.
- Preserve bucket-management method signatures and storage signatures.

## Non-Goals

- Renaming `bucket_override` to `bucket` while keeping the `Option<String>` shape.
- Keeping `effective_bucket` as a deprecated alias.
- Relocating caches from `AppState` to `MediaConfig`.
- Adding a `SUPABASE_STORAGE_BUCKET` environment variable.
- Changing Supabase Storage URL patterns.
- Changing public/private bucket authorization behavior.
- Changing media cache capacity, TTL, or invalidation semantics.
- Changing database schema or entities.
- Changing frontend behavior.

## Decisions

### 1. `MediaConfig.bucket` is a single required `String`

```rust
#[derive(Clone, Debug)]
pub struct MediaConfig {
    pub storage: SupabaseStorage,
    pub bucket: String,
    pub media_base_url: String,
}
```

The shape is the post-resolution target. There is no `Option<String>` and no `default_bucket` sibling.

Rationale:

- The composition root sets `bucket: "media".to_string()`. The default is now *implicit*: a global config that has not been overridden.
- Every per-request override is resolved once, in the API handler, and propagated as a `String`.
- Reserved-name validation in bucket create/delete handlers can refer to the configured bucket without a separate accessor.

Trade-off:

- The two previous fields were folded into one, so every `MediaConfig` fixture must be rewritten.

### 2. The `effective_bucket` helper is removed

There is no longer a "resolve override against default" step inside the application core. The application core consumes a `MediaConfig` whose `bucket` is already the target.

What replaces it:

- API handlers construct a request-scoped `MediaConfig` with `bucket: params.bucket.clone().unwrap_or_else(|| state.media_config.bucket.clone())` (or the equivalent for handlers that take the validated query directly).
- Command handlers receive that scoped config and pass `self.media_config.bucket.as_str()` to storage.

This eliminates the second source of truth (override) and makes the request flow one-pass.

### 3. `include_bucket_query: bool` is the new request-URL knob

The override-vs-default distinction must survive **only** in the response URL builder. Concretely:

- `api_create_media` records `let include_bucket_query = params.bucket.is_some();` after validation and passes it to `CreateMediaHandler::create_media` along with the scoped config.
- `api_list_media` and `api_get_media_metadata` do the same and pass it through their handlers.
- The response URL branches:

```rust
let url_path = if include_bucket_query {
    format!("{}/media/{}?bucket={}", base, path, self.media_config.bucket)
} else if is_image_content_type(&content_type) {
    format!("{}/media/images/{}", base, path)
} else {
    format!("{}/media/{}", base, path)
};
```

Without this flag, both cases (`?bucket=` omitted and `?bucket=media` supplied) would canonicalize to the default URL and the explicit-override URL would silently regress.

Trade-off:

- Three handlers each gain one new parameter (`include_bucket_query`). The handler traits therefore widen.

### 4. `MediaCacheKey.bucket` becomes `String`

`ReadMediaHandler` always knows the target bucket at the point where it builds a cache key (it has just resolved it through the scoped config). Making the field `String` removes the per-call `Option` and eliminates a distinct cache state for "unknown bucket".

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaCacheKey {
    pub bucket: String,
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
```

The bucket-segment-cache-coherence invariant (two requests targeting the same bucket and path share a cache entry) is preserved by deriving `bucket` from `self.media_config.bucket.clone()` once and using it for both the key and the storage call.

Trade-off:

- The `cache_key_without_bucket_is_distinct_from_bucket_scoped_key` test is rewritten as the equivalent `String`-based assertion.

### 5. Bucket create/delete read `MediaConfig.bucket` for reserved-name checks

```rust
if req.name == self.media_config.bucket {
    return Err(AppError::Validation(
        "name".to_string(),
        format!("cannot use reserved bucket name '{}'", self.media_config.bucket),
    ));
}
```

The deletion handler follows the same pattern. The constant `"media"` is no longer hard-coded in two places; the composition root remains the single literal source.

### 6. Public read flow uses a scoped `MediaConfig`

`api_get_media_image` and `api_get_media` resolve the validated query bucket against the global default and build a scoped `MediaConfig` whose `bucket` is the target. The visibility gate reads `media_config.bucket.as_str()`. The same scoped `Arc<MediaConfig>` is passed to `ReadMediaHandler`, which now needs no bucket parameter at all from the trait surface (`get_rendered_image(path, resize_params, bucket: Option<String>)` becomes `get_rendered_image(path, resize_params)`).

```rust
let scoped_config = Arc::new(MediaConfig {
    storage: state.media_config.storage.clone(),
    bucket: bucket.clone().unwrap_or_else(|| state.media_config.bucket.clone()),
    media_base_url: state.media_config.media_base_url.clone(),
});

enforce_bucket_visibility_gate(
    state.media_config.storage.clone(),
    state.bucket_visibility_cache.clone(),
    scoped_config.bucket.as_str(),
    &headers,
)
.await?;

let handler = ReadMediaHandler::new(scoped_config, state.media_cache.clone());
```

Trade-off:

- The API handler now does an `unwrap_or_else` instead of one helper call. The two resolution steps (gate + handler) become one.

### 7. All 19 `MediaConfig { ... }` literals are rewritten

Every production site and every test fixture collapses `default_bucket: "media".to_string()` and `bucket_override: None` (or `Some(name)`) into the single `bucket: "media".to_string()` (or `bucket: name.to_string()`).

Sites (one production + one test set):

- 6 production literals: `apps/api/src/bin/my-cms-api.rs`, `apps/api/src/api/media/{create,list,read/metadata,delete}/...`.
- 13 test literals: `apps/api/application_core/src/commands/media/mod.rs` (helper), create / list / metadata / read / delete / bucket/{create,delete,get,update,empty} command handler tests.

The literal shape becomes uniform: there is exactly one way to construct a `MediaConfig` and exactly one bucket field to fill in.

## Architecture

### Configuration and storage types

```rust
#[derive(Clone, Debug)]
pub struct MediaConfig {
    pub storage: SupabaseStorage,
    pub bucket: String,
    pub media_base_url: String,
}
```

`SupabaseStorage` is unchanged from the prior change (already bucket-neutral).

`AppState` is unchanged structurally; only the inner `MediaConfig` shape changes.

```rust
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    pub bucket_visibility_cache: Arc<Cache<String, bool>>,
    // ...
}
```

### Request flow (object operations)

```text
HTTP query ?bucket=
        │
        ▼
validate bucket syntax
        │
        ▼
include_bucket_query = params.bucket.is_some()
        │
        ▼
build scoped MediaConfig:
  bucket = params.bucket.unwrap_or(global.media_config.bucket)
        │
        ├──────────────▶ CreateMediaHandler::create_media(name, data, ct, include_bucket_query)
        ├──────────────▶ ListMediaHandler::list_media(prefix, include_bucket_query)
        ├──────────────▶ MetadataMediaHandler::get_metadata(path, include_bucket_query)
        ├──────────────▶ DeleteMediaHandler::delete_media(path)
        ├──────────────▶ DeleteMediaHandler::delete_media_batch(paths)
        │
        ▼
storage.<op>(media_config.bucket.as_str(), ...)
        │
        ▼
Supabase Storage REST URL
```

Response URL construction inside each command handler:

```rust
if include_bucket_query {
    format!("{}/media/{}?bucket={}", base, path, self.media_config.bucket)
} else {
    /* default URL (image or generic) */
}
```

### Request flow (public read)

```text
HTTP query ?bucket=
        │
        ▼
validate bucket syntax
        │
        ▼
build scoped MediaConfig:
  bucket = params.bucket.unwrap_or(global.media_config.bucket)
        │
        ├──────────────▶ visibility gate on scoped.bucket
        │
        ▼
ReadMediaHandler::get_rendered_image(path, params)
ReadMediaHandler::get_media_for_bucket(path)
        │
        ▼
storage.download_render(scoped.bucket, ...) / storage.download(scoped.bucket, ...)
```

The public read trait surface drops the `bucket: Option<String>` argument — the handler now reads the bucket from its own scoped config.

### Bucket-management flow

```rust
if req.name == self.media_config.bucket {
    return Err(AppError::Validation("name", "cannot use reserved bucket name '...'"));
}
```

Bucket-management storage signatures and payload DTOs are unchanged.

### Media cache

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MediaCacheKey {
    pub bucket: String,
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
```

The handler builds the key from `self.media_config.bucket.clone()` once per call.

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

The meaning of an absent `?bucket=` remains "use the default media bucket." The meaning of `?bucket=<name>` remains "target bucket `<name>`" with the response URL including `?bucket=<name>`.

## Data Flow and Database Schema

No database records are read or written by this refactor. No migration is required. No SeaORM entity regeneration is required. No Supabase Storage bucket migration is required.

## Risks and Mitigations

### Subtle response-URL regression when collapsing fields

**Risk:** Without a flag, the response URL for a default request and a request with `?bucket=media` would both render as `${MEDIA_BASE_URL}/media/<path>`, losing the explicit `?bucket=media` query.

**Mitigation:** `include_bucket_query: bool` is threaded through the create / list / metadata handlers and gates the `?bucket=` segment of the response URL. The existing wiremock-based tests for override and no-override URL shapes are kept and continue to assert the exact form.

### Missed call site

**Risk:** A missed `MediaConfig` literal or stray `effective_bucket` reference will fail compilation.

**Mitigation:** The change plan includes repository-wide searches for `MediaConfig\s*\{`, `default_bucket`, `bucket_override`, and `effective_bucket`. `cargo check --workspace` provides a second completeness check.

### `MediaCacheKey` widening

**Risk:** A reader that holds a cached `Option<String>` key will mis-hash after the field becomes `String`.

**Mitigation:** There is no on-disk cache; `moka` is in-process and is invalidated on restart. The handler rebuilds keys from the new shape on every call. The cache unit tests are updated in lockstep.

### Reserved-name policy divergence

**Risk:** The two bucket create/delete handlers could still compare against a hard-coded constant.

**Mitigation:** Remove both references to `self.media_config.default_bucket` and replace with `self.media_config.bucket`. Tests assert against the configured field.

### Performance

`include_bucket_query` adds no allocation beyond the existing `Option::is_some` call. The scoped `MediaConfig` is built once per request and is cheap to construct (three `Clone`s). The `unwrap_or_else` on the global default bucket is a single `String` clone per request, identical to the prior shape.

## Migration

This is a drop-in code migration:

1. Collapse `MediaConfig` fields and remove `effective_bucket`.
2. Update the composition root literal.
3. Update the four authenticated object-operation API handlers.
4. Update the five command handlers (create, list, metadata, delete, read).
5. Update the public read API handlers and the visibility gate.
6. Update the two reserved-name command handlers and tests.
7. Rewrite all 19 `MediaConfig { ... }` literals.
8. Run targeted and full Rust verification.
9. Verify OpenSpec conformance.
10. Sync the delta spec and archive the change.

There is no environment, database, storage-object, route, or client migration. Deployment rollback is a normal application binary rollback.
