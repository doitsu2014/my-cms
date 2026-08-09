## Why

The gateway binary's startup orchestrator (`apps/api/gateway/src/main.rs:57-88`)
hard-codes a per-domain migration dispatch arm that inspects
`MigrationDescriptor::id` prefixes (`"m2024"`, `"m2026"`) and forwards them
to `domain_posts::migrations_cli::run(conn)`. Every descriptor that does
not match one of those two prefixes is silently warned and skipped. The
hard-coded arm:

1. Leaks `domain_posts` knowledge into the gateway composition root
   (violates the `domain-interface` design intent that
   `openspec/changes/archive/2026-08-08-refactor-api-into-pluggable-domain-libraries/design.md:486`
   captures as "Each domain owns its migration set and its migration
   runner").
2. Prevents any future domain from adding a runner without modifying the
   gateway's `run_orchestrator` body.
3. Coexists with the standalone `domain_posts` binary
   (`apps/api/domain_posts/src/main.rs:1-176`) that exists only to run
   migrations for the Docker Swarm `migrate` compose service. That bin
   duplicates the entire HTTP gateway boot path.

The coordinator resolved open question #2 (per the parent change's
`design.md:604-609`): ship **all four** CLI verbs (`up`, `down`, `status`,
`--list`). This slice extends `DomainService` with a generic `run_migrations`
runner, refactors the orchestrator to use it, adds the
`my-cms-api migrate <verb>` CLI subcommand (covering all four verbs), and
deletes the `domain_posts` standalone binary. It deliberately does NOT
modify the Dockerfile, the docker-compose migrate service, or any
documentation — those are owned by Slice 3.

## What Changes

- **MODIFIED** `apps/api/domain_interface/src/lib.rs` — extend
  `DomainService` with `async fn run_migrations(&self, _conn: &DatabaseConnection, _descriptors: &[MigrationDescriptor]) -> Result<(), DomainConfigError>`
  with a default `Ok(())` implementation. Domains that own migrations
  (currently only `domain_posts`) override the method.
- **MODIFIED** `apps/api/domain_posts/src/service.rs` — `DomainPostService`
  overrides `run_migrations` to delegate to
  `crate::migrations_cli::run(conn)`.
- **MODIFIED** `apps/api/domain_posts/src/migrations_cli.rs` — extend
  `handle_args` with `down` and `status` arms. `down` invokes
  `Migrator::down(conn, None)` (sea-orm-migration 1.1.20 API). `status`
  prints the applied/pending state via
  `MigratorTrait::get_migration_status(conn)` (also 1.1.20 API); on API
  mismatch, fall back to a stub that prints the four migration ids with an
  "applied status unavailable in this sea-orm-migration version" note and
  exits 0.
- **MODIFIED** `apps/api/gateway/src/main.rs:57-88` — `run_orchestrator`
  iterates services, asks each for `migrations()`, deduplicates by id,
  sorts by id (preserves `depends_on` ordering for future domains), and
  calls `service.run_migrations(conn, &descriptors).await`. The hard-coded
  `if d.id.starts_with("m2024") || d.id.starts_with("m2026")` branch is
  deleted.
- **ADDED** `apps/api/gateway/src/migrate_cli.rs` — new module exporting
  `pub async fn handle_args(args: &[String]) -> ExitCode`. Parses the
  `migrate` subcommand's positional argument: `up` / `down` / `status` /
  `--list` / `--help` / unknown verb (exit 1, print usage to stderr). For
  each verb:
  - `up`: connects to `DATABASE_URL`, calls the orchestrator, exits 0/1.
  - `down`: forwards to `domain_posts::migrations_cli::handle_args` with
    `["down"]` so the existing domain-owned CLI does the work.
  - `status`: forwards to `domain_posts::migrations_cli::handle_args`
    with `["status"]`.
  - `--list`: forwards to `domain_posts::migrations_cli::handle_args`
    with `["--list"]`.
  - `--help`: prints usage to stdout, exits 0.
- **MODIFIED** `apps/api/gateway/src/main.rs:91-93` — `main` dispatches the
  `migrate` subcommand before any observability / database setup.
- **MODIFIED** `apps/api/domain_posts/Cargo.toml` — remove the
  `[[bin]]` block (lines 13-15); update the `description` (line 5) to
  remove the phrase "and a standalone bin".
- **DELETED** `apps/api/domain_posts/src/main.rs` (176 lines).

## Capabilities

### Modified Capabilities

- `domain-api-cutover`: The "Gateway is the sole deployed API binary"
  requirement is extended with the "Migrations remain a library function"
  scenario (already drafted in the parent change's spec). The "Generic
  migration orchestrator dispatch" requirement lands here with all five
  scenarios from the parent change's spec (orchestrator dispatches via
  trait, empty-descriptor skip, dependency order, failure envelope).
  The "Gateway exposes migration CLI subcommand" requirement lands here
  with all four scenarios (migrate up exits, migrate --list prints,
  default invocation still boots HTTP, Docker migrate service uses
  gateway binary — the last is partially owned by Slice 3).

## Impact

- Affected code (all MODIFIED unless noted ADDED/DELETED):
  - `apps/api/domain_interface/src/lib.rs` — extend trait with default
    `run_migrations`.
  - `apps/api/domain_posts/src/service.rs` — override `run_migrations`.
  - `apps/api/domain_posts/src/migrations_cli.rs` — add `down` and `status`
    arms.
  - `apps/api/gateway/src/main.rs` — refactor `run_orchestrator`; dispatch
    `migrate` CLI at top of `main`.
  - `apps/api/gateway/src/migrate_cli.rs` — ADDED.
  - `apps/api/domain_posts/Cargo.toml` — remove `[[bin]]`.
  - `apps/api/domain_posts/src/main.rs` — DELETED.
- Affected tests: new module-level tests in
  `domain_interface::lib::tests`, `gateway::migrate_cli::tests`,
  `domain_posts::migrations_cli::tests`. New testcontainer integration
  test in `gateway::orchestrator::tests` that boots the gateway against a
  fresh PG, runs `migrate up`, then runs a second `migrate up` and
  asserts idempotency.
- Affected tests in `apps/api/test_helpers`: no change. The crate already
  imports `domain_posts::migrations::{Migrator, MigratorTrait}` directly
  (per parent change's `design.md:133-136`).
- **BREAKING**: any operator that invoked `/app/domain_posts migrate up` in
  a `docker-compose.override.yaml` (the deployed image no longer contains
  that binary). Slice 3 retargets the docker-compose migrate service, but
  external overrides need a one-line edit per override (documented in
  `proposal.md` Impact of the parent change).
- **BREAKING**: any local developer workflow that used
  `cargo run -p domain_posts` to boot the standalone post-domain service.
  The replacement is `cargo run -p gateway` (composes every registered
  domain — strictly more functionality).
- No new HTTP routes. No new env vars. No new database migrations. The
  four canonical migration identities are preserved exactly.
- No Dockerfile change (Slice 3). No docker-compose change (Slice 3). No
  documentation change (Slice 3). No `domain_interface` method addition
  beyond `run_migrations` (Slice 1 did NOT touch `domain_interface`).

## Traceability to parent change

This slice is `tasks.md` §§3-5 of the parent change
`wire-all-domains-and-collapse-to-gateway-binary`. The parent change's
`proposal.md` (lines 39-67), `design.md` Decisions 1-3 + 5, and
`specs/domain-api-cutover/spec.md` "Generic migration orchestrator
dispatch" / "Gateway exposes migration CLI subcommand" ADDED Requirements
all apply verbatim to this slice.
