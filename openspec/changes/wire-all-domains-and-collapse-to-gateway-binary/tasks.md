## 1. Add `DomainUserService` and `domain_user::api::routes` aggregator

- [ ] 1.1 Read every existing handler module to confirm the exported handler symbol name and signature: `apps/api/domain_user/src/handlers/{create,delete,modify,read_list,read_one,reset_password}/mod.rs` and `apps/api/domain_user/src/handlers/supabase_admin_client.rs`. Record each handler's `pub` symbol and Axum extractor set in the new `api/routes.rs` design notes. **Verification:** `rg '^pub' apps/api/domain_user/src/handlers` returns one handler symbol per module.

- [ ] 1.2 Add `apps/api/domain_user/src/api/mod.rs` declaring `pub mod routes;` and `pub mod state;`. **Test-first:** `cargo check -p domain_user` exits 0 after this addition. **Verification:** `cargo check -p domain_user` exits 0; `rg 'pub mod routes' apps/api/domain_user/src/api/mod.rs` matches.

- [ ] 1.3 Add `apps/api/domain_user/src/api/state.rs` with a `UserApiState { supabase_admin_client: Arc<SupabaseAdminClient> }` struct plus `pub fn new(client: SupabaseAdminClient) -> Self`. Mirror the shape of `apps/api/domain_media/src/api/state.rs`. **Verification:** `cargo check -p domain_user` exits 0; `rg 'pub struct UserApiState' apps/api/domain_user/src/api/state.rs` matches.

- [ ] 1.4 Add `apps/api/domain_user/src/api/routes.rs` with `pub fn routes(state: UserApiState) -> Vec<RouteRegistration>`. Wire seven routes on `Mount::Administrator`:
  - `POST /users` → `create::handler`
  - `GET /users` → `read_list::handler`
  - `GET /users/:id` → `read_one::handler`
  - `PUT /users/:id` → `modify::handler`
  - `DELETE /users/:id` → `delete::handler`
  - `POST /users/:id/reset-password` → `reset_password::handler`

  **Test-first:** add `#[cfg(test)] mod tests` with a `routes_returns_administrator_mount_only` test that builds a `UserApiState` with a stub `SupabaseAdminClient` and asserts every returned `RouteRegistration` has `mount == Mount::Administrator` and the seven prefixes are present. **Verification:** `cargo test -p domain_user --lib api::routes` passes; the test asserts the seven prefixes.

- [ ] 1.5 Add `apps/api/domain_user/src/service.rs` with `pub struct DomainUserService { state: UserApiState }`, `pub fn new(client: SupabaseAdminClient) -> Self`, and `impl DomainService for DomainUserService` mirroring `apps/api/domain_media/src/service.rs:60-93`. The implementation must:
  - `health()` → `HealthDescriptor { name: "domain-user", version: env!("CARGO_PKG_VERSION") }`
  - `required_env()` → `&["SUPABASE_URL", "SUPABASE_SERVICE_ROLE_KEY"]`
  - `validate_config()` → iterates `required_env()` and returns `DomainConfigError::MissingEnv(var)` for the first missing one.
  - `migrations()` → `Vec::new()`
  - `register_routes(_ctx)` → `crate::api::routes::routes(self.state.clone())`
  - `startup_health(_ctx)` → `Ok(())` (default; override only if a probe is added).

  **Test-first:** add module-level tests `health_descriptor_is_domain_user`, `required_env_lists_two_vars`, `migrations_is_empty`, `validate_config_returns_missing_env_for_supabase_url_when_unset`, `domain_user_service_is_object_safe`. Mirror the tests in `apps/api/domain_auth/src/service.rs:65-172`. **Verification:** `cargo test -p domain_user --lib service` passes; `Box::<dyn DomainService>::from(Box::new(DomainUserService::new(stub)))` compiles.

- [ ] 1.6 Update `apps/api/domain_user/src/lib.rs` to add `pub mod api;` and `pub use service::DomainUserService;` next to the existing `pub use domain::error::AppError;`. **Verification:** `cargo build -p domain_user` exits 0; `rg 'pub use service::DomainUserService' apps/api/domain_user/src/lib.rs` matches.

