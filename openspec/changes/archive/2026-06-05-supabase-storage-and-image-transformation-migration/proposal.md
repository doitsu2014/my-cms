## Why

The my-cms media pipeline currently stores uploads on Contabo S3 (`sin1.contabostorage.com`) using the `rust-s3` crate and resizes images on the API server using the `image` crate. Both pieces add operational surface area (a separate S3 account, credentials, region) and CPU cost (the API does pixel work on every image request). The Supabase local stack already runs Supabase Storage and an imgproxy-backed image-transformation endpoint, so this change consolidates uploads, listing, metadata, and image resizing onto the same platform the rest of the stack uses.

## What Changes

- **Replace `S3MediaStorage` with `SupabaseStorage`.** A new `SupabaseStorage` struct in `application_core/src/commands/media/supabase_storage.rs` calls the Supabase Storage REST API (`/storage/v1/object/{bucket}/{path}`, `/storage/v1/object/list/public/{bucket}`, `/storage/v1/object/info/public/{bucket}/{path}`) over `reqwest` for upload, download, info, list, single delete, and batch delete. The auth header is `Bearer <service_role_key>` when set, falling back to `Bearer <anon_key>`.
- **Move image resizing out of the API and onto Supabase Image Transformation.** The `/media/images/{*path}` endpoint with `?w=` or `?h=` query params responds with `302 Temporary Redirect` to `{SUPABASE_URL}/storage/v1/render/image/public/{bucket}/{path}?width={w}&height={h}`. Requests without resize params still return the original file via Supabase Storage.
- **Update `AppError` to expose `StorageError(String)`.** Keep `S3Error(S3Error)` temporarily for any leftover call sites, then remove it in the cleanup task.
- **Update `AppState` and `MediaConfig` to carry `SupabaseStorage` instead of `S3MediaStorage`.** Read `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, and `SUPABASE_STORAGE_BUCKET` from env.
- **Remove the `rust-s3` and `image` Rust dependencies** and the S3 env vars from `services/.env`.

## Capabilities

### New Capabilities

- `supabase-storage`: media upload, download, metadata, list, and delete against Supabase Storage via the REST API, exposed to the rest of the backend through a `SupabaseStorage` client.
- `image-transformation`: dynamic image resizing delivered by Supabase Image Transformation — the API hands off `?w=&h=` requests to a Supabase render URL and serves the original image bytes for unresized requests.

### Modified Capabilities

<!-- None. The two capabilities are new; no existing capabilities have their requirements changed. -->

## Impact

- **New Rust file**: `services/application_core/src/commands/media/supabase_storage.rs`.
- **Modified Rust files**: `services/application_core/src/commands/media/mod.rs` (replace `MediaConfig.storage` with `SupabaseStorage`), `services/application_core/src/commands/media/{create,read,read/metadata,list,delete}/*_handler.rs`, `services/application_core/src/common/app_error.rs` (add `StorageError(String)`, later remove `S3Error`), `services/src/bin/my-cms-api.rs` (env-driven construction), `services/src/lib.rs` (`AppState.media_config`), `services/src/api/media/read/read_handler.rs` (redirect for resize), `services/src/presentation_models/api_response.rs` (map `StorageError`).
- **Cargo dependencies removed**: `rust-s3 = "0.37.0"`, `image = "0.25.9"`.
- **Env vars removed**: `S3_ENDPOINT`, `S3_BUCKET_NAME`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_DEFAULT_REGION`.
- **Env vars added**: `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY` (optional), `SUPABASE_STORAGE_BUCKET` (default `media`).
- **Bucket**: a public bucket named `media` is created in Supabase Storage on first use.
- **Local dev**: the Supabase Stack from `unified-docker-compose-with-supabase` runs the Storage API on `:5000` and imgproxy on `:5001`; the public URL is `http://localhost:8000`.
