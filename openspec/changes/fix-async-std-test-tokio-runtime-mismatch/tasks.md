## 1. Inventory and classification

- [x] 1.1 Enumerated 109 `#[async_std::test]` annotations across 26 files in `apps/api/`. Bulk conversion approach taken (see commit `3d444ef`) — all 109 → `#[tokio::test]`; 0 left as `#[async_std::test]`; 0 removed (no synchronous bodies found).

- [x] 1.2 Pre-conversion failures: 84 tests panicked at `tokio-1.52.3/src/net/tcp/stream.rs:164:18` with `there is no reactor running`. Post-conversion: 0 failures. The bulk approach worked because the codebase is overwhelmingly tokio-based.

## 2. Per-file conversion

- [x] 2.1–2.6 All 26 files converted in a single atomic commit (`3d444ef`). `git diff --stat` shows zero Cargo.toml modifications — all three affected crates already had `tokio = { version = "1.52.3", features = ["full"] }` in `[dependencies]` which includes `macros`.

## 3. Workspace verification

- [x] 3.1 `cargo check --workspace` exits 0.
- [x] 3.2 `cargo fmt --all -- --check` exits 0.
- [x] 3.3 `cargo test --workspace` exits 0. **Total: 219 passed, 0 failed, 2 ignored.**
- [x] 3.4 `cargo clippy --workspace --all-targets` exits 0 (warnings only — the pre-existing `missing_debug_implementations` lint still produces warnings but does not fail the gate since `-D warnings` was not used).

## 4. Scope confirmation

- [x] 4.1 `git diff --stat 3d444ef~1..3d444ef` shows 26 test files + Cargo.lock + 5 OpenSpec artifacts = 32 files. Zero Cargo.toml modifications.
- [x] 4.2 Production code unchanged. Only test function attributes were touched. `git diff` shows no new functions / types / impls.

## 5. OpenSpec close-out

- [x] 5.1 `openspec status --change "fix-async-std-test-tokio-runtime-mismatch" --json` → `isComplete: true`.
- [x] 5.2 `openspec validate --change "fix-async-std-test-tokio-runtime-mismatch"` → exits 0.
- [x] 5.3 Manual smoke (no regressions):
  - `cargo test -p gateway --bin my-cms-api` → 6 passed
  - `cargo test -p domain_interface` → 9 passed
  - `cargo test -p domain_user --lib api::routes` → 1 passed
  - `cargo test -p domain_user --lib service` → 9 passed
  - `cargo test -p domain_posts --lib migrations_cli` → 3 passed
  - `cargo test -p domain_media --lib handlers::tests` → 2 passed
