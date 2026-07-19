# supabase-storage Specification (Delta)

## MODIFIED Requirements

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

## ADDED Requirements

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
