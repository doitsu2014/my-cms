# Tasks: Lift Media Default Bucket onto MediaConfig

## 1. Configuration ownership and bucket-neutral storage

- [x] 1.1 Add focused tests for default and override resolution in `apps/api/application_core/src/commands/media/mod.rs`, covering `None -> default_bucket` and `Some("avatars") -> "avatars"`.
- [x] 1.2 Add `MediaConfig.default_bucket: String` and the shared `effective_bucket(config, override_)` helper in `apps/api/application_core/src/commands/media/mod.rs`.
- [x] 1.3 Remove `SupabaseStorage.bucket`, its `Debug` field, the bucket argument and assignment in `SupabaseStorage::new`, and the complete `with_bucket` method from `apps/api/application_core/src/commands/media/supabase_storage.rs`.
- [x] 1.4 Update `apps/api/src/bin/my-cms-api.rs` so `SupabaseStorage::new` is bucket-neutral and `MediaConfig.default_bucket` is initialized once with `"media".to_string()`; review `apps/api/src/lib.rs` and retain the existing `AppState::Debug` implementation because it already formats `MediaConfig`.
- [x] 1.5 Remove bucket fields from direct `SupabaseStorage` test fixtures in `apps/api/application_core/src/commands/media/supabase_storage.rs`, `apps/api/application_core/src/commands/media/read/read_handler.rs`, `apps/api/application_core/src/commands/media/bucket/access/access_handler.rs`, and `apps/api/src/api/media/read/read_handler.rs`.
- [x] 1.6 Add `default_bucket` and remove the obsolete `SupabaseStorage::new` bucket argument in every `MediaConfig` fixture in `apps/api/application_core/src/commands/media/create/create_handler.rs`, `apps/api/application_core/src/commands/media/list/list_handler.rs`, `apps/api/application_core/src/commands/media/read/metadata_handler.rs`, and the bucket-management test fixtures (create/delete/empty/get/update handlers).
- [x] 1.7 Verify Group 1 with `cargo check --manifest-path apps/api/Cargo.toml --workspace`; record all expected remaining compile errors as callers of the nine changed methods or remaining `MediaConfig` literals.

## 2. Explicit bucket arguments on all object operations

- [x] 2.1 Update `public_url` tests in `apps/api/application_core/src/commands/media/supabase_storage.rs` to pass `"media"` and `"contract-bucket"` explicitly.
- [x] 2.2 Change `SupabaseStorage::public_url` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept `bucket: &str` first and build the URL from that argument.
- [x] 2.3 Update render URL tests and change `SupabaseStorage::render_image_url` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept and use an explicit bucket while preserving optional width/height behavior.
- [x] 2.4 Update every direct `download_render` test in `apps/api/application_core/src/commands/media/supabase_storage.rs` to pass `"media"`, `"avatars"`, or `"xx"` explicitly based on its wiremock path.
- [x] 2.5 Change `SupabaseStorage::download_render` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept `bucket: &str` and forward it unchanged to `render_image_url`.
- [x] 2.6 Update all direct `upload` tests in `apps/api/application_core/src/commands/media/supabase_storage.rs` to pass the expected bucket explicitly.
- [x] 2.7 Change `SupabaseStorage::upload` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept `bucket: &str` first and use it in the object URL without changing byte, content-type, or cache-control parameter types.
- [x] 2.8 Update direct download tests and change `SupabaseStorage::download` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept and use an explicit bucket.
- [x] 2.9 Update direct metadata tests and change `SupabaseStorage::get_info` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept and use an explicit bucket.
- [x] 2.10 Update direct list tests and change `SupabaseStorage::list_objects` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept and use an explicit bucket while preserving `Option<&str>` prefix behavior.
- [x] 2.11 Update direct single-delete tests and change `SupabaseStorage::delete` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept and use an explicit bucket.
- [x] 2.12 Update direct batch-delete tests and change `SupabaseStorage::delete_batch` in `apps/api/application_core/src/commands/media/supabase_storage.rs` to accept and use an explicit bucket while preserving `&[String]`.
- [x] 2.13 Replace `with_bucket_returns_clone_with_replaced_bucket` and the contract-bucket clone setup in `apps/api/application_core/src/commands/media/supabase_storage.rs` with a test proving one storage client can target two explicit buckets without mutation.
- [x] 2.14 Verify Group 2 with `cargo test --manifest-path apps/api/Cargo.toml -p application_core commands::media::supabase_storage::tests`.

## 3. Media application and API call sites

