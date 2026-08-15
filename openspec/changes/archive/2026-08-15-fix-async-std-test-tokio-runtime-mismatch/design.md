## Context

### Source-derived current state (revalidated 2026-08-09)

After commit `b2d58d5` enabled the `attributes` feature on `async-std`, the workspace test suite compiles but **exits 101**. Specifically:

- `cargo test --workspace` reports **42 passed, 59 failed** in `domain_media --lib` alone (mostly `handlers::supabase_storage::tests::*`).
- All 59 failures share the same panic site: `tokio-1.52.3/src/net/tcp/stream.rs:164:18` with the message `there is no reactor running, must be called from the context of a Tokio 1.x runtime`.
- The tests are annotated `#[async_std::test]` but the bodies call `wiremock::MockServer::start()` (which internally uses `tokio::net::TcpListener::bind()`).

This is a pre-existing inconsistency in the codebase that pre-dates the `wire-all-domains-and-collapse-to-gateway-binary` work. It was first documented by the SE during the final integration step (commit `b2d58d5`).

### Affected test inventory

```
46 apps/api/domain_media/src/handlers/supabase_storage.rs
 6 apps/api/domain_user/src/handlers/supabase_admin_client.rs
 6 apps/api/domain_user/src/handlers/read_list/read_list_handler.rs
 4 apps/api/domain_user/src/handlers/create/create_handler.rs
 4 apps/api/domain_posts/src/handlers/vector_store/vector_store_pg.rs
 4 apps/api/domain_media/src/handlers/read/read_handler.rs
 4 apps/api/domain_media/src/handlers/bucket/access/access_handler.rs
 3 apps/api/domain_user/src/handlers/reset_password/reset_password_handler.rs
 3 apps/api/domain_user/src/handlers/modify/modify_handler.rs
 2 apps/api/domain_user/src/handlers/read_one/read_one_handler.rs
 2 apps/api/domain_user/src/handlers/delete/delete_handler.rs
 2 apps/api/domain_posts/src/handlers/category/modify/modify_handler.rs
 2 apps/api/domain_posts/src/handlers/category/create/create_handler.rs
 2 apps/api/domain_media/src/handlers/read/metadata_handler.rs
 2 apps/api/domain_media/src/handlers/list/list_handler.rs
 2 apps/api/domain_media/src/handlers/create/create_handler.rs
 2 apps/api/domain_media/src/handlers/bucket/update/update_handler.rs
 2 apps/api/domain_media/src/handlers/bucket/get/get_handler.rs
 2 apps/api/domain_media/src/handlers/bucket/delete/delete_handler.rs
 2 apps/api/domain_media/src/handlers/bucket/create/create_handler.rs
 1 apps/api/domain_posts/src/migrations/m20240409_151952_release_100.rs
 1 apps/api/domain_posts/src/handlers/tag_helper/read/read_handler.rs
 1 apps/api/domain_posts/src/handlers/tag_helper/create/create_handler.rs
 1 apps/api/domain_posts/src/handlers/category/read/category_read_handler.rs
 1 apps/api/domain_posts/src/handlers/category/delete/delete_handler.rs
 1 apps/api/domain_media/src/handlers/mod.rs
 1 apps/api/domain_media/src/handlers/bucket/empty/empty_handler.rs

Total: 109 #[async_std::test] annotations across 26 files.
```

### Constraint: do not change semantics

The conversion must not change test semantics — only the runtime. If a test fails after the conversion for a different reason than the runtime mismatch, that is a pre-existing test bug NOT covered by this change.

## Goals / Non-Goals

**Goals**
- All 109 `#[async_std::test]` annotations in test functions whose body calls tokio-only APIs are converted to `#[tokio::test]`.
- `cargo test --workspace` exits 0.
- No production code is changed.
- No HTTP route, env var, or migration is added/removed/changed.

**Non-Goals**
- No fix for tests that fail for non-runtime reasons (e.g. wrong assertion, missing setup). Document any such failure as a separate, follow-up issue.
- No switch of every test from async_std to tokio; only the ones whose body needs tokio.
- No refactor of test setup code (mock servers, fixtures, etc.).
- No deletion of any test.

## Decisions

### Decision 1: Selective attribute conversion

**Driver.** 109 annotations across 26 files. Converting all of them blindly may break async-std-only tests (if any exist). Inspecting each test body is the only safe approach.

**Decision.** Inspect each `#[async_std::test]` test body. If the body (or any helper it calls) uses `tokio::*`, `wiremock::*`, `reqwest::*` (async-std vs tokio are different), `moka::future::*`, or any other API known to require a Tokio runtime, convert the attribute to `#[tokio::test]`. Otherwise leave it as `#[async_std::test]`.

**Alternatives considered.**
- (a) **Convert all 109 to `#[tokio::test]`.** *Considered, possibly safer than per-test inspection.* But async-std-only tests (e.g. ones calling `async_std::fs::*`) would break. The codebase does not have many such tests, but the risk is real.
- (b) **Keep `#[async_std::test]` and add a tokio runtime inside the test body.** *Rejected*: pollutes test code with `tokio::runtime::Runtime::new().unwrap().block_on(...)` boilerplate.

**Consequences.** Per-test inspection adds ~30 minutes of work but is the safest path. Final count may be ~50-90 conversions (less than 109 if some async-std tests genuinely don't need tokio).

### Decision 2: No Cargo.toml changes

**Driver.** The `tokio` crate is already a dev-dependency with the `macros` feature in every crate that needs it. No new dependency is required.

**Decision.** Do NOT modify any Cargo.toml. Use only the existing toolchain.

**Verification.** `git diff --stat` after the change shows zero Cargo.toml modifications.

### Decision 3: Conversion verification is per-file, not whole-workspace

**Driver.** With ~100 test functions across 26 files, a per-function verification is too noisy. Per-file is the right granularity.

**Decision.** After converting all `#[async_std::test]` in one file, run `cargo test -p <crate> --lib <module>::tests`. Repeat for each file. After all files are converted, run `cargo test --workspace` once for the final exit-0 confirmation.

**Verification.** `cargo test --workspace` exits 0; pre-existing non-runtime failures are documented separately.

## Risks / Trade-offs

**[Risk]** Per-test inspection may misclassify a test (e.g. miss a tokio API call). → **Mitigation:** after conversion, run `cargo test --workspace`. Any remaining failure gets re-inspected.

**[Risk]** Some tests may fail for a non-runtime reason after the conversion (e.g. assertion error, missing fixture). → **Mitigation:** document each such failure in the change summary; treat as a separate follow-up.

**[Risk]** Test code review may be noisy. → **Mitigation:** group conversions by file in commits (one commit per file or per crate, depending on size). 26 small commits or 3 medium commits are both acceptable; pick the smallest reviewable unit.

## Migration Plan

Single atomic commit (or per-file commits if the diff is large). No schema change. No rollback plan needed beyond `git revert`.
