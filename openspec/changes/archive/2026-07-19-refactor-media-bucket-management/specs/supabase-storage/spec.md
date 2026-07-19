## MODIFIED Requirements

### Requirement: AppState and MediaConfig carry SupabaseStorage

`AppState` SHALL include a `media_config: Arc<MediaConfig>` whose
`storage` field is a `SupabaseStorage`. The construction site
(`apps/api/src/bin/my-cms-api.rs`) SHALL read `SUPABASE_URL`,
`SUPABASE_ANON_KEY`, and `SUPABASE_SERVICE_ROLE_KEY` (optional) from
env, SHALL NOT reference any S3 env var, and SHALL NOT read any
`SUPABASE_STORAGE_BUCKET` env var. The default bucket name SHALL be
the hard-coded string literal `"media"`, passed to
`SupabaseStorage::new()`. Per-request bucket overrides SHALL be
applied exclusively through `SupabaseStorage::with_bucket(name)` at
the API-handler call site.

#### Scenario: Boot with valid env (default bucket is "media")

- **WHEN** the API starts with `SUPABASE_URL=http://localhost:8000` and `SUPABASE_ANON_KEY=<key>`
- **THEN** `AppState.media_config.storage` is a `SupabaseStorage` configured with the bucket `"media"` (from the literal, not from env)
- **AND** the API responds to health checks

#### Scenario: Boot without SUPABASE_STORAGE_BUCKET env var

- **WHEN** the API starts in an environment that does NOT set `SUPABASE_STORAGE_BUCKET`
- **THEN** the API boots successfully and the storage default bucket is `"media"`
- **AND** no warning is emitted for the missing variable

#### Scenario: Service role key present

- **WHEN** `SUPABASE_SERVICE_ROLE_KEY` is also set
- **THEN** `SupabaseStorage::auth_key()` returns the service role key
- **AND** server-side operations bypass RLS

## ADDED Requirements

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
