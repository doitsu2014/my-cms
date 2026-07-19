# media-bucket-management Specification

## Purpose

The media feature currently treats Supabase Storage as a single fixed bucket (`media`) configured at boot. This capability gives administrators first-class lifecycle control over buckets — list, inspect, create, update, empty, and delete — and adds a per-request `?bucket=<name>` override on existing object operations so any bucket can be browsed and managed through the same admin UI. The default-bucket semantics on every existing endpoint are preserved when no override is supplied.

## ADDED Requirements

### Requirement: List all buckets

The system SHALL expose `GET /api/media/buckets` that returns every bucket in the project's Supabase Storage. The endpoint SHALL be gated by the `my-headless-cms-administrator` role. The implementation SHALL call `GET {SUPABASE_URL}/storage/v1/bucket` with `Authorization: Bearer <service_role_key>` and SHALL return the array of `Bucket` objects from the Supabase response.

A `Bucket` object SHALL contain at minimum: `id: string`, `name: string`, `public: boolean`, `fileSizeLimit: number | null`, `allowedMimeTypes: string[] | null`, `createdAt: string` (ISO 8601), `updatedAt: string` (ISO 8601), `owner: string | null`, `type: string` (e.g. `"STANDARD"`).

#### Scenario: Successful list

- **WHEN** an authenticated administrator calls `GET /api/media/buckets` and Supabase Storage responds with `200` and a JSON array of bucket objects
- **THEN** the API returns HTTP 200 with `{ data: Bucket[] }`

#### Scenario: List when no service role key is configured

- **WHEN** `SupabaseStorage::auth_key()` would return the anon key (no service role set)
- **THEN** the Supabase Storage list call returns `401` or `403`
- **AND** the API returns HTTP 500 with `AppError::StorageError(...)` (the operation cannot proceed without the service role key)

### Requirement: Get bucket details

The system SHALL expose `GET /api/media/buckets/{name}` that returns a single `Bucket` object. The `{name}` path parameter SHALL be the bucket name. The endpoint SHALL be gated by the `my-headless-cms-administrator` role and SHALL call `GET {SUPABASE_URL}/storage/v1/bucket/{name}`.

#### Scenario: Successful get

- **WHEN** an authenticated administrator calls `GET /api/media/buckets/media` and the bucket exists
- **THEN** the API returns HTTP 200 with `{ data: Bucket }` for the `media` bucket

#### Scenario: Bucket not found

- **WHEN** an authenticated administrator calls `GET /api/media/buckets/{name}` and Supabase Storage responds with `404`
- **THEN** the API returns HTTP 404 with `AppError::NotFound`

### Requirement: Create bucket with validation

The system SHALL expose `POST /api/media/buckets` that creates a new bucket. The request body SHALL be a JSON object with the fields `name: string`, `public: boolean`, `fileSizeLimit?: number`, `allowedMimeTypes?: string[]`. The endpoint SHALL be gated by the `my-headless-cms-administrator` role and SHALL call `POST {SUPABASE_URL}/storage/v1/bucket` with the same body, adding `Authorization: Bearer <service_role_key>`.

The `name` field SHALL match the pattern `^[a-z][a-z0-9_-]{2,62}$` — lowercase letters, digits, hyphens, and underscores only; must start with a lowercase letter; length 3 to 63 characters. The system SHALL reject any name that does not match with HTTP 400 and an `AppError::Validation("name", ...)` variant.

When the request body omits `public`, the system SHALL default to `public: false` (admin must opt into public explicitly). The response SHALL be HTTP 201 with `{ data: Bucket }`.

#### Scenario: Successful create

- **WHEN** the request body is `{ "name": "private-docs", "public": false, "fileSizeLimit": 5242880, "allowedMimeTypes": ["application/pdf"] }` and the name passes validation
- **THEN** the API issues `POST {SUPABASE_URL}/storage/v1/bucket` with the same body and Bearer auth
- **AND** the API returns HTTP 201 with `{ data: Bucket }` for the new bucket

#### Scenario: Default public is false

