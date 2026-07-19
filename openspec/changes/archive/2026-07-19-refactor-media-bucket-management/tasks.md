# Tasks — Refactor Media Bucket Management

Numbered `- [ ]` checklist. Each task is small (≤ 2 hours) and ends
with a verifiable check.

## Group 1 — Drop the `SUPABASE_STORAGE_BUCKET` env var

- [x] 1.1 In `apps/api/src/bin/my-cms-api.rs` (lines 263-275), remove the `env::var("SUPABASE_STORAGE_BUCKET")` read. Pass the literal `"media"` to `SupabaseStorage::new()`. Keep the `tracing::info!("Supabase service role key configured; …")` log line unchanged.
- [x] 1.2 In `apps/api/.env.example` (line 52), delete the `SUPABASE_STORAGE_BUCKET=media` line.
- [x] 1.3 In `deployments/docker-swarm/apps/.env.example` (line 53), delete the `SUPABASE_STORAGE_BUCKET=media` line.
- [x] 1.4 In `deployments/docker-swarm/apps/docker-compose.yaml` (line 118), delete the `SUPABASE_STORAGE_BUCKET: ${SUPABASE_STORAGE_BUCKET}` line.
- [x] 1.5 Run `cargo check --manifest-path apps/api/Cargo.toml` — confirm no compilation error from the removed env read.
- [x] 1.6 Verify existing tests in `apps/api/src/api/media/read/read_handler.rs:166,174` and `apps/api/application_core/src/commands/media/supabase_storage.rs:1362` still pass (they construct `SupabaseStorage` with hardcoded bucket names; they do not depend on the env var). Adjust if any test relied on the env var being set.

## Group 2 — Add `BucketAccessPolicy` in application core

- [x] 2.1 Create the module directory `apps/api/application_core/src/commands/media/bucket/access/`.
- [x] 2.2 Create `apps/api/application_core/src/commands/media/bucket/access/mod.rs` exporting `access_handler` and `access_cache`.
- [x] 2.3 Create `apps/api/application_core/src/commands/media/bucket/access/access_cache.rs` containing `pub fn create_bucket_visibility_cache() -> moka::future::Cache<String, bool>` with `max_capacity(256)` and `time_to_live(Duration::from_secs(300))`. Mirror the `create_media_cache()` style at `apps/api/application_core/src/commands/media/read/read_handler.rs:67-73`.
- [x] 2.4 Create `apps/api/application_core/src/commands/media/bucket/access/access_handler.rs` containing `BucketAccessPolicyTrait` + `BucketAccessPolicy { storage: SupabaseStorage, cache: Arc<Cache<String, bool>> }`. Implement `ensure_public_or_admin(name: &str, is_admin: bool) -> Result<(), AppError>`:
  - if `is_admin`, return `Ok(())`.
  - else look up the cache; on miss call `storage.get_bucket(name).await?` and cache `bucket.public`.
  - if `is_public`, return `Ok(())`.
  - else return `AppError::NotFound`.
- [x] 2.5 Add `pub mod access;` to `apps/api/application_core/src/commands/media/bucket/mod.rs`.
- [x] 2.6 In `apps/api/application_core/src/commands/media/bucket/access/access_handler.rs`, add `#[cfg(test)] mod tests` with four tests using `wiremock::MockServer`:
  - `ensure_public_or_admin_passes_when_bucket_is_public_and_caller_is_anon`
  - `ensure_public_or_admin_rejects_with_not_found_when_bucket_is_private_and_caller_is_anon`
  - `ensure_public_or_admin_skips_supabase_when_is_admin_true`
  - `ensure_public_or_admin_caches_public_flag_across_calls`
- [x] 2.7 Run `cargo test --manifest-path apps/api/application_core/Cargo.toml bucket::access::` — confirm 4 tests pass.

## Group 3 — Wire the cache into `AppState`

