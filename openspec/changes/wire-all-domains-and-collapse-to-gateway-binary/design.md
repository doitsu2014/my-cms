## Context

### Current state (source-derived, revalidated 2026-08-09)

The API workspace (`apps/api/Cargo.toml:1-2`) is a Cargo workspace with seven
members: `domain_auth`, `domain_interface`, `domain_media`, `domain_posts`,
`domain_user`, `gateway`, `test_helpers`. The `gateway` crate produces the
`my-cms-api` binary (`apps/api/gateway/Cargo.toml:9-11`) and is documented as
the sole runtime surface for production deployments.

**Gateway composition root** (`apps/api/gateway/src/main.rs:44-49`): the
manifest currently registers two domain services:

```rust
pub fn manifest() -> Vec<Box<dyn DomainService>> {
    vec![
        Box::new(DomainPostService::new()),
        Box::new(DomainAuthService::new()),
    ]
}
```

`DomainMediaService` is already implemented
(`apps/api/domain_media/src/service.rs:60`) and exposes
`migrations() -> Vec::new()` (line 79-82) plus
`register_routes(self.state.clone())` (line 84-88). It is not registered.
`domain_user` has no `service.rs` and does not implement `DomainService`
(`apps/api/domain_user/src/` contains `domain/`, `dto.rs`, `handlers/`,
`lib.rs`, `observability/` only — no `service.rs`, no `api/`).

**Gateway dependencies** (`apps/api/gateway/Cargo.toml:13-16`): only
`domain_interface`, `domain_posts`, `domain_auth` are declared. The
`domain_media` and `domain_user` path-dependencies are absent, so the
gateway crate cannot compile a registration that includes them.

**Migration orchestrator** (`apps/api/gateway/src/main.rs:57-88`): the
gateway already runs migrations at boot via `run_orchestrator`. The
implementation:

1. Collects `MigrationDescriptor`s from every registered `DomainService`
   (line 63).
2. Sorts by `id` and deduplicates by `id` (lines 66-67).
3. For each descriptor whose id starts with `m2024` or `m2026`, calls
   `domain_posts::migrations_cli::run(conn)` directly (lines 77-85). This
   is a hard-coded dispatch — it leaks `domain_posts` knowledge into the
   gateway and is the reason media/user/auth descriptors (which return
   `Vec::new()` from `migrations()`) are silently skipped.

The orchestrator runs at boot (line 109) before the HTTP listener is
bound. There is no CLI subcommand dispatch in `main.rs` — the binary only
supports the HTTP boot path.

**Standalone `domain_posts` binary** (`apps/api/domain_posts/Cargo.toml:13-15`):
declares `[[bin]] name = "domain_posts"`. `apps/api/domain_posts/src/main.rs:1-176`
boots an Axum server with the post-domain routes and dispatches a `migrate`
subcommand to `domain_posts::migrations_cli::handle_args`
(line 163-171). This binary exists only to let the Docker Swarm `migrate`
one-shot run schema migrations.

**Dockerfile** (`apps/api/Dockerfile:23-42`):

- Build line (line 36): `RUN cargo build --release --bin my-cms-api --bin domain_posts`
- Runtime copy (line 41-42): copies both binaries
- Default `CMD` (line 47): `["/app/my-cms-api"]`

The inline comment at lines 23-34 explicitly notes that the `domain_posts`
binary is required by the `migrate` compose service.

**Docker Swarm migrate service** (`deployments/docker-swarm/apps/docker-compose.yaml:56-70`):

```yaml
migrate:
  ...
  entrypoint: ["/app/domain_posts"]
  command: ["migrate", "up"]
```

This is the only consumer of the `domain_posts` binary in production.

**`domain_posts::migrations_cli`** (`apps/api/domain_posts/src/migrations_cli.rs:1-44`):
the library exposes three functions — `run(conn)` (line 14-16),
`list_identities()` (line 20-22), `handle_args(args)` (line 27-43). All
three are library-only and callable from any crate that depends on
`domain_posts`. The `handle_args` function is the CLI parser used by the
standalone bin; the gateway's orchestrator calls `run` directly.

### Graph evidence

