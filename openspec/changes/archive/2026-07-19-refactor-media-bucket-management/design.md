# Refactor Media Bucket Management — Design

## Context

The current media pipeline treats Supabase Storage as a flat, global
proxy: a single env-configured bucket (`"media"`, read from
`SUPABASE_STORAGE_BUCKET`), a single `SupabaseStorage` struct in
`AppState`, and two public-facing endpoints (`/media/{*path}` and
`/media/images/{*path}`) that use the service-role key to stream any
object out of any bucket. The just-archived `media-bucket-management`
change added bucket CRUD and a per-request `?bucket=<name>` override,
surfacing the existing `Bucket.public: bool` field on every
`GET /api/media/buckets/{name}` response — but the public delivery
endpoints never consult it.

This change is the smallest possible refactor that closes two gaps:

1. **Dead config** — the `SUPABASE_STORAGE_BUCKET` env var is no
   longer needed because callers already specify the bucket per
   request (via `?bucket=`), and the default `"media"` is hard-coded
   into URL templates anyway.
2. **Public leak** — anonymous requests to
   `/media/secret.pdf?bucket=private-docs` succeed today because the
   storage client streams any object the service-role key can see. The
   fix is to gate the public endpoint by `Bucket.public` (or an admin
   JWT).

The bucket-management surface (`/api/media/buckets/*`), the `Bucket`
DTO, and the `?bucket=` override on object operations are
**unchanged** — they were just delivered.

## Goals

- Remove `SUPABASE_STORAGE_BUCKET` from env, code, deployment files,
  and the canonical spec without changing observable behaviour for any
  existing request.
- Reject unauthenticated and non-admin requests for objects in private
  buckets with HTTP 404.
- Allow authenticated administrators to fetch objects in private
  buckets through the existing `/media/{*path}?bucket=<name>` endpoint
  with a small frontend tweak (a new `AuthenticatedImage` component).
- Add the lookup with a small in-memory cache so the check does not
  become a Supabase round-trip on every request.
- Keep the change additive on top of `media-bucket-management`: no
  schema change, no new middleware, no new env var.

## Non-Goals

- Signed URLs for private buckets (admin's browser hits our proxy
  through the new `AuthenticatedImage` component, which sends the JWT
  — no signed URL needed).
- Per-bucket image resize for non-default buckets.
- Per-bucket ACLs beyond the `public: true | false` flag.
- Explicit cache invalidation on `UpdateBucketHandler` (5-minute TTL
  is acceptable for an admin UI).
- Removing the `media` bucket's reserved-name treatment.
- Migrating `MEDIA_BASE_URL` or other env vars.

## Decisions

### 1. Hard-code the default bucket as `"media"` in `construct_app_state()`

Replace

```rust
let supabase_storage_bucket =
    env::var("SUPABASE_STORAGE_BUCKET").unwrap_or_else(|_| "media".to_string());
…
let storage = SupabaseStorage::new(…, supabase_storage_bucket);
```

with

```rust
let storage = SupabaseStorage::new(…, "media");
```

The default bucket is no longer a deployment-level concern; it is a
code constant. The single valid value (`"media"`) is preserved; no
caller can change it.

**Reason:** The env var defaults to `"media"` today. Removing the env
read collapses to the same observable behaviour for every existing
deployment while deleting the variable from `.env.example`,
docker-compose.yaml, and the spec. Per-request overrides still flow
through `with_bucket(name)`. No migration step is required — existing
env files simply drop the line, no warning is emitted.

### 2. Bucket-type source: `Bucket.public` from `SupabaseStorage::get_bucket(name)` (cached)

The bucket type is the `public: bool` field of the existing `Bucket`
DTO (defined in
`apps/api/application_core/src/commands/media/bucket/dto.rs`), already
populated by `SupabaseStorage::get_bucket(name)` (defined in
`supabase_storage.rs:485`).

A new in-memory `Cache<String, bool>` (moka, 5-minute TTL, 256-entry
capacity) on `AppState` keys `(bucket_name -> is_public)`. On cache
miss, the handler calls `SupabaseStorage::get_bucket(name)` and
inserts the result. On cache hit, the handler short-circuits.

**Reason:** Three options were considered:

| Option | Description | Trade-off |
|---|---|---|
| (a) Hard-coded list | Maintain a constant `private_buckets: &[&str]` in code | Brittle, requires redeploy to add/remove a private bucket |
| (b) DB-backed | Add a `bucket_type` column or `buckets` table mirrored from Supabase | Schema drift, sync overhead, second source of truth |
| (c) Config-driven | Read a TOML/YAML mapping at boot | Same drift risk as (a), more complex |
| **(d) Read `Bucket.public` from Supabase at request time (chosen)** | Use the existing `get_bucket` API | One Supabase round-trip per cache-miss; otherwise free |

