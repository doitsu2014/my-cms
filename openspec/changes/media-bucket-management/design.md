# Media Bucket Management — Design

## Context

The current media pipeline in my-cms is built around a single, boot-configured Supabase Storage bucket. `SupabaseStorage` (in `apps/api/application_core/src/commands/media/supabase_storage.rs`) holds an immutable `bucket: String` field that is read from `SUPABASE_STORAGE_BUCKET` (default `media`) at startup in `construct_app_state()` (`apps/api/src/bin/my-cms-api.rs:249-261`). Every method — `upload`, `download`, `get_info`, `list_objects`, `delete`, `delete_batch` — bakes that single bucket into its URL template.

The existing API surface for media (`/api/media`, `/api/media/info/{*path}`, `/api/media/delete/{*path}`) is mounted on `protected_router()` behind a `SupabaseAuthLayer` that accepts both `my-headless-cms-writer` and `my-headless-cms-administrator`. The public read endpoints (`/media/{*path}`, `/media/images/{*path}`) are on `public_router()` and currently serve from the `media` bucket only.

This change introduces a new `media-bucket-management` capability that:

1. Adds a typed `Bucket` model and six bucket-CRUD command handlers (`ListBucketsHandler`, `GetBucketHandler`, `CreateBucketHandler`, `UpdateBucketHandler`, `DeleteBucketHandler`, `EmptyBucketHandler`) under `apps/api/application_core/src/commands/media/bucket/`, following the existing Command Pattern (`*HandlerTrait` + `*Handler { media_config: Arc<MediaConfig> }`).
2. Adds six REST endpoints under `/api/media/buckets/...` mounted on `protected_administrator_router()` (the only router that restricts to `my-headless-cms-administrator`).
3. Threads an optional per-request `?bucket=<name>` override through the existing object-operation handlers (`list`, `create`, `delete`, `metadata`) so the admin UI can browse any bucket through the same page.
4. Adds a new admin UI page at `/admin/media/buckets` and a bucket-selector dropdown on the existing `/admin/media` page.

The existing `media` bucket remains the default for every operation. No existing endpoint changes shape or response when no `?bucket=` is supplied.

## Goals

- Provide full bucket lifecycle management (list/get/create/update/empty/delete) via REST and admin UI.
- Preserve the existing `media` bucket as the implicit default — no breaking changes.
- Thread a per-request bucket override through object operations without duplicating `SupabaseStorage` methods.
- Restrict bucket-management endpoints to `my-headless-cms-administrator`.
- Enforce bucket name validation (`^[a-z][a-z0-9_-]{2,62}$`) at both the API boundary and the frontend Zod schema.
- Return 409 on attempted deletion of a non-empty bucket without `?purge=true`.

## Non-Goals

- **Per-bucket image-resize support** — the existing `/media/images/{*path}?w=&h=` redirect continues to operate on the `media` bucket only. Adding per-bucket resize (e.g. `/media/images/{bucket}/{path}`) is deferred; a follow-up change can introduce it.
- **Signed URLs for private buckets** — the response `url` for objects in a non-default bucket points to the public Supabase object URL, which only works for public buckets. A signed-URL endpoint is deferred.
- **Bucket-level audit logging** — `#[instrument]` will surface bucket operations in traces, but no persistent audit table.
- **Optimistic locking on bucket CRUD** — concurrent edits use last-write-wins; the second of two concurrent deletes gets `BucketNotFound` from Supabase.
- **GraphQL exposure** — bucket operations are REST-only for v1.
- **Soft delete / trash view for buckets** — Supabase delete is hard.
- **Bulk import / CSV** for buckets.
- **Changing the existing `media` bucket's public flag** via the new update endpoint — the proposal preserves the `media` bucket's public=true behavior; admins can still toggle it but no migration is required.

## Decisions

### 1. Storage clone with `with_bucket` override

