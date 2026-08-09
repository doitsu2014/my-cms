## MODIFIED Requirements

### Requirement: Gateway is the sole deployed API binary
The workspace SHALL NOT publish a per-domain standalone binary that exposes CMS routes. The container image SHALL contain exactly one HTTP-serving binary (`my-cms-api`). Migrations are NOT considered HTTP-serving and remain an in-process operator command of the gateway binary.

#### Scenario: No standalone domain binary is shipped
- **WHEN** `cargo build --release --workspace --bins` runs from `apps/api/`
- **THEN** exactly one HTTP-serving binary (`my-cms-api`) is produced
- **AND** `target/release/domain_posts`, `target/release/domain_media`, `target/release/domain_user`, `target/release/domain_auth` are absent
- **AND** `apps/api/Dockerfile` builds the workspace (the gateway depends on every `domain_*` crate) and the runtime stage `COPY --from=build` line copies exactly one binary (`my-cms-api`)
- **AND** the runtime image `ls /app` contains only `my-cms-api`

#### Scenario: No per-domain bin target exists in route-owning domains
- **WHEN** the workspace is inspected after the change
- **THEN** `apps/api/domain_posts/Cargo.toml` does not declare a `[[bin]]` block
- **AND** `apps/api/domain_media/Cargo.toml` does not declare a `[[bin]]` block
- **AND** `apps/api/domain_user/Cargo.toml` does not declare a `[[bin]]` block
- **AND** `apps/api/domain_auth/Cargo.toml` MAY continue to declare a placeholder `[[bin]]` only if its `main.rs` is preserved verbatim; the binary MUST NOT be shipped in the Docker image

#### Scenario: Migrations remain a library function
- **WHEN** `domain_posts::migrations_cli::run(conn)` is inspected
- **THEN** it remains callable from `gateway` without spawning a child process
- **AND** the `Migrator::up(conn, None)` semantics are unchanged
- **AND** `domain_posts::migrations::Migrator` continues to expose the four canonical migration identities in the original order

### Requirement: Gateway exposes migration CLI subcommand
The `my-cms-api` binary SHALL accept a first positional argument `migrate` followed by a subcommand verb (`up`, `down`, `status`, or `--list`). When invoked as `my-cms-api migrate <verb>` the binary SHALL run the gateway's migration orchestrator against the shared `DatabaseConnection`, exit with `ExitCode::SUCCESS` on success or `ExitCode::FAILURE` on failure, and MUST NOT bind the HTTP listener. When invoked without arguments the binary SHALL continue to bind the HTTP listener (existing behaviour).

#### Scenario: Docker migrate service uses the gateway binary
- **WHEN** the Docker Swarm `migrate` service starts
- **THEN** its `entrypoint` is `["/app/my-cms-api"]`
- **AND** its `command` is `["migrate", "up"]`
- **AND** no `/app/domain_posts` binary is referenced anywhere in `deployments/`

### Requirement: Operator-facing documentation matches the deployed CLI surface
Every operator-facing doc file that names a `domain_posts` migration CLI invocation SHALL be updated to name the gateway-binary equivalent. Historical references in `openspec/changes/archive/` and `docs/superpowers/plans/2026-08-08-remove-legacy-migration-crate.md` are exempt (decision archaeology).

#### Scenario: docs/api-architecture.md names the gateway binary
- **WHEN** the file is inspected after the change
- **THEN** no live operator reference to `domain_posts migrate up`, `cargo run -p domain_posts -- migrate`, or `/app/domain_posts` remains outside explicitly-labelled historical sections

#### Scenario: docs/pluggable-domain-refactor.md marks the standalone bin removed
- **WHEN** the file is inspected after the change
- **THEN** Stage 4 description marks the standalone-binary removal as completed
- **AND** every `cargo run -p domain_posts` reference is replaced with `cargo run -p gateway`
- **AND** no live reference to `cargo run -p domain_posts -- migrate --list` remains

#### Scenario: docs/ai-platform.md names the gateway binary
- **WHEN** the file is inspected after the change
- **THEN** line 58 names `cargo run -p gateway -- migrate [--list]`
- **AND** no live reference to `cargo run -p domain_posts -- migrate [--list]` remains

#### Scenario: .opencode/agents/product-owner.md names the gateway binary
- **WHEN** the file is inspected after the change
- **THEN** line 72 names `/app/my-cms-api migrate up`
- **AND** no live reference to `/app/domain_posts` remains outside historical notes

#### Scenario: .opencode/agents/software-architect.md points at the gateway binary
- **WHEN** the file is inspected after the change
- **THEN** the migration-CLI row (around line 75-97) points at `apps/api/gateway/src/main.rs`
- **AND** no live reference to `apps/api/domain_posts/src/main.rs` remains outside historical notes
