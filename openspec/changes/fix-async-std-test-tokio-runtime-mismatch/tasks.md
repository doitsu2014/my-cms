## 1. Inventory and classification

- [ ] 1.1 Enumerate every `#[async_std::test]` annotation in `apps/api/` (109 across 26 files per the design.md inventory). For each, classify as either:
  - **TOKIO** (body or callees use `tokio::*`, `wiremock::*`, `reqwest::*`, or any other tokio-runtime API).
  - **ASYNC_STD** (body uses only async-std APIs).
  - **NEITHER** (synchronous body — these may need no attribute at all).

  Write the classification to a scratch file (e.g. `/tmp/async-std-classification.txt`) so it can be referenced during the conversions.

  **Verification:** `grep -rn "async_std::test" apps/api/ --include="*.rs" | wc -l` returns 109; the scratch file lists one row per annotation with its classification.

- [ ] 1.2 Run `cargo test --workspace 2>&1 | grep -E "FAILED$"` and capture the list of failing test names. Cross-reference with the classification to confirm every failure maps to a `TOKIO` classification.

  **Verification:** `cargo test --workspace` exits non-zero with a list of failures; the failures are all in the `TOKIO` set.

## 2. Per-file conversion

For each file in the inventory (in dependency order: leaf modules first, then aggregator modules):

- [ ] 2.1 Read the entire file.
- [ ] 2.2 For each `#[async_std::test]` classified as `TOKIO`: replace the attribute with `#[tokio::test]`. Verify `tokio = { version = "1.52.3", features = ["macros", "rt"] }` (or `["full"]`) is already in the file's crate's `[dev-dependencies]`. If not, add it. Do NOT change any other code in the test body.
- [ ] 2.3 For each `#[async_std::test]` classified as `ASYNC_STD`: leave the attribute unchanged.
- [ ] 2.4 For each `#[async_std::test]` classified as `NEITHER`: remove the attribute (test becomes a plain `#[test]`).
- [ ] 2.5 Run `cargo test -p <crate> --lib <module>::tests` for the modified crate. Confirm zero failures in this file's tests (pre-existing failures documented separately).
- [ ] 2.6 Commit the file's changes: `git commit -m "fix(tests): convert #[async_std::test] to #[tokio::test] in <file>"`.

**Verification after all files:** `git log --oneline -26` (or however many files converted) shows one commit per file; `git diff --stat` shows zero Cargo.toml modifications.

## 3. Workspace verification

- [ ] 3.1 `cargo check --workspace` exits 0.
- [ ] 3.2 `cargo fmt --all -- --check` exits 0.
- [ ] 3.3 `cargo test --workspace` exits 0. If any test fails for a non-runtime reason, document it in the change summary and continue (the runtime fix is the goal; non-runtime test bugs are a separate follow-up).
- [ ] 3.4 `cargo clippy --workspace --all-targets` exits 0 (warnings only). The `-- -D warnings` flag may still fail on the pre-existing `missing_debug_implementations` lint, which is a separate issue.

## 4. Scope confirmation

- [ ] 4.1 `git diff --stat` shows ONLY test files (under `apps/api/<domain>/src/**/*.rs`, files matching the inventory). Zero Cargo.toml modifications.
- [ ] 4.2 No production code changed. Confirm with `git diff -- 'apps/api/*/src/**/*.rs' | grep -E "^\+[^+]" | grep -v "^+++" | grep -v "// test"` and verify no new functions / types / impls in production paths.

## 5. OpenSpec close-out

- [ ] 5.1 `openspec status --change "fix-async-std-test-tokio-runtime-mismatch" --json` reports `isComplete: true`.
- [ ] 5.2 `openspec validate --change "fix-async-std-test-tokio-runtime-mismatch"` exits 0.
- [ ] 5.3 Manual smoke: `cargo test --workspace` exits 0; rerun the full Slice 1/2/3 verification gates to confirm no regressions:
  - `cargo test -p gateway --bin my-cms-api` — 6 passed
  - `cargo test -p domain_interface` — 9 passed