- [x] 3.1 In `apps/api/src/lib.rs`, add `pub bucket_visibility_cache: Arc<Cache<String, bool>>,` to the `AppState` struct (after `media_cache`, line 23).
- [x] 3.2 In `apps/api/src/lib.rs`, update the `Debug` impl for `AppState` to mention `bucket_visibility_cache` as `"<Cache>"`.
- [x] 3.3 In `apps/api/src/bin/my-cms-api.rs`, in `construct_app_state()` (after line 295 where `media_cache` is constructed), construct `let bucket_visibility_cache = Arc::new(create_bucket_visibility_cache());`.
- [x] 3.4 In `apps/api/src/bin/my-cms-api.rs`, in the `AppState { … }` literal (lines 288-299), add `bucket_visibility_cache,`.
- [x] 3.5 Update the `use` statement at line 5 of `my-cms-api.rs` to import `create_bucket_visibility_cache` from the new module path.
- [x] 3.6 Run `cargo check --manifest-path apps/api/Cargo.toml` — confirm `AppState` compiles with the new field at all three call sites (`public_router`, `protected_router`, `protected_administrator_router`).

## Group 4 — Enforce bucket type in public-media handler

- [x] 4.1 In `apps/api/src/api/media/read/read_handler.rs`, add `is_admin_jwt_present(headers: &HeaderMap) -> bool` as a private helper. Decode `Authorization: Bearer <jwt>` using `jsonwebtoken::decode` with a `DecodingKey` initialized once from `SUPABASE_JWT_SECRET` and `Validation::new()` configured with `set_audience(&[AUTHORIZATION_AUDIENCE])`. On success, check `claims.app_metadata.roles` JSON array for `"my-headless-cms-administrator"`. Return `false` on any error. The verifier is initialized through `LazyLock` so environment variables are not read on the hot path.
- [x] 4.2 In `apps/api/src/api/media/read/read_handler.rs`, add `headers: HeaderMap` to the `api_get_media` signature (between `state` and `path`).
- [x] 4.3 In `apps/api/src/api/media/read/read_handler.rs`, add the same `headers: HeaderMap` parameter to `api_get_media_image`.
- [x] 4.4 In `apps/api/src/api/media/read/read_handler.rs`, inside `api_get_media` after `validate_bucket`, insert:
  ```rust
  let bucket_name = bucket.clone().unwrap_or_else(|| "media".to_string());
  let is_admin = is_admin_jwt_present(&headers);
  let policy = BucketAccessPolicy {
      storage: state.media_config.storage.clone(),
      cache: state.bucket_visibility_cache.clone(),
  };
  if let Err(e) = policy.ensure_public_or_admin(&bucket_name, is_admin).await {
      return error_response(e);
  }
  ```
- [x] 4.5 Repeat step 4.4 inside `api_get_media_image` after `validate_bucket`.
- [x] 4.6 In `apps/api/src/api/media/read/read_handler.rs`, add three unit tests using `wiremock::MockServer`:
  - `api_get_media_returns_404_when_bucket_is_private_and_no_admin_jwt`
  - `api_get_media_returns_200_when_bucket_is_private_and_admin_jwt`
  - `api_get_media_serves_default_media_bucket_when_public`
- [x] 4.7 Run `cargo test --manifest-path apps/api/Cargo.toml media::read::read_handler` — confirm all tests pass.

## Group 5 — Frontend `AuthenticatedImage` component