`code-review-graph_get_minimal_context(task="audit API gateway composition,
domain crates wiring, and migrate CLI surface")` reports the repository
has 412 files / 1729 nodes / 16972 edges / risk `low (0.40)`. The graph
was built at commit `8d087dce...` (branch `refactor/my-cms-api`) while
the current HEAD is `458ce1fed...`; `head_matches_build: false`. The
staleness (~15 hours) does not affect the three gaps under review — every
cited file path and symbol was revalidated against current source in this
analysis. Limitation recorded per AGENTS.md §"Mandatory graph gate".

### Constraints (from the coordinator and AGENTS.md)

- Honour the layered architecture (API / Application Core / DB) adapted
  to the DDD layout (`gateway`, `domain_*`, `domain_interface`).
- Do not edit `domain_interface` beyond what is strictly required.
- Migrations run with `sea-orm-migration`; the gateway hosts a `Migrator`
  iterating all registered domain migrators in a deterministic order.
- `domain_posts` becomes a **library** crate that exposes its migrator
  via `pub`. The `[[bin]]` and `src/main.rs` are removed.
- Docker compose `migrate` service retargets to the gateway image with
  `entrypoint: ["/app/my-cms-api"]` and `command: ["migrate", "up"]`.
- Verification gate: `cargo check`, `cargo test`, `cargo fmt -- --check`,
  `cargo clippy`, `pnpm --dir apps/web build` (likely none for web).
- `code-review-graph` MCP gate runs at proposal and design time (this
  document) and again before implementation; limitations recorded.

### Active changes that overlap

`purge-legacy-cms-and-application-core` (41/47 tasks, last modified
2026-08-08T06:55:30Z) explicitly states in its `proposal.md` (line 49):
> "Wiring media/user domains into the gateway composition is explicitly
> out of scope and will be handled in a separate follow-up change."

This change is that follow-up. The two changes are **non-overlapping on
files** (the in-progress change is finalising `purge-legacy-cms-and-application-core`'s
last six tasks) and **non-conflicting on behaviour** (the in-progress
change already redirects the operator CLI to `domain_posts migrate up`).
The current change supersedes that CLI dispatch by moving it under
`my-cms-api migrate up`.

## Goals / Non-Goals

**Goals**

- Every CMS API route (post, category, tag, translation, GraphQL, media,
  bucket, user, password reset) is reachable through the deployed
  `my-cms-api` binary.
- The deployed container image contains exactly one HTTP-serving binary
  (`my-cms-api`).
- Migrations are an operator command of the gateway binary; no per-domain
  migration binary exists.
- The migration orchestrator is generic and dispatches per-domain via a
  trait method — no hard-coded id-prefix branches.
- All four canonical migration identities
  (`m20240409_151952_release_100`, `m20250330_151455_release_110`,
  `m20260126_040610_release_300`, `m20260531_000001_pgvector`) are
  preserved exactly. No schema change.
- Local development workflow `cargo run -p gateway -- migrate up` works.
- The Docker Swarm `migrate` one-shot uses the gateway binary.
- Documentation that references `domain_posts migrate up` or
  `cargo run -p domain_posts -- migrate` is updated.

**Non-Goals**

- No new HTTP routes. No new environment variables. No new database
  migrations. No new SeaORM entities.
- No change to authentication/authorization semantics. The
  `supabase-auth` capability and the role-OR semantics in
  `domain_auth::factory::auth_layer_from_env` are unchanged.
- No change to GraphQL mounts (`/posts/graphql/{immutable,mutable}`).
- No change to media cache invalidation, MIME allow-list, body-limit, or
  bucket lifecycle semantics.
- No change to OpenTelemetry / Jaeger wiring.
- No change to k8s Helm charts (`deployments/k8s/charts/my-cms-api/`) —
  the Helm chart already targets the `my-cms-api` deployment and
  delegates migration to the Docker Swarm one-shot.
- No new capability spec under `openspec/specs/` — only the existing
  `domain-api-cutover` capability is extended.

## Decisions

### Decision 1: Generic migration dispatch via `DomainService` extension

