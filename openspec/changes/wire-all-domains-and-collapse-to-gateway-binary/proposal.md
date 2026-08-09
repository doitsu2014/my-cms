## Why

The `my-cms-api` gateway is documented as the sole runtime surface for production
deployments, but the gateway does not depend on or register the `domain_media`
and `domain_user` crates, the container image still ships the `domain_posts`
standalone binary solely so the Docker Swarm `migrate` one-shot can invoke
`domain_posts migrate up`, and `domain_user` does not yet expose a
`domain_interface::DomainService` adapter. The coordinator identified three
concrete gaps that violate the user's design:

1. The gateway's composition root registers only `DomainPostService` and
   `DomainAuthService`. Media routes and user routes are unreachable through
   the deployed gateway, even though both crates exist in the workspace.
2. The container image ships two binaries (`my-cms-api`, `domain_posts`) and
   the `migrate` compose service invokes a domain-specific binary instead of
   the gateway.
3. `domain_posts` owns a `[[bin]]` target and a `src/main.rs` only so the
   `migrate` compose service can run schema migrations. That bin is not used
   by any other workflow.

The change collapses the deployment surface to the gateway binary, wires
`domain_media` and `domain_user` into the gateway composition, and removes
the `domain_posts` standalone binary in favour of a `my-cms-api migrate`
subcommand.

## What Changes

- **ADDED** `domain_media` and `domain_user` are listed in
  `apps/api/gateway/Cargo.toml` `[dependencies]` and registered in
  `gateway::manifest()` as `Box<dyn DomainService>` alongside
  `DomainPostService` and `DomainAuthService`. The gateway boots with four
  registered domain services.
- **ADDED** `domain_user` exposes a `DomainUserService` that implements
  `domain_interface::DomainService` and contributes route registrations,
  health, configuration validation, and an empty migration descriptor list.
  `domain_user` gains a thin `apps/api/domain_user/src/api/routes.rs`
  aggregator that wraps the seven existing handler modules into a single
  `Vec<RouteRegistration>`.
- **ADDED** `my-cms-api migrate up|down|status|--list` CLI subcommand. When
  the binary is invoked with `migrate <verb>` it MUST run the migration
  orchestrator against the shared `DatabaseConnection` and exit, without
  binding the HTTP listener. The default invocation (no args) continues to
  boot the HTTP server.
- **MODIFIED** (**BREAKING**) `apps/api/domain_posts/Cargo.toml` no longer
  declares a `[[bin]]` target. `apps/api/domain_posts/src/main.rs` is
  deleted. Migrations remain in `domain_posts::migrations` (library) and
  `domain_posts::migrations_cli` (library) so the gateway can call them.
- **MODIFIED** (**BREAKING**) `apps/api/Dockerfile` builds and ships only
  the `my-cms-api` binary. The default `CMD` is unchanged
  (`["/app/my-cms-api"]`).
- **MODIFIED** `deployments/docker-swarm/apps/docker-compose.yaml` `migrate`
  service sets `entrypoint: ["/app/my-cms-api"]` and `command: ["migrate",
  "up"]`. The service still builds from the same `Dockerfile` and still
  depends on `init-wait` and gates `my-cms-api` on completion.
- **MODIFIED** `gateway::run_orchestrator` dispatches migrations per-domain
  via a generic interface (a `MigrationRunner` associated function or a
  parallel `MigrationRunner` trait) instead of hard-coding
  `if d.id.starts_with("m2024") || d.id.starts_with("m2026")`.
- **MODIFIED** `gateway::run_orchestrator` is callable from both the HTTP
  boot path and the CLI subcommand path. The HTTP boot path continues to
  run migrations at startup. The CLI subcommand path runs the same
  orchestrator and exits. Both paths are idempotent
  (sea-orm-migration tracks applied migrations).
- **MODIFIED** `gateway::manifest()` adds `DomainMediaService::new(...)` and
  `DomainUserService::new(...)`. The gateway description in
  `apps/api/gateway/Cargo.toml` (line 5) is updated to mention the full
  domain inventory.