- **WHEN** the request body is `{ "name": "private-docs" }` (no `public` field)
- **THEN** the Supabase request body includes `"public": false`
- **AND** the created bucket is private

#### Scenario: Reject name starting with a digit

- **WHEN** the request body's `name` is `"3d-models"`
- **THEN** the API returns HTTP 400 with `AppError::Validation("name", "must start with a lowercase letter")`
- **AND** no bucket is created in Supabase

#### Scenario: Reject name with uppercase letters

- **WHEN** the request body's `name` is `"MyBucket"`
- **THEN** the API returns HTTP 400 with `AppError::Validation("name", "...")`
- **AND** no bucket is created in Supabase

#### Scenario: Reject name shorter than 3 characters

- **WHEN** the request body's `name` is `"ab"`
- **THEN** the API returns HTTP 400 with `AppError::Validation("name", "...")`
- **AND** no bucket is created in Supabase

#### Scenario: Reject name longer than 63 characters

- **WHEN** the request body's `name` is 64 lowercase alphanumeric characters
- **THEN** the API returns HTTP 400 with `AppError::Validation("name", "...")`
- **AND** no bucket is created in Supabase

#### Scenario: Reject reserved name

- **WHEN** the request body's `name` is `"media"` (the configured default bucket)
- **THEN** the API returns HTTP 400 with `AppError::Validation("name", "cannot use reserved bucket name 'media'")`

#### Scenario: Bucket already exists

- **WHEN** the request body is valid and a bucket with that name already exists in Supabase
- **THEN** Supabase returns `409`
- **AND** the API returns HTTP 409 with `AppError::Conflict("Bucket '<name>' already exists")`

### Requirement: Update bucket configuration

The system SHALL expose `PUT /api/media/buckets/{name}` that updates the mutable fields of an existing bucket. The request body SHALL be a JSON object with any subset of `public?: boolean`, `fileSizeLimit?: number | null`, `allowedMimeTypes?: string[] | null`. At least one field SHALL be present. The endpoint SHALL be gated by the `my-headless-cms-administrator` role and SHALL call `POST {SUPABASE_URL}/storage/v1/bucket/{name}` with the same body (Supabase uses POST for bucket updates).

Fields not present in the request body SHALL be left unchanged. Setting `fileSizeLimit: null` or `allowedMimeTypes: null` SHALL remove the respective constraint on the bucket. The `{name}` path parameter SHALL be validated against the same name pattern as create (`^[a-z][a-z0-9_-]{2,62}$`); an invalid name returns HTTP 400.

#### Scenario: Toggle public flag

- **WHEN** an authenticated administrator sends `PUT /api/media/buckets/private-docs` with body `{ "public": true }`
- **THEN** the API issues `POST {SUPABASE_URL}/storage/v1/bucket/private-docs` with `{ "public": true }`
- **AND** the API returns HTTP 200 with `{ data: Bucket }` showing `public: true`

#### Scenario: Update file size limit

- **WHEN** an authenticated administrator sends `PUT /api/media/buckets/media` with body `{ "fileSizeLimit": 10485760 }`
- **THEN** the bucket's `file_size_limit` is updated to 10485760 bytes
- **AND** the API returns HTTP 200 with `{ data: Bucket }`

#### Scenario: Update allowed MIME types

- **WHEN** an authenticated administrator sends `PUT /api/media/buckets/media` with body `{ "allowedMimeTypes": ["image/png", "image/jpeg"] }`
- **THEN** the bucket's `allowed_mime_types` is updated to the provided list
- **AND** the API returns HTTP 200

#### Scenario: Empty body rejected

- **WHEN** the request body is `{}` (no fields to update)
- **THEN** the API returns HTTP 400 with `AppError::Validation("body", "at least one field must be present")`

#### Scenario: Update non-existent bucket

- **WHEN** the path `{name}` does not exist in Supabase Storage
- **THEN** Supabase returns `404`
- **AND** the API returns HTTP 404 with `AppError::NotFound`

### Requirement: Delete bucket with purge semantics