**Driver.** The current orchestrator (`gateway/src/main.rs:77-85`) hard-codes
a per-domain dispatch arm:

```rust
if d.id.starts_with("m2024") || d.id.starts_with("m2026") {
    domain_posts::migrations_cli::run(conn).await.map_err(...)?;
} else {
    warn!("migration {} has no runner registered", d.id);
}
```

This leaks `domain_posts` knowledge into the gateway and silently warns
for any descriptor from a future domain. The design intent documented in
`openspec/changes/archive/2026-08-08-refactor-api-into-pluggable-domain-libraries/design.md:486`
("Each domain owns its migration set and its migration runner") is
violated by this hard-coded branch.

**Decision.** Extend `domain_interface::DomainService` with one new
async method:

```rust
async fn run_migrations(
    &self,
    _conn: &sea_orm::DatabaseConnection,
    _descriptors: &[MigrationDescriptor],
) -> Result<(), DomainConfigError> {
    Ok(())
}
```

Domains that own migrations (currently only `domain_posts`) override
this default. `domain_media`, `domain_user`, and `domain_auth` use the
default no-op. The gateway orchestrator iterates the services, asks each
for `migrations()`, deduplicates, sorts, and calls `run_migrations(conn,
&descriptors)` once per service.

**Alternatives considered.**

- (a) **Pass a closure into the descriptor.** Add a `runner: fn(&DatabaseConnection) -> Future<Output=...>` to `MigrationDescriptor`. *Rejected*: forces every migration id to know its runner; re-introduces the per-domain hard coding.
- (b) **Add a separate `MigrationRunner` trait.** Define a new trait alongside `DomainService`. *Rejected*: doubles the registration surface in `manifest()`; the same domain service registers for both traits, which adds ceremony without buying composability.
- (c) **Keep the hard-coded branch but extend it.** Add `else if d.id.starts_with("media")` etc. *Rejected*: scales linearly with domains; the existing design comment ("future domains extend this with their own runner") was a stop-gap, not the goal.

**Consequences.** `domain_interface` grows by one method. The default
implementation is `Ok(())` so existing implementors (`DomainPostService`,
`DomainMediaService`, `DomainAuthService`) compile unchanged. Only
`DomainPostService` overrides the method to delegate to
`domain_posts::migrations_cli::run(conn)`. The orchestrator simplifies to:

```rust
for service in services {
    let descriptors = service.migrations();
    if descriptors.is_empty() { continue; }
    service.run_migrations(conn, &descriptors).await
        .map_err(|e| DomainConfigError::MigrationExecution(format!("{}: {}", service.health().name, e)))?;
}
```

**Contracts.** No API change. The CLI subcommand surface (see Decision 2)
calls the same orchestrator with no behavioural difference.

**Migration / rollout.** Edit `apps/api/domain_interface/src/lib.rs` to add
the trait method (one line plus default impl). Add a `run_migrations` impl
on `DomainPostService` (delegates to `domain_posts::migrations_cli::run`).
Remove the hard-coded `if d.id.starts_with(...)` arm in
`gateway/src/main.rs:77-85`. No database change. No public API change.

**Verification.** `cargo check -p domain_interface -p domain_posts -p
domain_media -p domain_user -p domain_auth -p gateway` exits 0. The
orchestrator test (Decision 5) covers empty-descriptor and
multi-descriptor cases.

### Decision 2: `my-cms-api migrate <verb>` CLI subcommand

**Driver.** The coordinator's Gap 2 + Gap 3 require migrations to be
runnable from the gateway binary. The Docker Swarm `migrate` service must
be retargeted (`deployments/docker-swarm/apps/docker-compose.yaml:56-70`)
and the `domain_posts` standalone bin must be retired
(`apps/api/domain_posts/src/main.rs:1-176` deleted).

**Decision.** Add a tiny CLI parser at the top of
`gateway::main`:

```rust
let args: Vec<String> = std::env::args().skip(1).collect();
if args.first().map(|s| s.as_str()) == Some("migrate") {
    return run_migrate_cli(&args[1..]).await;
}
```

