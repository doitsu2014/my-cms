# Tasks: Collapse MediaConfig Bucket into a Single Field

## 1. Collapse MediaConfig and update composition root
- [x] 1.1 In apps/api/application_core/src/commands/media/mod.rs: replace default_bucket + bucket_override with bucket: String.
- [x] 1.2 Remove effective_bucket function and its 2 focused tests from mod.rs.
- [x] 1.3 In apps/api/src/bin/my-cms-api.rs: replace global literal with bucket: "media".to_string().
- [x] 1.4 Verify: rg shows zero effective_bucket definitions; struct has exactly one bucket field.

## 2. Update authenticated object-operation API handlers (parallelizable)
- [x] 2.1 apps/api/src/api/media/create/create_handler.rs — include_bucket_query flag + scoped MediaConfig
- [x] 2.2 apps/api/src/api/media/list/list_handler.rs — same
- [x] 2.3 apps/api/src/api/media/read/metadata_handler.rs — same
- [x] 2.4 apps/api/src/api/media/delete/delete_handler.rs — both paths, no flag
- [x] 2.5 Verify: rg -n 'MediaConfig\s*\{|default_bucket|bucket_override' apps/api/src/api/media — all use only bucket

## 3. Update object-operation command handlers (parallelizable)
- [x] 3.1 commands/media/create/create_handler.rs — upload + URL formatting; include_bucket_query flag
- [x] 3.2 commands/media/list/list_handler.rs — list_objects + URL formatting; include_bucket_query flag
- [x] 3.3 commands/media/read/metadata_handler.rs — get_info + URL formatting; include_bucket_query flag
- [x] 3.4 commands/media/delete/delete_handler.rs — delete + delete_batch; no flag
- [x] 3.5 commands/media/read/read_handler.rs — MediaCacheKey.bucket: String; download + download_render
- [x] 3.6 Verify: rg confirms no old resolution path remains

## 4. Update public reads + visibility gate
- [x] 4.1 api_get_media_image — scoped MediaConfig construction
- [x] 4.2 Image visibility gate reads media_config.bucket.as_str()
- [x] 4.3 api_get_media — same scoped pattern
- [x] 4.4 Preserve invalid-query, visibility, authorization, status-code, response-header, body behavior
- [x] 4.5 Verify: rg -n 'effective_bucket|default_bucket|bucket_override' apps/api/src/api/media/read returns zero

## 5. Update reserved-name policy
- [x] 5.1 bucket/create/create_handler.rs: req.name == self.media_config.bucket.as_str()
- [x] 5.2 bucket/delete/delete_handler.rs: same for deletion
- [x] 5.3 Verify bucket API handlers pass global state.media_config.clone()
- [x] 5.4 Verify rg -n 'default_bucket|bucket_override' returns zero

## 6. Update inline tests + 19 MediaConfig fixtures (parallelizable)
- [x] 6.1 commands/media/mod.rs helper fixture (none — struct only)
- [x] 6.2 create handler (2 fixtures)
- [x] 6.3 list handler (2 fixtures)
- [x] 6.4 metadata handler (2 fixtures)
- [x] 6.5 read handler (cache key + command fixture)
- [x] 6.6 bucket create + delete handlers (fixtures + reserved-name tests)
- [x] 6.7 bucket get/update/empty handlers (1 fixture each)
- [x] 6.8 Run cargo test -p application_core commands::media::{create,list,read,delete}
- [x] 6.9 Run cargo test -p application_core commands::media::bucket + cms api::media::read::read_handler::tests
- [x] 6.10 rg -n 'MediaConfig\s*\{' apps/api — all 20 use only bucket

## 7. Repository verification gate
- [x] 7.1 rg -n 'default_bucket|bucket_override|effective_bucket\s*\(' apps/api → 0 matches
- [x] 7.2 cargo check --workspace
- [x] 7.3 cargo test --workspace
- [x] 7.4 cargo fmt --all -- --check
- [x] 7.5 cargo clippy --workspace --all-targets (record pre-existing; 0 new)
- [x] 7.6 pnpm --dir apps/web build

## 8. OpenSpec archive
- [x] 8.1 openspec validate "collapse-mediaconfig-bucket-into-single-field" --strict (0 issues)
- [x] 8.2 openspec archive "collapse-mediaconfig-bucket-into-single-field" --yes
- [x] 8.3 openspec list --json + ls archive directory
