## Purpose

TBD — capability added by the `supabase-storage-and-image-transformation-migration` change.

## ADDED Requirements

### Requirement: Resize requests redirect to Supabase Image Transformation

The API endpoint `GET /media/images/{*path}` SHALL return HTTP 302 Temporary Redirect to `{SUPABASE_URL}/storage/v1/render/image/public/{bucket}/{path}?width={w}&height={h}` when the request includes `?w=` or `?h=` query parameters. The redirect URL is built by `SupabaseStorage::render_image_url(path, width, height)`.

#### Scenario: Resize with width only

- **WHEN** the client requests `GET /media/images/foo.webp?w=300`
- **THEN** the API returns HTTP 302
- **AND** the `Location` header points at `{SUPABASE_URL}/storage/v1/render/image/public/media/foo.webp?width=300`

#### Scenario: Resize with both dimensions

- **WHEN** the client requests `GET /media/images/foo.webp?w=300&h=200`
- **THEN** the API returns HTTP 302
- **AND** the `Location` header points at `{SUPABASE_URL}/storage/v1/render/image/public/media/foo.webp?width=300&height=200`

#### Scenario: Resize with height only

- **WHEN** the client requests `GET /media/images/foo.webp?h=200`
- **THEN** the API returns HTTP 302
- **AND** the `Location` header points at `{SUPABASE_URL}/storage/v1/render/image/public/media/foo.webp?height=200`

### Requirement: Unresized image requests serve the original

The API endpoint `GET /media/images/{*path}` SHALL serve the original image bytes (200 OK with the stored `Content-Type` and a long-lived `Cache-Control: public, max-age=31536000, immutable`) when no `w` or `h` query parameter is present.

#### Scenario: No resize params

- **WHEN** the client requests `GET /media/images/foo.webp` with no query string
- **THEN** the API returns HTTP 200 with the original image bytes
- **AND** the `Content-Type` header is `image/webp`
- **AND** the `Cache-Control` header is `public, max-age=31536000, immutable`

#### Scenario: Resize params explicitly absent (empty)

- **WHEN** the client requests `GET /media/images/foo.webp?w=&h=`
- **THEN** the API treats the request as an unresized request
- **AND** returns HTTP 200 with the original image bytes

### Requirement: Image resizing no longer runs on the API server

The `application_core/src/commands/media/read/read_handler.rs` module SHALL NOT call the `image` crate or any other CPU-bound image-processing library. All resizing SHALL be delegated to Supabase Image Transformation.

#### Scenario: `image` crate absent

- **WHEN** a developer greps `services/application_core/src` for `use image::`
- **THEN** no matches are found
- **AND** `services/application_core/Cargo.toml` does not list the `image` crate

#### Scenario: `image` crate absent in workspace

- **WHEN** `cargo build` is run with `image` removed from both `services/Cargo.toml` and `services/application_core/Cargo.toml`
- **THEN** the build succeeds
- **AND** the API serves `/media/images/{*path}` requests as documented

### Requirement: `render_image_url` builds a valid Supabase render URL

`SupabaseStorage::render_image_url(path, width, height)` SHALL return `{supabase_url}/storage/v1/render/image/public/{bucket}/{path}` followed by `?width={w}` when `width` is set, `&height={h}` when `height` is set, and `&` between the two when both are set. When neither is set, no query string is appended.

#### Scenario: Both dimensions

- **WHEN** `render_image_url("foo.webp", Some(300), Some(200))` is called
- **THEN** the URL ends with `?width=300&height=200`

#### Scenario: Only width

- **WHEN** `render_image_url("foo.webp", Some(300), None)` is called
- **THEN** the URL ends with `?width=300`

#### Scenario: No dimensions

- **WHEN** `render_image_url("foo.webp", None, None)` is called
- **THEN** the URL has no query string

### Requirement: Resize is compatible with the `media` bucket

Supabase Image Transformation SHALL be configured to read from the public `media` bucket (the same bucket used by `supabase-storage`). The local dev compose stack from `unified-docker-compose-with-supabase` runs the `storage` and `imgproxy` services that back this transformation.

#### Scenario: Local dev resize

- **WHEN** a developer starts the compose stack, creates a public `media` bucket, uploads an image, and visits `GET /media/images/foo.webp?w=300`
- **THEN** the API returns 302 to the Supabase render URL
- **AND** following the redirect returns a 300-px-wide image