`run_migrate_cli` parses one of `up`, `down`, `status`, `--list` and
delegates to `domain_posts::migrations_cli::handle_args` (which already
supports `up` and `--list`). For `down` and `status` the parser is extended
inside `domain_posts::migrations_cli` (two extra arms on `handle_args`,
no domain-agnostic alternative exists because only `domain_posts` owns
migrations today). Both modes connect to `DATABASE_URL`, run the
orchestrator, and exit.

**Alternatives considered.**

- (a) **Reimplement the CLI in the gateway with a new dependency on `clap` / `argh`.** *Rejected*: the existing `handle_args` already covers the verbs; adding a parsing dep is unnecessary ceremony.
- (b) **Spawn the gateway as a child process from the `migrate` compose service and exec `cargo run -p gateway -- serve`.** *Rejected*: doubles the runtime complexity and contradicts the goal.
- (c) **Use the Docker `HEALTHCHECK` + `until` polling pattern instead of a CLI subcommand.** *Rejected*: changes operator UX; the explicit subcommand matches the existing `domain_posts migrate up` operator memory.

**Consequences.** `apps/api/domain_posts/src/main.rs` is deleted.
`domain_posts::migrations_cli::handle_args` retains its `up` and
`--list` arms; new `down` and `status` arms are added inside the same
function (the gateway simply forwards arguments). The gateway inherits
the existing error envelope (`Result<(), String>`), converts to
`ExitCode`.

**Contracts.** Public CLI surface:

| Invocation              | Behaviour                                                  |
|-------------------------|------------------------------------------------------------|
| `my-cms-api`            | Boot HTTP server (existing).                                |
| `my-cms-api migrate up` | Apply pending migrations, exit `0` / `1`.                   |
| `my-cms-api migrate --list` | Print migration ids to stdout, exit `0`.                |
| `my-cms-api migrate down`  | Revert last migration, exit `0` / `1`. (Forwarded.)       |
| `my-cms-api migrate status`| Print applied/pending state, exit `0`. (Forwarded.)       |
| `my-cms-api migrate --help`| Print usage, exit `0`.                                    |

**Migration / rollout.** No database change. Existing operators that
pinned `/app/domain_posts migrate up` in compose overrides must update
to `/app/my-cms-api migrate up`. This is a one-line edit per override;
documented in `proposal.md` Impact.

**Verification.** Manual smoke (`cargo run -p gateway -- migrate --list`
prints four ids; `cargo run -p gateway` boots `/health` returning 200).
Testcontainer integration test (Decision 5) covers `migrate up` against
a fresh database.

### Decision 3: Keep boot-time auto-migrations

**Driver.** The current gateway runs the orchestrator unconditionally at
boot (`apps/api/gateway/src/main.rs:109`). Retiring this behaviour
("only the `migrate` compose service runs migrations") would be a
behavioural break for any operator who relies on the gateway
self-healing. The Docker Swarm compose file already enforces
`my-cms-api` depends_on `migrate` (`docker-compose.yaml:120-122`), so
the operator path is closed-circuit.

**Decision.** Keep auto-migrations at boot AND expose `my-cms-api migrate
<verb>`. Both paths share the same orchestrator. The CLI subcommand path
exits without binding the listener; the boot path runs the orchestrator
then continues to bind.

**Alternatives considered.**

- (a) **Disable auto-migrations at boot.** *Rejected*: a missed `migrate` service (e.g. compose override) would crash the gateway on a stale schema; the safety net is small and useful.
- (b) **Gate auto-migrations behind an env flag (`MIGRATE_AT_BOOT=0`).** *Considered but deferred*: adds config surface without a current need. If a future change introduces a hot-restart scenario, the flag can be added without breaking the CLI surface.

**Consequences.** Both paths are idempotent (sea-orm-migration tracks
applied migrations). The first migration to apply on a fresh database is
either via the `migrate` compose service (production) or via the boot
path (local dev without compose).

**Verification.** Testcontainer integration test runs `migrate up` and
then boots the gateway against the same database; the second run is a
no-op (no pending migrations) and the gateway starts successfully.

### Decision 4: `DomainUserService` and `domain_user::api::routes`