Option (d) wins because:

1. Zero schema change — Supabase Storage is already the source of
   truth for `public`.
2. Zero drift — flipping `public` in the admin UI immediately reflects
   at the next cache miss.
3. Already built — `SupabaseStorage::get_bucket` and the `Bucket` DTO
   exist (added by `media-bucket-management`).
4. Cheap with cache — at 5-minute TTL, an idle admin session triggers
   zero Supabase round-trips for the bucket check; a busy one
   triggers ~1 per unique bucket per 5 minutes.

### 3. Admin scope on the public endpoint: inline JWT decode in the handler

In `api_get_media` and `api_get_media_image`, after extracting the
bucket name, the handler reads the `Authorization: Bearer <jwt>`
header. If present, it decodes the JWT using the same
`SUPABASE_JWT_SECRET` + `AUTHORIZATION_AUDIENCE` as the existing
`SupabaseAuthLayer` (factored into a small helper
`is_admin_jwt_present(headers: &HeaderMap) -> bool`). If the JWT is
valid and its `app_metadata.roles` contains
`"my-headless-cms-administrator"`, the handler treats the request as
admin-scoped.

**Reason:** Two options were considered:

| Option | Description | Trade-off |
|---|---|---|
| **(a) Inline JWT decode in the handler (chosen)** | Add `is_admin_jwt_present(headers)` helper; treat presence of a valid admin JWT as admin scope | Localized to two handlers; no middleware; no risk to non-media endpoints |
| (b) Optional `SupabaseAuthLayer` on `public_router` | Mount `SupabaseAuthLayer` with `required_roles = []` to make it optional | Affects all public routes (`/`, `/healthz`, `/graphql/immutable`); larger blast radius |

(a) is preferred because the public router hosts non-media endpoints
(`/`, `/healthz`, `/graphql/immutable`) that must remain
unauthenticated and unrestricted. (b) would either need a custom
"optional auth" middleware (YAGNI) or affect unrelated endpoints.

The JWT decode reuses the same `jsonwebtoken` crate already in
`Cargo.toml` (used by `SupabaseAuthLayer`) and the same env vars. The
helper is `<30` lines and unit-testable with a hard-coded JWT.

### 4. HTTP status for forbidden private-bucket access: **404 (not found)**

When a request is rejected because the bucket is private and the
caller is not an administrator, the API returns HTTP 404 with body
`{"error":"Not found"}`. The error variant is `AppError::NotFound`
(existing).

**Reason:** Two options were considered:

| Option | Description | Trade-off |
|---|---|---|
| **(a) 404 (chosen)** | Return `Not found` — hides the existence of the bucket | Matches existing pattern (Supabase 404 already maps to `AppError::NotFound`); does not leak bucket names |
| (b) 403 | Return `Forbidden` with a clear message | Clearer for legitimate admin debugging; leaks the existence of the bucket to anonymous probers |

For an internal CMS that should not leak its storage structure, 404 is
the conservative choice.

### 5. Admin preview mechanism: `<AuthenticatedImage>` component (frontend)

The admin media browser currently renders thumbnails via plain
`<img src={media.url}>` in two components
(`media-grid-item.tsx:108-114` and `media-preview-modal.tsx:76-81`).
Because `<img>` tags do not carry `Authorization` headers, the browser
request to the API is anonymous and would 404 on private buckets
despite the backend bypass. The fix is a new
`<AuthenticatedImage src token />` component (~40 lines) that:

1. Receives `src` (the URL returned by `media.url`) and `token`
   (from `useAuth()`).
2. On mount, calls `authenticatedFetch(src, token, { cache: 'no-store' })`.
3. Converts the response to a `Blob` and then a blob URL via
   `URL.createObjectURL(blob)`.
4. Sets the blob URL as the `<img src>`.
5. On unmount or `src` change, revokes the blob URL with
   `URL.revokeObjectURL`.

This is a small, contained change — two existing components swap
`<img>` for `<AuthenticatedImage>` and the parent page passes the
token down.

**Reason:** Alternatives were considered and rejected:

| Option | Description | Trade-off |
|---|---|---|
| Embedding JWT in URL query string (`?admin_token=...`) | `<img>` would carry it natively | Logs, referer headers, browser history — security anti-pattern |
| Cookie-based admin session | `<img>` sends cookies automatically | Requires changes to the existing JWT auth flow; large blast radius |
| Signed URLs from Supabase | `<img>` fetches Supabase directly | Changes the URL shape returned by metadata API; expires mid-session; out of scope per user |
| **(a) Frontend `<AuthenticatedImage>` (chosen)** | Self-contained, follows existing `authenticatedFetch` pattern | One extra fetch round-trip per preview; tiny memory pressure until unmount |

