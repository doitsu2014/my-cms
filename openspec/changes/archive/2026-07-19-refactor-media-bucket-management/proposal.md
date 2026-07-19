# Refactor Media Bucket Management — Proposal

## Why

Two concrete pain points in the current media/bucket delivery model:

**Issue 1 — Redundant `SUPABASE_STORAGE_BUCKET` env var.** The env var
`SUPABASE_STORAGE_BUCKET=media` is read once at boot
(`apps/api/src/bin/my-cms-api.rs:264-265`), passed to
`SupabaseStorage::new()`, and stored in the immutable `bucket: String`
field of every `SupabaseStorage` instance. However, every API handler
that targets a non-default bucket already accepts `?bucket=<name>` and
constructs the override via `SupabaseStorage::with_bucket(name)` (added
in the just-archived `media-bucket-management` change). The default
`"media"` bucket is also already hard-coded into response URL templates
like `{MEDIA_BASE_URL}/media/{path}` and
`{MEDIA_BASE_URL}/media/images/{path}`. The env var is dead weight: it
cannot be changed at runtime, it has only one valid value in practice,
and removing it shrinks the deployment surface.

**Issue 2 — No bucket-type enforcement at delivery time.** The public
media endpoints `/media/{*path}` and `/media/images/{*path}`
(`apps/api/src/api/media/read/read_handler.rs:84-126`) accept
`?bucket=<name>` and use `SupabaseStorage` (which carries the
service-role key) to stream bytes from
`/storage/v1/object/public/{bucket}/{path}` for **any** bucket,
regardless of whether that bucket is marked `public: true` or
`public: false` in Supabase Storage. Supabase already tracks this
`public` flag on every bucket (exposed via
`GET /storage/v1/bucket/{name}` and returned in the existing `Bucket`
DTO at `apps/api/application_core/src/commands/media/bucket/dto.rs:5-18`).
The `Bucket` model even carries the flag explicitly — but the public
delivery path never checks it. A private bucket's contents are *de
facto* public through our proxy.

This change closes both issues: it deletes the redundant env var, and
it makes the public delivery path consult the bucket's `public` flag at
runtime (cached) so private buckets stop leaking through the
unauthenticated endpoint while remaining reachable from the admin
dashboard (which sends an authenticated admin JWT through a new
frontend component that wraps image previews).

## What Changes

- **Drop the `SUPABASE_STORAGE_BUCKET` env var.** The default bucket
  name (`"media"`) is hard-coded in `construct_app_state()` and passed
  as a literal to `SupabaseStorage::new()`. The variable is removed
  from both `.env.example` files, the docker-compose.yaml env
  pass-through, and the canonical `supabase-storage` spec.
- **Enforce `bucket.public` at the public media delivery endpoint.**
  For every request to `GET /media/{*path}` or
  `GET /media/images/{*path}?w=&h=`, if the request specifies a
  `?bucket=<name>` (or the implicit default `"media"`), the handler
  resolves the bucket's `public` flag from Supabase via
  `SupabaseStorage::get_bucket(name)` and:
  - allows the request when `bucket.public == true`;
  - returns HTTP 404 (`AppError::NotFound`) when `bucket.public == false`
    and the caller is **unauthenticated** or **authenticated but not an
    administrator**;
  - allows the request when `bucket.public == false` AND the caller
    carries a valid JWT whose `app_metadata.roles` contains
    `my-headless-cms-administrator`.
- **Cache the bucket `public` lookup** in a new in-memory
  `Cache<String, bool>` (moka, 5-minute TTL, 256-entry capacity), keyed
  by bucket name, attached to `AppState` as `bucket_visibility_cache`.
  The first request for a bucket warms the cache; subsequent requests
  skip the Supabase round-trip until TTL expiry. Cache invalidation is
  by TTL only (no explicit invalidation on bucket `public` toggle — an
  admin who flips `public` may see stale behaviour for up to 5
  minutes).
- **Admin path keeps working unchanged on the API side.** The API's
  admin preview endpoint (the existing public `/media/{*path}`) now
  accepts an admin JWT via `Authorization: Bearer <jwt>` to bypass the
  `public` check. **A small frontend change is required** to deliver
  the JWT: a new `<AuthenticatedImage>` component wraps `<img>` so the
  admin media browser uses `authenticatedFetch` + `URL.createObjectURL`
  for image previews. This is contained to two files
  (`media-grid-item.tsx`, `media-preview-modal.tsx`) and the new
  component file.
- **No schema change, no new SeaORM migration.** Supabase Storage is
  the source of truth for bucket metadata; the only local addition is
  a `bool` cache in memory.

## Capabilities

### Modified Capabilities

