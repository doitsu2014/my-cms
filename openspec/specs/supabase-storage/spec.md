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

`AppState` SHALL include a `media_config: Arc<MediaConfig>` whose `storage` field is a `SupabaseStorage`. The construction site (`services/src/bin/my-cms-api.rs`) SHALL read `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY` (optional), and `SUPABASE_STORAGE_BUCKET` (default `media`) from env and SHALL NOT reference any S3 env var.

#### Scenario: Boot with valid env

- **WHEN** the API starts with `SUPABASE_URL=http://localhost:8000` and `SUPABASE_ANON_KEY=<key>`
- **THEN** `AppState.media_config.storage` is a `SupabaseStorage` configured with the bucket `media`
- **AND** the API responds to health checks

#### Scenario: Service role key present

- **WHEN** `SUPABASE_SERVICE_ROLE_KEY` is also set
- **THEN** `SupabaseStorage::auth_key()` returns the service role key
- **AND** server-side operations bypass RLS

### Requirement: StorageError is exposed in AppError

`AppError` SHALL include a `StorageError(String)` variant. The `Display` impl SHALL render it as `Storage error: <msg>`, and the API response mapping SHALL translate it to a connection-error response with the original message.

#### Scenario: Error surfaces in API response

- **WHEN** a media handler returns `AppError::StorageError("Upload failed: 500")`
- **THEN** the API responds with the configured error code for `ConnectionError`
- **AND** the response body includes the original message

