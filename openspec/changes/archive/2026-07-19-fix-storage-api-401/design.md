# Fix Storage-API 401 on Browser Reads — Design

## Context

The `fix-public-url-construction` change made client-facing media response URLs use `MEDIA_BASE_URL` instead of the internal Docker host `SUPABASE_INTERNAL_URL`. The URL shape is `{MEDIA_BASE_URL}/storage/v1/object/{bucket}/{path}`, which is browser-DNS-resolvable. However, when a browser issues a plain `<img src>` request, it sends no `apikey` and no `Authorization` headers.

Supabase Storage v1.60.2 requires an `apikey` (or `Bearer <jwt>`) on every request — even for `public: true` buckets. The bucket's `public` flag governs only object-level RLS; the apikey check is gateway-level.

Trace of the 401:

1. Browser → `GET http://localhost:8989/storage/v1/object/media/foo.png` (no headers)
2. Traefik (`deployments/docker-swarm/traefik/dynamic/my-cms.yml:49-55`) routes Host=`CMS_SUPABASE_HOST` → `supabase-kong:8000`. No header injection.
3. Kong storage route (`deployments/docker-swarm/supabase/volumes/api/kong.yml:209-225`) has `key-auth` disabled; relies on `request-transformer` to mint `Authorization: $LUA_AUTH_EXPR`.
4. `$LUA_AUTH_EXPR` (set in `kong-entrypoint.sh:35-38`) is `Bearer $(headers.apikey)`. With no `apikey` header, this resolves to `Bearer ` (empty bearer).
5. Kong forwards `Authorization: Bearer ` to storage-api:5000.
6. storage-api returns **401**.

A private-bucket read would 401 regardless of apikey injection, because the anon key lacks `storage.objects` SELECT permission on private buckets. So fixing this requires authenticating with the service-role key — which means going through our API, which already has the service-role key configured.

## Goals

- Browser-initiated GETs for objects in any bucket SHALL succeed with 200 + bytes.
- The fix SHALL work for both public and private buckets.
- The default-bucket behavior SHALL remain unchanged (`{MEDIA_BASE_URL}/media/{path}`).
- The fix SHALL be infrastructure-free (no Traefik middleware, no env var changes).
- API SHALL continue to cache via the existing moka cache so the extra hop is amortized.

## Non-Goals

- Implementing Supabase signed URLs (would be a larger feature with expiration management).
- Fixing the image-resize redirect path (`/media/images/{*path}?w=&h=`). That handler redirects to `storage.render_image_url()` which currently bakes in `storage.supabase_url` (= `SUPABASE_INTERNAL_URL`, browser-unresolvable). This is a separate bug and is tracked separately.
- Changing Kong's storage route or `LUA_AUTH_EXPR` semantics.
- Removing the `MediaConfig::storage.supabase_url` field.

## Decisions

### Decision 1 — Change the response URL to the API proxy shape

When a `?bucket=` override is in effect, the response `url` field SHALL be `{MEDIA_BASE_URL}/media/{path}?bucket={bucket}` instead of `{MEDIA_BASE_URL}/storage/v1/object/{bucket}/{path}`.

**Reason:** The API proxy endpoint `GET /media/{*path}` (`apps/api/src/api/media/read/read_handler.rs:115-145`) already:
- Accepts `?bucket=` (added by `fix-public-url-construction`)
- Validates the bucket name against the same regex as everywhere else
- Builds `state.media_config.storage.with_bucket(name)` and calls `download()` with `Authorization: Bearer <service_role>` and `apikey: <service_role>` headers
- Returns 200 + bytes + correct `Content-Type` + `Cache-Control: public, max-age=31536000, immutable`
- Is wired on `public_router()` (`apps/api/src/bin/my-cms-api.rs:101-104`) — no auth required at the Axum layer; auth happens upstream in the API when talking to Supabase

The browser hits the API (which is already routed via Traefik `api-service`), the API proxies the read to Supabase with the service-role key, returns the bytes. Cached on subsequent hits.

### Decision 2 — Update exactly three URL-construction sites

Per the `fix-public-url-construction` design, the URL is built in three handlers when `bucket_override = Some(name)`:

| File | Current URL | New URL |
|------|-------------|---------|
| `apps/api/application_core/src/commands/media/create/create_handler.rs` (the `Some(name)` arm) | `format!("{}/storage/v1/object/{}/{}", media_base_url, name, path)` | `format!("{}/media/{}?bucket={}", media_base_url, path, name)` |
| `apps/api/application_core/src/commands/media/list/list_handler.rs` (the `Some(name)` arm) | same | same |
| `apps/api/application_core/src/commands/media/read/metadata_handler.rs` (the `Some(name)` arm) | same | same |

The `None` arm (default bucket) remains `{MEDIA_BASE_URL}/media/{path}` for non-images and `{MEDIA_BASE_URL}/media/images/{path}` for images — unchanged.

### Decision 3 — No frontend change required

