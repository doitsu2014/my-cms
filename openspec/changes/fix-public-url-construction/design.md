# Fix Media Response URL Construction & Public Read Endpoint — Design

## Context

The `MediaConfig` struct holds two distinct URLs:

| Field | Source | Audience | Local Docker value |
|-------|--------|----------|-------------------|
| `storage.supabase_url` | `SUPABASE_INTERNAL_URL` (with fallback to `SUPABASE_URL`) | API container → Supabase (outbound) | `http://supabase-kong:8000` |
| `media_base_url` | `MEDIA_BASE_URL` (with fallback to `http://{HOST}:{PORT}`) | Browser → API (client-facing) | `http://localhost:8989` |

The previous `fix-supabase-internal-url-docker-networking` change correctly redirected the API's outbound calls to `SUPABASE_INTERNAL_URL`. However, three handlers now also reuse `storage.supabase_url` when building **client-facing response URLs** in the `bucket_override = Some(...)` branch:

- `apps/api/application_core/src/commands/media/create/create_handler.rs:45-63`
- `apps/api/application_core/src/commands/media/list/list_handler.rs:36-50`
- `apps/api/application_core/src/commands/media/read/metadata_handler.rs:25-39`

Result: when a user uploads via `POST /api/media?bucket=hi29831`, the response `url` field is `http://supabase-kong:8000/storage/v1/object/hi29831/...png`. A browser cannot resolve `supabase-kong`.

A separate issue: public read endpoints `GET /media/{*path}` and `GET /media/images/{*path}?w=&h=` do not accept `?bucket=` and only serve the default `media` bucket. Even after the response-URL fix, browser-accessible reads of PRIVATE buckets still 404 because Supabase returns "Bucket not found" for unauthenticated reads of private buckets (intentional security behavior).

## Goals

- Client-facing response URLs SHALL use a host the browser can resolve (`MEDIA_BASE_URL`).
- Public read endpoints `/media/{*path}` and `/media/images/{*path}` SHALL accept `?bucket=` and proxy through the API with the service-role key, enabling browser access to BOTH public and private buckets.
- Path shape SHALL remain `/storage/v1/object/{bucket}/{path}` (matches Supabase Storage v1.60.2 contract).
- Existing default-bucket behavior SHALL remain unchanged.
- Internal outbound API calls SHALL remain on `SUPABASE_INTERNAL_URL` (no regression).

## Non-Goals

- Implementing Supabase signed URLs.
- Changing Supabase Storage bucket-default visibility semantics.
- Changing the path shape of public read endpoints (`/media/{*path}` stays).
- Refactoring `SupabaseStorage` URL construction beyond what's needed.

## Decisions

### Decision 1 — Use `MEDIA_BASE_URL` for client-facing response URLs

In the three handlers, replace `self.media_config.storage.supabase_url` with `self.media_config.media_base_url` inside the `Some(bucket)` arm. The path template stays:

```text
{MEDIA_BASE_URL}/storage/v1/object/{bucket}/{path}
```

The `None` arm already uses `media_base_url` correctly; the fix unifies both branches.

### Decision 2 — Public read endpoints honor `?bucket=`

Enhance `apps/api/src/api/media/read/read_handler.rs` to accept an optional `bucket: Option<String>` query param on both `api_get_media` and `api_get_media_image`. When present and valid, the handler:

1. Validates the bucket name with `is_valid_bucket_name`
2. Builds `state.media_config.storage.with_bucket(name)` and uses that to call `download()`
3. Adds the service-role auth header (already done via `bearer_auth(self.auth_key())` in `SupabaseStorage::download`)

The image-resize redirect endpoint (`/media/images/{*path}?w=&h=`) constructs the redirect URL by calling `render_image_url()`. When `?bucket=` is present, the handler must construct the URL inline using the bucket name (not `render_image_url()` which always uses `self.bucket`). Pattern:

```rust
let render_url = match bucket.as_deref() {
    Some(name) => format!("{}/storage/v1/render/image/public/{}/{}?width={}", supabase_url, name, path, width),
    None => storage.render_image_url_with_query(path, width),  // or inline equivalent
};
```

### Decision 3 — Do not alter the `public_url()` helper

Leave `SupabaseStorage::public_url(path)` unchanged. It is documented as a client-facing public URL builder and is not invoked by the three handlers in scope.

### Decision 4 — Keep existing env var topology

No env var changes. The fix is purely code-side.

## Affected Files

| File | Change |
|------|--------|
| `apps/api/application_core/src/commands/media/create/create_handler.rs` | Swap `storage.supabase_url` → `media_base_url` in `Some(bucket)` branch |
| `apps/api/application_core/src/commands/media/list/list_handler.rs` | Same swap |
| `apps/api/application_core/src/commands/media/read/metadata_handler.rs` | Same swap |
| `apps/api/src/api/media/read/read_handler.rs` | Add `bucket: Option<String>` query param to `api_get_media` and `api_get_media_image`; honor it via `storage.with_bucket(name)` |

## Test Plan

1. **Unit tests (TDD, RED → GREEN):** for each of the three handlers, add a test that:
   - Constructs a `MediaConfig` with `media_base_url = "http://localhost:8989"` and `storage.supabase_url = "http://supabase-kong:8000"`
   - Sets `bucket_override = Some("hi29831")`
   - Asserts the returned `url` starts with `http://localhost:8989/storage/v1/object/hi29831/` — NOT `http://supabase-kong:8000/...`

2. **Public read endpoint tests:** add tests for `api_get_media` and `api_get_media_image` that:
   - Without `?bucket=`: still serves the default bucket
   - With `?bucket=hi29831`: builds a `SupabaseStorage` with `bucket = "hi29831"` and calls `download()`

3. **Existing tests stay green:** all 49+ tests in `supabase_storage.rs` plus bucket-management tests must pass unchanged.

4. **Manual integration verification:**
   - `curl -F 'file=@x.png' -H 'Authorization: Bearer …' 'http://localhost:8989/api/media?bucket=hi29831'`
   - Confirm response `url` starts with `http://localhost:8989/storage/v1/object/hi29831/`
   - `curl -X PUT -H 'Authorization: Bearer …' -H 'Content-Type: application/json' -d '{"public":true}' http://localhost:8989/api/media/buckets/hi29831`
   - Paste the returned URL in a browser → expect 200 + image bytes
   - `curl -i 'http://localhost:8989/media/foo.png?bucket=hi29831'` → expect 200 + image bytes (proxy through API with auth)

5. **Repository verification gate:**
   ```bash
   cargo check
   cargo test
   cargo fmt -- --check
   cargo clippy
   pnpm --dir apps/web build
   ```

## Migration Plan

- Backward compat: existing clients get a working URL — strict improvement.
- No env, no DB, no coordinated frontend deploy.

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| A future handler inherits the `storage.supabase_url` bug | URL leak | Add a code comment near each fix: `// Response URLs MUST use media_base_url, not storage.supabase_url.` |
| Public read endpoint now makes 2 requests (browser → API → Supabase) | Slight latency increase | Acceptable for admin reads; documented. |
| `?bucket=` with invalid name returns 400 | UX nit | Validation error returns clear message via `bucket_name_error` helper. |