- [x] 3.1 Resolve `bucket_name` with `effective_bucket` and pass it to `upload` in `apps/api/application_core/src/commands/media/create/create_handler.rs`; retain existing response-URL behavior and update both inline tests.
- [x] 3.2 Remove `with_bucket` from `apps/api/src/api/media/create/create_handler.rs`; clone bucket-neutral storage and copy `default_bucket` into the request-scoped `MediaConfig`.
- [x] 3.3 Resolve `bucket_name` and pass it to `list_objects` in `apps/api/application_core/src/commands/media/list/list_handler.rs`; retain prefix and response-URL behavior and update both inline tests.
- [x] 3.4 Remove `with_bucket` from `apps/api/src/api/media/list/list_handler.rs`; clone bucket-neutral storage and copy `default_bucket`.
- [x] 3.5 Resolve `bucket_name` and pass it to `get_info` in `apps/api/application_core/src/commands/media/read/metadata_handler.rs`, then remove `with_bucket` and copy `default_bucket` in `apps/api/src/api/media/read/metadata_handler.rs`; update override and default tests.
- [x] 3.6 Resolve `bucket_name` and pass it to `delete` and `delete_batch` in `apps/api/application_core/src/commands/media/delete/delete_handler.rs`.
- [x] 3.7 Remove both `with_bucket` branches in `apps/api/src/api/media/delete/delete_handler.rs`; clone bucket-neutral storage and copy `default_bucket` for single and batch handlers.
- [x] 3.8 Refactor `ReadMediaHandler` and its constructors in `apps/api/application_core/src/commands/media/read/read_handler.rs` to hold `Arc<MediaConfig>` instead of a bucket-bearing `Arc<SupabaseStorage>`.
- [x] 3.9 Remove both storage-cloning branches in `apps/api/application_core/src/commands/media/read/read_handler.rs`; resolve the effective bucket and pass it to `download_render` and `download`.
- [x] 3.10 Remove `resolve_storage` and the hard-coded `"media"` fallbacks from `apps/api/src/api/media/read/read_handler.rs`; use shared `effective_bucket` for the visibility gate and construct `ReadMediaHandler` from `state.media_config.clone()`.
- [x] 3.11 Replace the `resolve_storage` field-assertion tests and update public-read visibility tests in `apps/api/src/api/media/read/read_handler.rs`; update read-command tests in `apps/api/application_core/src/commands/media/read/read_handler.rs` for explicit default and override buckets.
- [x] 3.12 Verify Group 3 with `cargo test --manifest-path apps/api/Cargo.toml -p application_core commands::media` and `cargo test --manifest-path apps/api/Cargo.toml -p cms api::media::read::read_handler::tests`.

## 4. Bucket-management policy and fixtures

- [x] 4.1 Remove `RESERVED_BUCKET_NAME` and compare `CreateBucketRequest.name` with `self.media_config.default_bucket` in `apps/api/application_core/src/commands/media/bucket/create/create_handler.rs`; update the reserved-name test fixture.
- [x] 4.2 Remove `RESERVED_BUCKET_NAME` and compare the delete target with `self.media_config.default_bucket` in `apps/api/application_core/src/commands/media/bucket/delete/delete_handler.rs`; update the reserved-name test fixture.
- [x] 4.3 Add `default_bucket` and remove the old storage-constructor bucket argument in fixtures in `apps/api/application_core/src/commands/media/bucket/get/get_handler.rs`, `apps/api/application_core/src/commands/media/bucket/update/update_handler.rs`, and `apps/api/application_core/src/commands/media/bucket/empty/empty_handler.rs`.
- [x] 4.4 Verify in `apps/api/application_core/src/commands/media/supabase_storage.rs` and `apps/api/application_core/src/commands/media/bucket/access/access_handler.rs` that `list_buckets`, `get_bucket`, `create_bucket`, `update_bucket`, `empty_bucket`, and `delete_bucket` retain their existing explicit contracts and pass their existing tests.
- [x] 4.5 Verify Group 4 with `cargo test --manifest-path apps/api/Cargo.toml -p application_core commands::media::bucket`.

## 5. Repository verification gate

- [x] 5.1 Run `rg -n '\.bucket\b|bucket:\s*' apps/api/application_core/src/commands/media/supabase_storage.rs apps/api/application_core/src/commands/media/read/read_handler.rs apps/api/application_core/src/commands/media/bucket/access/access_handler.rs apps/api/src/api/media/read/read_handler.rs` and confirm no `SupabaseStorage.bucket` field access or initializer remains.
- [x] 5.2 Run `rg -n 'with_bucket\s*\(' apps/api` and confirm there are zero matches.
- [x] 5.3 Run method-specific searches for `upload`, `download`, `get_info`, `list_objects`, `delete`, `delete_batch`, `public_url`, `render_image_url`, and `download_render` under `apps/api`; confirm every `SupabaseStorage` object call passes a bucket first.
- [x] 5.4 Run `cargo fmt --manifest-path apps/api/Cargo.toml --all -- --check` and `cargo clippy --manifest-path apps/api/Cargo.toml --workspace --all-targets -- -D warnings`. NOTE: `cargo fmt --check` is clean (exit 0). `cargo clippy -D warnings` fails with 19 pre-existing `useless_vec` errors in `translate_handler.rs` (lib test) — out of scope. Re-run without `-D warnings` shows 29 total warnings: 27 in unrelated files (translate_handler, supabase_auth, app_error, vector_store_pg, post_modify, category_modify, etc.) and 2 in PR-touched files (`supabase_storage.rs:728`, `create_handler.rs:30`) that are pre-existing (`git blame` confirms lines predating this PR). **0 new warnings introduced by this change.**
- [x] 5.5 Run `cargo check --manifest-path apps/api/Cargo.toml --workspace` and `cargo test --manifest-path apps/api/Cargo.toml --workspace`.
- [x] 5.6 Run `pnpm --dir apps/web build` to complete the repository verification gate even though no frontend source is changed.

## 6. OpenSpec verification and archive

- [x] 6.1 Run `openspec status --change "lift-media-default-bucket-onto-mediaconfig" --json` and `openspec verify --change "lift-media-default-bucket-onto-mediaconfig"`; resolve all critical findings.
- [x] 6.2 Run `openspec sync --change "lift-media-default-bucket-onto-mediaconfig"` and verify the delta is merged into `openspec/specs/supabase-storage/spec.md`.
- [x] 6.3 Run `openspec archive "lift-media-default-bucket-onto-mediaconfig"` and confirm the change appears under `openspec/changes/archive/<date>-lift-media-default-bucket-onto-mediaconfig/`.