**Driver.** `domain_user` has no `service.rs` and no `api/` module. The
seven existing handler subdirectories (`apps/api/domain_user/src/handlers/{create,delete,modify,read_list,read_one,reset_password,supabase_admin_client}`)
need to be wrapped into `Vec<RouteRegistration>` so the gateway can mount
them on `Mount::Administrator`. The handler functions are already
implemented (e.g. `apps/api/domain_user/src/handlers/supabase_admin_client.rs:67-275`)
and tested via wiremock against GoTrue.

**Decision.** Add `apps/api/domain_user/src/service.rs` and
`apps/api/domain_user/src/api/mod.rs` + `apps/api/domain_user/src/api/routes.rs`.
The structure mirrors `domain_media` (`apps/api/domain_media/src/service.rs:24-93`,
`apps/api/domain_media/src/api/routes.rs:85`):

```rust
// service.rs
pub struct DomainUserService { state: UserApiState }

#[async_trait]
impl DomainService for DomainUserService {
    fn health(&self) -> HealthDescriptor { HealthDescriptor { name: "domain-user", version: env!("CARGO_PKG_VERSION") } }
    fn required_env(&self) -> &'static [&'static str] { &["SUPABASE_URL", "SUPABASE_SERVICE_ROLE_KEY"] }
    fn validate_config(&self) -> Result<(), DomainConfigError> { /* env check */ }
    fn migrations(&self) -> Vec<MigrationDescriptor> { Vec::new() }
    fn register_routes(&self, _ctx: &DomainContext) -> Vec<RouteRegistration> {
        crate::api::routes::routes(self.state.clone())
    }
    async fn startup_health(&self, _ctx: &DomainContext) -> Result<(), DomainConfigError> { Ok(()) }
}

// api/routes.rs
pub fn routes(state: UserApiState) -> Vec<RouteRegistration> {
    use axum::{routing::{get, post, put, delete}, Router};
    let mut admin: Router<UserApiState> = Router::new()
        .route("/users",          post(create::handler))
        .route("/users",          get(read_list::handler))
        .route("/users/:id",      get(read_one::handler))
        .route("/users/:id",      put(modify::handler))
        .route("/users/:id",      delete(delete::handler))
        .route("/users/:id/reset-password", post(reset_password::handler));
    vec![RouteRegistration { mount: Mount::Administrator, router: admin, prefix: "users" }]
}
```

The seven handler `mod.rs` files already expose `pub(super) fn handler(...)`
function symbols (verified by reading
`apps/api/domain_user/src/handlers/{create,delete,modify,read_list,read_one,reset_password}/mod.rs`).
The exact wrapping is captured in tasks.md §3.

**Alternatives considered.**

- (a) **Re-implement the user handlers inside `domain_user::api`.** *Rejected*: duplicates the existing handlers; violates the architecture (handlers own behaviour, adapters extract/serialise).
- (b) **Keep `domain_user` handler-only and add the routes aggregator inside `domain_user::service` directly.** *Considered but rejected*: blurs the layer between adapter and service; matches the pattern in `domain_media` (separate `api/routes.rs`) and `domain_posts` (`apps/api/domain_posts/src/api/routes.rs`).

**Consequences.** New `UserApiState` struct holds a `SupabaseAdminClient`
(or the smaller subset the handlers need). The existing
`SupabaseAdminClient` (`apps/api/domain_user/src/handlers/supabase_admin_client.rs:47`)
constructs from `SUPABASE_URL` + `SUPABASE_SERVICE_ROLE_KEY`, which
becomes the `DomainUserService::required_env` set. No new env vars.

**Migration / rollout.** Additive; no schema or contract change. Existing
wiremock tests for `SupabaseAdminClient` continue to cover the handler
layer; the new aggregator adds an Axum-level integration test (Decision 5).

**Verification.** `cargo test -p domain_user` continues to pass. New
`api::routes` module-level test confirms the seven routes register with
`Mount::Administrator`.

### Decision 5: Test strategy

**Driver.** Per AGENTS.md §"Verify Before Commit" the verification gate
is `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy`.
The repo's existing pattern (`apps/api/test_helpers`) uses PostgreSQL
testcontainers + wiremock for GoTrue; `domain_user`'s tests already use
wiremock (`apps/api/domain_user/Cargo.toml:36`).

