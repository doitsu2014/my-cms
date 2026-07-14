# media-bucket-management — Proposal

## Why

The media feature today treats Supabase Storage as a **single fixed bucket** (`media`, configured at boot via `SUPABASE_STORAGE_BUCKET`). Admins can upload, list, preview, and delete files inside it, but they have **no way to create, inspect, configure, or delete buckets themselves** — any bucket change requires out-of-band SQL/Postgres intervention.

This blocks a handful of natural use cases that have already surfaced:

- **Public vs. private assets** — the current `media` bucket is public. Sensitive files (contracts, internal docs) need a private bucket served via signed URLs only.
- **Content-type isolation** — keeping avatars, post images, and post attachments in separate buckets so policies, quotas, and lifecycles can differ.
- **Multi-tenant / experimental buckets** — short-lived buckets for testing, staging tenants, or one-off campaigns without polluting the canonical `media` bucket.

This change gives admins first-class bucket CRUD in the admin UI and the API, while preserving the existing `media` bucket as the default so no current behavior breaks.

## What Changes

- **Application Core — `SupabaseStorage` extension.** Add bucket-level methods (`list_buckets`, `get_bucket`, `create_bucket`, `update_bucket`, `empty_bucket`, `delete_bucket`) wrapping Supabase Storage's bucket REST endpoints (`/storage/v1/bucket`, `/storage/v1/bucket/{name}`, `/storage/v1/bucket/{name}/empty`). All methods require the service-role key (already used by `auth_key()` when set).
- **Application Core — new `bucket` command module.** Handlers under `apps/api/application_core/src/commands/media/bucket/` following the existing Command Pattern (`ListBucketsHandler`, `GetBucketHandler`, `CreateBucketHandler`, `UpdateBucketHandler`, `DeleteBucketHandler`, `EmptyBucketHandler`). Each returns a typed `Bucket` model and an `AppError` on failure (e.g., `BucketAlreadyExists`, `BucketNotFound`, `StorageError`).
- **API — new `bucket` route module.** REST endpoints under `/api/media/buckets/...`, gated by the existing `my-headless-cms-administrator` auth layer:
  - `GET    /api/media/buckets` — list all buckets
  - `GET    /api/media/buckets/{name}` — get bucket details
  - `POST   /api/media/buckets` — create a bucket
  - `PUT    /api/media/buckets/{name}` — update `public`, `file_size_limit`, `allowed_mime_types`
  - `DELETE /api/media/buckets/{name}?purge=true|false` — delete (optionally empty first)
  - `POST   /api/media/buckets/{name}/empty` — empty all objects
- **API — per-request bucket override on object operations.** Existing media endpoints (`GET /api/media`, `POST /api/media`, `DELETE /api/media`, etc.) accept an optional `?bucket=<name>` query param. When present, the operation targets that bucket; when absent, behavior is **unchanged** (defaults to the boot-configured `media` bucket). Same for the public image endpoint (`/media/{*path}`) — no change there, it stays on the `media` bucket since the path doesn't carry a bucket segment.
- **Admin UI — new `/admin/media/buckets` page.** List existing buckets, create new ones (form: name, public flag, file size limit, allowed MIME types), edit config, empty, and delete (with a confirm-purge prompt for non-empty buckets). Route registered under the admin layout.
- **Admin UI — bucket selector on the media browser.** `apps/web/src/app/admin/media/page.tsx` gains a bucket dropdown so the admin can browse any bucket, not just `media`. Selection persists via `?bucket=<name>` URL param (consistent with the backend).
- **Default bucket semantics preserved.** The `media` bucket remains the implicit default for all existing object operations. No existing endpoint changes shape or response.
- **No SeaORM migrations.** Supabase Storage is the source of truth for buckets; the database is untouched.

No breaking changes. All existing endpoints continue to work exactly as today when no `?bucket=` is supplied.

## Capabilities

### New Capabilities

- `media-bucket-management`: Full bucket lifecycle — list/get/create/update/delete/empty — exposed as REST endpoints under `/api/media/buckets/...` and as a new admin UI page. Also owns the per-request `?bucket=<name>` override on existing object operations so admins can browse and manage any bucket from the same UI. Includes the typed `Bucket` model, bucket-name validation rules (lowercase, alphanumeric, dashes/underscores), and the `purge` flag semantics on delete.