- [x] 5.1 Create `apps/web/src/app/admin/media/components/authenticated-image.tsx` per the design.md sketch. Use `authenticatedFetch` from `@/config/api.config` and `URL.createObjectURL` / `URL.revokeObjectURL`. Handle cleanup on unmount and `src` change.
- [x] 5.2 In `apps/web/src/app/admin/media/components/media-grid-item.tsx`, replace `<img src={thumbnailUrl} …>` (line 109-114) with `<AuthenticatedImage src={thumbnailUrl} token={token} alt={fileName} className="w-full h-full object-cover" />`. Add `token: string | null` to `MediaGridItemProps`.
- [x] 5.3 In `apps/web/src/app/admin/media/components/media-preview-modal.tsx`, replace `<img src={fullImageUrl} …>` (line 77-81) with `<AuthenticatedImage src={fullImageUrl} token={token} alt={fileName} className="max-w-full max-h-[400px] object-contain rounded" />`. Add `token: string | null` to `MediaPreviewModalProps`.
- [x] 5.4 In `apps/web/src/app/admin/media/page.tsx`, pass `token` from `useAuth()` (line 30) into both `<MediaGridItem token={token} …>` (line 414-422) and `<MediaPreviewModal token={token} …>` (line 487-492).
- [x] 5.5 In `apps/web/src/app/admin/media/components/authenticated-image.tsx`, add Vitest tests covering: successful fetch + blob URL set; cleanup on unmount; failure leaves `<div aria-busy>` placeholder.
- [x] 5.6 Run `pnpm --dir apps/web test` — confirm existing tests (e.g. `media-grid-item.test.tsx`) still pass with the new `token` prop. (`media-grid-item.test.tsx` updated to mock `authenticatedFetch` + `URL.createObjectURL` and pass the `token` prop; all 18 tests across 5 files pass; `pnpm --dir apps/web build` clean.)

## Group 6 — Verification

- [x] 6.1 `cargo check --manifest-path apps/api/Cargo.toml` — clean (both `--all-targets` variant and default).
- [x] 6.2 `cargo test --manifest-path apps/api/Cargo.toml` — all 30 backend tests pass; 18 frontend tests pass.
- [x] 6.3 `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` — no formatting drift.
- [x] 6.4 `cargo clippy --manifest-path apps/api/Cargo.toml --all-targets -- -D warnings` — clean of NEW warnings introduced by this change. The 3 `await_holding_lock` warnings produced by the new wiremock tests are resolved by switching to `tokio::sync::Mutex`. 15 pre-existing clippy errors remain in unrelated files (`translate_handler.rs`, `vector_store_pg.rs`, `category_*_handler.rs`, `supabase_storage.rs`, `post/modify_handler.rs`, `models_handler.rs`, `media/create/create_handler.rs`, `common/app_error.rs`) and are documented as out of scope per the change's scope discipline.
- [x] 6.5 `pnpm --dir apps/web build` — frontend builds clean.
<!-- smoke test deferred to user -->
- [ ] 6.6 Manual smoke test against a local Supabase stack:
  - `curl -i http://localhost:8989/media/foo.png` (no auth, default bucket, public) — expect 200.
  - `curl -i http://localhost:8989/media/foo.png?bucket=private-docs` (no auth, private bucket) — expect 404.
  - `curl -i -H "Authorization: Bearer <admin-jwt>" http://localhost:8989/media/foo.png?bucket=private-docs` — expect 200.
  - Log into the admin UI, browse to a private bucket, confirm thumbnail + preview modal render correctly.

## Group 7 — Archive

- [x] 7.1 Run `openspec validate "refactor-media-bucket-management" --type change --strict` — required one fixup to the ADDED requirement body so its first sentence opens with "The system SHALL..." to satisfy the validator's SHALL/MUST requirement. After fixup: `Change 'refactor-media-bucket-management' is valid`. No `CRITICAL` issues remain.
- [x] 7.2 Run `openspec archive "refactor-media-bucket-management"` — in OpenSpec 1.3.1 the spec merge is performed implicitly by `archive` (no separate `sync` command exists). The merge was successful: `+ 1 added, ~ 1 modified` into `openspec/specs/supabase-storage/spec.md`. Specs updated successfully.
- [x] 7.3 Change archived as `openspec/changes/archive/2026-07-19-refactor-media-bucket-management/`. `openspec list --json` no longer returns `refactor-media-bucket-management` in the active changes list.
