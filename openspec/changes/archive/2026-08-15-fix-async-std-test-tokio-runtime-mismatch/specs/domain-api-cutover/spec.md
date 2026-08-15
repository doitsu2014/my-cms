## ADDED Requirements

### Requirement: Cargo workspace test suite exits 0
The workspace `cargo test --workspace` command SHALL exit 0 after all pre-existing test runtime-mismatch issues are resolved. Tests that depend on tokio-only APIs SHALL use the `#[tokio::test]` attribute; tests that depend on async-std-only APIs SHALL continue to use `#[async_std::test]`. Mixing the attribute with a runtime that does not match the body's API calls (e.g. `#[async_std::test]` + `wiremock::MockServer::start()`) is forbidden.

#### Scenario: All #[async_std::test] functions use async-std-compatible APIs
- **WHEN** `cargo test --workspace` runs
- **THEN** every test annotated with `#[async_std::test]` succeeds OR is one of the explicitly-allowed async-std runtime tests
- **AND** no test panics with `there is no reactor running, must be called from the context of a Tokio 1.x runtime` from inside `tokio-1.52.3/src/net/tcp/stream.rs`

#### Scenario: All #[tokio::test] functions use tokio-compatible APIs
- **WHEN** `cargo test --workspace` runs
- **THEN** every test annotated with `#[tokio::test]` either passes or is one of the explicitly-allowed tokio runtime tests
- **AND** no test panics with an async-std-specific runtime error

#### Scenario: cargo test --workspace exits 0
- **WHEN** `cargo test --workspace` is run from `apps/api/`
- **THEN** the exit code is 0
- **AND** the summary shows zero failures
- **AND** the only `ignored` tests are pre-existing (e.g. testcontainer tests requiring Docker)

### Requirement: Verification evidence covers full workspace test suite
The `domain-api-cutover` "Verification evidence" requirement's "Focused verification succeeds" scenario SHALL be extended to require the full workspace `cargo test --workspace` to pass, not just per-crate gates. Pre-existing test failures that block this command MUST be remediated before archive.

#### Scenario: cargo test --workspace exits 0 with no pre-existing failures
- **WHEN** `cargo test --workspace` runs from `apps/api/`
- **THEN** exit code is 0
- **AND** every test failure that existed prior to this change is fixed or explicitly marked `#[ignore]` with a documented reason
- **AND** the per-crate gates (`cargo check`, `cargo fmt --check`, `cargo clippy`) still pass