The system SHALL expose `DELETE /api/media/buckets/{name}` that deletes a bucket. The endpoint SHALL accept an optional query parameter `purge: boolean`. The endpoint SHALL be gated by the `my-headless-cms-administrator` role and SHALL call `DELETE {SUPABASE_URL}/storage/v1/bucket/{name}` with a body of `{}` (when `purge=false` or omitted) or `{"purge": true}` (when `purge=true`).

When `purge=false` (or omitted) and the bucket contains objects, Supabase returns `400`; the system SHALL translate that response to HTTP 409 with `AppError::Conflict("Bucket '<name>' is not empty; pass ?purge=true to delete with all objects")`. When `purge=true`, the system SHALL include `{"purge": true}` in the DELETE body so Supabase empties the bucket atomically before deletion.

The `{name}` path parameter SHALL be validated against `^[a-z][a-z0-9_-]{2,62}$`. Reserved name `"media"` SHALL be rejected with HTTP 400 to prevent accidental deletion of the default bucket.

#### Scenario: Delete empty bucket without purge

- **WHEN** an authenticated administrator sends `DELETE /api/media/buckets/old-test` and the bucket is empty
- **THEN** the API issues `DELETE {SUPABASE_URL}/storage/v1/bucket/old-test` with body `{}`
- **AND** the API returns HTTP 200 with `{ data: { message: "Bucket deleted" } }`

#### Scenario: Delete non-empty bucket without purge returns 409

- **WHEN** an authenticated administrator sends `DELETE /api/media/buckets/media?purge=false` and the bucket contains objects
- **THEN** the API issues the delete with body `{}`
- **AND** Supabase returns `400`
- **AND** the API returns HTTP 409 with `AppError::Conflict("Bucket 'media' is not empty; pass ?purge=true to delete with all objects")`

#### Scenario: Delete non-empty bucket with purge

- **WHEN** an authenticated administrator sends `DELETE /api/media/buckets/old-test?purge=true` and the bucket contains objects
- **THEN** the API issues `DELETE {SUPABASE_URL}/storage/v1/bucket/old-test` with body `{"purge": true}`
- **AND** Supabase empties the bucket and deletes it atomically
- **AND** the API returns HTTP 200

#### Scenario: Delete non-existent bucket

- **WHEN** an authenticated administrator sends `DELETE /api/media/buckets/{name}` and no bucket with that name exists
- **THEN** Supabase returns `404`
- **AND** the API returns HTTP 404 with `AppError::NotFound`

#### Scenario: Reject deletion of reserved name

- **WHEN** an authenticated administrator sends `DELETE /api/media/buckets/media`
- **THEN** the API returns HTTP 400 with `AppError::Validation("name", "cannot delete reserved bucket name 'media'")`
- **AND** no delete request is issued to Supabase

### Requirement: Empty a bucket without deleting it

The system SHALL expose `POST /api/media/buckets/{name}/empty` that removes all objects from a bucket while keeping the bucket itself. The endpoint SHALL be gated by the `my-headless-cms-administrator` role and SHALL call `POST {SUPABASE_URL}/storage/v1/bucket/{name}/empty` with Bearer auth.

The `{name}` path parameter SHALL be validated against `^[a-z][a-z0-9_-]{2,62}$`. On success the response SHALL be HTTP 200 with `{ data: { message: "Bucket '<name>' emptied" } }`. If the bucket does not exist, Supabase returns `404` and the API returns HTTP 404 with `AppError::NotFound`.

#### Scenario: Empty a bucket with objects

- **WHEN** an authenticated administrator sends `POST /api/media/buckets/media/empty` and the bucket contains objects
- **THEN** the API issues `POST {SUPABASE_URL}/storage/v1/bucket/media/empty`
- **AND** all objects are removed from the bucket
- **AND** the API returns HTTP 200

#### Scenario: Empty an already-empty bucket

- **WHEN** an authenticated administrator sends `POST /api/media/buckets/{name}/empty` and the bucket is empty
- **THEN** the API issues the empty call
- **AND** the API returns HTTP 200 (idempotent)

#### Scenario: Empty non-existent bucket

