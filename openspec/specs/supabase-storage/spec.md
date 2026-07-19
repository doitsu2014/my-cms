# supabase-storage Specification

## Purpose
TBD - created by archiving change supabase-storage-and-image-transformation-migration. Update Purpose after archive.
## Requirements
### Requirement: Files upload to Supabase Storage

The system SHALL upload media to the Supabase Storage `media` bucket via the REST API endpoint `POST {SUPABASE_URL}/storage/v1/object/{bucket}/{path}`. The request SHALL be authenticated with `Bearer <service_role_key>` (or `anon_key` as a fallback) and SHALL set `x-upsert: true` so re-uploads overwrite the existing object.

#### Scenario: Successful upload

- **WHEN** the API receives a `POST /api/media` with valid auth, file bytes, content type, and filename
- **THEN** the file is uploaded to the `media` bucket at a slugified `{nanoid} {name}.{ext}` path
- **AND** the response body contains `{ path, url }` where `url` is `{MEDIA_BASE_URL}/media/images/{path}` for image content types and `{MEDIA_BASE_URL}/media/{path}` otherwise

#### Scenario: Upload failure surfaces as StorageError

- **WHEN** Supabase Storage responds with a non-2xx status
- **THEN** the handler returns `AppError::StorageError("Upload failed (<status>): <body>")`
- **AND** the API responds with the corresponding error code (4xx or 5xx)

### Requirement: Files are downloaded from Supabase Storage

The system SHALL fetch media from the `media` bucket via `GET {SUPABASE_URL}/storage/v1/object/public/{bucket}/{path}`. A 404 SHALL be translated to `AppError::NotFound`; other non-2xx responses SHALL be translated to `AppError::StorageError`.

#### Scenario: Original image served

- **WHEN** the client requests `GET /media/images/{path}` (no resize params)
- **THEN** the API returns HTTP 200 with the original image bytes
- **AND** the `Content-Type` header matches the stored MIME type
- **AND** the `Cache-Control` header is `public, max-age=31536000, immutable`

#### Scenario: Non-image file served

- **WHEN** the client requests `GET /media/{path}` for a non-image file
- **THEN** the API returns the original bytes with the appropriate content type

#### Scenario: Missing object

- **WHEN** the client requests a path that does not exist
- **THEN** the API returns HTTP 404

### Requirement: Object metadata is exposed

The system SHALL expose object metadata (size, content type, last modified) via `GET {SUPABASE_URL}/storage/v1/object/info/public/{bucket}/{path}`.

#### Scenario: Metadata endpoint

- **WHEN** the client requests `GET /api/media/info/{path}` with valid auth
- **THEN** the API returns `{ path, url, content_type, size, last_modified }`
- **AND** a missing object returns 404

### Requirement: Bucket contents are listable

The system SHALL list objects in the `media` bucket via `POST {SUPABASE_URL}/storage/v1/object/list/public/{bucket}` with `{ prefix, limit: 1000, offset: 0, sortBy: { column: "name", order: "asc" } }`. A prefix filter MAY be supplied.

#### Scenario: List all files

- **WHEN** the client requests `GET /api/media` with valid auth
- **THEN** the API returns an array of `{ path, url, content_type, size }` entries (capped at 1000)

#### Scenario: List with prefix

- **WHEN** the client requests `GET /api/media?prefix=images/`
- **THEN** the API returns only objects whose name starts with `images/`

### Requirement: Single and batch deletion

The system SHALL support deleting a single object via `DELETE {SUPABASE_URL}/storage/v1/object/{bucket}/{path}` and deleting a batch via `DELETE {SUPABASE_URL}/storage/v1/object/{bucket}/delete` with `{ prefixes: [path, …] }`. Both endpoints require Bearer auth.

#### Scenario: Single delete

- **WHEN** the client requests `DELETE /api/media/delete/{path}` with valid auth
- **THEN** the object is removed from the `media` bucket
- **AND** the API returns 204

#### Scenario: Batch delete

- **WHEN** the client sends `DELETE /api/media` with a JSON body `{ paths: ["a.png", "b.png"] }` and valid auth
- **THEN** the API calls Supabase batch delete
- **AND** returns the count of deleted objects

### Requirement: AppState and MediaConfig carry SupabaseStorage