- [ ] 1.7 Run the focused verification for step 1:
  ```bash
  cargo check -p domain_user --all-targets
  cargo test  -p domain_user
  cargo fmt   -p domain_user -- --check
  cargo clippy -p domain_user --all-targets -- -D warnings
  ```
  All commands exit 0. Record the output in the verification log.

## 2. Wire `domain_media` and `domain_user` into the gateway composition

- [ ] 2.1 Update `apps/api/gateway/Cargo.toml`: add `domain_media = { path = "../domain_media" }` and `domain_user = { path = "../domain_user" }` to `[dependencies]` (alphabetical order, matching the existing `domain_auth`/`domain_posts`/`domain_interface` lines). Update the `description` on line 5 to mention the four-domain composition. **Verification:** `cargo check -p gateway` exits 0; `rg 'domain_media|domain_user' apps/api/gateway/Cargo.toml` returns both lines.

- [ ] 2.2 Update `apps/api/gateway/src/main.rs:44-49` `manifest()` to register four domain services. The exact new shape:
  ```rust
  pub fn manifest() -> Vec<Box<dyn DomainService>> {
      vec![
          Box::new(DomainPostService::new()),
          Box::new(DomainAuthService::new()),
          Box::new(DomainMediaService::new(/* TODO: pass MediaConfig */)),
          Box::new(DomainUserService::new(/* TODO: pass SupabaseAdminClient */)),
      ]
  }
  ```
  Construct `MediaConfig` and `SupabaseAdminClient` from env vars at the top of `main` before `manifest()` is called, fail fast on missing env (use `DomainConfigError::MissingEnv`). Mirror the construction pattern in `apps/api/domain_posts/src/main.rs:33-57` for the database / GraphQL contexts. **Verification:** `cargo check -p gateway` exits 0; the function returns four entries; `rg 'Box::new\(DomainMediaService|Box::new\(DomainUserService' apps/api/gateway/src/main.rs` matches both lines.

- [ ] 2.3 Run the focused verification for step 2:
  ```bash
  cargo check -p gateway
  cargo test  -p gateway
  cargo fmt   -p gateway -- --check
  cargo clippy -p gateway --all-targets -- -D warnings
  ```
  **Expected:** `cargo check` exits 0. `cargo test -p gateway` passes (no gateway tests exist yet; the command exits 0 with "0 tests"). Record the output.

## 3. Extend `DomainService` with `run_migrations` and refactor the orchestrator

- [ ] 3.1 Add the new method to `apps/api/domain_interface/src/lib.rs` immediately after the existing `startup_health` default method (around line 184):
  ```rust
  /// Run the migrations declared by `migrations()` against the shared
  /// connection. Domains that own no migrations use the default no-op.
  /// Domains that own migrations (currently `domain_posts`) override
  /// and delegate to their `migrations_cli::run` helper.
  async fn run_migrations(
      &self,
      _conn: &sea_orm::DatabaseConnection,
      _descriptors: &[MigrationDescriptor],
  ) -> Result<(), DomainConfigError> {
      Ok(())
  }
  ```
  **Test-first:** add `domain_service_run_migrations_default_is_ok` to the existing `tests` module (`apps/api/domain_interface/src/lib.rs:186-267`). The test constructs a stub `DomainService` and asserts the default returns `Ok(())`. **Verification:** `cargo test -p domain_interface` passes; the stub compiles.

- [ ] 3.2 Override `run_migrations` on `DomainPostService` in `apps/api/domain_posts/src/service.rs`:
  ```rust
  async fn run_migrations(
      &self,
      conn: &sea_orm::DatabaseConnection,
      _descriptors: &[MigrationDescriptor],
  ) -> Result<(), DomainConfigError> {
      crate::migrations_cli::run(conn).await.map_err(|e| {
          DomainConfigError::MigrationExecution(format!("domain_posts: {}", e))
      })
  }
  ```
  Add `sea_orm_migration::MigratorTrait` to the use list (or keep via `crate::migrations_cli`). **Verification:** `cargo check -p domain_posts` exits 0; `rg 'async fn run_migrations' apps/api/domain_posts/src/service.rs` matches.