- **WHEN** an authenticated administrator sends `POST /api/media/buckets/{name}/empty` and no bucket with that name exists
- **THEN** Supabase returns `404`
- **AND** the API returns HTTP 404 with `AppError::NotFound`

### Requirement: Per-request bucket override on object operations

The existing object-operation endpoints (`GET /api/media`, `POST /api/media`, `GET /api/media/info/{*path}`, `DELETE /api/media/delete/{*path}`, `DELETE /api/media`) SHALL accept an optional query parameter `bucket: string`. When `bucket` is present, the operation SHALL target that bucket instead of the boot-configured default (`media`); when `bucket` is absent, behavior is unchanged.

The `bucket` value SHALL be validated against `^[a-z][a-z0-9_-]{2,62}$`. An invalid value SHALL cause HTTP 400 with `AppError::Validation("bucket", ...)`. A valid but non-existent bucket SHALL cause the Supabase call to return `404`, which the API surfaces as HTTP 404 with `AppError::NotFound`.

The response shape for object operations SHALL be unchanged. When a `bucket` override is in effect, the `url` field of the response SHALL be constructed as `{MEDIA_BASE_URL}/media/{path}?bucket={bucket}` — the API public-read endpoint, which proxies the request through the API with the service-role key so the returned URL is browser-accessible for the target bucket. When no override is in effect, the existing `{MEDIA_BASE_URL}/media/{path}` shape is preserved (for images: `{MEDIA_BASE_URL}/media/images/{path}`).

The public read endpoint `/media/{*path}` (and `/media/images/{*path}`) SHALL NOT honor the override; it always serves from the default `media` bucket. The image-resize redirect endpoint SHALL NOT honor the override; it always redirects to the Supabase render URL for the default `media` bucket.

#### Scenario: List objects in a non-default bucket

- **WHEN** an authenticated user calls `GET /api/media?bucket=private-docs`
- **THEN** the API calls `POST {SUPABASE_URL}/storage/v1/object/list/private-docs` with the standard list body
- **AND** the API returns the objects from `private-docs`
- **AND** each `url` field is `{MEDIA_BASE_URL}/media/{path}?bucket=private-docs`

#### Scenario: Upload to a non-default bucket

- **WHEN** an authenticated user calls `POST /api/media?bucket=avatars` with multipart body
- **THEN** the API issues `POST {SUPABASE_URL}/storage/v1/object/avatars/{slugified-path}` with `x-upsert: true`
- **AND** the response `url` is `{MEDIA_BASE_URL}/media/{slugified-path}?bucket=avatars`

#### Scenario: Delete a single object from a non-default bucket

- **WHEN** an authenticated user calls `DELETE /api/media/delete/foo.png?bucket=avatars`
- **THEN** the API issues `DELETE {SUPABASE_URL}/storage/v1/object/avatars/foo.png`
- **AND** the API returns HTTP 200

#### Scenario: Batch delete from a non-default bucket

- **WHEN** an authenticated user calls `DELETE /api/media?bucket=avatars` with JSON body `{ paths: ["a.png", "b.png"] }`
- **THEN** the API issues `DELETE {SUPABASE_URL}/storage/v1/object/avatars/delete` with the paths body
- **AND** the API returns the deleted count

#### Scenario: Object metadata for a non-default bucket

- **WHEN** an authenticated user calls `GET /api/media/info/foo.png?bucket=avatars`
- **THEN** the API calls `GET {SUPABASE_URL}/storage/v1/object/info/avatars/foo.png`
- **AND** the response contains the metadata for `avatars/foo.png`

#### Scenario: Override with invalid bucket name

- **WHEN** the `bucket` query parameter is `"BadName"` (uppercase)
- **THEN** the API returns HTTP 400 with `AppError::Validation("bucket", "must start with a lowercase letter; only [a-z0-9_-] allowed")`

#### Scenario: Override with non-existent bucket

- **WHEN** the `bucket` query parameter is `"does-not-exist"` and no such bucket exists
- **THEN** the Supabase list/upload/delete call returns `404`
- **AND** the API returns HTTP 404 with `AppError::NotFound`