`AppState` SHALL include `media_config: Arc<MediaConfig>`. `MediaConfig` SHALL include a bucket-neutral `SupabaseStorage`, a single required `bucket: String`, and the existing `media_base_url`. `MediaConfig` SHALL NOT include a `default_bucket` field or a `bucket_override` field.

The construction site at `apps/api/src/bin/my-cms-api.rs` SHALL read the existing Supabase URL and credential environment variables, SHALL NOT reference an S3 environment variable, and SHALL NOT read a `SUPABASE_STORAGE_BUCKET` environment variable. It SHALL initialize `MediaConfig.bucket` from the literal `"media"`.

`SupabaseStorage` SHALL NOT contain a bucket field. `SupabaseStorage::new` SHALL construct connection and authentication state without receiving a bucket name. Object-operation methods SHALL receive their target bucket explicitly.

The application core SHALL NOT resolve an "effective bucket" inside command handlers. The target bucket SHALL be supplied to the command handler via the `MediaConfig` it receives. API handlers SHALL resolve the target bucket at the request boundary by selecting the validated `?bucket=` query value when present, and the inherited global `MediaConfig.bucket` otherwise, and SHALL construct a request-scoped `MediaConfig` whose `bucket` is that target.

#### Scenario: Boot with valid environment

- **WHEN** the API starts with the required Supabase URL and credential environment variables
- **THEN** `AppState.media_config.storage` is a bucket-neutral `SupabaseStorage`
- **AND** `AppState.media_config.bucket` is `"media"`
- **AND** the default is initialized from the application literal rather than an environment variable
- **AND** the API responds to health checks

#### Scenario: Boot without SUPABASE_STORAGE_BUCKET

- **WHEN** the API starts in an environment that does not set `SUPABASE_STORAGE_BUCKET`
- **THEN** the API boots successfully
- **AND** `AppState.media_config.bucket` is `"media"`
- **AND** no warning is emitted for the missing variable

#### Scenario: Request omits bucket override

- **WHEN** a media request does not supply `?bucket=`
- **THEN** the API builds a request-scoped `MediaConfig` whose `bucket` is the inherited global bucket `"media"`
- **AND** that scoped `MediaConfig` is passed to the invoked command handler
- **AND** the command handler invokes the `SupabaseStorage` object method with `bucket = "media"`

#### Scenario: Request supplies bucket override

- **WHEN** a media request supplies a valid `?bucket=avatars`
- **THEN** the API builds a request-scoped `MediaConfig` whose `bucket` is `"avatars"`
- **AND** the scoped `MediaConfig` is passed to the invoked command handler
- **AND** the command handler invokes the `SupabaseStorage` object method with `bucket = "avatars"`
- **AND** no clone or mutation of storage state is required to select the bucket

#### Scenario: Service role key is present

- **WHEN** `SUPABASE_SERVICE_ROLE_KEY` is configured
- **THEN** `SupabaseStorage::auth_key()` returns the service role key
- **AND** bucket selection does not alter authentication state

### Requirement: StorageError is exposed in AppError

`AppError` SHALL include a `StorageError(String)` variant. The `Display` impl SHALL render it as `Storage error: <msg>`, and the API response mapping SHALL translate it to a connection-error response with the original message.

#### Scenario: Error surfaces in API response

- **WHEN** a media handler returns `AppError::StorageError("Upload failed: 500")`
- **THEN** the API responds with the configured error code for `ConnectionError`
- **AND** the response body includes the original message

### Requirement: Public media delivery requires public bucket or admin scope

The system SHALL require that the public media endpoints
`GET /media/{*path}` and `GET /media/images/{*path}` (mounted on
`public_router()` in `apps/api/src/bin/my-cms-api.rs`) consult the
bucket's `public` flag (exposed by Supabase Storage and surfaced by
`SupabaseStorage::get_bucket(name) -> Result<Bucket, AppError>`) before
streaming the object bytes. The system SHALL resolve the bucket name
as follows:

- If the request carries no `?bucket=` query param, the bucket is the
  implicit default `"media"`.
- If the request carries `?bucket=<name>` where `<name>` matches the
  bucket-name regex `^[a-z][a-z0-9_-]{2,62}$`, the bucket is that
  name. Invalid names return HTTP 400 (unchanged behaviour).

