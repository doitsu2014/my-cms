## Context

The my-cms media pipeline is currently split across three different systems:

1. **S3 (Contabo)** at `https://sin1.contabostorage.com` hosts every uploaded file. The Rust API uses `rust-s3 = "0.37.0"` with credentials in `services/.env` (`S3_ENDPOINT`, `S3_BUCKET_NAME`, `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`).
2. **The `image` crate** runs on the API server. When the frontend requests a thumbnail, the `ReadMediaHandler` decodes the bytes, resizes, re-encodes, and streams the result back — expensive CPU work for every image fetch.
3. **The API returns the resized bytes directly** to the client. There is no CDN; resize parameters flow through the API for every image.

The Supabase local stack delivered by `unified-docker-compose-with-supabase` already runs the Storage API and the imgproxy-backed image-transformation endpoint, so the migration is a pure backend refactor that can be developed against the running stack.

## Goals / Non-Goals

**Goals:**

- Replace the S3 client with a `SupabaseStorage` client that calls the Supabase Storage REST API for upload, download, info, list, single delete, and batch delete.
- Move image resizing out of the API and onto Supabase Image Transformation. The API only serves original bytes; resize requests are answered with a `302` to the Supabase render URL.
- Replace `rust-s3` and `image` Rust dependencies with `reqwest` (already a dep) and a thin `SupabaseStorage` struct.
- Keep the API surface (`POST /api/media`, `GET /media/images/{*path}`, `GET /media/{*path}`, `GET /media/info/{*path}`, `GET /media`, `DELETE /media/delete/{*path}`, `DELETE /media`) unchanged so the React admin does not need a UI change.

**Non-Goals:**

- Migrating existing S3-hosted media into Supabase Storage. The legacy `S3Error` is kept temporarily in `AppError` to ease the cutover; once the new code path is verified, `S3Error` is removed in the cleanup task.
- Adding signed-URL or private-bucket support. The `media` bucket is public.
- Replacing the in-memory moka cache for original image fetches. The cache stays.
- Frontend changes — the API surface and response shape are unchanged.

## Decisions

### Decision 1: A single `SupabaseStorage` client, not a trait-based abstraction

`SupabaseStorage` is a struct with concrete methods (`upload`, `download`, `get_info`, `list_objects`, `delete`, `delete_batch`, `public_url`, `render_image_url`). It is wrapped in `Arc` and held by `MediaConfig`. There is no `MediaStorage` trait and no second implementation.

**Rationale:** YAGNI. There is one storage backend, one set of call sites, and no sign that a second backend is on the horizon. A trait would be cargo-culted abstraction. Alternatives considered: a `MediaStorage` trait with `S3Storage` and `SupabaseStorage` impls — rejected, the cleanup task explicitly removes S3; a `dyn MediaStorage` indirection — rejected, it adds an Arc clone and a v-table for no current benefit.

### Decision 2: Use the `service_role` key for server-side calls, fall back to `anon_key`

`SupabaseStorage::auth_key()` returns `service_role_key` when set, otherwise `anon_key`. Every server-side request — upload, list, info, delete — uses this key.

**Rationale:** the `service_role` key bypasses Row-Level Security and lets the API manage the bucket programmatically. The `anon_key` is a safe fallback for environments that do not provision `service_role_key`. Alternatives considered: a per-user JWT forwarded from the request — rejected, it would couple media operations to the calling user's RLS policies, which is wrong for a CMS where the service manages the bucket on the user's behalf.

### Decision 3: Image resize is a 302 redirect, not a server-side render

`api_get_media_image` inspects the `w` and `h` query params. If either is present, it returns `Redirect::temporary(state.media_config.storage.render_image_url(&path, w, h))`. If neither is present, it serves the original bytes via `ReadMediaHandler::fetch_media`.

**Rationale:** Supabase Image Transformation runs on a separate imgproxy instance that is already configured in the Supabase compose file. Doing the resize in the API would duplicate that work and add CPU pressure on the Rust binary. A 302 is the right call: the client pays a single round-trip to the CDN-fronted Supabase Storage, then caches the result. Alternatives considered: a server-side proxy that streams the resized bytes — rejected, it removes the CDN's edge cache and ties the API to imgproxy's response semantics.

### Decision 4: Keep the in-memory moka cache for original image fetches

`ReadMediaHandler` keeps a `Cache<MediaCacheKey, CachedMedia>` (capacity 500, TTL 1 h, idle 30 min). The cache key is `(path, width, height)`. Resize requests now have `width = None, height = None` for the unresized path; the cache hit rate for original bytes is preserved.