**Decision.** Three test layers:

1. **Module-level tests** (no testcontainer):
   - `domain_user::service::tests` — `DomainUserService` is object-safe,
     `required_env` returns the two env vars, `migrations()` is empty,
     `health()` returns the correct descriptor.
   - `gateway::migrate_cli::tests` — argument parsing for `up`,
     `--list`, `down`, `status`, `--help`, unknown verbs (exit 1).
2. **Testcontainer integration test** (uses `test_helpers::setup_test_space`):
   - `my-cms-api migrate --list` prints the four ids.
   - `my-cms-api migrate up` against a fresh testcontainer applies all
     four migrations in order; the `sea_orm_migration` tracking table
     records them.
   - A second `my-cms-api migrate up` is a no-op (idempotent).
   - `gateway::run_orchestrator` against the registered services runs
     the four descriptors and skips `domain_media`/`domain_user`/
     `domain_auth` (empty `migrations()`).
3. **Wiremock external-contract tests** (already present):
   - `domain_user` GoTrue contract (`SupabaseAdminClient`).
   - `domain_media` Supabase Storage contract (already covered).

**Alternatives considered.**

- (a) **Live-service tests.** *Rejected*: per AGENTS.md, deterministic seams preferred over live Supabase / OpenAI.
- (b) **Skip the orchestrator test, rely on manual smoke.** *Rejected*: the orchestrator's topological sort and dispatch change is the most material behaviour; manual smoke does not exercise ordering.

**Verification.** All three layers run in `cargo test --workspace`. The
gateway-level tests use `apps/api/test_helpers` directly (no live services).

### Decision 6: Docker image and compose migration

**Driver.** Coordinator Gap 2 + Gap 3 require (a) the Docker image
ships only `my-cms-api`, (b) the `migrate` compose service invokes the
gateway binary.

**Decision.**

1. `apps/api/Dockerfile:36` becomes
   `RUN cargo build --release --bin my-cms-api`.
2. `apps/api/Dockerfile:41` keeps `COPY --from=builder /app/target/release/my-cms-api /app/my-cms-api`.
3. `apps/api/Dockerfile:42` (the `domain_posts` COPY line) is **deleted**.
4. `apps/api/Dockerfile:23-34` (the multi-line comment explaining the
   two-binary build) is rewritten to a single-binary comment.
5. `deployments/docker-swarm/apps/docker-compose.yaml:67-68` becomes
   `entrypoint: ["/app/my-cms-api"]` and `command: ["migrate", "up"]`.

**Alternatives considered.**

- (a) **Keep the `domain_posts` binary in the image for forensic debugging.** *Rejected*: ships a binary the operator no longer invokes; the staged `purge-legacy-cms-and-application-core` change removed the prior `migration` bin for the same reason.
- (b) **Multi-stage build with two `cargo build` invocations.** *Rejected*: build time regression; not justified.

**Migration / rollout.** The new image replaces the previous image. The
`migrate` compose service retains the same `depends_on` chain
(init-wait → migrate → my-cms-api). Rollback requires redeploying the
prior image and prior compose file together (single atomic change per
release tag).

**Verification.** `docker build -f apps/api/Dockerfile -t my-cms-api:dev .`
produces an image whose `ls /app` shows `my-cms-api` only.
`docker compose -f deployments/docker-swarm/apps/docker-compose.yaml up
migrate` exits `0` and applies the four migrations.

### Decision 7: Documentation updates

**Driver.** Five doc files reference the retiring `domain_posts migrate`
CLI surface. Out-of-date docs cause operator confusion.

**Decision.** Update each occurrence with the gateway-binary equivalent:

| File | Current | Replacement |
|------|---------|-------------|
| `docs/api-architecture.md:1,9,76,98,107,509,523,578` | `domain_posts migrate up` | `my-cms-api migrate up` (operator) / `cargo run -p gateway -- migrate up` (local) |
| `docs/pluggable-domain-refactor.md:44,120,122` | `domain_posts migrate up`, `cargo run -p domain_posts -- migrate --list`, `cargo run -p domain_posts` | `my-cms-api migrate up`, `cargo run -p gateway -- migrate --list`, `cargo run -p gateway` |
| `docs/ai-platform.md:58` | `cargo run -p domain_posts -- migrate [--list]` | `cargo run -p gateway -- migrate [--list]` |
| `.opencode/agents/product-owner.md:72` | `/app/domain_posts migrate up` | `/app/my-cms-api migrate up` |
| `.opencode/agents/software-architect.md:97` | `apps/api/domain_posts/src/main.rs` | `apps/api/gateway/src/main.rs` |

The historical references in `openspec/changes/archive/2026-08-08-*`
are **left unchanged** (they are archaeology, not operator guidance).

**Alternatives considered.**

- (a) **Delete the historical doc references entirely.** *Rejected*: the archive folder preserves design history; deleting changes erases decision context.

**Verification.** `rg 'domain_posts migrate|cargo run -p domain_posts -- migrate|/app/domain_posts' docs/ .opencode/ agents/` returns only archive-folder matches.

## Risks / Trade-offs

**[Risk]** Removing the `domain_posts` `[[bin]]` breaks any local
developer workflow that uses `cargo run -p domain_posts` to boot a
standalone post-domain service. → **Mitigation:** the replacement is
`cargo run -p gateway`, which composes every registered domain — strictly
more functionality. Documented in `proposal.md` Impact.

**[Risk]** The `domain_user` aggregator route handlers may not yet be
fully wired (`apps/api/domain_user/src/handlers/*` each expose
`mod.rs`, but the exact public symbol for each handler was not exhaustively
read in this design pass). → **Mitigation:** task 3.1 in `tasks.md` opens
with a focused read of every `handlers/*/mod.rs` to confirm the handler
function name and signature before writing the aggregator.

**[Risk]** The orchestrator's topological sort (Decision 1) does not
currently honour `MigrationDescriptor::depends_on` — it sorts by `id` and
dedupes by `id`. With zero current descriptors that declare `depends_on`,
the change is behaviour-preserving today, but a future domain that adds
a dependent migration will rely on topological ordering. →
**Mitigation:** the new scenario in `spec.md` ("Descriptor dependency
order is respected") makes the contract explicit. The testcontainer
integration test adds a synthetic two-descriptor test
(domain_user-test-domain → domain_posts::migration) once a second domain
owns migrations. For the current change, no descriptor declares
`depends_on`; the test verifies the no-dependency case.

**[Risk]** The graph server snapshot is stale (built at `8d087dce...`,
HEAD at `458ce1fed...`). → **Mitigation:** every cited file path and
symbol was revalidated against current source via `read`/`grep`. The
graph is used only as a navigation aid; no graph-derived claim is
unsupported by direct source inspection.

**[Risk]** The active `purge-legacy-cms-and-application-core` change
(41/47 tasks) is mid-flight. If that change lands first with a different
operator-CLI surface than expected, this change's Dockerfile + compose
update may need a rebase. → **Mitigation:** both changes target
`apps/api/Dockerfile` and `deployments/docker-swarm/apps/docker-compose.yaml`.
The `purge` change's last six tasks are doc-only (per its `tasks.md`
sections 8.x and 9.5). Coordinate via the `software-engineer` review
step before merging.

**[Risk]** Keeping auto-migrations at boot (Decision 3) duplicates the
`migrate` compose service. In a hot-restart scenario (gateway restarted
without re-running `migrate`) the boot path is the safety net. →
**Mitigation:** both paths are idempotent; the additional cost is one
`SELECT` against the migration tracking table per boot, which is
negligible.

**[Risk]** Removing the `domain_posts` `[[bin]]` does not, by itself,
remove `cargo run -p domain_posts -- migrate` for local development
because the migration CLI is implemented in the library
(`domain_posts::migrations_cli`). → **Mitigation:** the local command
becomes `cargo run -p gateway -- migrate up`. This is documented in
`docs/api-architecture.md` and the agent files.

## Migration Plan

### Database
None. No schema change. The four canonical migration identities are
preserved exactly. The migration `up` history is byte-identical before
and after the change.

