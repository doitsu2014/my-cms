# fix-image-render-redirect

## Context

After `fix-storage-api-401`, the upload/list/metadata response URLs are correct
and point at the API's media proxy (`{MEDIA_BASE_URL}/media/{path}?bucket={bucket}`).

However, the **image-render endpoint** `GET /media/images/{*path}?w=&h=` still
returns HTTP 307 redirect to the internal Supabase render URL
(`{SUPABASE_INTERNAL_URL}/storage/v1/render/image/public/{bucket}/{path}?width=...`).

When a browser follows that redirect it hits the internal `supabase-kong:8000`
hostname, which is unreachable from outside the Docker network. This breaks the
`/admin/media` grid (default bucket), where each thumbnail is requested through
the API proxy and then redirected.

The same `redirect-to-internal-Supabase` pattern was already removed for upload,
list and metadata. This change finishes the job by proxying the resize through
the API as well, with service-role credentials attached to the upstream call.

## Decision

Replace the redirect-based resize handler with a **proxy handler**:

- `GET /media/images/{*path}?w=&h=&bucket=` continues to exist at the same URL.
- Instead of returning 307, it calls Supabase Image Transformation via a
  service-role-authenticated HTTP GET and returns the rendered image bytes
  (HTTP 200, `Content-Type` from upstream, `Cache-Control: public,
  max-age=31536000, immutable`).
- The browser never sees the internal Supabase host.

In addition, several frontend files independently reconstruct media URLs from
`data.path` instead of using `data.url` / `media.url` returned by the backend.
Those reconstructions are brittle and a regression risk. They are updated to
use the backend-provided URL directly.

## Affected files

### Backend (Layer 2 — architectural fix)

| File | Change |
|---|---|
| `apps/api/application_core/src/commands/media/supabase_storage.rs` | Add `download_render(path, width, height) -> Result<(Vec<u8>, String), AppError>`; share URL pattern with `render_image_url` |
| `apps/api/application_core/src/commands/media/read/read_handler.rs` | Add `bucket: Option<String>` to `MediaCacheKey`; add `get_rendered_image(path, resize_params, bucket)` command |
| `apps/api/src/api/media/read/read_handler.rs` | Replace `Redirect::temporary(&render_url)` with call to `ReadMediaHandler::get_rendered_image`; return bytes with `Content-Type` + `Cache-Control` |

### Frontend (Layer 1 — minimal URL source swap)

| File | Change |
|---|---|
| `apps/web/src/app/admin/media/components/media-grid-item.tsx` | Use `media.url` instead of `getMediaImageUrl(...)` |
| `apps/web/src/app/admin/media/components/media-preview-modal.tsx` | Use `media.url` consistently |
| `apps/web/src/app/admin/components/inputs/thumbnail-input.tsx` | Use `uploadResponse.data.url` directly |
| `apps/web/src/app/admin/components/inputs/rich-text-editor/tiptap-editor.tsx` | Use `uploadResponse.data.url` for inline image insert |
| `apps/web/src/app/admin/components/inputs/rich-text-editor/toolbar/toolbar.tsx` | Use `uploadResponse.data.url` for toolbar upload |

### Spec

| File | Change |
|---|---|
| `openspec/specs/image-transformation/spec.md` | Update requirements / scenarios that mandate 302 redirect to mandate 200 proxy response with bytes |

## Test plan

### Backend (Rust, wiremock)

- `SupabaseStorage::download_render` calls `/storage/v1/render/image/public/{bucket}/{path}?width=&height=` with `Authorization: Bearer` and `apikey` headers.
- Returns `(Vec<u8>, String)` for 200 OK with `content-type` body.
- Returns `AppError::NotFound` on 404 or Supabase 400 with `statusCode: 404` body.
- Returns `AppError::StorageError` on 5xx.
- `MediaCacheKey` equality treats `bucket` as part of the key (same path + dimensions in two buckets → distinct cache entries).
- `ReadMediaHandler::get_rendered_image` writes to cache on miss and reads on hit.
- `api_get_media_image` integration test: when `?w=` or `?h=` is present, the handler calls `get_rendered_image` and returns 200 with bytes (NOT 307).

### Frontend (Vitest)

- `MediaGridItem` uses `media.url` as `<img src>` for image media (no resize query param construction).
- `MediaPreviewModal` uses `media.url` as `<img src>` for image media.
- `ThumbnailsInput` stores `data.data.url` from the upload response directly (no `getMediaImageUrl` call).

## Out of scope

- Re-encoding / transpiling the rendered image. We stream Supabase's bytes through unchanged.
- Bucket-aware resize URLs in upload metadata (`MediaModel.url` still uses `/media/images/{path}` without resize params). Layer 1 swap uses that URL as-is; the resize query params that some frontend call-sites appended are dropped.
- Pre-existing DB rows that already contain stale `supabase-kong` URLs in the `posts.content` column or similar — those need a separate data migration.
