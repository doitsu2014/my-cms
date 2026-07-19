# Collapse MediaConfig Bucket into a Single Field

## Why

`MediaConfig` currently carries **two** bucket fields plus a resolution helper:

```rust
pub struct MediaConfig {
    pub storage: SupabaseStorage,
    pub default_bucket: String,
    pub media_base_url: String,
    pub bucket_override: Option<String>,
}

pub fn effective_bucket<'a>(config: &'a MediaConfig, override_: Option<&'a str>) -> &'a str {
    override_.unwrap_or(config.default_bucket.as_str())
}
```

The two-field shape encodes a fact about a request (override or default) inside a value that is supposed to describe application policy. It also leaks that fact into every command handler, every test fixture, and the response-URL builder for create / list / metadata — places where the override-versus-default distinction is needed for *response URL construction* but not for *storage targeting*.

The distinction matters because the response URL for a default request has the form `${MEDIA_BASE_URL}/media/<path>` (no query), while a request with `?bucket=avatars` must yield `${MEDIA_BASE_URL}/media/<path>?bucket=avatars` (with query). Without a separate flag, collapsing `default_bucket` and `bucket_override` into a single `bucket` field would silently canonicalize both cases to the same URL — a subtle response-body regression that would only be caught by behavioral tests.

After this refactor `MediaConfig.bucket` is the single source of truth for the request's target bucket. The "override vs default" distinction lives where it belongs: in the API layer (which knows whether `?bucket=` was supplied) and is passed to the response-URL builder as a separate `include_bucket_query: bool` argument.

## What Changes

- Replace `default_bucket: String` and `bucket_override: Option<String>` with `bucket: String` on `MediaConfig`.
- Remove `effective_bucket(config, override_)` from `apps/api/application_core/src/commands/media/mod.rs`.
- API handlers (create, list, metadata, delete single + batch) construct request-scoped `MediaConfig` values whose `bucket` field is either the validated `?bucket=` query value or the inherited global default.
- API handlers record `include_bucket_query = params.bucket.is_some()` and pass that flag to the corresponding command handler so the response URL keeps the `?bucket=` segment when the client supplied one.
- The composition root at `apps/api/src/bin/my-cms-api.rs` initializes the global `MediaConfig` with `bucket: "media".to_string()`.
- Bucket create/delete command handlers compare `req.name == self.media_config.bucket.as_str()` (and the same pattern for deletion) instead of reading `default_bucket`.
- The `ReadMediaHandler` is updated:
  - `MediaCacheKey.bucket` becomes `String` (was `Option<String>`).
  - The handler derives `bucket` from `self.media_config.bucket.clone()` and passes it to `download` / `download_render`.
- The public read handlers (`api_get_media_image`, `api_get_media`) construct a request-scoped `MediaConfig` whose `bucket` is either the validated query or the global default; the visibility gate reads `media_config.bucket.as_str()`; the same scoped `Arc<MediaConfig>` is passed to `ReadMediaHandler`.
- All 19 `MediaConfig { ... }` literals (6 production + 13 tests) collapse `default_bucket` and `bucket_override` into the single `bucket` field.
- The focused tests `effective_bucket_returns_default_when_override_is_none` and `effective_bucket_returns_override_when_present` are removed (the helper is gone).
- The cache key tests in `read_handler` are updated to use `String` bucket keys; the `cache_key_without_bucket_is_distinct_from_bucket_scoped_key` test becomes the equivalent `String`-based assertion.

**BREAKING:** This changes the internal Rust shape of `MediaConfig`, removes a public-but-internal helper, and changes `MediaCacheKey.bucket` from `Option<String>` to `String`. It does not change any external HTTP route, query parameter, request body, or response body.

## Capabilities

### Modified Capabilities

- `supabase-storage`: `MediaConfig` collapses `default_bucket` + `bucket_override` into a single required `bucket: String`; the `effective_bucket` helper is removed; the request URL builder gains an explicit `include_bucket_query` flag.

### Related Behavioral Areas

- `media`: Create, list, metadata, single-delete, and batch-delete handlers operate on a `MediaConfig` whose `bucket` is already resolved to the request target. Response URLs continue to include `?bucket=<name>` iff the request supplied `?bucket=`.
- `media-bucket-management`: Reserved-name validation reads `MediaConfig.bucket`. Bucket-management storage signatures are unchanged.
- `media-cache`: `MediaCacheKey.bucket` is `String`. Cache capacity, TTL, and invalidation semantics are unchanged.

Only `supabase-storage` receives a delta spec because the HTTP behavior of `media`, `media-bucket-management`, and `media-cache` is preserved.

## Impact

- Approximately twelve production Rust modules are modified.
- Approximately ten Rust modules containing inline media tests require fixture or assertion updates.
- One configuration type, `MediaConfig`, collapses two fields into one and drops a sibling helper.
- `MediaCacheKey.bucket` becomes `String` instead of `Option<String>`.
- The composition root remains the single literal source of `"media"`.
- No environment-variable change.
- No database migration.
- No SeaORM entity generation.
- No external API route, query parameter, request body, or response-body change.
- No frontend change.

## Non-Goals

- Keeping `effective_bucket` as a deprecated alias.
- Adding a compatibility constructor that accepts both fields.
- Relocating caches from `AppState` to `MediaConfig`.
- Adding a `SUPABASE_STORAGE_BUCKET` environment variable.
- Changing Supabase Storage URL patterns.
- Changing public/private bucket authorization behavior.
- Changing media cache capacity, TTL, or invalidation semantics.
- Changing database schema or generated entities.

## Open Questions

There are no blocking product questions for the focused refactor.