### Deployment
1. Land this change on a feature branch.
2. Build the new image locally: `docker build -f apps/api/Dockerfile -t my-cms-api:dev .`
3. Run the verification gate locally (see "Verification Plan" below).
4. Push the image and update the Swarm stack. The `migrate` compose
   service picks up the new image automatically because it shares the
   `Dockerfile` with `my-cms-api`.
5. Verify `docker compose ... up migrate` applies the four migrations
   against a fresh database volume.
6. Verify `docker compose ... up my-cms-api` boots and `/health`
   returns `200 OK`.

### Rollback
- Single atomic rollback: redeploy the prior image and prior compose
  file. Both are restored from the release tag.
- No data rollback. No schema rollback.
- If the rollback is needed because `domain_user` routes misbehave,
  the immediate workaround is to comment out `Box::new(DomainUserService::new())`
  in `gateway::manifest()` and rebuild — `domain_user` registration is
  the only piece introduced by this change that touches user-facing
  HTTP routes.

### Order of operations (suggested commit chain)
1. Add `DomainUserService` + `domain_user::api::routes` + tests. Verify
   `cargo test -p domain_user`. **Independent of the rest.**
2. Add `domain_media` and `domain_user` to `gateway/Cargo.toml`. Add
   them to `manifest()`. Verify `cargo check -p gateway`. **Independent
   of 3.**
3. Extend `DomainService` with `run_migrations`. Override in
   `DomainPostService`. Refactor `run_orchestrator`. Verify
   `cargo test -p gateway`. **Independent of 1, 2.**
4. Add `migrate` CLI subcommand to gateway. Verify
   `cargo run -p gateway -- migrate --list`. **Depends on 3.**
5. Delete `apps/api/domain_posts/src/main.rs` and the `[[bin]]` block.
   Verify `cargo build --release -p domain_posts` succeeds (lib only).
   **Depends on 3, 4.**
6. Update `Dockerfile` and `docker-compose.yaml`. **Depends on 5.**
7. Update documentation. **Depends on 5, 6.**
8. Run the full verification gate. **Depends on all.**

## Open Questions

1. **Should the orchestrator's topological sort honour `depends_on`
   now, or wait for a domain that actually declares a dependency?**
   *Current default:* keep `sort_by_key(|d| d.id)` and `dedup_by_key(|d| d.id)`.
   A topological sort is a small follow-up if a future domain adds a
   dependent migration. **No action in this change.**

2. **Should `migrate down` and `migrate status` be implemented now, or
   deferred?** *Coordinator's instruction:* "e.g. `my-cms-api migrate up`"
   is the example. `down` and `status` are operator conveniences. The
   spec scenario accepts all four verbs. **Recommendation:** implement
   `up` + `--list` in this change; defer `down` + `status` to a
   follow-up. **PO decision requested.**

3. **Should `MIGRATE_AT_BOOT=0` become a configurable flag?** *Current
   default:* auto-migrate at boot unconditionally. The flag is a
   trivial follow-up if a hot-restart scenario emerges. **No action.**

4. **Should `domain_auth`'s placeholder bin (`apps/api/domain_auth/src/main.rs`
   prints "not implemented yet") be removed at the same time?** The
   Dockerfile comment at `apps/api/Dockerfile:32-34` already notes it
   is not shipped. The bin itself does not ship in the container image
   but does exist in the workspace build. **Recommendation:** leave for
   a separate change; out of scope here.

5. **Should the `run_migrations` method on `DomainService` be added
   unconditionally, or only when a domain needs it (separate trait)?**
   *Decision 1 selected the unconditional method with a default `Ok(())`
   implementation.* This adds one method to the contract. The
   alternative (separate `MigrationRunner` trait) doubles the
   registration surface. **No PO action needed; documented in Decision 1.**

6. **Should `domain_user` `required_env` include `AUTHORIZATION_AUDIENCE`?**
   *Decision 4 currently lists `SUPABASE_URL` + `SUPABASE_SERVICE_ROLE_KEY`.*
   The audience is enforced at the gateway auth layer, not in the user
   domain. **No change recommended.**
