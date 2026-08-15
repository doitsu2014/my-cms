## Why

Phase A cleanup of the API workspace, extended to retire the transitional `migration` crate now that `domain_posts` exposes an operator-facing migration CLI. The `cms` legacy root package (`apps/api/src/**`), the `legacy_bootstrap` binary, and the `application_core` crate exist only to keep legacy paths compiling even though the deployed `my-cms-api` binary is produced exclusively by the `gateway` crate (see `apps/api/Cargo.toml` — a pure virtual workspace after this change — and `apps/api/gateway/Cargo.toml` `[[bin]] name = "my-cms-api"`). The `gateway` `Cargo.toml` also lists `application_core` and `migration` as path-dependencies that are not used anywhere in `gateway/src/main.rs` — the migration orchestrator already calls `domain_posts::migrations_cli::run(conn)` directly. After Phase A the `migration` crate survives only as a re-export shim for `apps/api/test_helpers/src/lib.rs` and a thin CLI binary (`cli::run_cli(migration::Migrator)`) shipped through `apps/api/Dockerfile` and invoked by the `deployments/docker-swarm/apps/docker-compose.yaml` `migrate` service. `domain_posts` now owns the canonical `Migrator` (`domain_posts::migrations::Migrator`), the operator-facing migration CLI (`cargo run -p domain_posts -- migrate` — its standalone bin is `target/release/domain_posts` per `apps/api/domain_posts/Cargo.toml` `[[bin]] name = "domain_posts"`), and the per-domain migrations orchestrator already invoked by `gateway::run_orchestrator`. This change therefore extends Phase A by removing `apps/api/migration/` wholesale, switching `test_helpers` to import `domain_posts::migrations::{Migrator, MigratorTrait}` directly, re-targeting the Docker image and operator workflow to the `domain_posts` bin, and updating documentation consistently. Wiring media/user domains into the gateway composition is explicitly out of scope and will be handled in a separate follow-up change.

## What Changes