The bucket's `public` flag SHALL be fetched via
`SupabaseStorage::get_bucket(name)` with the result cached in
`AppState.bucket_visibility_cache: Arc<Cache<String, bool>>` (moka,
TTL 300 s, capacity 256 entries).

The endpoint SHALL allow the request and stream the bytes when one of
the following is true:

- The bucket's `public` flag is `true`.
- The bucket's `public` flag is `false` AND the request carries an
  `Authorization: Bearer <jwt>` header whose decoded
  `app_metadata.roles` JSON array contains the string
  `"my-headless-cms-administrator"`. JWT decoding reuses the same
  `SUPABASE_JWT_SECRET` and `AUTHORIZATION_AUDIENCE` env vars as the
  existing `SupabaseAuthLayer`.

The endpoint SHALL return HTTP 404 with `AppError::NotFound` (response
body `{"error":"Not found"}`) when all of the following are true:

- The bucket's `public` flag is `false`.
- The request either has no `Authorization` header, OR carries a
  header that does not decode to a valid JWT, OR decodes to a JWT
  whose `app_metadata.roles` does not contain
  `"my-headless-cms-administrator"`.

The endpoint SHALL propagate `AppError::StorageError` and
`AppError::NotFound` from `SupabaseStorage::get_bucket` unchanged
(HTTP 502 / 404 respectively).

#### Scenario: Public bucket, anonymous request — served

- **WHEN** a request `GET /media/foo.png` (no auth header) reaches the endpoint
- **AND** `SupabaseStorage::get_bucket("media")` returns `Bucket { name: "media", public: true, ... }`
- **THEN** the API streams the bytes from `/storage/v1/object/public/media/foo.png`
- **AND** the response is HTTP 200 with the object's content type and `Cache-Control: public, max-age=31536000, immutable`

#### Scenario: Public bucket, override to non-default public bucket — served

- **WHEN** a request `GET /media/foo.png?bucket=avatars` (no auth header) reaches the endpoint
- **AND** `SupabaseStorage::get_bucket("avatars")` returns `Bucket { name: "avatars", public: true, ... }`
- **THEN** the API streams the bytes from `/storage/v1/object/public/avatars/foo.png`
- **AND** the response is HTTP 200

#### Scenario: Private bucket, anonymous request — rejected with 404

- **WHEN** a request `GET /media/secret.pdf?bucket=private-docs` (no auth header) reaches the endpoint
- **AND** `SupabaseStorage::get_bucket("private-docs")` returns `Bucket { name: "private-docs", public: false, ... }`
- **THEN** the API returns HTTP 404 with body `{"error":"Not found"}`
- **AND** no Supabase storage read is performed for the object bytes

#### Scenario: Private bucket, non-admin JWT — rejected with 404

- **WHEN** a request `GET /media/secret.pdf?bucket=private-docs` carries a valid JWT whose `app_metadata.roles` is `["my-headless-cms-writer"]`
- **AND** the bucket is `public: false`
- **THEN** the API returns HTTP 404 with body `{"error":"Not found"}`
- **AND** no Supabase storage read is performed for the object bytes

#### Scenario: Private bucket, administrator JWT — served

- **WHEN** a request `GET /media/secret.pdf?bucket=private-docs` carries a valid JWT whose `app_metadata.roles` is `["my-headless-cms-administrator"]`
- **AND** the bucket is `public: false`
- **THEN** the API streams the bytes from `/storage/v1/object/private-docs/secret.pdf` (using the service-role key)
- **AND** the response is HTTP 200

#### Scenario: Cache hit — no Supabase round-trip for repeat request

- **WHEN** a request `GET /media/foo.png?bucket=avatars` is made twice within 300 seconds
- **THEN** the first request calls `SupabaseStorage::get_bucket("avatars")` once and populates the cache
- **AND** the second request reads the cached `public: true` value and skips the Supabase round-trip

#### Scenario: Cache miss after TTL — fresh fetch

- **WHEN** the cache entry for a bucket name expires after 300 seconds
- **THEN** the next request for that bucket re-fetches `get_bucket(name)` and refreshes the cache

#### Scenario: Invalid bucket name — 400 (preserved)

- **WHEN** `?bucket=BadName` is supplied (uppercase)
- **THEN** the API returns HTTP 400 with `AppError::Validation("bucket", ...)` (unchanged behaviour)

#### Scenario: Bucket not found in Supabase — 404 (preserved)