## Architecture

### Module layout

```
apps/api/application_core/src/commands/media/
├── mod.rs                                       # UNCHANGED
├── supabase_storage.rs                          # UNCHANGED
├── bucket/                                      # UNCHANGED (added by media-bucket-management)
│   ├── mod.rs                                   # MODIFY: add `pub mod access;`
│   ├── dto.rs                                   # UNCHANGED (Bucket.public already exposed)
│   ├── …                                        # existing CRUD submodules
│   └── access/                                  # NEW
│       ├── mod.rs                               # NEW: pub mod {access_handler, access_cache}
│       ├── access_cache.rs                      # NEW: create_bucket_visibility_cache()
│       └── access_handler.rs                    # NEW: BucketAccessPolicy trait + struct + tests

apps/api/src/api/media/read/
└── read_handler.rs                              # MODIFY: add bucket-type gate + admin-JWT check

apps/api/src/bin/my-cms-api.rs                   # MODIFY: drop env read, hardcode "media", wire cache
apps/api/src/lib.rs                              # MODIFY: AppState gains bucket_visibility_cache

apps/web/src/app/admin/media/components/
├── authenticated-image.tsx                      # NEW
├── media-grid-item.tsx                          # MODIFY: use <AuthenticatedImage>
└── media-preview-modal.tsx                      # MODIFY: use <AuthenticatedImage>

apps/web/src/app/admin/media/page.tsx            # MODIFY: pass token down to grid + preview
```

### `BucketAccessPolicy`

```rust
// apps/api/application_core/src/commands/media/bucket/access/access_handler.rs

pub trait BucketAccessPolicyTrait {
    fn ensure_public_or_admin(
        &self,
        bucket_name: &str,
        is_admin: bool,
    ) -> impl std::future::Future<Output = Result<(), AppError>>;
}

pub struct BucketAccessPolicy {
    pub storage: SupabaseStorage,
    pub cache: Arc<Cache<String, bool>>,
}

impl BucketAccessPolicyTrait for BucketAccessPolicy {
    async fn ensure_public_or_admin(
        &self,
        bucket_name: &str,
        is_admin: bool,
    ) -> Result<(), AppError> {
        if is_admin {
            // Admin bypasses the public/private check entirely.
            return Ok(());
        }
        let is_public = if let Some(cached) = self.cache.get(&bucket_name.to_string()).await {
            cached
        } else {
            let bucket = self.storage.get_bucket(bucket_name).await?;
            let public = bucket.public;
            self.cache.insert(bucket_name.to_string(), public).await;
            public
        };
        if is_public { Ok(()) } else { Err(AppError::NotFound) }
    }
}
```

### Modified public-media handler

```rust
// apps/api/src/api/media/read/read_handler.rs (sketch)

#[instrument(skip(state, headers))]
pub async fn api_get_media(
    state: State<AppState>,
    headers: HeaderMap,
    Path(path): Path<String>,
    Query(params): Query<ReadQueryParams>,
) -> Response {
    let bucket = match validate_bucket(params.bucket.as_deref()) {
        Ok(b) => b,
        Err(resp) => return *resp,
    };

    let bucket_name = bucket.clone().unwrap_or_else(|| "media".to_string());
    let is_admin = is_admin_jwt_present(&headers);
    let policy = BucketAccessPolicy {
        storage: state.media_config.storage.clone(),
        cache: state.bucket_visibility_cache.clone(),
    };
    if let Err(e) = policy.ensure_public_or_admin(&bucket_name, is_admin).await {
        return error_response(e);
    }

    let storage = resolve_storage(&state.media_config.storage, bucket.clone());
    let handler = ReadMediaHandler::new(Arc::new(storage), state.media_cache.clone());
    match handler.get_media_for_bucket(path, bucket).await {
        Ok(cached_media) => media_response(cached_media.data, cached_media.content_type),
        Err(e) => error_response(e),
    }
}
```

### Frontend `<AuthenticatedImage>` (sketch)