### Modified Capabilities

_None._ The existing `supabase-storage` capability's requirements are unchanged: the `media` bucket continues to be the default, all object-operation behavior is preserved when no override is supplied, and the `AppState.media_config.storage` boot contract is untouched. The new `media-bucket-management` capability owns all new behavior so no delta spec is needed on `supabase-storage`.

## Impact

- **API layer** — new module `apps/api/src/api/media/bucket/{list,get,create,update,delete,empty}/`. Add optional `bucket` query param to `ListQueryParams` (list/create/delete handlers). New routes registered in `protected_router()` in `apps/api/src/bin/my-cms-api.rs`.
- **Application Core** — extend `apps/api/application_core/src/commands/media/supabase_storage.rs` with 6 new bucket methods + 1 new `Bucket`/`BucketConfig` struct. New `apps/api/application_core/src/commands/media/bucket/` module with 6 command handlers (one per operation) following the existing trait pattern. New `BucketAlreadyExists` and `BucketNotFound` `AppError` variants if not already representable via `StorageError`/`NotFound`.
- **Models / DTOs** — new `Bucket` and `BucketConfig` types in `apps/api/application_core/src/commands/media/mod.rs` (re-exported from the bucket module).
- **Frontend — admin UI** — new page `apps/web/src/app/admin/media/buckets/page.tsx` and components (bucket table, create/edit modal, delete-with-purge confirm). Bucket selector dropdown on `apps/web/src/app/admin/media/page.tsx`. New types in `apps/web/src/models/MediaModels.ts`. New API wrapper helpers in `apps/web/src/config/api.config.ts` (or co-located) for the bucket endpoints.
- **Database / migrations** — none.
- **Auth** — bucket endpoints sit behind the existing `my-headless-cms-administrator` role check (same as today's media endpoints). No new permissions needed for v1.
- **Frontend route** — `/admin/media/buckets` added to the admin layout; a "Buckets" link appears in the media page header next to the Upload/Refresh buttons.
- **Spec files created by this change** — `openspec/changes/media-bucket-management/specs/media-bucket-management/spec.md`.

## Open Questions & Assumptions

Flagged for the Software Architect to resolve during design/spec phase:

1. **Public/private default** — Assume new buckets default to `public: false` (safer; admin opts into public explicitly). The existing `media` bucket stays public because its behavior is preserved.
2. **Per-bucket resize support** — The `image-transformation` spec assumes the `media` bucket. Out of scope for this change; the resize endpoint continues to operate on `media` only. Architect should decide whether to add per-bucket resize support (e.g., `/media/images/{bucket}/{path}?w=...`) now or defer it.
3. **Bucket name validation** — Proposed rule: lowercase, `^[a-z0-9_-]{3,63}$`, must start with a letter. Enforced by Zod schema on the frontend and a `regex` check in the `CreateBucketHandler`.
4. **Empty vs. delete semantics** — Supabase requires `{"purge": true}` in the DELETE body to delete a non-empty bucket. Assume we expose this as `?purge=true|false` on `DELETE /api/media/buckets/{name}` and reject `?purge=false` on non-empty buckets with a 409 Conflict (force the admin to call `/empty` first or confirm `?purge=true`). A separate `POST /api/media/buckets/{name}/empty` exists for the explicit "empty without delete" case.
5. **Service-role key** — Bucket operations always require the service-role key; anon key cannot manage buckets. The existing `auth_key()` already prefers service role when set. Architect should confirm this is sufficient and add a runtime check that warns at startup if `SUPABASE_SERVICE_ROLE_KEY` is missing.
6. **GraphQL exposure** — Out of scope for this change. Bucket operations are REST-only.
7. **Concurrency / locking** — Assume no concurrent bucket CRUD from multiple admins for v1. If two admins delete the same bucket, Supabase returns a 404 on the second call; we surface that as `BucketNotFound`. Architect should decide if optimistic locking or a "soft delete" step is warranted.
8. **Audit logging** — Out of scope for v1. Bucket CRUD will appear in traces via `#[instrument]` but no persistent audit table.