- **WHEN** `?bucket=does-not-exist` is supplied and no such bucket exists
- **THEN** `SupabaseStorage::get_bucket("does-not-exist")` returns `AppError::NotFound`
- **AND** the API returns HTTP 404 (unchanged behaviour)

#### Scenario: Supabase storage unreachable — 502 (preserved)

- **WHEN** `SupabaseStorage::get_bucket(name)` returns `AppError::StorageError(...)` (network failure, 5xx, etc.)
- **THEN** the API returns HTTP 502 (preserved from `error_status()` mapping)

### Requirement: Read-side storage methods take bucket as parameter

Every bucket-targeting object method on `SupabaseStorage` SHALL receive `bucket: &str` as its first argument after `&self`.

The requirement applies to:

- `upload`
- `download`
- `get_info`
- `list_objects`
- `delete`
- `delete_batch`
- `public_url`
- `render_image_url`
- `download_render`

Each method SHALL construct its Supabase Storage URL from the supplied bucket argument. No object method SHALL read a bucket from `SupabaseStorage` state.

`download_render` SHALL forward its supplied bucket unchanged to `render_image_url`.

#### Scenario: Object operation targets explicit bucket

- **WHEN** a caller invokes an object method with `bucket = "avatars"`
- **THEN** the generated Supabase Storage request URL contains the `avatars` bucket segment
- **AND** no previously used or default bucket stored on the client can influence the request

#### Scenario: Same storage client targets multiple buckets

- **WHEN** the same `SupabaseStorage` instance is used first with bucket `"media"` and then with bucket `"avatars"`
- **THEN** the first operation targets `"media"`
- **AND** the second operation targets `"avatars"`
- **AND** no storage clone or mutation is required between the operations

#### Scenario: Render download preserves explicit bucket

- **WHEN** `download_render("avatars", path, width, height)` is invoked
- **THEN** it calls the render endpoint for bucket `"avatars"`
- **AND** width and height query behavior remains unchanged

### Requirement: Bucket-management storage methods select buckets explicitly

Bucket-management methods SHALL remain independent of object-operation defaults.

Every bucket-targeting bucket-management method SHALL receive the target explicitly:

- `get_bucket` receives `name: &str`.
- `create_bucket` receives the name in its payload.
- `update_bucket` receives `name: &str`.
- `empty_bucket` receives `name: &str`.
- `delete_bucket` receives `name: &str`.

`list_buckets` SHALL remain bucket-agnostic and SHALL not consult an object default.

No bucket-management method SHALL read a bucket from `SupabaseStorage` state.

#### Scenario: Get bucket uses explicit name

- **WHEN** a caller invokes `get_bucket("private-docs")`
- **THEN** the request targets `/storage/v1/bucket/private-docs`
- **AND** `MediaConfig.default_bucket` does not affect the operation

#### Scenario: Create bucket uses payload name

- **WHEN** a caller invokes `create_bucket` with a payload whose name is `"private-docs"`
- **THEN** the request creates `"private-docs"`
- **AND** no storage-client bucket state is consulted

#### Scenario: List buckets is independent of media default

- **WHEN** a caller invokes `list_buckets()`
- **THEN** the request lists all buckets
- **AND** changing `MediaConfig.default_bucket` would not alter the request

### Requirement: Object-operation response URLs preserve the bucket query when supplied

When a media object operation receives a validated `?bucket=<name>` query, the response URL it constructs SHALL include `?bucket=<name>` as a trailing query segment. When the request omits `?bucket=`, the response URL SHALL use the default URL form without a trailing query segment.

For create / list / metadata operations, this distinction is communicated by an `include_bucket_query: bool` parameter that the API handler passes alongside the request-scoped `MediaConfig`. The parameter SHALL be `true` if and only if the request supplied `?bucket=` and that value passed validation.

#### Scenario: Default response URL when ?bucket= is omitted

- **WHEN** a `POST /api/media` request omits `?bucket=`
- **THEN** the create handler is invoked with `include_bucket_query = false`
- **AND** the returned `url` is `${MEDIA_BASE_URL}/media/images/<path>` for image content types, or `${MEDIA_BASE_URL}/media/<path>` otherwise
- **AND** the URL contains no `?bucket=` segment

#### Scenario: Explicit-bucket response URL when ?bucket= is supplied