#### Scenario: No override preserves existing behavior

- **WHEN** an authenticated user calls `GET /api/media` (no `bucket` param)
- **THEN** the operation targets the boot-configured default bucket (typically `media`)
- **AND** each `url` field is `{MEDIA_BASE_URL}/media/{path}` (existing shape)

### Requirement: Service-role key required for bucket operations

Every endpoint under `/api/media/buckets/...` SHALL require a valid `SUPABASE_SERVICE_ROLE_KEY` to be configured on the API server. The system SHALL validate at startup that `SUPABASE_SERVICE_ROLE_KEY` is set; if it is not, the existing `.expect()` panic in `construct_app_state()` SHALL be preserved (the API cannot boot without it for media operations). When the key is present, the system SHALL emit a single `tracing::info!` log line at boot confirming the service role key is configured, so operators can verify in the log stream that bucket management will work.

Every call to a bucket endpoint SHALL be authenticated against Supabase with the service role key, never with the anon key. If `SupabaseStorage::auth_key()` returns the anon key, every bucket call SHALL fail with HTTP 500 and `AppError::StorageError(...)` — the anon key is not authorized to manage buckets.

#### Scenario: Service role key present at boot

- **WHEN** the API boots with `SUPABASE_SERVICE_ROLE_KEY` set in the environment
- **THEN** `SupabaseStorage::auth_key()` returns the service role key
- **AND** the boot log includes `info!("Supabase service role key configured; bucket management endpoints enabled")`

#### Scenario: Bucket endpoint called when anon key would be used

- **WHEN** the API was started without `SUPABASE_SERVICE_ROLE_KEY` (which currently causes a panic at boot)
- **AND** a bucket endpoint would have been called
- **THEN** the call cannot happen because the API did not boot
- **WHEN** instead, the API is forced to use the anon key for bucket calls
- **THEN** Supabase returns `401` or `403`
- **AND** the API surfaces the failure as `AppError::StorageError(...)` with HTTP 500

### Requirement: Admin UI buckets page

The React admin SHALL expose `/admin/media/buckets` as a new page under the admin layout. The page SHALL list all buckets returned by `GET /api/media/buckets` in a table with columns: `Name`, `Public` (badge), `File Size Limit` (human-readable or `—`), `Allowed MIME Types` (truncated to first 3 + `+N more`), `Created`, `Actions`.

The page SHALL provide:
- A "New Bucket" button that opens a creation modal with fields: `name`, `public` (toggle, default `false`), `fileSizeLimit` (optional number, bytes), `allowedMimeTypes` (optional comma-separated text → array).
- A per-row "Edit" button that opens an edit modal pre-filled with the bucket's current mutable fields. The `name` field SHALL be read-only in edit mode.
- A per-row "Empty" button that opens a confirmation modal (`"This will permanently delete all objects in '<name>'. Continue?"`) and on confirm calls `POST /api/media/buckets/{name}/empty`.
- A per-row "Delete" button that opens a confirmation modal. If the bucket is non-empty, the modal SHALL include a "Force delete with all objects" checkbox that maps to `?purge=true`. On confirm, the page calls `DELETE /api/media/buckets/{name}?purge=<purge>`.

The page SHALL be accessible only to authenticated administrators. The route in `apps/web/src/App.tsx` SHALL be:

```tsx
<Route path="/admin/media/buckets" element={<ProtectedRoute><AdminOnlyRoute><AdminBucketsPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
```

#### Scenario: List buckets

- **WHEN** an administrator navigates to `/admin/media/buckets`
- **THEN** the page calls `GET /api/media/buckets`
- **AND** renders the bucket list with Public badges, file size limits, and allowed MIME types
- **AND** shows a skeleton loader while the request is in flight
- **AND** shows an empty state with a "Create your first bucket" CTA if the list is empty

#### Scenario: Create a private bucket

- **WHEN** the administrator fills the create modal with `name="private-docs"`, `public=false`, `fileSizeLimit=5242880`, `allowedMimeTypes="application/pdf,application/msword"`
- **AND** submits
- **THEN** the page calls `POST /api/media/buckets` with the JSON body
- **AND** on success, shows a success toast and refreshes the list

