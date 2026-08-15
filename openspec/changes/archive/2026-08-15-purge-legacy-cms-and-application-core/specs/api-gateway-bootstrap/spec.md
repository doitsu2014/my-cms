## MODIFIED Requirements

### Requirement: Public behavior is preserved during rollout
The refactor SHALL preserve all current public route paths including `/`, `/health`, `/healthz`, `/media/**`, `/categories/**`, `/posts/**`, `/tags`, `/users/**`, administrator bucket/migration paths, and both GraphQL paths until an explicit future change modifies them. The composed gateway SHALL serve the same route set as the current `my-cms-api` during the transition; the legacy `application_core` crate SHALL be removed from the workspace as part of the Phase A cleanup, and the `migration` crate SHALL remain only as the standalone SeaORM CLI binary library (`apps/api/migration/src/main.rs` and `apps/api/migration/src/lib.rs`) reachable from `apps/api/test_helpers/src/lib.rs`. The `cms` legacy root package and the `legacy_bootstrap` binary SHALL be removed in Phase A; categories/tags/media/users domain cutover into the gateway composition is staged separately.

#### Scenario: Health and representative domain contracts remain stable
- **WHEN** the old and refactored gateway are exercised with the same valid configuration and request fixtures
- **THEN** representative status codes, response envelopes, auth decisions, and GraphQL endpoints match

#### Scenario: Composed gateway and standalone domain share the same observable behavior
- **WHEN** the same `domain_posts` crate is run as `cargo run -p domain_posts` (standalone) and as `cargo run -p gateway` (composed) with the same env-var surface
- **THEN** for each post-related route the response status, envelope, auth decision, and observable behavior are identical between the two runtimes

#### Scenario: application_core is absent from the workspace after Phase A
- **WHEN** the Phase A cleanup completes
- **THEN** `apps/api/application_core/` no longer exists in the repository
- **AND** `apps/api/Cargo.toml` `members` does not list `application_core`
- **AND** no `Cargo.toml` under `apps/api/` declares an `application_core` path-dependency

#### Scenario: migration crate is removed after the gateway cutover
- **WHEN** the Phase A cleanup completes
- **THEN** no `apps/api/migration/` directory or `migration` workspace member exists
- **AND** `apps/api/test_helpers/src/lib.rs` imports `domain_posts::migrations::{Migrator, MigratorTrait}` directly
- **AND** the gateway continues to call `domain_posts::migrations_cli::run` without a migration-crate indirection