- **MODIFIED** Documentation references that name `domain_posts migrate up`
  or `cargo run -p domain_posts -- migrate` are updated to
  `my-cms-api migrate up` (or `cargo run -p gateway -- migrate up` locally).
  Affected: `docs/api-architecture.md`, `docs/pluggable-domain-refactor.md`,
  `docs/ai-platform.md`, `.opencode/agents/product-owner.md`,
  `.opencode/agents/software-architect.md`.

## Capabilities

### New Capabilities

- `gateway-migration-cli`: CLI subcommand surface on `my-cms-api` that
  exposes migration orchestration (up, down, status, list) as a standalone
  operator-facing command. The subcommand runs the same `run_orchestrator`
  helper used at HTTP boot and exits with a non-zero code on failure.

### Modified Capabilities

- `domain-api-cutover`: The "Single domain-owned API runtime" requirement
  is tightened so that every CMS API route (including media and user
  administrator routes) is reachable through the gateway binary. A new
  scenario forbids per-domain standalone binaries from being deployed as
  the API surface. A second new scenario requires the gateway to expose a
  `migrate` CLI subcommand reachable from the deployed container.

## Impact

- Affected code:
  - `apps/api/gateway/Cargo.toml` — add `domain_media` and `domain_user`
    path-dependencies; update description.
  - `apps/api/gateway/src/main.rs` — wire the two new services in
    `manifest()`; refactor `run_orchestrator` to be generic; add a CLI
    subcommand parser; remove hard-coded `m2024`/`m2026` prefix dispatch.
  - `apps/api/domain_posts/Cargo.toml` — remove `[[bin]]` block.
  - `apps/api/domain_posts/src/main.rs` — **deleted**.
  - `apps/api/domain_user/Cargo.toml` — add `domain_interface` is already
    present; no change required.
  - `apps/api/domain_user/src/lib.rs` — re-export `service::DomainUserService`
    and the new `api` module.
  - `apps/api/domain_user/src/service.rs` — **new**.
  - `apps/api/domain_user/src/api/mod.rs` — **new**.
  - `apps/api/domain_user/src/api/routes.rs` — **new**.
  - `apps/api/Dockerfile` — drop the `domain_posts` bin from the build and
    runtime images; update the inline comments.
  - `deployments/docker-swarm/apps/docker-compose.yaml` — retarget the
    `migrate` service to the gateway binary.
- Affected docs: `docs/api-architecture.md`, `docs/pluggable-domain-refactor.md`,
  `docs/ai-platform.md`, `.opencode/agents/product-owner.md`,
  `.opencode/agents/software-architect.md`.
- Affected deployment: the shipped container image contains one binary
  (`my-cms-api`) instead of two. Local builds run as
  `cargo run -p gateway -- migrate up`. The Docker Swarm `migrate` one-shot
  is functionally equivalent but smaller.
- **BREAKING** for any operator that pinned `/app/domain_posts migrate up` in
  a `docker-compose.override.yaml`. The replacement is
  `/app/my-cms-api migrate up`.
- **BREAKING** for any local workflow that used `cargo run -p domain_posts`
  to boot the standalone post microservice. The replacement is
  `cargo run -p gateway` (which composes every registered domain). This
  matches the design intent documented in
  `openspec/changes/archive/2026-08-08-refactor-api-into-pluggable-domain-libraries/design.md`
  ("composed mode").
- Affected observability: the gateway startup banner reports four
  registered domain services instead of two. Existing Jaeger traces are
  unaffected.
- Affected tests: the `apps/api/test_helpers` crate continues to import
  `domain_posts::migrations::{Migrator, MigratorTrait}` directly (no change
  required; this path was already fixed by the
  `purge-legacy-cms-and-application-core` change).
- No schema changes. No new database migrations. Migration identities and
  ordering are preserved exactly.
