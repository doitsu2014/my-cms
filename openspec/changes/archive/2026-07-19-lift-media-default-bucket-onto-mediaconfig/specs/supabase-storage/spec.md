## MODIFIED Requirements

### Requirement: AppState and MediaConfig carry SupabaseStorage

`AppState` SHALL include `media_config: Arc<MediaConfig>`. `MediaConfig` SHALL include a bucket-neutral `SupabaseStorage`, a required `default_bucket: String`, the existing `media_base_url`, and the existing optional `bucket_override`.

The construction site at `apps/api/src/bin/my-cms-api.rs` SHALL read the existing Supabase URL and credential environment variables, SHALL NOT reference an S3 environment variable, and SHALL NOT read a `SUPABASE_STORAGE_BUCKET` environment variable. It SHALL initialize `MediaConfig.default_bucket` from the literal `"media"`.

`SupabaseStorage` SHALL NOT contain a bucket field. `SupabaseStorage::new` SHALL construct connection and authentication state without receiving a bucket name. Object-operation methods SHALL receive their target bucket explicitly.

The system SHALL resolve an effective bucket by using the validated request override when present and `MediaConfig.default_bucket` otherwise.

#### Scenario: Boot with valid environment

- **WHEN** the API starts with the required Supabase URL and credential environment variables
- **THEN** `AppState.media_config.storage` is a bucket-neutral `SupabaseStorage`
- **AND** `AppState.media_config.default_bucket` is `"media"`
- **AND** the default is initialized from the application literal rather than an environment variable
- **AND** the API responds to health checks

#### Scenario: Boot without SUPABASE_STORAGE_BUCKET

- **WHEN** the API starts in an environment that does not set `SUPABASE_STORAGE_BUCKET`
- **THEN** the API boots successfully
- **AND** `AppState.media_config.default_bucket` is `"media"`
- **AND** no warning is emitted for the missing variable

#### Scenario: Request omits bucket override

- **WHEN** a media request does not supply `?bucket=`
- **THEN** the API uses `MediaConfig.default_bucket` as the target bucket name
- **AND** that bucket is passed explicitly to the invoked `SupabaseStorage` object method

#### Scenario: Request supplies bucket override

- **WHEN** a media request supplies a valid `?bucket=avatars`
- **THEN** the API uses `"avatars"` as the effective bucket
- **AND** passes `"avatars"` explicitly to the invoked `SupabaseStorage` object method
- **AND** does not clone or mutate storage state to select the bucket

#### Scenario: Service role key is present

- **WHEN** `SUPABASE_SERVICE_ROLE_KEY` is configured
- **THEN** `SupabaseStorage::auth_key()` returns the service role key
- **AND** bucket selection does not alter authentication state

## ADDED Requirements

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