- [ ] 3.3 Refactor `apps/api/gateway/src/main.rs:57-88` `run_orchestrator` to use the new trait method. The new body iterates services, collects descriptors, dedupes by id, and calls `service.run_migrations(conn, &descriptors_for_service).await`. **Remove the hard-coded `if d.id.starts_with(...)` branch.** The new body:
  ```rust
  for service in services {
      let descriptors = service.migrations();
      if descriptors.is_empty() { continue; }
      let name = service.health().name;
      info!("running migrations for {}", name);
      service.run_migrations(conn, &descriptors).await
          .map_err(|e| DomainConfigError::MigrationExecution(format!("{}: {}", name, e)))?;
  }
  Ok(())
  ```
  Update the `Result<(), String>` return type to `Result<(), DomainConfigError>` (or wrap with `.map_err(|e| format!(...))` if the caller still expects `String`; check line 109). **Test-first:** add `gateway::orchestrator::tests` with `runs_post_migrations_and_skips_empty_services` (requires testcontainer). **Verification:** `cargo test -p gateway` passes; `rg 'm2024|m2026' apps/api/gateway/src/main.rs` returns no matches.

- [ ] 3.4 Run the focused verification for step 3:
  ```bash
  cargo check -p domain_interface -p domain_posts -p gateway
  cargo test  -p domain_interface -p domain_posts -p gateway
  cargo fmt   -p domain_interface -p domain_posts -p gateway -- --check
  cargo clippy -p domain_interface -p domain_posts -p gateway --all-targets -- -D warnings
  ```

## 4. Add `migrate` CLI subcommand to `my-cms-api`

- [ ] 4.1 Add a `migrate_cli` submodule under `apps/api/gateway/src/` (file `apps/api/gateway/src/migrate_cli.rs`) with:
  - `pub async fn handle_args(args: &[String]) -> ExitCode`
  - Parses: `up`, `down`, `status`, `--list`, `--help`, unknown verb (prints usage to stderr, exits 1).
  - Forwards `up` and `--list` to a thin wrapper around `domain_posts::migrations_cli::handle_args` (add `down` and `status` arms there if needed — see task 4.3).
  - Builds the manifest and connects to `DATABASE_URL` via the existing `connect_database` helper.

  **Test-first:** add module-level tests covering each verb (`up_forwards_to_handle_args`, `--list_forwards_to_handle_args`, `unknown_verb_exits_one`, `missing_database_url_exits_one`, `migrate_up_runs_orchestrator`). Stub the database connection by injecting a `DatabaseConnection` factory. **Verification:** `cargo test -p gateway --lib migrate_cli` passes; the unknown verb test asserts `ExitCode::FAILURE`.

- [ ] 4.2 Update `apps/api/gateway/src/main.rs:91` `main` to dispatch the `migrate` subcommand before any observability / database setup. Add at the top of `main`:
  ```rust
  let args: Vec<String> = std::env::args().skip(1).collect();
  if args.first().map(|s| s.as_str()) == Some("migrate") {
      return migrate_cli::handle_args(&args[1..]).await;
  }
  ```
  **Verification:** `cargo run -p gateway -- migrate --help` prints usage and exits 0; `cargo run -p gateway -- migrate --list` prints the four migration ids and exits 0; `cargo run -p gateway` boots the HTTP listener (existing behaviour).

- [ ] 4.3 (Optional, scope-dependent on PO decision) Extend `domain_posts::migrations_cli::handle_args` (`apps/api/domain_posts/src/migrations_cli.rs:27-43`) to accept `down` and `status` verbs. If `down`/`status` is deferred per the open question in `design.md`, document the deferral in the change tasks log and skip. Default action: implement `down` via `Migrator::down(conn, None)` and `status` via a custom `SELECT` against `sea_orm_migration::MigratorTrait::get_migration_status` (or print a stub if unavailable in sea-orm-migration 1.1). **Verification:** `cargo run -p gateway -- migrate --list` still works; `cargo run -p gateway -- migrate status` prints the applied/pending state and exits 0.

