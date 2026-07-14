# Tasks — media-bucket-management

## Backend — foundation

- [ ] **1. [T] Add `Bucket` and `BucketConfig` DTOs + `BUCKET_NAME_REGEX` constant**
  - Define `Bucket`, `CreateBucketRequest`, `UpdateBucketRequest`. Add `pub const BUCKET_NAME_REGEX: &str = r"^[a-z][a-z0-9_-]{2,62}$";` and `pub fn is_valid_bucket_name`.
  - **Verify:** `cargo check -p application_core`

- [ ] **2. Extend `SupabaseStorage` with `with_bucket` and 6 bucket methods**
  - Add `with_bucket`, `list_buckets`, `get_bucket`, `create_bucket`, `update_bucket`, `empty_bucket`, `delete_bucket` methods.
  - **Verify:** `cargo check -p application_core`

- [ ] **3. [T] WireMock unit tests for the 6 new `SupabaseStorage` bucket methods**
  - Add tests: `list_buckets_returns_array`, `get_bucket_returns_404`, `create_bucket_posts_body`, `update_bucket_posts_to_path`, `empty_bucket_posts_to_empty_path`, `delete_bucket_purge_true/false`, `error_messages_never_include_service_role_key`.
  - **Verify:** `cargo test -p application_core supabase_storage`

- [ ] **4. Register `commands::media::bucket` in `commands::media::mod.rs`**
  - **Verify:** `cargo check`

## Backend — command handlers

- [ ] **5. [T] `ListBucketsHandler` (trait + struct + impl)**
  - Files: `apps/api/application_core/src/commands/media/bucket/list/{mod.rs, list_handler.rs}`. Struct: `pub struct ListBucketsHandler { pub media_config: Arc<MediaConfig> }`. Trait: `fn list_buckets(&self) -> impl Future<Output = Result<Vec<Bucket>, AppError>>`. Calls `self.media_config.storage.list_buckets().await?`. Add `#[instrument]`.
  - **Verify:** `cargo check -p application_core`

- [ ] **6. [T] `GetBucketHandler` (validates name)**
  - Trait: `fn get_bucket(&self, name: &str) -> Result<Bucket, AppError>`. Validates via `is_valid_bucket_name`; returns `AppError::Validation("name", "...")` on fail. Calls `self.media_config.storage.get_bucket(name).await?`.
  - **Verify:** `cargo check -p application_core`

- [ ] **7. [T] `CreateBucketHandler` (validates name, reserved-name guard, public default false)**
  - Trait: `fn create_bucket(&self, req: CreateBucketRequest) -> Result<Bucket, AppError>`. Validates `req.name` (regex + reserved "media" check). Coerces `public` default to `false` when `None`. Calls `self.media_config.storage.create_bucket(&payload).await?`. Maps Supabase 409 to `AppError::Conflict("Bucket '<name>' already exists")`.
  - **Verify:** `cargo check -p application_core`

- [ ] **8. [T] `UpdateBucketHandler` (validates name, empty-body guard)**
  - Trait: `fn update_bucket(&self, name: &str, req: UpdateBucketRequest) -> Result<Bucket, AppError>`. Validates `name`. Rejects empty body (no fields present) with `AppError::Validation("body", "at least one field must be present")`.
  - **Verify:** `cargo check -p application_core`

- [ ] **9. [T] `DeleteBucketHandler` (validates name, reserved-name guard, 400→409 translation)**
  - Trait: `fn delete_bucket(&self, name: &str, purge: bool) -> Result<(), AppError>`. Validates `name`. Rejects `name == "media"` with `AppError::Validation("name", "cannot delete reserved bucket name 'media'")`. In the `StorageError` arm, inspect the message for the Supabase "not empty" pattern; if matched, return `AppError::Conflict("Bucket '<name>' is not empty; pass ?purge=true to delete with all objects")`.
  - **Verify:** `cargo check -p application_core`

- [ ] **10. [T] `EmptyBucketHandler` (validates name)**
  - Trait: `fn empty_bucket(&self, name: &str) -> Result<(), AppError>`. Validates `name`. Calls `self.media_config.storage.empty_bucket(name).await?`.
  - **Verify:** `cargo check -p application_core`

## Backend — API layer

- [ ] **11. Six thin handlers in `apps/api/src/api/media/bucket/`**
  - Files: `apps/api/src/api/media/bucket/{mod.rs, list/list_handler.rs, get/get_handler.rs, create/create_handler.rs, update/update_handler.rs, delete/delete_handler.rs, empty/empty_handler.rs}`. Each mirrors `apps/api/src/api/category/*` exactly. `delete_handler.rs` uses `Query<DeleteBucketParams { pub purge: Option<bool> }>`; `update_handler.rs` validates that at least one field is present.
  - **Verify:** `cargo check`

- [ ] **12. Wire routes into `protected_administrator_router()`**
  - Files: `apps/api/src/bin/my-cms-api.rs`. Add three `.route(...)` blocks for `/media/buckets`, `/media/buckets/{name}`, `/media/buckets/{name}/empty`.
  - **Verify:** `cargo check`

- [ ] **13. Add boot info log confirming service role key is configured**
  - Files: `apps/api/src/bin/my-cms-api.rs`. Add `tracing::info!("Supabase service role key configured; bucket management endpoints enabled")` immediately after the existing `.expect(...)` in `construct_app_state`.
  - **Verify:** `cargo check`

