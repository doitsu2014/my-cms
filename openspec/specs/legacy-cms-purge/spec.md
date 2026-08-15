# legacy-cms-purge Specification

## Purpose
TBD - created by archiving change purge-legacy-cms-and-application-core. Update Purpose after archive.
## Requirements
### Requirement: Legacy cms root package is removed
The API workspace SHALL NOT contain a `cms` library or a `cms` binary target. The `apps/api/src/**` tree (which provides `src/lib.rs`, `src/api/**`, `src/common/**`, `src/presentation_models/**`, and the `src/bin/legacy_bootstrap.rs` binary) SHALL be deleted. The root `apps/api/Cargo.toml` SHALL NOT declare a `[package]` block or a `[[bin]] name = "my-cms-api"` entry pointing at the gateway.

#### Scenario: No cms library source remains
- **WHEN** the repository is searched for the `cms` library
- **THEN** no `apps/api/src/lib.rs` or `src/api/**` file exists
- **AND** `cargo metadata --manifest-path apps/api/Cargo.toml --format-version=1` reports zero packages named `cms`

#### Scenario: No legacy_bootstrap binary remains
- **WHEN** the repository is searched for the `legacy_bootstrap` binary
- **THEN** no `apps/api/src/bin/legacy_bootstrap.rs` file exists
- **AND** `cargo build --workspace --bins` does not produce a `legacy_bootstrap` binary

#### Scenario: Workspace manifest is a pure virtual workspace
- **WHEN** `apps/api/Cargo.toml` is read
- **THEN** the file contains a `[workspace]` table with a `members` array
- **AND** the file does NOT contain a `[package]` table
- **AND** the file does NOT contain a `[[bin]]` table
- **AND** the file does NOT contain a root `[dependencies]` table

### Requirement: application_core transitional crate is removed
The API workspace SHALL NOT contain the `application_core` crate. The `apps/api/application_core/**` directory (Cargo.toml, README.md, and the `src/{lib.rs,commands/mod.rs,common/{mod.rs,app_error.rs,datetime_generator.rs,extensions.rs},entities/mod.rs}` source tree) SHALL be deleted. The `application_core` path-dependency SHALL be removed from both `apps/api/Cargo.toml` and `apps/api/gateway/Cargo.toml`.

#### Scenario: application_core directory is deleted
- **WHEN** the repository is searched for the `application_core` crate
- **THEN** no `apps/api/application_core/` directory exists

#### Scenario: application_core is not a workspace member
- **WHEN** `apps/api/Cargo.toml` `members` array is read
- **THEN** it does NOT contain the string `"application_core"`

#### Scenario: application_core is not a declared dependency
- **WHEN** every `Cargo.toml` in `apps/api/` is read
- **THEN** no `[dependencies] application_core = { path = ... }` or `[dev-dependencies] application_core = { path = ... }` entry exists

### Requirement: Gateway does not depend on retired crates
The `apps/api/gateway/Cargo.toml` SHALL NOT declare `application_core` or `migration` as a path-dependency. The gateway binary SHALL continue to compile and SHALL continue to call `domain_posts::migrations_cli::run(conn)` directly (not through any `migration::` re-export).

#### Scenario: Gateway manifest drops retired path-deps
- **WHEN** `apps/api/gateway/Cargo.toml` is read
- **THEN** it does NOT contain an `application_core = { path = "../application_core" }` entry
- **AND** it does NOT contain a `migration = { path = "../migration" }` entry

#### Scenario: Gateway migration orchestrator is unchanged
- **WHEN** `apps/api/gateway/src/main.rs` is read
- **THEN** `run_orchestrator` calls `domain_posts::migrations_cli::run(conn)` (and not anything from `application_core` or `migration`)
- **AND** no `use application_core` or `use migration` statement exists in any file under `apps/api/gateway/src/`

### Requirement: migration workspace member is removed and operator CLI is re-targeted
The API workspace SHALL NOT contain the `migration` crate. The canonical migrations SHALL remain in `domain_posts`, `test_helpers` SHALL import them directly, and operator deployment SHALL invoke the existing `domain_posts migrate` CLI.

#### Scenario: migration crate is absent
- **WHEN** the workspace is inspected
- **THEN** no `apps/api/migration/` directory exists
- **AND** `apps/api/Cargo.toml` does not list `migration` as a member
- **AND** no `Cargo.toml` under `apps/api/` declares a path dependency on `migration`

#### Scenario: test helpers use canonical migrations
- **WHEN** `apps/api/test_helpers/src/lib.rs` is compiled
- **THEN** it imports `domain_posts::migrations::{Migrator, MigratorTrait}`
- **AND** `cargo check -p test_helpers --all-targets` exits 0

#### Scenario: operator image uses domain_posts CLI
- **WHEN** the release image and Docker Swarm migration service are inspected
- **THEN** `apps/api/Dockerfile` copies `target/release/domain_posts`
- **AND** the migration service invokes `/app/domain_posts migrate up`

#### Scenario: canonical migration identities are preserved
- **WHEN** `domain_posts::migrations::Migrator` is inspected or run
- **THEN** the four existing migration identities remain unchanged and ordered as before

### Requirement: Shipped binaries are re-targeted
The release image SHALL continue to ship the gateway API binary and one operator migration binary, changing only the operator binary from `migration` to `domain_posts`.

#### Scenario: Dockerfile copy list is re-targeted
- **WHEN** `apps/api/Dockerfile` is read
- **THEN** it copies `target/release/my-cms-api` and `target/release/domain_posts` into `/app/`
- **AND** it does not copy `target/release/migration`

#### Scenario: my-cms-api remains gateway-owned
- **WHEN** `cargo build --release --workspace` runs after cleanup
- **THEN** `target/release/my-cms-api` and `target/release/domain_posts` exist
- **AND** no `target/release/migration` binary is produced

### Requirement: No behavioral or interface regression
The Phase A cleanup SHALL NOT change HTTP routes, GraphQL mounts, request/response shapes, status/error mapping, authentication audience, role authorization, or external integration behavior. Canonical SeaORM migration identities SHALL remain unchanged. The `domain_*`, `gateway`, and `test_helpers` crates SHALL continue to pass their existing tests.

#### Scenario: Workspace verification gate passes
- **WHEN** the full repository verification gate runs
- **THEN** `cargo check --workspace` exits 0
- **AND** `cargo build --release --workspace` exits 0
- **AND** `cargo test --workspace` exits 0
- **AND** `cargo fmt -- --check` exits 0
- **AND** `cargo clippy --workspace --all-targets --all-features -- -D warnings` exits 0

#### Scenario: Domain crates are unaffected
- **WHEN** each `domain_*` crate is built and tested
- **THEN** `cargo check -p domain_auth -p domain_interface -p domain_media -p domain_posts -p domain_user --all-targets` exits 0
- **AND** no `domain_*` crate gains a new dependency on `application_core` or `cms`

#### Scenario: No forbidden source paths remain
- **WHEN** the repository is searched after the cleanup
- **THEN** no file path matching `apps/api/src/**` exists
- **AND** no file path matching `apps/api/application_core/**` exists
- **AND** `rg -t rust 'use cms::' apps/api/` returns no matches
- **AND** `rg -t rust 'use application_core::' apps/api/` returns no matches