- Delete `apps/api/src/` wholesale: the legacy `cms` library (`src/lib.rs`), the `legacy_bootstrap` binary (`src/bin/legacy_bootstrap.rs`), the entire `src/api/**` HTTP-adapter tree (`src/api/{post,media,user,tag,administrator,public,delete}/**`), and the empty `src/common/**` and `src/presentation_models/**` shims.
- Delete `apps/api/application_core/` wholesale: the transitional compatibility crate, including its `commands`, `common` (`app_error`, `datetime_generator`, `extensions`), and `entities` (re-export of `domain_posts::entities::*`) modules.
- Remove the `application_core` workspace member from `apps/api/Cargo.toml` `members` and the `application_core` path-dependency from both `apps/api/Cargo.toml` and `apps/api/gateway/Cargo.toml`. **BREAKING** for any local consumer that still imports `application_core::*`; a baseline audit confirms none remain after the source trees are removed.
- Remove the unused `migration` path-dependency from `apps/api/gateway/Cargo.toml` (the gateway never imports `migration::*`; the migration orchestrator already uses `domain_posts::migrations_cli::run`).
- Convert `apps/api/Cargo.toml` to a pure virtual workspace by removing the root `[package]`, the duplicated `[[bin]] name = "my-cms-api" path = "gateway/src/main.rs"` (the `gateway` crate already declares its own bin), and the root-level `[dependencies]` block that only fed the deleted `cms` package.
- **Extended in this revision:** Delete `apps/api/migration/` wholesale: the standalone CLI binary (`src/main.rs` invoking `cli::run_cli(migration::Migrator)`), the re-export shim library (`src/lib.rs` re-exporting `domain_posts::migrations::*`), the `Cargo.toml` (`name = "migration"`, `domain_posts = { path = "../domain_posts" }`, `sea-orm`, `sea-orm-migration`), and the `README.md`. **BREAKING** for the operator-facing Docker `migrate` service which currently invokes `/app/migration up`; the replacement command is `/app/domain_posts migrate up` (see Capabilities/Impact below).
- **Extended in this revision:** Remove `"migration"` from the `apps/api/Cargo.toml` `[workspace] members` array so the crate is no longer a workspace member after deletion.
- **Extended in this revision:** Switch `apps/api/test_helpers/Cargo.toml` to depend on `domain_posts = { path = "../domain_posts" }` (in `[dev-dependencies]`) instead of `migration = { path = "../migration" }`, and update `apps/api/test_helpers/src/lib.rs` to import `domain_posts::migrations::{Migrator, MigratorTrait}` instead of `migration::{Migrator, MigratorTrait}`. The canonical migrator is unchanged (`domain_posts::migrations::Migrator`); only the import path changes.
- **Extended in this revision:** Update `apps/api/Dockerfile` to copy `target/release/domain_posts` (produced by the `domain_posts` crate's `[[bin]] name = "domain_posts"`) instead of `target/release/migration`. The shipped binaries become `my-cms-api` (gateway) and `domain_posts` (operator-facing migration CLI + standalone microservice).
- **Extended in this revision:** Update `deployments/docker-swarm/apps/docker-compose.yaml` `migrate` service to set `entrypoint: ["/app/domain_posts"]` and `command: ["migrate", "up"]`, replacing the previous `entrypoint: ["/app/migration"]` + `command: ["up"]`.
- No behavioral, route, schema, auth, role, error-mapping, or external-integration change. **No SeaORM migration-identity change.** The four canonical migration identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`) live in `apps/api/domain_posts/src/migrations/` and are not touched. The `my-cms-api` binary (gateway) and the `domain_posts` bin (now also serving as the operator migration CLI via `migrate` subcommand) continue to be produced.

## Capabilities

### New Capabilities
- `legacy-cms-purge`: Phase A workspace cleanup plus the extended migration-crate removal. Retires the `cms` legacy root package, the `application_core` transitional crate, the `migration` standalone-CLI / re-export shim, and the stale `application_core`/`migration` path-dependencies on the `gateway` crate. The standalone `migration` binary is replaced by the `domain_posts` bin's `migrate` subcommand; the operator-facing workflow in `deployments/docker-swarm/apps/docker-compose.yaml` and the `apps/api/Dockerfile` image copy list are updated in lockstep.

### Modified Capabilities
- `api-gateway-bootstrap`: Modify the requirement that currently keeps `application_core` and `migration` as transitional compatibility shims for categories/tags/media/users/ai. After this change `application_core` no longer exists in the workspace; `migration` no longer exists either. `test_helpers` imports `domain_posts::migrations::{Migrator, MigratorTrait}` directly; the gateway continues to call `domain_posts::migrations_cli::run(conn)` directly (no `migration::` indirection).
- `domain-api-cutover`: Modify the "Legacy runtime is retired safely" scenario so the legacy bootstrap binary, the obsolete `cms::api::*` module tree, the `application_core` crate, and the `migration` crate are all retired together in Phase A even though the media and user domain services are still being wired into `gateway::manifest()` in a separate follow-up change.

## Impact

Affected paths (Phase A, already landed in the working tree at the time of this revision):
- Deleted: `apps/api/src/**` (legacy `cms` library + `legacy_bootstrap` binary + empty `src/{common,presentation_models}/**` shims); `apps/api/application_core/**` (`Cargo.toml`, `README.md`, `src/lib.rs`, `src/commands/mod.rs`, `src/common/{mod.rs,app_error.rs,datetime_generator.rs,extensions.rs}`, `src/entities/mod.rs`).
- Edited manifests: `apps/api/Cargo.toml` (workspace `members` array, root `[package]`, `[[bin]]`, root `[dependencies]`, `application_core` path-dep), `apps/api/gateway/Cargo.toml` (`application_core` and `migration` path-deps removed).

Affected paths (extended scope, to be landed in this revision):
- Deleted: `apps/api/migration/**` (`Cargo.toml`, `README.md`, `src/lib.rs`, `src/main.rs`).
- Edited manifests: `apps/api/Cargo.toml` (`"migration"` removed from the `members` array); `apps/api/test_helpers/Cargo.toml` (drop `migration` dep; add `domain_posts` dev-dep); `apps/api/gateway/Cargo.toml` (already done in Phase A — `migration` path-dep removed; nothing further changes for the gateway).
- Edited Docker/operator surface: `apps/api/Dockerfile` (replace `COPY --from=build /usr/local/my-cms/target/release/migration .` with `COPY --from=build /usr/local/my-cms/target/release/domain_posts .`); `deployments/docker-swarm/apps/docker-compose.yaml` `migrate` service (replace `entrypoint: ["/app/migration"]` + `command: ["up"]` with `entrypoint: ["/app/domain_posts"]` + `command: ["migrate", "up"]`).
- Edited source: `apps/api/test_helpers/src/lib.rs` (replace `use migration::{Migrator, MigratorTrait};` with `use domain_posts::migrations::{Migrator, MigratorTrait};`).

Untouched runtime code:
- `apps/api/gateway/**` (gateway binary source; never imported `migration::*`).
- `apps/api/domain_posts/**` (canonical `Migrator` at `src/migrations/mod.rs`, `migrations_cli::run` at `src/migrations_cli.rs`, `domain_posts` bin's `migrate` subcommand dispatch at `src/main.rs` lines 28–32 / 152–160 — all preserved verbatim).
- `apps/api/domain_auth/**`, `apps/api/domain_interface/**`, `apps/api/domain_media/**`, `apps/api/domain_user/**`.

Untouched infra:
- `deployments/k8s/charts/my-cms-api/**`, `.github/workflows/ci-my-cms.yml` (neither references `migration` crate path).
- Helm chart values/templates do not reference the `migration` binary name; the Docker image swap does not affect the Helm chart.

Documentation refresh:
- `docs/api-architecture.md` legacy-shims section (§11) — remove the `apps/api/migration/` subsection and update the §1 mermaid graph; update the Phase A cleanup banner at the top; mark the `migration` deletion step in §12 staged cutover as completed in this change.
- `docs/pluggable-domain-refactor.md` — remove `migration/` from the workspace members tree; update the deployment-modes table to note the operator CLI is now `cargo run -p domain_posts -- migrate` (or `target/release/domain_posts migrate` in the container); mark Stage 4's `application_core` and `migration` removal as completed.
- `docs/adding-a-domain.md` — remove `migration` from the example workspace members and the migration CLI pattern guidance (point at `domain_posts::migrations_cli::handle_args` instead).
- `.opencode/agents/software-architect.md` — remove the `apps/api/migration/` row from the runtime inventory table.
- `.opencode/agents/product-owner.md` — change "CLI in `apps/api/migration/`" to "CLI in `domain_posts` (canonical; `cargo run -p domain_posts -- migrate` / `domain_posts migrate` in the container)".
- `.agents/skills/map-my-cms-api-architecture/SKILL.md` — remove `apps/api/migration/src/` references.
- `.agents/skills/map-my-cms-api-architecture/references/api-architecture.md` — drop the `apps/api/migration/src/` row from the migrations inventory.
- `.agents/skills/design-my-cms-api-change/references/change-checklists.md` — drop the `migration/API/application-core task` shorthand reference or replace with `domain-posts migrations_cli`.

No database, HTTP-route, GraphQL, auth, role, error-mapping, migration-identity, or external-integration change. No SeaORM entity regeneration needed. No frontend change.

Build graph: `cargo build --release --workspace` continues to succeed and now produces two binaries (`my-cms-api`, `domain_posts`). `cargo test --workspace`, `cargo fmt -- --check`, and `cargo clippy --workspace --all-targets` continue to succeed. The shipped container image contains `my-cms-api` and `domain_posts` binaries; the Docker Swarm `migrate` service invokes `/app/domain_posts migrate up`.

Operator migration: existing local operators that pinned `/app/migration up` (e.g. via a `docker-compose.override.yaml`) must update to `/app/domain_posts migrate up` (or rely on the new `deployments/docker-swarm/apps/docker-compose.yaml` default). This is a one-line entrypoint/command edit. The migration identities and `up` semantics are unchanged.