- [ ] **14. Backend verify gate**
  - **Verify:** `cd apps/api && cargo check && cargo test -p application_core && cargo fmt -- --check && cargo clippy --all-targets -- -D warnings`
  - Note: pre-existing clippy errors in `apps/api/application_core/src/commands/ai/*` and `apps/api/application_core/src/common/app_error.rs` are out of scope.

## Backend — `?bucket=` override on object operations

- [ ] **15. Extend `ListQueryParams` and `api_list_media` to accept `?bucket=`**
  - Files: `apps/api/src/api/media/list/list_handler.rs`. Add `pub bucket: Option<String>` to `ListQueryParams`. Validate the bucket name; if present, build `MediaConfig { storage: state.media_config.storage.with_bucket(name), media_base_url: ... }` and pass to handler.
  - **Verify:** `cargo check`

- [ ] **16. Extend `api_create_media` to accept `?bucket=` and swap response URL**
  - Files: `apps/api/src/api/media/create/create_handler.rs`, `apps/api/application_core/src/commands/media/create/create_handler.rs`. Extend trait method with optional bucket override; when present, use `{SUPABASE_URL}/storage/v1/object/{bucket}/{path}` for response URL.
  - **Verify:** `cargo check`

- [ ] **17. Extend `api_delete_media` and `api_delete_media_batch` to accept `?bucket=`**
  - Files: `apps/api/src/api/media/delete/delete_handler.rs`, `apps/api/application_core/src/commands/media/delete/delete_handler.rs`. Extend trait methods with optional bucket override; pass through to `storage.delete` / `storage.delete_batch`.
  - **Verify:** `cargo check`

- [ ] **18. Extend `api_get_media_metadata` to accept `?bucket=`**
  - Files: `apps/api/src/api/media/read/metadata_handler.rs`. Add `Query<MetadataQueryParams { bucket: Option<String> }>`.
  - **Verify:** `cargo check`

- [ ] **19. Backend verify gate (post-override changes)**
  - **Verify:** `cd apps/api && cargo check && cargo test && cargo fmt -- --check && cargo clippy --all-targets -- -D warnings`

## Frontend — foundation

- [ ] **20. Domain types in `apps/web/src/models/MediaModels.ts`**
  - Add `BucketModel`, `CreateBucketRequest`, `UpdateBucketRequest` interfaces.
  - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [ ] **21. Zod schemas in `apps/web/src/schemas/bucket.schema.ts`**
  - Create `createBucketSchema`, `updateBucketSchema`, `bucketNameSchema` (regex matches `^[a-z][a-z0-9_-]{2,62}$`).
  - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [ ] **22. API URL helpers in `apps/web/src/config/api.config.ts`**
  - Add `getBucketsApiUrl()`, `getBucketApiUrl(name)`, `getEmptyBucketApiUrl(name)`.
  - **Verify:** `pnpm --dir apps/web tsc --noEmit`

## Frontend — pages

- [ ] **23. `/admin/media/buckets` page (list + create + edit + empty + delete modals)**
  - Files: `apps/web/src/app/admin/media/buckets/page.tsx`. Mirror the categories page structure. Modals: Create (name + public toggle + fileSizeLimit + allowedMimeTypes text), Edit (same minus name), Empty (confirmation only), Delete (confirmation + "Force delete with all objects" checkbox). Each modal uses Zod schema from task 21.
  - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [ ] **24. Register `/admin/media/buckets` route in `App.tsx`**
  - Files: `apps/web/src/App.tsx`. Add `<Route>` wrapped in `<AdminOnlyRoute>`.
  - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [ ] **25. Bucket selector dropdown on `/admin/media`**
  - Files: `apps/web/src/app/admin/media/page.tsx`. Add `useSearchParams`, fetch bucket list on mount, render DaisyUI `<select>` in header next to "Refresh"/"Upload" only if `buckets.length > 1`. On change, call `setSearchParams({ bucket: newValue })` and refetch. Pass `?bucket=<value>` on every object API call. Add "Buckets" link in header.
  - **Verify:** `pnpm --dir apps/web tsc --noEmit`

## Frontend — verification

- [ ] **26. `pnpm build` clean**
  - **Verify:** `pnpm --dir apps/web build`

- [ ] **27. Full verification gate**
  - **Verify:** `cd apps/api && cargo check && cargo test && cargo fmt -- --check && cargo clippy --all-targets -- -D warnings && pnpm --dir apps/web build`

- [ ] **28. Manual smoke against local Supabase stack**
  - Start local stack (`deployments/docker-swarm/`).
  - Sign in as seeded administrator; navigate to `/admin/media/buckets`; list buckets; create private bucket `test-private` (with `fileSizeLimit=1024`, `allowedMimeTypes=["image/png"]`); verify new bucket appears.
  - Upload a small image to `test-private` via `/admin/media?bucket=test-private`; verify upload succeeds.
  - Edit `test-private` and toggle `public=true`; verify change persists.
  - Try to delete `test-private` without `?purge=true`; verify API returns 409 and UI shows conflict message.
  - Empty `test-private` via Empty button; verify all files removed but bucket remains.
  - Delete `test-private` (now empty); verify bucket is gone.
  - Try to delete `media` bucket; verify API returns 400 with reserved-name message.
  - Verify no `SERVICE_ROLE_KEY` appears in API response bodies or log output.
  - Confirm boot log includes "Supabase service role key configured" info line.

## Hand-off

When all 28 tasks are complete and the verification gate passes, the change is ready for the coder to archive:

1. `openspec-verify-change media-bucket-management`
2. `openspec-sync-specs media-bucket-management`
3. `openspec-archive-change media-bucket-management`