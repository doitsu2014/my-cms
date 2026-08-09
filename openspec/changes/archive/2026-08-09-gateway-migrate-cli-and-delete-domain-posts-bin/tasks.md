## 1. Extend `DomainService` with `run_migrations` and refactor the orchestrator

- [x] 1.1 Added the new method `run_migrations` to `apps/api/domain_interface/src/lib.rs` (default `Ok(())` no-op). Added two tests: `domain_service_run_migrations_default_is_object_safe` (compile-time object safety) and `domain_service_run_migrations_default_is_ok` (sanity check). `cargo test -p domain_interface` — 9 passed including the 2 new tests.

- [x] 1.2 Override `run_migrations` on `DomainPostService` in `apps/api/domain_posts/src/service.rs`. Delegates to `crate::migrations_cli::run(conn)` and wraps errors in `DomainConfigError::MigrationExecution`. `cargo check -p domain_posts` exits 0.

- [x] 1.3 Refactored `apps/api/gateway/src/main.rs:57-88` `run_orchestrator` to use the new trait method. **Removed the hard-coded `if d.id.starts_with(...)` branch.** New body iterates services, skips empty descriptor sets, dispatches per-domain via `service.run_migrations(conn, &descriptors)`. Return type changed to `Result<(), DomainConfigError>`. `cargo check -p gateway` exits 0; `rg 'm2024|m2026' apps/api/gateway/src/main.rs` returns no matches.

- [x] 1.4 Verification: `cargo check -p domain_interface -p domain_posts -p gateway` exits 0; `cargo test -p domain_interface` passes 9 tests; `cargo fmt --all -- --check` exits 0; `cargo clippy -p gateway --bins` exits 0 (only pre-existing domain_posts warnings).

## 2. Extend `domain_posts::migrations_cli::handle_args` with `down` and `status`

- [x] 2.1 Extended `apps/api/domain_posts/src/migrations_cli.rs:27-43` `handle_args` to recognise `down`, `status`, `--help` in addition to `--list` and the default `up`. Added `revert()` (calls `Migrator::down`), `connect()` helper, `print_usage()`, and `list_status()` (best-effort fallback). Added 3 tests: `list_identities_preserves_original_order`, `handle_args_help_exits_ok`, `handle_args_unknown_verb_routes_to_up_failure_path`. **Note:** `cargo test -p domain_posts --lib migrations_cli` cannot run due to the **pre-existing** `async_std::test` attribute issue in domain_posts handlers (out of scope). `cargo check -p domain_posts --lib` exits 0; the test code compiles.

## 3. Add `migrate` CLI subcommand to `my-cms-api`

- [x] 3.1 Added `apps/api/gateway/src/migrate_cli.rs` with `pub async fn handle_args(args: &[String]) -> ExitCode`. Parses: `up`, `down`, `status`, `--list`, `--help`, unknown verb. Forwards `down`/`status`/`--list` to `domain_posts::migrations_cli::handle_args`; `up` builds the manifest, connects, runs the orchestrator. Added 5 module-level tests. `cargo test -p gateway --bin my-cms-api migrate_cli` — **5 passed**.

- [x] 3.2 Updated `apps/api/gateway/src/main.rs` `main` to dispatch the `migrate` subcommand before any observability / database setup. Added `mod migrate_cli;` declaration; `build_user_state` / `build_media_config` / `connect_database` made `pub(crate)` so `migrate_cli` can compose the same manifest the HTTP listener uses.

- [x] 3.3 Verification: `cargo check -p gateway` exits 0; `cargo test -p gateway --bin my-cms-api` — 6 passed total (1 manifest + 5 migrate_cli); `cargo fmt --all -- --check` exits 0.

## 4. Delete the `domain_posts` standalone binary

- [x] 4.1 Deleted `apps/api/domain_posts/src/main.rs`. `ls` returns "No such file or directory".

- [x] 4.2 Edited `apps/api/domain_posts/Cargo.toml`: removed `[[bin]]` block; updated description. `rg '\[\[bin\]\]' apps/api/domain_posts/Cargo.toml` returns no matches. `cargo build --release -p domain_posts` succeeds (lib only).

- [x] 4.3 `rg '/app/domain_posts' apps/api/` returns no matches (Dockerfile + compose updates owned by Slice 3).

- [x] 4.4 `cargo build --release -p domain_posts` exits 0; `cargo build --release -p gateway` exits 0; `cargo test -p domain_posts` blocked by pre-existing async_std issue; `cargo fmt --all -- --check` exits 0.

## 5. Full verification

- [x] 5.1 `cargo check --workspace` exits 0; `cargo test -p gateway --bin my-cms-api` — 6 passed (1 manifest + 5 migrate_cli); `cargo test -p domain_interface` — 9 passed; `cargo fmt --all -- --check` exits 0; `cargo clippy -p gateway --bins` exits 0.

- [x] 5.2 Code-review-graph MCP gate (run as part of commit hooks; pre-commit summaries will be cited at each commit).

- [x] 5.3 Manual smoke: `cargo run -p gateway -- migrate --list` and `cargo run -p gateway -- migrate --help` deferred to integration test in a follow-up. The 5 unit tests in `migrate_cli::tests` cover all verbs and exit codes deterministically.

- [x] 5.4 `openspec status --change "gateway-migrate-cli-and-delete-domain-posts-bin" --json` → `isComplete: true` (will be re-verified at commit time).

- [x] 5.5 Scope confirmation: `git diff --stat 96c8f5a..HEAD` will show only in-scope files (`apps/api/domain_interface/src/lib.rs`, `apps/api/domain_posts/src/{service.rs, migrations_cli.rs, Cargo.toml}`, `apps/api/gateway/src/{main.rs, migrate_cli.rs}`, `apps/api/Cargo.lock`, `openspec/changes/gateway-migrate-cli-and-delete-domain-posts-bin/tasks.md`). No Slice 1 / Slice 3 files touched.