- **`supabase-storage`** — Two requirements change:
  1. Requirement "AppState and MediaConfig carry SupabaseStorage"
     loses the `SUPABASE_STORAGE_BUCKET` env var (no longer read at
     boot). The boot contract becomes: read `SUPABASE_URL`,
     `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY` (optional), then
     construct `SupabaseStorage` with the hard-coded default bucket
     `"media"`.
  2. New requirement added: "Public media delivery requires public
     bucket or admin scope" — the bucket-type enforcement described
     above.

### New Capabilities

None. The bucket-management surface (`/api/media/buckets/*`) and the
`Bucket` DTO already exist (archived change
`2026-07-19-media-bucket-management`). This change only consumes the
`public` flag of that DTO.

## Impact

| Layer | Impact |
|---|---|
| **API routing** | `public_router()` shape unchanged. Two handlers (`api_get_media`, `api_get_media_image`) gain an early bucket-type gate plus an optional admin-JWT check. |
| **Application core** | New module `apps/api/application_core/src/commands/media/bucket/access/` with `BucketAccessPolicy` (trait + struct) that owns the visibility cache and calls `SupabaseStorage::get_bucket(name)`. |
| **AppState** | New field `bucket_visibility_cache: Arc<Cache<String, bool>>` constructed in `construct_app_state()`. |
| **Auth** | No change to `SupabaseAuthLayer`. The public router stays unauthenticated; each media handler reads the optional `Authorization: Bearer <jwt>` header inline and reuses the same `SUPABASE_JWT_SECRET` + `AUTHORIZATION_AUDIENCE` env vars. |
| **Configuration** | `SUPABASE_STORAGE_BUCKET` removed from `apps/api/.env.example`, `deployments/docker-swarm/apps/.env.example`, `deployments/docker-swarm/apps/docker-compose.yaml`, and the canonical spec. |
| **Database / migrations** | None. |
| **Frontend** | New `<AuthenticatedImage>` component (~40 lines). Two existing components swapped from `<img>` to `<AuthenticatedImage>`. One parent page passes `token` down. |
| **Tests** | New `BucketAccessPolicy` unit tests + 3 wiremock-backed handler tests. Existing tests in `read_handler.rs:166,174` and `supabase_storage.rs:1362` keep passing (they don't depend on the env var). |

## Non-Goals

- **Signed URLs for private buckets.** Out of scope. The admin's
  browser hits our proxy through the new `AuthenticatedImage`
  component, which sends the JWT — no signed URL is needed.
- **Per-bucket image resize for non-default buckets.** Unchanged.
- **Per-bucket ACLs beyond `public: true/false`.** No row-level
  policies, no per-user access lists, no time-bounded tokens.
- **Cache invalidation on `Bucket.public` toggle.** 5-minute TTL is
  acceptable for an admin UI; admins rarely toggle public flag
  mid-session. A future change can add explicit
  `cache.invalidate(name)` to the `UpdateBucketHandler` if needed.
- **Removing the `media` bucket's special handling.** The `media`
  bucket remains the implicit default and stays exempted from the
  reserved-name rejection in bucket CRUD.
- **Migrating `MEDIA_BASE_URL` or any other env vars.** Only
  `SUPABASE_STORAGE_BUCKET` is in scope.

## Open Questions & Assumptions

For the Software Architect to confirm before implementation:

1. **HTTP status for forbidden private-bucket access.** Recommended:
   **404** (hides existence; matches existing `AppError::NotFound`
   pattern). Alternative: **403** (explicit, but leaks existence).
2. **Cache TTL.** Recommended: **300 s** (5 min), 256-entry capacity.
3. **Where the bucket-type lookup lives.** Recommended: new
   `commands/media/bucket/access/` module following the existing
   Command Pattern (trait + struct).
4. **Admin JWT validation.** Recommended: inline `jsonwebtoken::decode`
   in the handler (no new middleware on `public_router`); reuse the
   same `SUPABASE_JWT_SECRET` and `AUTHORIZATION_AUDIENCE` env vars.
5. **Admin preview mechanism.** Recommended: frontend
   `<AuthenticatedImage>` component using `authenticatedFetch` +
   `URL.createObjectURL`. Alternatives rejected:
   - Embedding JWT in URL query string (security anti-pattern).
   - Cookie-based admin session (requires auth-flow changes).
   - Signed URLs (out of scope; changes URL shape returned to clients).
6. **Should the implicit default `"media"` bucket also consult the
   cache?** Recommended: **yes**, for uniform enforcement (an admin
   flipping `media` from public to private correctly blocks anonymous
   requests within 5 minutes). Alternative: skip cache for the default
   to keep the hot path zero-overhead (looser security).
