## 1. Add `MediaConfig::from_env` factory in `domain_media`

- [x] 1.1 Read the existing `MediaConfig` struct and `SupabaseStorage::new` constructor to confirm the four env-var → field mapping. **Verification:** `rg 'pub struct MediaConfig|pub fn new' apps/api/domain_media/src/handlers/mod.rs apps/api/domain_media/src/handlers/supabase_storage/mod.rs` returns one struct definition and one `new` constructor.

- [x] 1.2 Add `pub fn from_env() -> Result<MediaConfig, DomainConfigError>` to `apps/api/domain_media/src/handlers/mod.rs`. The factory reads `SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY`, `MEDIA_BUCKET`, `MEDIA_BASE_URL` (in that order) and returns `DomainConfigError::MissingEnv(<var>)` on the first missing one. The factory MUST NOT panic. **Test-first:** add `#[cfg(test)] mod tests` with `media_config_from_env_returns_ok_when_all_vars_set` (sets all four vars, calls the factory, asserts the four fields) and `media_config_from_env_returns_missing_env_for_supabase_url_when_unset` (unsets only `SUPABASE_URL`, asserts `Err(DomainConfigError::MissingEnv("SUPABASE_URL"))`). **Verification:** `cargo test -p domain_media --lib handlers::tests` passes both tests.

## 2. Add `DomainUserService` + `domain_user::api::routes` aggregator

- [x] 2.1 Read every existing handler module to confirm the exported handler symbol name and signature: `apps/api/domain_user/src/handlers/{create,delete,modify,read_list,read_one,reset_password}/mod.rs` and `apps/api/domain_user/src/handlers/supabase_admin_client.rs`. Record each handler's `pub` struct (e.g. `CreateUserHandler`) and `pub` trait (e.g. `CreateUserHandlerTrait`) in the new `api/routes.rs` design notes. **Verification:** `rg 'pub struct|pub trait' apps/api/domain_user/src/handlers` returns seven handler structs + seven traits.

- [x] 2.2 Add `apps/api/domain_user/src/api/mod.rs` declaring `pub mod routes;` and `pub mod state;`. **Verification:** `cargo check -p domain_user` exits 0; `rg 'pub mod routes|pub mod state' apps/api/domain_user/src/api/mod.rs` matches both lines.

- [x] 2.3 Add `apps/api/domain_user/src/api/state.rs` with `pub struct UserApiState { pub supabase_admin_client: Arc<SupabaseAdminClient> }` plus `pub fn new(client: SupabaseAdminClient) -> Self` and a `Debug` impl that redacts the service-role key. Mirror `apps/api/domain_media/src/api/state.rs`. **Verification:** `cargo check -p domain_user` exits 0; `rg 'pub struct UserApiState|pub fn new' apps/api/domain_user/src/api/state.rs` matches.

- [x] 2.4 Add `apps/api/domain_user/src/api/routes.rs` with `pub fn routes(state: UserApiState) -> Vec<RouteRegistration>`. Wire six Axum routes on `Mount::Administrator` (one each for create / read_list / read_one / modify / delete / reset_password). The HTTP adapters construct the corresponding handler struct from `state.supabase_admin_client.clone()` and call the trait method. **Test-first:** add `#[cfg(test)] mod tests` with `routes_returns_administrator_mount_only` (asserts every returned `RouteRegistration` has `mount == Mount::Administrator` and `prefix == "users"`). **Verification:** `cargo test -p domain_user --lib api::routes` passes.

- [x] 2.5 Add `apps/api/domain_user/src/service.rs` with `pub struct DomainUserService { state: UserApiState }`, `pub fn new(client: SupabaseAdminClient) -> Self`, `pub fn from_state(state: UserApiState) -> Self`, and `impl DomainService for DomainUserService` mirroring `apps/api/domain_media/src/service.rs:60-93`. **Verification:** `cargo check -p domain_user --lib` exits 0; `cargo fmt -p domain_user -- --check` exits 0.

- [x] 2.6 Update `apps/api/domain_user/src/lib.rs` to add `pub mod api;`, `pub mod service;`, and `pub use service::DomainUserService;` next to the existing `pub use domain::error::AppError;`. **Verification:** `cargo build -p domain_user` exits 0; `rg 'pub mod api|pub mod service|pub use service::DomainUserService' apps/api/domain_user/src/lib.rs` matches all three lines.

- [x] 2.7 Run the focused verification for step 2:
  ```bash
  cargo check -p domain_user --lib          # exits 0 (only 2 pre-existing missing-debug warnings)
  cargo fmt   -p domain_user -- --check     # exits 0
  cargo clippy -p domain_user --lib         # exits 0 with 2 pre-existing missing-debug warnings (no -D warnings)
  ```
  Note: `cargo check -p domain_user --all-targets`, `cargo test -p domain_user`, and `cargo clippy -p domain_user --all-targets -- -D warnings` all fail due to the **pre-existing** `async_std::test` attribute and `missing_debug_implementations` issues that are out of scope for Slice 1 (see user prompt). These will be addressed in a separate change.