- **WHEN** a `POST /api/media?bucket=avatars` request is validated
- **THEN** the create handler is invoked with `include_bucket_query = true`
- **AND** the returned `url` is `${MEDIA_BASE_URL}/media/<path>?bucket=avatars`

#### Scenario: Default response URL for list when ?bucket= is omitted

- **WHEN** a `GET /api/media` request omits `?bucket=`
- **THEN** the list handler is invoked with `include_bucket_query = false`
- **AND** every entry's `url` is `${MEDIA_BASE_URL}/media/<name>` without a query segment

#### Scenario: Explicit-bucket response URL for list when ?bucket= is supplied

- **WHEN** a `GET /api/media?bucket=avatars` request is validated
- **THEN** the list handler is invoked with `include_bucket_query = true`
- **AND** every entry's `url` is `${MEDIA_BASE_URL}/media/<name>?bucket=avatars`

#### Scenario: Default response URL for metadata when ?bucket= is omitted

- **WHEN** a `GET /api/media/info/<path>` request omits `?bucket=`
- **THEN** the metadata handler is invoked with `include_bucket_query = false`
- **AND** the returned `url` is `${MEDIA_BASE_URL}/media/<path>` without a query segment

#### Scenario: Explicit-bucket response URL for metadata when ?bucket= is supplied

- **WHEN** a `GET /api/media/info/<path>?bucket=avatars` request is validated
- **THEN** the metadata handler is invoked with `include_bucket_query = true`
- **AND** the returned `url` is `${MEDIA_BASE_URL}/media/<path>?bucket=avatars`

### Requirement: Read-side media cache key is bucket-string typed

The `MediaCacheKey` carried by `moka::future::Cache` SHALL have a `bucket: String` field. `Option<String>` SHALL NOT be used as the bucket type for the cache key. The handler SHALL build the cache key by cloning `self.media_config.bucket` once per call and SHALL use the same value for both the cache key and the storage call.

#### Scenario: Cache key with two distinct bucket values are distinct

- **WHEN** the same path and dimensions are used with `bucket = "media"` and `bucket = "avatars"`
- **THEN** the two cache keys are distinct
- **AND** the cache returns the correct entry for each bucket

#### Scenario: Cache key with same bucket, path, and dimensions are equal

- **WHEN** the same bucket, path, and dimensions are used twice
- **THEN** the two cache keys are equal
- **AND** the second request hits the cache populated by the first

#### Scenario: Cache key with different dimensions are distinct

- **WHEN** the same bucket and path are used with different `width` or `height` values
- **THEN** the two cache keys are distinct

### Requirement: Reserved-name policy reads MediaConfig.bucket

Bucket create and bucket delete command handlers SHALL compare the requested bucket name against `self.media_config.bucket.as_str()` to detect the reserved application default. They SHALL NOT compare against a hard-coded constant.

The reserved-name error message SHALL interpolate `self.media_config.bucket` so the message reflects the configured default at runtime.

#### Scenario: Bucket create rejects the configured default

- **WHEN** a `POST /api/admin/buckets` request supplies the configured default bucket name
- **THEN** the create handler returns `AppError::Validation("name", "cannot use reserved bucket name '<configured>'")`
- **AND** no Supabase request is issued

#### Scenario: Bucket delete rejects the configured default

- **WHEN** a `DELETE /api/admin/buckets/<name>` request targets the configured default bucket name
- **THEN** the delete handler returns `AppError::Validation("name", "cannot delete reserved bucket name '<configured>'")`
- **AND** no Supabase request is issued

### Requirement: MediaConfig has no effective_bucket helper

The application core SHALL NOT export a free function named `effective_bucket`. Bucket resolution SHALL happen once, in the API layer, when constructing the request-scoped `MediaConfig`. Command handlers SHALL consume a `MediaConfig` whose `bucket` field is already the request target.

#### Scenario: effective_bucket is not referenced from production code

- **WHEN** the repository is searched for `effective_bucket`
- **THEN** no production code reference exists
- **AND** the helper definition is removed from `apps/api/application_core/src/commands/media/mod.rs`

#### Scenario: effective_bucket tests are removed

- **WHEN** `cargo test -p application_core commands::media` is run
- **THEN** no test named `effective_bucket_returns_*` is present
- **AND** no test references the removed helper

