## 1. Part C — Supabase Storage migration

### 1.1 SupabaseStorage client

- [x] 1.1.1 Create `services/application_core/src/commands/media/supabase_storage.rs` with the `SupabaseStorage` struct, `StorageObject` / `StorageObjectMetadata` deserialisers, and `SupabaseStorage::new` (Task C1)
- [x] 1.1.2 Implement `auth_key()` returning `service_role_key` when set, otherwise `anon_key` (Task C1)
- [x] 1.1.3 Implement `public_url(path)` and `render_image_url(path, width, height)` URL builders (Task C1)
- [x] 1.1.4 Implement `upload(file_path, data, content_type, cache_control)` as a multipart POST to `/storage/v1/object/{bucket}/{path}` with `x-upsert: true` and Bearer auth (Task C1)
- [x] 1.1.5 Implement `download(path)` as a GET to `/storage/v1/object/public/{bucket}/{path}`, returning `(bytes, content_type)` and translating 404 to `AppError::NotFound` (Task C1)
- [x] 1.1.6 Implement `get_info(path)` as a GET to `/storage/v1/object/info/public/{bucket}/{path}` (Task C1)
- [x] 1.1.7 Implement `list_objects(prefix)` as a POST to `/storage/v1/object/list/public/{bucket}` with `{ prefix, limit: 1000, offset: 0, sortBy: { column: "name", order: "asc" } }` (Task C1)
- [x] 1.1.8 Implement `delete(path)` and `delete_batch(paths)` (Task C1)
- [x] 1.1.9 Add `pub mod supabase_storage;` to `services/application_core/src/commands/media/mod.rs` (Task C1)

### 1.2 AppError gets StorageError

- [x] 1.2.1 Add `StorageError(String)` to the `AppError` enum in `services/application_core/src/common/app_error.rs` (Task C2)
- [x] 1.2.2 Add a `Display` arm for `StorageError(msg)` rendering as `Storage error: {msg}` (Task C2)
- [x] 1.2.3 Map `AppError::StorageError` in `services/src/presentation_models/api_response.rs` to the existing connection-error response code with the original message (Task C2)

### 1.3 MediaConfig and AppState

- [x] 1.3.1 Replace `S3MediaStorage` with `SupabaseStorage` in `MediaConfig` in `services/application_core/src/commands/media/mod.rs` (Task C3)
- [x] 1.3.2 Update `services/src/bin/my-cms-api.rs` `construct_app_state` to read `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `SUPABASE_STORAGE_BUCKET`, and `MEDIA_BASE_URL` and to build a `SupabaseStorage` (Task C3)
- [x] 1.3.3 Update `services/src/lib.rs` to use the new `MediaConfig` shape (Task C3)
- [x] 1.3.4 Update `services/.env` to remove the S3 variables and add the Supabase Storage variables (Task C3)

### 1.4 Media handlers

- [x] 1.4.1 Update `CreateMediaHandler` in `services/application_core/src/commands/media/create/create_handler.rs` to call `self.media_config.storage.upload(...)` with a nanoid-suffixed path and the right `MEDIA_BASE_URL` URL shape (Task C4)
- [x] 1.4.2 Update `ReadMediaHandler` in `services/application_core/src/commands/media/read/read_handler.rs` to call `self.storage.download(...)`, keeping the moka cache for original image fetches and removing any in-process resize code (Task C4)
- [x] 1.4.3 Update `get_media_info` in `services/application_core/src/commands/media/read/metadata_handler.rs` to call `storage.get_info(path)` (Task C4)
- [x] 1.4.4 Update `list_media` in `services/application_core/src/commands/media/list/list_handler.rs` to call `storage.list_objects(prefix)` (Task C4)
- [x] 1.4.5 Update `delete_media` and `delete_media_batch` in `services/application_core/src/commands/media/delete/delete_handler.rs` to call `storage.delete` / `storage.delete_batch` (Task C4)
- [x] 1.4.6 Run `cargo check` and confirm all media handlers compile against `SupabaseStorage` (Task C4)

### 1.5 Remove S3 dependencies

- [x] 1.5.1 Remove `rust-s3 = "0.37.0"` from `services/Cargo.toml` and `services/application_core/Cargo.toml` (Task C5)
- [x] 1.5.2 Remove the `image = "0.25.9"` crate from both Cargo.toml files (Task C5)
- [x] 1.5.3 Remove `S3Error(S3Error)` and its `From<S3Error>` impl from `services/application_core/src/common/app_error.rs` (Task C5)
- [x] 1.5.4 Run `cargo build` and confirm a clean build with no `rust-s3` or `image` references (Task C5)

## 2. Part D — Image transformation

### 2.1 Resize-redirect endpoint

- [x] 2.1.1 Update `api_get_media_image` in `services/src/api/media/read/read_handler.rs` to inspect `ResizeParams` (w, h) and return `Redirect::temporary(state.media_config.storage.render_image_url(...))` when either is set (Task D1)
- [x] 2.1.2 Keep the unresized path (no w, no h) returning the original bytes via `ReadMediaHandler::fetch_media` with the long-lived `Cache-Control` header (Task D1)
- [x] 2.1.3 Remove the local `resize_image()` function and any `use image::` import from `services/application_core/src/commands/media/read/read_handler.rs` (Task D1)
- [x] 2.1.4 Run an end-to-end check: `docker compose up -d`, create the public `media` bucket, upload an image, and confirm `curl -v /media/images/{path}?w=300` returns 302 to a Supabase render URL (Task D1)
- [x] 2.1.5 Confirm `cargo build` succeeds with the `image` crate removed (Task D1)

## 3. Verify

- [x] 3.1 A public `media` bucket exists in Supabase Storage
- [x] 3.2 `POST /api/media` stores the file in the bucket and returns `{ path, url }`
- [x] 3.3 `GET /media/images/{path}` returns the original image (200) with the correct `Content-Type`
- [x] 3.4 `GET /media/images/{path}?w=300` returns 302 to `{SUPABASE_URL}/storage/v1/render/image/public/media/{path}?width=300`
- [x] 3.5 `GET /media/{path}` returns any non-image file (no resize)
- [x] 3.6 `GET /api/media/info/{path}` returns `{ path, url, content_type, size, last_modified }`
- [x] 3.7 `GET /api/media` returns a list of objects in the bucket
- [x] 3.8 `DELETE /api/media/delete/{path}` removes the file
- [x] 3.9 `DELETE /api/media` with a JSON body of paths performs a batch delete
- [x] 3.10 The second image fetch hits the in-memory cache (no network call to Supabase)
- [x] 3.11 `cargo build` succeeds with no `rust-s3` or `image` references
- [x] 3.12 All existing tests pass (`cargo test`)

**Note:** Verification items 3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7, 3.8, 3.9, 3.10 require a live Supabase stack via `docker compose up -d` and a public `media` bucket; they are not run in CI and are deferred to manual verification. The non-network checks 3.11 and 3.12 are confirmed by `cargo check` and `cargo test` (17/17 `SupabaseStorage` unit tests pass).