The frontend treats `url` as an opaque string (`<img src={url}>`, `<a href={url}>`). Switching from the Supabase-direct shape to the API-proxy shape requires zero frontend code changes. The new URL is browser-resolvable, returns bytes, and carries immutable cache headers identical to the existing default-bucket response.

### Decision 4 — No backend handler change

The three handler files do not change shape — only the URL string built inside the `Some(bucket)` arm changes. The `api_get_media` and `api_get_media_image` handlers in `read_handler.rs` already accept `?bucket=` from the prior change. No new routes, no new command handlers, no new tests of the endpoint itself (already covered by `fix-public-url-construction`).

## Affected Files

| File | Change |
|------|--------|
| `apps/api/application_core/src/commands/media/create/create_handler.rs` | In `Some(bucket)` arm: swap URL template from `/storage/v1/object/{bucket}/{path}` to `/media/{path}?bucket={bucket}` |
| `apps/api/application_core/src/commands/media/list/list_handler.rs` | Same swap |
| `apps/api/application_core/src/commands/media/read/metadata_handler.rs` | Same swap |
| `openspec/changes/media-bucket-management/specs/media-bucket-management/spec.md` (line 201 + scenarios at lines 210, 216) | Update URL-shape language; see Part 4 of the architect report |

No changes to:
- `apps/api/src/api/media/read/read_handler.rs` (already supports `?bucket=`)
- `apps/api/application_core/src/commands/media/supabase_storage.rs` (URL building is correct on the server side; this is a client-facing response URL only)
- `deployments/docker-swarm/traefik/dynamic/my-cms.yml` (no middleware needed)
- `deployments/docker-swarm/traefik/.env.example` (no env var needed)
- Frontend code (URL is opaque)

## Test Plan

1. **Unit tests (RED → GREEN)** — for each of the three handlers, update the existing test that asserts the URL shape:
   - Construct a `MediaConfig` with `media_base_url = "http://localhost:8989"` and `storage.supabase_url = "http://supabase-kong:8000"`
   - Set `bucket_override = Some("hi29831")`
   - Assert the returned `url` equals `http://localhost:8989/media/{path}?bucket=hi29831` (NOT `http://localhost:8989/storage/v1/object/hi29831/{path}`)

2. **Default-bucket regression** — verify the `None` arm still produces `{MEDIA_BASE_URL}/media/{path}` (or `/media/images/{path}` for images).

3. **Manual integration verification:**
   - `curl -F 'file=@x.png' -H 'Authorization: Bearer …' 'http://localhost:8989/api/media?bucket=hi29831'`
   - Paste the returned `url` into a browser → expect 200 + image bytes (no 401)
   - Toggle the bucket to private via `PUT /api/media/buckets/hi29831` with `{"public": false}`, upload again, paste the new `url` into a browser → still expect 200 (proves private buckets work too)
   - `curl -i 'http://localhost:8989/media/foo.png?bucket=hi29831'` → expect 200 + image bytes (existing path, regression check)

4. **Repository verification gate:**
   ```bash
   cargo check
   cargo test
   cargo fmt -- --check
   cargo clippy
   pnpm --dir apps/web build
   ```

## Migration Plan

- **Backward compatible:** existing clients see a new URL shape but the URL is functionally equivalent (browser-resolvable, returns same bytes, same cache headers).
- **No env, no DB, no coordinated frontend deploy.**
- **No data migration:** no Supabase Storage operations required.
- **Rollback:** revert the three-line URL change in the three handlers; no schema, no infra, no spec archival needed if the change is still unarchived.

## Risks / Trade-offs

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Extra hop (browser → API → Supabase) for non-default buckets | High | Low latency cost | Moka cache (500 entries, 1h TTL) absorbs repeated reads. The existing default-bucket path already takes this hop. |
| API becomes a hot path for media | Medium | API load increases | Cache absorbs it. If needed, future change can move to Traefik-injected signed URLs. |
| Image-resize redirect path (`/media/images/...?w=&h=`) still has a separate bug (browsers redirected to internal Docker URL) | Medium | Resize images break for default bucket too | **Out of scope** — separate bug, separate fix. Document in the task description; do NOT bundle. |
| Cache key doesn't include `bucket` | Low | Wrong bucket served if path collides | `MediaCacheKey` only keys on `path`. Two buckets with the same object path would collide — extremely unlikely (admin would notice), acceptable for v1. Could be hardened later by adding `bucket: Option<String>` to `MediaCacheKey`. |
| `?bucket=avatars` URL leaked to users | Low | Users bypass API | Acceptable: the URL is public-ish (immutable cache header) by design for the public bucket. For private buckets the API still gates via the service-role key. |

## Related Changes

- **Prior:** `openspec/changes/fix-public-url-construction/design.md` — fixed the DNS half of the same problem (URL host now browser-resolvable). This change fixes the auth half.
- **Prior:** `openspec/changes/media-bucket-management/spec.md` — authored the URL-shape language at line 201 that this change revises.