- [ ] 4.4 Run the focused verification for step 4:
  ```bash
  cargo check -p gateway
  cargo test  -p gateway
  cargo fmt   -p gateway -- --check
  cargo clippy -p gateway --all-targets -- -D warnings
  ```
  Plus a manual smoke:
  ```bash
  cargo run -p gateway -- migrate --list
  # Expected output: four lines, one per migration id, in original order.
  ```

## 5. Delete the `domain_posts` standalone binary

- [ ] 5.1 Delete `apps/api/domain_posts/src/main.rs` (176 lines, single binary entrypoint). **Verification:** `ls apps/api/domain_posts/src/main.rs` returns "No such file or directory".

- [ ] 5.2 Edit `apps/api/domain_posts/Cargo.toml`: remove the `[[bin]]` block (lines 13-15) and update the `description` (line 5) to remove the phrase "and a standalone bin". The `[lib]` block stays. **Verification:** `rg '\[\[bin\]\]' apps/api/domain_posts/Cargo.toml` returns no matches; `cargo build --release -p domain_posts` succeeds (lib only).

- [ ] 5.3 Verify that no other crate or compose file references the `domain_posts` binary path `/app/domain_posts`:
  ```bash
  rg -g '!openspec/changes/archive/**' -g '!docs/superpowers/**' '/app/domain_posts' .
  ```
  **Expected:** no matches. (Per design Decision 7, `docs/` references are updated in step 7; archive and historical plans are left as-is.)

- [ ] 5.4 Run the focused verification for step 5:
  ```bash
  cargo build --release -p domain_posts
  cargo build --release -p gateway
  cargo test  -p domain_posts
  cargo fmt   -p domain_posts -- --check
  cargo clippy -p domain_posts --all-targets -- -D warnings
  ```

## 6. Update `apps/api/Dockerfile` and `deployments/docker-swarm/apps/docker-compose.yaml`

- [ ] 6.1 Edit `apps/api/Dockerfile:36` to build only the gateway binary:
  ```dockerfile
  RUN cargo build --release --bin my-cms-api
  ```
  Edit `apps/api/Dockerfile:23-34` to replace the multi-binary comment with a single-binary comment that references the `migrate` compose service. **Verification:** `rg -- '--bin my-cms-api --bin domain_posts' apps/api/Dockerfile` returns no matches; `rg -- '--bin my-cms-api' apps/api/Dockerfile` returns exactly one match.

- [ ] 6.2 Edit `apps/api/Dockerfile:41-42` to copy only the gateway binary. Delete line 42 (`COPY --from=builder /app/target/release/domain_posts /app/domain_posts`). **Verification:** `rg 'domain_posts' apps/api/Dockerfile` returns only the comment reference (after the rewrite) and no `COPY` line.

- [ ] 6.3 Edit `deployments/docker-swarm/apps/docker-compose.yaml:67-68` to retarget the `migrate` service:
  ```yaml
  entrypoint: ["/app/my-cms-api"]
  command: ["migrate", "up"]
  ```
  **Verification:** `rg '"/app/domain_posts"' deployments/` returns no matches; `rg '"/app/my-cms-api"' deployments/docker-swarm/apps/docker-compose.yaml` returns the new entrypoint.

- [ ] 6.4 Run the focused verification for step 6:
  ```bash
  docker build -f apps/api/Dockerfile -t my-cms-api:dev apps/api
  docker run --rm my-cms-api:dev migrate --list
  # Expected: four lines, one per migration id, in original order.
  ```

## 7. Update documentation

- [ ] 7.1 `docs/api-architecture.md` — replace every `domain_posts migrate up` reference with `my-cms-api migrate up` (operator path) and every `cargo run -p domain_posts -- migrate` reference with `cargo run -p gateway -- migrate up` (local path). Keep archive and historical references unchanged. **Verification:** `rg 'domain_posts migrate|cargo run -p domain_posts -- migrate|/app/domain_posts' docs/api-architecture.md` returns no matches outside explicitly-labelled historical sections.