**Decision:** Add a `with_bucket(name: &str) -> SupabaseStorage` method on `SupabaseStorage` that returns a clone of `self` with the `bucket` field replaced. All existing methods (`upload`, `download`, `get_info`, `list_objects`, `delete`, `delete_batch`) keep their current signatures; the override is applied by the API handler before constructing the command handler.

```rust
impl SupabaseStorage {
    pub fn with_bucket(&self, name: &str) -> Self {
        let mut cloned = self.clone();
        cloned.bucket = name.to_string();
        cloned
    }
}
```

**Reason:** The `SupabaseStorage` struct is `#[derive(Clone)]` and the `bucket` field is the only piece that varies per request. Cloning the struct is cheap (two strings + a `reqwest::Client` which is internally `Arc`'d). The API handler can do `state.media_config.storage.with_bucket("avatars")` and pass the result to the command handler. This keeps the existing method signatures stable for all call sites that don't need the override. Alternatives considered: adding `bucket: Option<&str>` to every method (rejected — six method signatures to touch, every existing caller would need a `None`); introducing a `MediaStorage` trait with two impls (rejected — YAGNI, the supabase-storage archive explicitly rejected this).

### 2. URL response when bucket override is in effect

**Decision:** When the object operation has a `?bucket=<name>` override, the `url` field in the response SHALL be `{SUPABASE_URL}/storage/v1/object/{bucket}/{path}`. When no override is in effect, the existing `{MEDIA_BASE_URL}/media/{path}` (or `{MEDIA_BASE_URL}/media/images/{path}` for images) is preserved.

**Reason:** The API's public read endpoint `/media/{*path}` always serves from the `media` bucket (the path doesn't carry a bucket segment). So for non-default buckets, the only useful URL is the direct Supabase object URL. This works for public buckets out of the box; for private buckets the admin would need a signed-URL flow (out of scope). The shape change is gated by the override — callers that never pass `?bucket=` see no change.

### 3. Bucket name validation: Zod on the frontend, regex in the command handler

**Decision:** The bucket name pattern `^[a-z][a-z0-9_-]{2,62}$` is enforced in two places:

- **Frontend** — a Zod schema in `apps/web/src/schemas/bucket.schema.ts` (`.regex(/^[a-z][a-z0-9_-]{2,62}$/, "must start with a lowercase letter; only [a-z0-9_-] allowed")`) is used by both the create and edit forms and the bucket-name path-param parser.
- **Backend** — a `regex::Regex::new(r"^[a-z][a-z0-9_-]{2,62}$").unwrap()` constant lives in the `commands::media::bucket` module. Each handler that accepts a bucket name (create body, update path, delete path, empty path, override query param) calls `is_valid_bucket_name(name) -> bool` and returns `AppError::Validation("name" | "bucket", "...")` on failure.