**Rationale:** the cache smooths repeat requests for the same original file, which is the common case. The 302 redirects bypass the cache by design — Supabase Storage handles its own CDN/edge cache for resize. Alternatives considered: drop the cache — rejected, every repeat request would hit the network; cache the render URLs — rejected, the redirect is already cheap (a hashmap lookup + a 302 response).

### Decision 5: `AppError::StorageError(String)` alongside the existing `S3Error`

During the cutover, `S3Error` is kept so that any call site that has not yet been migrated still compiles. Once all media handlers are updated, the cleanup task removes `S3Error` and its `From<S3Error>` impl.

**Rationale:** keeps the diff to a single commit, lets the agent verify the new path without a flag-day cutover. Alternatives considered: a hard cutover in a single commit — rejected, that would block on every call site at once and lengthen the change.

### Decision 6: `MediaConfig` and `AppState` construction move to env-driven factory functions

`construct_app_state()` reads `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `SUPABASE_STORAGE_BUCKET` (default `media`), and `MEDIA_BASE_URL` from env. It builds a `SupabaseStorage` and a `MediaConfig` directly — no S3 client.

**Rationale:** keeps the construction site in one place. The cleanup task later removes the S3 branch. Alternatives considered: a builder pattern on `SupabaseStorage` — rejected, the env-driven factory is already a builder at the AppState level.

### Decision 7: `OpenSpec` keeps the storage and image-transformation work as two capabilities under one change

Like the auth/vector change, the storage and image-transformation work is "two independent sub-plans that can be executed in parallel" per the legacy plan. They share a single OpenSpec change with two capabilities (`supabase-storage` and `image-transformation`) so the archive lifecycle and "why" are shared.

**Rationale:** matches the framing in the original plan and lets subagents split the work along capability boundaries.

## Risks / Trade-offs

- **[Resize via 302 means the Supabase Storage URL is exposed to clients]** → fine for a public bucket, but the API no longer hides the storage backend. Mitigation: `MEDIA_BASE_URL` can be a CNAME pointing at the Supabase Storage host if a stable URL is required.
- **[Removing the `image` crate eliminates a code path that did CPU work on the API]** → if a future requirement needs server-side image processing (e.g. blurhash, EXIF stripping), the work has to be re-added. Mitigation: Supabase Image Transformation covers the common case (resize, format conversion); a follow-up change can add EXIF stripping on upload.
- **[Public bucket means no signed-URL flows]** → every uploaded file is publicly readable by URL. Mitigation: filenames are random nanoid slugs (`{nanoid} {original}.{ext}` to slug), which provides unguessability in practice. For private media, a follow-up can move to a private bucket + signed-URL flow.
- **[Batch delete returns a `Vec<DeletedObject>` that we map to a count]** → we do not currently check that every requested path was actually deleted, only that the call returned success. Mitigation: the response is logged and surfaced; a follow-up can verify the count against the request.
- **[imgproxy requires the `media` bucket to be public]** → the bucket must be created in Studio or via the SQL admin API with `public = true`. Mitigation: the local dev script (or the migration `volumes/db/init/...`) creates the bucket; a one-time setup step in the README documents the production setup.
- **[Cache invalidation is per-process]** → the moka cache lives in the API process; restarting the API flushes the cache. Mitigation: TTL is 1 h and the content-addressed URLs (`/media/{nanoid}.{ext}`) are stable, so cold cache hits return identical bytes.

## Migration Plan

- Backend: tasks C1 → C3 land in order (client → AppError + AppState → handler migration). Task C4 (image delivery) lands alongside the handler changes. Task C5 (Cargo cleanup) is last.
- Frontend: no changes required. The API surface and response shape are unchanged.
- Local dev: bring up the stack with `docker compose up -d` (from the prior change), open Studio at `:8000`, create the public `media` bucket, then `cargo run` the API. Verify with `curl` calls in the verification checklist.
- Rollback: revert the change's PR; `rust-s3` and `image` come back unchanged; no schema or env changes are required to undo the deployment.

## Open Questions

- Should the API log 4xx/5xx responses from Supabase Storage (upload/delete failures) in a structured way for observability? (Deferred to a follow-up; the responses are surfaced as `StorageError(String)` for now.)
- Is the in-memory moka cache sufficient under load, or do we need a shared cache (e.g. Redis) for a multi-instance API? (Deferred; current deployment is single-instance.)