- [ ] 7.2 `docs/pluggable-domain-refactor.md` — replace `cargo run -p domain_posts -- migrate --list` with `cargo run -p gateway -- migrate --list` and `cargo run -p domain_posts` (standalone) with `cargo run -p gateway` (composed). Update Stage 4 description to mark the standalone-binary removal as completed. **Verification:** `rg 'cargo run -p domain_posts' docs/pluggable-domain-refactor.md` returns no matches.

- [ ] 7.3 `docs/ai-platform.md:58` — replace `cargo run -p domain_posts -- migrate [--list]` with `cargo run -p gateway -- migrate [--list]`. **Verification:** `rg 'domain_posts -- migrate' docs/ai-platform.md` returns no matches.

- [ ] 7.4 `.opencode/agents/product-owner.md:72` — replace `/app/domain_posts migrate up` with `/app/my-cms-api migrate up`. **Verification:** `rg 'domain_posts' .opencode/agents/product-owner.md` returns no matches outside historical notes.

- [ ] 7.5 `.opencode/agents/software-architect.md:97` — replace `apps/api/domain_posts/src/main.rs` with `apps/api/gateway/src/main.rs` in the migration-CLI row. **Verification:** `rg 'domain_posts/src/main.rs' .opencode/agents/software-architect.md` returns no matches outside historical notes.

- [ ] 7.6 `docs/superpowers/plans/2026-08-08-remove-legacy-migration-crate.md` — leave as historical archaeology (per `design.md` Decision 7); no edits.

- [ ] 7.7 Run the focused verification for step 7:
  ```bash
  rg 'domain_posts migrate|cargo run -p domain_posts -- migrate|/app/domain_posts|domain_posts/src/main.rs' \
     docs/ .opencode/
  ```
  **Expected:** only matches under `openspec/changes/archive/` and `docs/superpowers/`.

## 8. Full verification

- [ ] 8.1 Run the AGENTS.md §"Verify Before Commit" gate:
  ```bash
  cargo check                 # workspace
  cargo test                  # workspace
  cargo fmt -- --check        # workspace
  cargo clippy                # workspace
  pnpm --dir apps/web build   # likely a no-op (no web changes); record outcome
  ```
  **Expected:** every command exits 0. Record the output of each.

- [ ] 8.2 Run the code-review-graph MCP gate per AGENTS.md §"Phase 3" before claiming done:
  ```bash
  # Use the code-review-graph MCP tools (require the project to be registered).
  get_minimal_context(task="wire-all-domains-and-collapse-to-gateway-binary")
  detect_changes(base=HEAD, include_source=true, max_depth=2)
  get_impact_radius(max_depth=2)
  tests_for(target="apps/api/gateway/src/main.rs")
  ```
  Resolve every material finding or document why it is not applicable. If the graph server is unavailable, record the limitation and substitute `git diff HEAD~1 -- apps/api/ deployments/docker-swarm/apps/` plus the per-crate `cargo test` output.

- [ ] 8.3 Run the manual smoke:
  ```bash
  cargo run -p gateway -- migrate --list
  # Expected: four migration ids in original order.
  cargo run -p gateway
  # Expected: HTTP listener on :8989; `curl http://localhost:8989/health` returns 200 with "CMS is running successfully!".
  ```

- [ ] 8.4 Run the testcontainer integration smoke (if a local Docker daemon is available):
  ```bash
  docker compose -f deployments/docker-swarm/supabase/docker-compose.yaml \
                 --env-file deployments/docker-swarm/supabase/.env up -d
  docker compose -f deployments/docker-swarm/apps/docker-compose.yaml \
                 --env-file deployments/docker-swarm/apps/.env up migrate
  # Expected: migrate container exits 0; the four migrations are recorded.
  docker compose -f deployments/docker-swarm/apps/docker-compose.yaml \
                 --env-file deployments/docker-swarm/apps/.env up my-cms-api
  # Expected: my-cms-api boots and serves /health, /posts/graphql/immutable, /users (after auth), /media.
  ```
  Record each command's output in the verification log.

- [ ] 8.5 Run the OpenSpec status check:
  ```bash
  openspec status --change "wire-all-domains-and-collapse-to-gateway-binary" --json
  ```
  **Expected:** all `applyRequires` artifacts report `done`. The change is ready for the `software-engineer` review and apply.