## 3. Wire `domain_media` and `domain_user` into the gateway composition

- [ ] 3.1 Update `apps/api/gateway/Cargo.toml`: add `domain_media = { path = "../domain_media" }` and `domain_user = { path = "../domain_user" }` to `[dependencies]` (alphabetical order, matching the existing `domain_auth`/`domain_posts`/`domain_interface` lines). Update the `description` on line 5 to mention the four-domain composition. **Verification:** `cargo check -p gateway` exits 0; `rg 'domain_media|domain_user' apps/api/gateway/Cargo.toml` returns both lines.

- [ ] 3.2 Update `apps/api/gateway/src/main.rs`. The new shape (paste this block):
  ```rust
  pub fn manifest(
      media_config: Arc<MediaConfig>,
      user_state: domain_user::api::state::UserApiState,
  ) -> Vec<Box<dyn DomainService>> {
      vec![
          Box::new(DomainPostService::new()),
          Box::new(DomainAuthService::new()),
          Box::new(DomainMediaService::new(media_config)),
          Box::new(DomainUserService::from_state(user_state)),
      ]
  }
  ```
  At the top of `main`, after `init_observability()`, read env vars and construct `SupabaseAdminClient`, `MediaConfig` (via `MediaConfig::from_env()`), and `UserApiState`. Fail-fast on missing env via `eprintln!` + `ExitCode::FAILURE`. **Test-first:** add a module-level test `manifest_with_four_services_returns_four_entries` that calls `manifest(test_media_config, test_user_state)` and asserts `services.len() == 4` and every entry's `health().name` is one of the four domain names. **Verification:** `cargo test -p gateway --lib main` passes; `rg 'Box::new\(DomainMediaService|Box::new\(DomainUserService' apps/api/gateway/src/main.rs` matches both lines.

- [ ] 3.3 Run the focused verification for step 3:
  ```bash
  cargo check -p gateway
  cargo test  -p gateway
  cargo fmt   -p gateway -- --check
  cargo clippy -p gateway --all-targets -- -D warnings
  ```
  All commands exit 0. Record the output.

## 4. Full verification

- [ ] 4.1 Run the AGENTS.md §"Verify Before Commit" gate for the changed crates:
  ```bash
  cargo check -p domain_user -p domain_media -p gateway
  cargo test  -p domain_user -p domain_media -p gateway
  cargo fmt   -p domain_user -p domain_media -p gateway -- --check
  cargo clippy -p domain_user -p domain_media -p gateway --all-targets -- -D warnings
  ```
  **Expected:** every command exits 0. Record the output.

- [ ] 4.2 Run the code-review-graph MCP gate per AGENTS.md §"Phase 3" before claiming done:
  ```bash
  get_minimal_context(task="wire-domain-user-and-domain-media-into-gateway")
  detect_changes(base=HEAD, include_source=true, max_depth=2)
  get_impact_radius(max_depth=2)
  tests_for(target="apps/api/gateway/src/main.rs")
  ```
  Resolve every material finding or document why it is not applicable. If the graph server is unavailable, record the limitation and substitute `git diff HEAD~1 -- apps/api/domain_user/ apps/api/domain_media/ apps/api/gateway/` plus the per-crate `cargo test` output.

- [ ] 4.3 Run the manual smoke:
  ```bash
  SUPABASE_URL=http://localhost SUPABASE_SERVICE_ROLE_KEY=dummy \
  MEDIA_BUCKET=test MEDIA_BASE_URL=http://localhost \
  DATABASE_URL=postgres://localhost/test \
  SUPABASE_JWT_SECRET=dummy OPENAI_API_KEY=dummy \
  cargo run -p gateway
  # Expected: startup banner reports four registered domain services;
  # HTTP listener binds on :8989 and `curl http://localhost:8989/health` returns 200.
  ```
  Record the output (truncate if long).

- [ ] 4.4 Run the OpenSpec status check:
  ```bash
  openspec status --change "wire-domain-user-and-domain-media-into-gateway" --json
  ```
  **Expected:** `isComplete: true`; every `applyRequires` artifact reports `done`. The change is ready for the `software-engineer` review and apply.

- [ ] 4.5 Confirm no other slice's work has been touched:
  ```bash
  git status --porcelain | rg 'openspec/changes/(gateway-migrate-cli-and-delete-domain-posts-bin|single-binary-docker-image-and-docs|wire-all-domains-and-collapse-to-gateway-binary)/|apps/api/(Dockerfile|migration)|deployments/|docs/'
  # Expected: no matches.
  ```