#### Scenario: Client-side validation blocks invalid name

- **WHEN** the administrator enters `name="3d-models"` in the create modal
- **THEN** the Zod schema rejects the input before the API call
- **AND** an inline error message is shown under the `name` field

#### Scenario: Edit a bucket's public flag

- **WHEN** the administrator clicks Edit on `private-docs` and toggles `public` to `true`
- **AND** submits
- **THEN** the page calls `PUT /api/media/buckets/private-docs` with `{ "public": true }`
- **AND** on success, shows a success toast and refreshes the list

#### Scenario: Empty a bucket

- **WHEN** the administrator clicks Empty on a non-empty bucket and confirms
- **THEN** the page calls `POST /api/media/buckets/{name}/empty`
- **AND** on success, shows a success toast and refreshes the list

#### Scenario: Delete an empty bucket

- **WHEN** the administrator clicks Delete on an empty bucket and confirms
- **THEN** the page calls `DELETE /api/media/buckets/{name}?purge=false`
- **AND** on success, shows a success toast and refreshes the list

#### Scenario: Delete a non-empty bucket without purge

- **WHEN** the administrator clicks Delete on a non-empty bucket and confirms without checking "Force delete with all objects"
- **THEN** the page calls `DELETE /api/media/buckets/{name}?purge=false`
- **AND** the API returns `409`
- **AND** the page shows the API error message in a toast: `"Bucket is not empty; pass ?purge=true to delete with all objects"`

### Requirement: Bucket selector on the media browser

The existing admin page `apps/web/src/app/admin/media/page.tsx` SHALL gain a "Bucket" dropdown selector in the header row, next to the existing "Refresh" and "Upload" buttons. The dropdown SHALL be populated from `GET /api/media/buckets` on page mount.

The currently selected bucket SHALL persist in the URL as `?bucket=<name>`. On page load, the page SHALL read the `bucket` query param; if present and valid, all object API calls (`GET /api/media`, `POST /api/media`, `DELETE /api/media`, `GET /api/media/info/{*path}`, `DELETE /api/media/delete/{*path}`) SHALL include `?bucket=<name>`. If the param is absent, the default behavior is preserved (no `bucket` param sent, targeting the boot-configured default).

When the administrator changes the dropdown selection, the page SHALL update the URL via `useSearchParams` and refetch the media list. A "Buckets" link in the header SHALL navigate to `/admin/media/buckets`.

The dropdown SHALL be hidden if only one bucket is returned (the default case where only `media` exists). It SHALL appear if two or more buckets are present.

#### Scenario: Default bucket selected

- **WHEN** an administrator visits `/admin/media` (no `bucket` param) and only the `media` bucket exists
- **THEN** the dropdown is hidden
- **AND** the existing behavior is preserved (no `bucket` param sent to the API)

#### Scenario: Non-default bucket selected via URL

- **WHEN** an administrator visits `/admin/media?bucket=avatars` and the `avatars` bucket exists
- **THEN** the dropdown shows `avatars` as the selected value
- **AND** the media list call is `GET /api/media?bucket=avatars`
- **AND** the displayed files come from the `avatars` bucket

#### Scenario: Switching buckets via dropdown

- **WHEN** the administrator selects `private-docs` from the dropdown while on the media page
- **THEN** the URL updates to `/admin/media?bucket=private-docs`
- **AND** the media list refetches and shows objects from `private-docs`

#### Scenario: Multiple buckets present

- **WHEN** the bucket list call returns two or more buckets
- **THEN** the dropdown is rendered with all bucket names as options
- **AND** the default selection is the URL param value, or the default `media` bucket if no param is present

#### Scenario: Non-existent bucket in URL

- **WHEN** the URL is `/admin/media?bucket=does-not-exist`
- **THEN** the dropdown shows `does-not-exist` as the selected value (the page does not pre-validate)
- **AND** the media list call returns `404`
- **AND** the page shows an empty state with a "Bucket not found" message