## Why

After commit `b2d58d5` enabled the `attributes` feature on `async-std`, `cargo test --workspace` compiles but exits **101** because 84 pre-existing tests panic with `there is no reactor running, must be called from the context of a Tokio 1.x runtime`. The tests use the `#[async_std::test]` attribute but internally call tokio-only APIs (`wiremock::MockServer::start()`, `tokio::net::TcpStream::connect`, etc.).

This pre-existing inconsistency is now the only thing blocking a green `cargo test --workspace`. Slice 1-3 of `wire-all-domains-and-collapse-to-gateway-binary` (now archived) and commit `b2d58d5` already shipped the structural fixes; this change ships the runtime alignment.

## What Changes

- **MODIFIED** 109 test functions across 26 files: replace `#[async_std::test]` with `#[tokio::test]` where the test body calls tokio-only APIs. Files affected:
  - `apps/api/domain_media/src/handlers/supabase_storage.rs` (46)
  - `apps/api/domain_user/src/handlers/supabase_admin_client.rs` (6)
  - `apps/api/domain_user/src/handlers/read_list/read_list_handler.rs` (6)
  - `apps/api/domain_user/src/handlers/create/create_handler.rs` (4)
  - `apps/api/domain_posts/src/handlers/vector_store/vector_store_pg.rs` (4)
  - `apps/api/domain_media/src/handlers/read/read_handler.rs` (4)
  - `apps/api/domain_media/src/handlers/bucket/access/access_handler.rs` (4)
  - 19 other files (1-3 each)
- **UNCHANGED** any test that does NOT call tokio-only APIs — those continue to use `#[async_std::test]`.

## Capabilities

### Modified Capabilities

- `domain-api-cutover` (Verification evidence requirement): the "Focused verification succeeds" scenario now extends to require the workspace test suite exits 0, not just per-crate gates. Pre-existing test failures are no longer acceptable.

## Impact

- Affected tests: 109 annotations, of which ~84 currently fail at runtime. After the change, all 109 should compile and run.
- No production code changes.
- No Cargo.toml changes (the `tokio = { features = ["macros"] }` and `tokio-test` are already in dev-deps where needed).
- No HTTP route changes.
- No new env vars.
- The 11 RED tests that were unblocked by commit `b2d58d5` (slice-1 RED tests) remain GREEN — they don't touch tokio-only APIs.

## Traceability

- Root cause: async_std/tokio runtime inconsistency was the last pre-existing failure documented after the merge of `wire-all-domains-and-collapse-to-gateway-binary` into `refactor/my-cms-api` (commit `9862026`).
- Related fix: commit `b2d58d5` already enabled the `attributes` feature on `async-std`; this change completes the alignment by switching attributes to tokio where the body needs tokio.