**Reason:** Defensive validation at both ends. The regex is already a production dependency (`regex = "1"` in `application_core/Cargo.toml:49`). The `^[a-z]` anchor enforces "must start with a letter" (the proposal's open question #3). Length 3–63 mirrors Supabase's own bucket-name length constraint.

### 4. 409 conflict on non-empty delete without purge

**Decision:** When `DELETE /api/media/buckets/{name}?purge=false` is called and the bucket is non-empty, the API returns HTTP 409 with `AppError::Conflict("Bucket '<name>' is not empty; pass ?purge=true to delete with all objects")`. When `?purge=true` is passed, the API includes `{"purge": true}` in the Supabase DELETE body so Supabase empties and deletes atomically.

**Reason:** The product-owner explicitly asked for 409 in the proposal (open question #4). The existing `AppError::Conflict(String)` variant added by the `add-user-management-admin-page` change is the right variant. The implementation does NOT pre-check emptiness via `list_objects` (race condition) — it relies on Supabase returning `400` for the empty-body DELETE on a non-empty bucket and translates that to 409. The 400-to-409 translation is done in the `DeleteBucketHandler` by inspecting the status code.

### 5. Service-role key handling: keep the existing panic, add a boot log

**Decision:** The existing `.expect("SUPABASE_SERVICE_ROLE_KEY must be set")` in `construct_app_state()` is preserved. A single `tracing::info!("Supabase service role key configured; bucket management endpoints enabled")` log line is added immediately after the key is loaded.

**Reason:** The product-owner asked for "warn at startup if `SUPABASE_SERVICE_ROLE_KEY` is missing." The current code panics on missing, which is a stricter (and arguably better) guarantee — the API cannot function without the service role key for media operations. Relaxing the panic to a warning would be a behavior change with downside risk (silent degradation on upload/download/list/delete). Keeping the panic and adding an info log gives operators visibility into the healthy case without introducing a new failure mode. The info log is gated by a `if !service_role_key.is_empty()` check, but since `.expect()` already filters that case, the log fires unconditionally on every boot.

### 6. Bucket endpoints go on `protected_administrator_router()`

**Decision:** The six new routes (`/media/buckets`, `/media/buckets/{name}`, etc.) are added to `protected_administrator_router()` in `apps/api/src/bin/my-cms-api.rs`, NOT to `protected_router()`.

**Reason:** `protected_router()` allows both `my-headless-cms-writer` and `my-headless-cms-administrator`, which is too permissive for bucket management. `protected_administrator_router()` restricts to `my-headless-cms-administrator` only, which is what the proposal requires. The split means `/media/*` routes are spread across two routers, but the frontend doesn't notice — `getApiUrl('/media/buckets')` resolves to the same base URL regardless of which router mounts it. The existing `/administrator/database/migration` and `/users/*` routes follow the same pattern.

### 7. Bucket endpoints are REST-only

**Decision:** No GraphQL mutations or queries are added for bucket operations.

**Reason:** The proposal explicitly says "GraphQL exposure — Out of scope for this change. Bucket operations are REST-only." The existing media module has a REST-only surface too (no GraphQL for object operations), so this is consistent.

### 8. Reserved bucket name "media" cannot be deleted or recreated

**Decision:** The `CreateBucketHandler` rejects a request body with `name == "media"` with `AppError::Validation("name", "cannot use reserved bucket name 'media'")`. The `DeleteBucketHandler` rejects a path param of `media` with the same error. The `UpdateBucketHandler` and `EmptyBucketHandler` do NOT have this restriction — admins can still toggle the `media` bucket's public flag and empty it if needed.

**Reason:** The `media` bucket is the implicit default for every existing object operation. Accidentally deleting it would break the CMS until the bucket was recreated. The product-owner proposal (open question #7) acknowledges "no concurrency locking" but does not specifically call out a delete guard. This is a small, conservative safety net that follows the same pattern as the user-management "self-delete guard" (Decision 1 in `add-user-management-admin-page/design.md`).

### 9. The `?bucket=<name>` override does NOT affect the public read endpoint or image resize

**Decision:** The endpoints `GET /media/{*path}` and `GET /media/images/{*path}?w=&h=` continue to serve from the `media` bucket regardless of any `?bucket=` query param. Only the admin object operations (`/api/media*`) honor the override.

**Reason:** The product-owner proposal says: "Same for the public image endpoint (`/media/{*path}`) — no change there, it stays on the `media` bucket since the path doesn't carry a bucket segment." The public read endpoint is unauthenticated and uses the `media` bucket's CDN; introducing a per-request bucket override on the public endpoint would require path re-structuring (e.g. `/media/{bucket}/{*path}`), which is out of scope.

### 10. No concurrency locking or optimistic versioning

**Decision:** If two admins delete the same bucket concurrently, the second admin's call gets HTTP 404 with `AppError::NotFound` (the bucket no longer exists by the time the second request reaches Supabase). There is no retry, no optimistic version, no ETag.

**Reason:** The proposal says "no concurrent bucket CRUD from multiple admins for v1." Bucket operations are rare and admin-only; the rare race is acceptable. A follow-up change can introduce optimistic concurrency if real-world usage surfaces contention.

### 11. No audit logging in v1

**Decision:** `#[instrument]` is applied to every new command handler, so bucket operations appear in traces with the bucket name and operation. No persistent audit table is added.

**Reason:** The proposal says "Audit logging out of scope for v1." This matches the convention established by the `add-user-management-admin-page` change (which also defers persistent audit).

## Architecture

### Module layout

```
apps/api/application_core/src/commands/media/
├── mod.rs                          # ADD: pub mod bucket; pub use bucket::{Bucket, BucketConfig}
├── supabase_storage.rs             # MODIFY: add 6 bucket methods + with_bucket override
├── bucket/                         # NEW
│   ├── mod.rs                      # pub mod {list, get, create, update, delete, empty}
│   ├── dto.rs                      # Bucket, BucketConfig, BUCKET_NAME_REGEX
│   ├── list/
│   │   ├── mod.rs
│   │   └── list_handler.rs         # ListBucketsHandlerTrait + struct + impl
│   ├── get/
│   │   ├── mod.rs
│   │   └── get_handler.rs          # GetBucketHandlerTrait + struct + impl
│   ├── create/
│   │   ├── mod.rs
│   │   ├── create_request.rs       # CreateBucketRequest { name, public, file_size_limit?, allowed_mime_types? }
│   │   └── create_handler.rs       # CreateBucketHandlerTrait + struct + impl (validates name, sets public default false)
│   ├── update/
│   │   ├── mod.rs
│   │   ├── update_request.rs       # UpdateBucketRequest { public?, file_size_limit?, allowed_mime_types? }
│   │   └── update_handler.rs       # UpdateBucketHandlerTrait + struct + impl
│   ├── delete/
│   │   ├── mod.rs
│   │   └── delete_handler.rs       # DeleteBucketHandlerTrait + struct + impl (translates 400→409, reserved-name guard)
│   └── empty/
│       ├── mod.rs
│       └── empty_handler.rs        # EmptyBucketHandlerTrait + struct + impl

apps/api/src/api/media/
├── mod.rs                          # MODIFY: pub mod bucket;
├── bucket/                         # NEW
│   ├── mod.rs                      # pub mod {list, get, create, update, delete, empty}
│   ├── list/list_handler.rs        # api_list_buckets
│   ├── get/get_handler.rs          # api_get_bucket (Path<String>)
│   ├── create/create_handler.rs    # api_create_bucket (Json<CreateBucketRequest>)
│   ├── update/update_handler.rs    # api_update_bucket (Path<String>, Json<UpdateBucketRequest>)
│   ├── delete/delete_handler.rs    # api_delete_bucket (Path<String>, Query<DeleteBucketParams>)
│   └── empty/empty_handler.rs      # api_empty_bucket (Path<String>)

apps/api/src/api/media/list/list_handler.rs    # MODIFY: accept ?bucket query param
apps/api/src/api/media/create/create_handler.rs # MODIFY: accept ?bucket query param
apps/api/src/api/media/delete/delete_handler.rs # MODIFY: accept ?bucket query param
apps/api/src/api/media/read/metadata_handler.rs # MODIFY: accept ?bucket query param

apps/api/src/bin/my-cms-api.rs                  # MODIFY: register 6 routes on protected_administrator_router()

apps/web/src/
├── app/admin/media/page.tsx                    # MODIFY: add bucket selector dropdown + URL param
├── app/admin/media/buckets/                    # NEW
│   └── page.tsx                                # AdminBucketsPage
├── components/admin-only-route.tsx             # (existing, reused)
├── schemas/bucket.schema.ts                    # NEW: createBucketSchema, updateBucketSchema
├── models/MediaModels.ts                       # MODIFY: add BucketModel, BucketConfig types
└── App.tsx                                     # MODIFY: register /admin/media/buckets route
```

### `SupabaseStorage` extension

The struct in `apps/api/application_core/src/commands/media/supabase_storage.rs` gains:

```rust
impl SupabaseStorage {
    /// Returns a clone of self with the bucket field replaced.
    pub fn with_bucket(&self, name: &str) -> Self { ... }

    /// GET /storage/v1/bucket
    pub async fn list_buckets(&self) -> Result<Vec<Bucket>, AppError> { ... }

    /// GET /storage/v1/bucket/{name}
    pub async fn get_bucket(&self, name: &str) -> Result<Bucket, AppError> { ... }

    /// POST /storage/v1/bucket with body { name, public, file_size_limit, allowed_mime_types }
    pub async fn create_bucket(&self, req: &CreateBucketPayload) -> Result<Bucket, AppError> { ... }

    /// POST /storage/v1/bucket/{name} with body { public?, file_size_limit?, allowed_mime_types? }
    pub async fn update_bucket(&self, name: &str, req: &UpdateBucketPayload) -> Result<Bucket, AppError> { ... }

    /// POST /storage/v1/bucket/{name}/empty
    pub async fn empty_bucket(&self, name: &str) -> Result<(), AppError> { ... }

    /// DELETE /storage/v1/bucket/{name} with body { "purge": bool }
    pub async fn delete_bucket(&self, name: &str, purge: bool) -> Result<(), AppError> { ... }
}
```

The existing `upload`, `download`, `get_info`, `list_objects`, `delete`, `delete_batch` keep their signatures; their URL templates use `self.bucket` which is overridden by `with_bucket` at the call site.

### `Bucket` / `BucketConfig` DTOs

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    pub id: String,
    pub name: String,
    pub public: bool,
    #[serde(rename = "file_size_limit")]
    pub file_size_limit: Option<u64>,
    #[serde(rename = "allowed_mime_types")]
    pub allowed_mime_types: Option<Vec<String>>,
    pub owner: Option<String>,
    #[serde(rename = "type")]
    pub bucket_type: String,           // "STANDARD" | "ANALYTICS" | etc.
    #[serde(rename = "created_at")]
    pub created_at: String,            // ISO 8601
    #[serde(rename = "updated_at")]
    pub updated_at: String,            // ISO 8601
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBucketRequest {
    pub name: String,
    pub public: Option<bool>,          // default false
    pub file_size_limit: Option<u64>,
    pub allowed_mime_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBucketRequest {
    pub public: Option<bool>,
    pub file_size_limit: Option<Option<u64>>,  // nested Option for "clear the limit"
    pub allowed_mime_types: Option<Option<Vec<String>>>,
}
```

`UpdateBucketRequest`'s nested-Option pattern mirrors the patch-style "absent vs present-null vs present-value" semantics used in `add-user-profile-fields-and-reset-password` (an absent field is "no change"; a present-null field is "clear the value").

### Routing

`protected_administrator_router()` in `apps/api/src/bin/my-cms-api.rs` gains:

```rust
.route(
    "/media/buckets",
    get(api::media::bucket::list::list_handler::api_list_buckets)
        .post(api::media::bucket::create::create_handler::api_create_bucket),
)
.route(
    "/media/buckets/{name}",
    get(api::media::bucket::get::get_handler::api_get_bucket)
        .put(api::media::bucket::update::update_handler::api_update_bucket)
        .delete(api::media::bucket::delete::delete_handler::api_delete_bucket),
)
.route(
    "/media/buckets/{name}/empty",
    post(api::media::bucket::empty::empty_handler::api_empty_bucket),
)
```

No changes to `protected_router()` or `public_router()`. The bucket routes sit behind the same `SupabaseAuthLayer` as `/users/*` (administrator-only).

### Per-request bucket override on object operations

Each affected API handler accepts a new `bucket: Option<String>` query param:

```rust
#[derive(Debug, Deserialize)]
pub struct ListQueryParams {
    pub prefix: Option<String>,
    pub bucket: Option<String>,   // NEW
}

// In api_list_media:
let storage = match &params.bucket {
    Some(name) => state.media_config.storage.with_bucket(name),
    None => state.media_config.storage.clone(),
};
let handler = ListMediaHandler {
    media_config: Arc::new(MediaConfig {
        storage,
        media_base_url: state.media_config.media_base_url.clone(),
    }),
};
```

The `CreateMediaHandler`, `DeleteMediaHandler`, and `MetadataMediaHandler` follow the same pattern. The `CreateMediaHandler` ALSO needs to swap the response URL when an override is in effect (Decision 2 above):

```rust
let url_path = match bucket_override {
    Some(name) => format!("{}/storage/v1/object/{}/{}", supabase_url, name, beautiful_media_name),
    None => if is_image_content_type(&content_type) {
        format!("{}/media/images/{}", media_base_url, beautiful_media_name)
    } else {
        format!("{}/media/{}", media_base_url, beautiful_media_name)
    },
};
```

The `?bucket=` value is validated by each handler before constructing the override storage: `if let Some(name) = &params.bucket { if !is_valid_bucket_name(name) { return Validation("bucket", "..."); } }`.

### Error mapping

| Supabase response | Command handler `AppError` | API HTTP status |
|---|---|---|
| 200/201 success | — | 200/201 |
| 400 malformed body (e.g. create with bad name from Supabase) | `AppError::Validation(field, msg)` | 400 |
| 401/403 (no service role key) | `AppError::StorageError(msg)` | 500 |
| 404 (get/update/delete/empty on missing bucket) | `AppError::NotFound` | 404 |
| 409 (create with existing name) | `AppError::Conflict(msg)` | 409 |
| 400 on DELETE (bucket non-empty, no purge) → translated | `AppError::Conflict("Bucket '<name>' is not empty; pass ?purge=true to delete with all objects")` | 409 |
| 5xx upstream | `AppError::StorageError(sanitised msg)` | 500 |
| Network failure | `AppError::StorageError(msg)` | 500 |
| Invalid bucket name (regex fail) | `AppError::Validation("name"\|"bucket", msg)` | 400 |
| Reserved name "media" on create/delete | `AppError::Validation("name", "cannot use/delete reserved bucket name 'media'")` | 400 |

The existing `AppError::Conflict(String)` variant (added by `add-user-management-admin-page`) is the right variant for both 409 cases. No new `AppError` variants are required.

### Frontend design

**Type definitions** (`apps/web/src/models/MediaModels.ts`):
- `BucketModel` — mirrors the backend DTO: `id`, `name`, `public`, `fileSizeLimit`, `allowedMimeTypes`, `owner`, `type`, `createdAt`, `updatedAt`.
- `CreateBucketRequest` — `{ name, public?, fileSizeLimit?, allowedMimeTypes? }`.
- `UpdateBucketRequest` — `{ public?, fileSizeLimit?, allowedMimeTypes? }`.

**Zod schemas** (`apps/web/src/schemas/bucket.schema.ts`):
- `createBucketSchema` — `name` regex-validated, `public` boolean default `false`, `fileSizeLimit` optional positive number, `allowedMimeTypes` optional array of strings.
- `updateBucketSchema` — same minus `name` (it's a path param, not editable).
- `bucketNameSchema` — `z.string().regex(/^[a-z][a-z0-9_-]{2,62}$/, "...")`, reused for the `?bucket=` query-param parser.

**API helpers** (`apps/web/src/config/api.config.ts`):
- `getBucketsApiUrl()`, `getBucketApiUrl(name)`, `getEmptyBucketApiUrl(name)` — three small URL builders mirroring the existing `getApiUrl` helper.

**`/admin/media/buckets` page** (`apps/web/src/app/admin/media/buckets/page.tsx`):
- List table mirroring the categories page structure (skeleton, empty state, pagination).
- New Bucket button → modal with the create form.
- Per-row Edit → modal with the update form (name read-only).
- Per-row Empty → confirmation dialog → on confirm, `POST /api/media/buckets/{name}/empty`.
- Per-row Delete → confirmation dialog with a "Force delete with all objects" checkbox → on confirm, `DELETE /api/media/buckets/{name}?purge=<bool>`.
- Uses `useAuth().token` via the existing `authenticatedFetch` helper.

**Bucket selector on `/admin/media`** (`apps/web/src/app/admin/media/page.tsx`):
- Read `?bucket=` via `useSearchParams` on mount.
- Bucket list fetched from `GET /api/media/buckets` on mount (cached for the page lifetime).
- DaisyUI `<select>` dropdown in the header next to "Refresh" and "Upload".
- On change: update URL via `setSearchParams({ bucket: newValue })` and refetch.
- Hidden when the bucket list has exactly one entry.

**Route registration** (`apps/web/src/App.tsx`):
```tsx
<Route path="/admin/media/buckets" element={<ProtectedRoute><AdminOnlyRoute><AdminBucketsPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
```

The `AdminOnlyRoute` wrapper is reused from the `add-user-management-admin-page` change.

## Risks / Trade-offs

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Admin accidentally deletes a bucket that has in-use objects | Low | High (broken media) | The `media` bucket is reserved against delete. The 409 on non-empty delete without `?purge=true` forces a two-step intent ("empty first" or "force"). The UI checkbox is explicit. |
| `?bucket=` override on uploads to private buckets returns an unusable URL (no signed-URL flow) | Medium | Low (admin-only) | The spec documents this limitation. A follow-up can add a signed-URL endpoint. For v1, admins who upload to private buckets use the Supabase Studio to share files. |
| Race condition: two admins delete the same bucket | Low | None | Second delete gets `404 BucketNotFound`, which is the correct semantic. |
| 400→409 translation relies on Supabase returning 400 for "bucket non-empty, no purge" | Low | Medium | This is a documented Supabase behavior. If Supabase changes the status code, the `DeleteBucketHandler` will surface the raw error. A unit test in the handler asserts the translation. |
| `SupabaseStorage::clone()` copies the `reqwest::Client` (which is `Arc`'d internally) — cloning 1000× creates 1000 `Arc` clones, not 1000 sockets | Negligible | None | `reqwest::Client` is explicitly designed for cheap cloning. No risk. |
| The `with_bucket` override is silently ignored if a handler forgets to call it | Low | Medium (wrong bucket targeted) | The Decision 1 helper makes "use override" a one-line change at each call site. Code review (task group gate) catches missing call sites. The `Bucket` DTO serialization round-trip test catches URL mismatches. |
| Bucket name regex differs between frontend and backend | Low | Low (validation inconsistency) | The regex is hard-coded in both places; a follow-up could centralize it. For v1, the visual match is sufficient. |

## Migration Plan

No breaking changes. The `media` bucket continues to be the default for every existing endpoint when no `?bucket=` is supplied. The new `media-bucket-management` capability is purely additive.

- **Backend**: deploy with the new `bucket/` module, `SupabaseStorage` extensions, and route registrations. Existing object-operation handlers get a new `?bucket=` query param (Axum ignores unknown query params by default, so no client breakage).
- **Frontend**: deploy with the new `/admin/media/buckets` route and the bucket selector. Existing `/admin/media` callers see the same page (selector is hidden if only one bucket exists).
- **No database migration**: Supabase Storage is the source of truth for buckets.
- **No env var changes**: `SUPABASE_SERVICE_ROLE_KEY` is already required.
- **Rollback**: revert the change's PR; no schema, env, or data changes to undo.