```tsx
// apps/web/src/app/admin/media/components/authenticated-image.tsx
import { useEffect, useState } from 'react';
import { authenticatedFetch } from '@/config/api.config';

interface AuthenticatedImageProps {
  src: string;
  token: string | null;
  alt: string;
  className?: string;
}

export default function AuthenticatedImage({
  src, token, alt, className,
}: AuthenticatedImageProps) {
  const [blobUrl, setBlobUrl] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    let createdUrl: string | null = null;
    (async () => {
      const response = await authenticatedFetch(src, token, { cache: 'no-store' });
      if (!response.ok) return;
      const blob = await response.blob();
      if (cancelled) return;
      createdUrl = URL.createObjectURL(blob);
      setBlobUrl(createdUrl);
    })();
    return () => {
      cancelled = true;
      if (createdUrl) URL.revokeObjectURL(createdUrl);
    };
  }, [src, token]);

  return blobUrl
    ? <img src={blobUrl} alt={alt} className={className} />
    : <div className={className} aria-busy="true" />;
}
```

### Error mapping

| Path | HTTP | Body | Source |
|---|---|---|---|
| Bucket not in Supabase | 404 | `{"error":"Not found"}` | `SupabaseStorage::get_bucket` → `AppError::NotFound` |
| Bucket is private, caller not admin | 404 | `{"error":"Not found"}` | `BucketAccessPolicy` → `AppError::NotFound` |
| Bucket is public (or admin) | 200 | bytes | existing path |
| Invalid bucket name | 400 | `{"error":"invalid bucket: ..."}` | existing `validate_bucket` |
| Supabase unreachable on `get_bucket` | 502 | `{"error":"..."}` | existing `error_status` for `AppError::StorageError` |
| Object bytes 404 / 5xx (after policy passes) | 404 / 502 | `{"error":"..."}` | existing `error_response` |

## Risks / Trade-offs

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Cache stale after admin toggles `Bucket.public` from `true` to `false` | Low | Low | 5-minute TTL bounds staleness. Admin can wait or restart the API. |
| Cache stale after admin toggles `Bucket.public` from `false` to `true` | Medium | Medium | Same as above. If product-owner wants tighter freshness, add `cache.invalidate(name)` to the `UpdateBucketHandler` (single-line follow-up). |
| JWT decode adds latency to every public-media request | Negligible | None | JWT decode with HS256 is ~10 µs. No Supabase round-trip on cache hit. |
| Admin JWT decode uses a different code path than `SupabaseAuthLayer` | Low | Low | Extract the role-check helper into a shared module (`apps/api/src/common/jwt_decode.rs`) so both layers use the same logic. |
| The new env-var-free boot breaks a deployment that hard-codes a non-`"media"` bucket in env | None | None | The env var was already optional (defaulted to `"media"`). Removing the read leaves behaviour unchanged. |
| Anonymous browser tab on a private-bucket URL now gets 404 mid-session | Low | Low | Intended fix. Admin signs in to access. |
| `<AuthenticatedImage>` blob URLs leak memory if cleanup is missed | Low | Low | `useEffect` cleanup always calls `URL.revokeObjectURL`. Vitest test asserts cleanup. |
| `<AuthenticatedImage>` adds a fetch round-trip per preview | Low | Low | Acceptable for an admin-only view; bulk thumbnails still load in parallel. |
| The default `"media"` bucket is checked even when no override is supplied | Low | Negligible | One cache lookup (or one Supabase round-trip on first hit) per request. Uniform enforcement outweighs the cost. |
| `is_admin_jwt_present` returns `false` on token-decode error rather than failing the request | None | None | This is intentional — non-admin callers proceed normally; only the bucket-type check is affected. |

## Migration Plan

No breaking changes for the documented happy paths:

- Public bucket, anonymous — request succeeds (same as today).
- Public bucket, admin — request succeeds (same as today).
- Private bucket, admin — request succeeds (NEW: previously succeeded
  via the proxy too; now succeeds through the explicit admin JWT path
  in `AuthenticatedImage`).
- Private bucket, anonymous or non-admin — request returns 404
  (NEW: previously leaked bytes; now correctly rejected).

Deployment steps:

1. Apply the change. Drop `SUPABASE_STORAGE_BUCKET` from `.env` and
   docker-compose env file (no warning emitted; the line is silently
   ignored).
2. Restart the API. The new `bucket_visibility_cache` is constructed
   at boot and the hard-coded `"media"` default is in effect.
3. (Optional) Manually verify: `curl -i
   http://localhost:8989/media/foo.png?bucket=private-docs` returns 404
   (no JWT); `curl -i -H "Authorization: Bearer <admin-jwt>"
   http://localhost:8989/media/foo.png?bucket=private-docs` returns
   200.

Rollback: revert the change's PR. No schema, env, or data changes to
undo. The old `SUPABASE_STORAGE_BUCKET` env var is restored.
