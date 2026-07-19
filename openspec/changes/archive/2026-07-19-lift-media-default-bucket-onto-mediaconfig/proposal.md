# Lift Media Default Bucket onto MediaConfig

## Why

`SupabaseStorage` currently carries a mutable-looking `bucket: String` even though the client also exposes bucket-management methods that receive their target bucket explicitly. This creates an asymmetric contract:

- Object read/write operations obtain the bucket implicitly from `self.bucket`.
- Bucket-management operations receive the bucket name as a method argument or payload field.

The asymmetry makes `SupabaseStorage` an unreliable representation of what it actually owns. Callers must remember to invoke `with_bucket(name)` before an object operation, and a missed call silently targets the default bucket instead of producing a compile-time error.

Moving the default bucket to `MediaConfig` makes both types honest:

- `SupabaseStorage` owns connection, authentication, and HTTP-client state.
- `MediaConfig` owns application media policy, including the required default bucket.
- Every bucket-targeting storage operation states its target explicitly.

This removes the class of bugs where a caller forgets to construct a request-scoped storage clone with `with_bucket`.

## What Changes

- Remove `SupabaseStorage.bucket`.
- Remove the bucket argument from `SupabaseStorage::new`.
- Remove `SupabaseStorage::with_bucket`.
- Add `MediaConfig.default_bucket: String`.
- Keep the default value `"media"` in the composition root; do not introduce an environment variable.
- Add `bucket: &str` as the first argument after `&self` to:
  - `upload`
  - `download`
  - `get_info`
  - `list_objects`
  - `delete`
  - `delete_batch`
  - `public_url`
  - `render_image_url`
  - `download_render`
- Update object-operation URL construction to use the explicit method argument.
- Add a shared `effective_bucket(config, override_)` helper that resolves a request override or falls back to `MediaConfig.default_bucket`.
- Update create, list, read, metadata, single-delete, and batch-delete call sites.
- Remove request-scoped storage cloning performed solely to change a bucket.
- Replace `RESERVED_BUCKET_NAME` literals in bucket create/delete handlers with `MediaConfig.default_bucket`.
- Update inline tests to assert explicit bucket arguments through generated request URLs rather than inspecting `SupabaseStorage.bucket`.

**BREAKING:** This changes the internal Rust constructor and method signatures of `SupabaseStorage`. It does not change an external HTTP API.

## Capabilities

### Modified Capabilities

- `supabase-storage`: Make `SupabaseStorage` bucket-neutral, move the application default to `MediaConfig`, and require explicit bucket arguments for all object operations.

### Related Behavioral Areas

- `media`: Create, list, read, metadata, single-delete, and batch-delete handlers resolve their target from the request override or `MediaConfig.default_bucket`.
- `media-bucket-management`: Reserved-name validation uses `MediaConfig.default_bucket`; bucket-management storage signatures remain explicit and unchanged.

Only `supabase-storage` receives a delta spec because the HTTP behavior of the related media and bucket-management capabilities is preserved.

## Impact

- Approximately twelve production Rust modules are modified.
- Approximately ten Rust modules containing inline media tests require fixture or assertion updates.
- One configuration type, `MediaConfig`, gains a required field.
- `SupabaseStorage` loses one field and one cloning helper.
- The composition root remains the single literal source of `"media"`.
- No environment-variable change.
- No database migration.
- No SeaORM entity generation.
- No external API route, query parameter, request body, or response-body change.
- No frontend change.

## Non-Goals

- Keeping `SupabaseStorage::with_bucket` as a deprecated alias.
- Adding a compatibility constructor that still accepts a bucket.
- Unifying or renaming bucket-management methods.
- Renaming `bucket_override`.
- Moving media or bucket-visibility caches from `AppState` to `MediaConfig`.
- Adding a `SUPABASE_STORAGE_BUCKET` environment variable.
- Changing Supabase Storage URL patterns.
- Changing public/private bucket authorization behavior.
- Changing media cache capacity, TTL, or key semantics.
- Changing database schema or generated entities.

## Open Questions

There are no blocking product questions for the focused refactor.

The design assumes the cache fields shown in the illustrative target shape were contextual rather than a request to relocate cache ownership. The current implementation keeps `media_cache` and `bucket_visibility_cache` on `AppState`; moving them should be proposed separately if desired.