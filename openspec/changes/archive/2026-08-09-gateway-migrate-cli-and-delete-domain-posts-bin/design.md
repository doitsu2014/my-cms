## Context

### Source-derived current state (revalidated 2026-08-09)

**Migration orchestrator** (`apps/api/gateway/src/main.rs:57-88`): the
gateway runs migrations at boot via `run_orchestrator`. The implementation:

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

**`domain_posts::migrations_cli`** (`apps/api/domain_posts/src/migrations_cli.rs:1-44`):
the library exposes three functions — `run(conn)` (line 14-16),
`list_identities()` (line 20-22), `handle_args(args)` (line 27-43). All
three are library-only and callable from any crate that depends on
`domain_posts`. The `handle_args` function is the CLI parser used by the
standalone bin; the gateway's orchestrator calls `run` directly.

**DomainInterface trait** (`apps/api/domain_interface/src/lib.rs:156-184`):
the `DomainService` trait exposes six methods (`health`, `required_env`,
`validate_config`, `migrations`, `register_routes`, `startup_health`).
The new `run_migrations` method lands at line 184 (immediately after
`startup_health`).

### Graph evidence

`code-review-graph_get_minimal_context(task="decompose parent wire-all-domains...")`
reports 1675 nodes / 16779 edges / risk `low (0.00)`. The graph was built
at HEAD (`5735e47`), branch `feat/wire-all-domains-and-collapse-to-gateway-binary`,
`head_matches_build: true`. The graph is current; every cited file path
and symbol was revalidated against current source via `read`/`grep`.

### Constraints (from AGENTS.md, the parent change, and pre-existing findings)

- Honour the layered architecture (gateway / domain_* / domain_interface).
- `DomainService::run_migrations` MUST have a default `Ok(())` impl so
  existing implementors compile unchanged. (`DomainPostService`,
  `DomainMediaService`, `DomainAuthService` are the three existing
  implementors; `DomainUserService` is added by Slice 1.)
- `Migrator::down` and `MigratorTrait::get_migration_status` are
  sea-orm-migration 1.1.20 APIs. If `get_migration_status` does not exist
  in this version, fall back to a stub.
- No new HTTP routes. No new env vars. No new database migrations. The
  four canonical migration identities are preserved exactly.
- Pre-existing `async_std::test` failures in `domain_posts` /
  `domain_media` / `domain_user` are unrelated; do not regress them.
- The `purge-legacy-cms-and-application-core` change (41/47 tasks) edits
  the same Dockerfile but does not touch any Rust file in this slice's
  scope. Coordinate via the software-engineer review step before merging.

## Goals / Non-Goals

**Goals**

- `DomainService::run_migrations` is a trait method with a default `Ok(())`
  no-op implementation. `DomainPostService` overrides it.
- `gateway::run_orchestrator` dispatches per-domain via the new trait
  method. The hard-coded `m2024` / `m2026` prefix branch is deleted.
- `my-cms-api migrate <verb>` works for `up`, `down`, `status`, `--list`,
  and `--help`. Unknown verbs exit 1.
- `apps/api/domain_posts/src/main.rs` is deleted. `[[bin]]` is removed
  from `domain_posts/Cargo.toml`. `domain_posts` compiles as a library
  only.
- All four canonical migration identities
  (`m20240409_151952_release_100`, `m20250330_151455_release_110`,
  `m20260126_040610_release_300`, `m20260531_000001_pgvector`) are
  preserved exactly. `cargo run -p gateway -- migrate --list` prints them
  in original order.
- The CLI subcommand path never binds the HTTP listener.
- The default invocation (`my-cms-api` with no args) continues to boot
  the HTTP server and run migrations at startup (idempotent against
  sea-orm-migration's tracking table).

**Non-Goals**

- No Dockerfile change (Slice 3).
- No docker-compose change (Slice 3).
- No documentation change (Slice 3).
- No `MediaConfig::from_env` change (Slice 1).
- No `DomainUserService` change (Slice 1).
- No new HTTP routes. No new env vars. No new database migrations.
- No change to `domain_auth`'s placeholder bin
  (`apps/api/domain_auth/src/main.rs` is not in this slice's scope).
- No change to the `k8s` Helm chart (`deployments/k8s/`) — already
  targets the `my-cms-api` deployment.
- No change to OpenTelemetry / Jaeger wiring.

## Decisions

### Decision 1: Generic migration dispatch via `DomainService::run_migrations`

**Driver.** The hard-coded `if d.id.starts_with("m2024") ||
d.id.starts_with("m2026")` arm at `gateway/src/main.rs:77-85` is a
violation of the design intent captured in the archived
`refactor-api-into-pluggable-domain-libraries` change ("Each domain owns
its migration set and its migration runner").

**Decision.** Extend `domain_interface::DomainService` with one new async
method:

```rust
async fn run_migrations(
    &self,
    _conn: &sea_orm::DatabaseConnection,
    _descriptors: &[MigrationDescriptor],
) -> Result<(), DomainConfigError> {
    Ok(())
}
```

Domains that own migrations (currently only `domain_posts`) override this
default. `domain_media`, `domain_user`, `domain_auth` use the default
no-op. The gateway orchestrator iterates the services, asks each for
`migrations()`, deduplicates, sorts, and calls
`run_migrations(conn, &descriptors)` once per service.

**Alternatives considered.**
- (a) **Pass a closure into the descriptor.** *Rejected*: forces every
  migration id to know its runner; re-introduces per-domain hard coding.
- (b) **Add a separate `MigrationRunner` trait.** *Rejected*: doubles the
  registration surface in `manifest()`.
- (c) **Keep the hard-coded branch but extend it.** *Rejected*: scales
  linearly with domains.

**Consequences.** `domain_interface` grows by one method (default `Ok(())`).
The orchestrator simplifies to:

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

**Contracts.** No API change. The CLI subcommand surface (Decision 2)
calls the same orchestrator with no behavioural difference.

**Migration / rollout.** Edit `apps/api/domain_interface/src/lib.rs` (one
trait method + default impl). Override on `DomainPostService`. Remove the
hard-coded `if d.id.starts_with(...)` arm. No database change.

**Verification.** `cargo check -p domain_interface -p domain_posts -p gateway`
exits 0.

### Decision 2: `my-cms-api migrate <verb>` CLI subcommand

**Driver.** The standalone `domain_posts` bin exists only to run
migrations for the Docker Swarm `migrate` compose service
(`apps/api/domain_posts/src/main.rs:163-171`). Retiring the bin requires
moving the migration CLI surface to the gateway binary.

**Decision.** Add a tiny CLI parser at the top of `gateway::main`:

```rust
let args: Vec<String> = std::env::args().skip(1).collect();
if args.first().map(|s| s.as_str()) == Some("migrate") {
    return migrate_cli::handle_args(&args[1..]).await;
}
```

`migrate_cli::handle_args` parses one of `up`, `down`, `status`, `--list`,
`--help`, unknown verb (exits 1, prints usage to stderr). The four
verbs map to:

- `up` — connect to `DATABASE_URL`, call `run_orchestrator`, exit 0/1.
- `down` — forward to `domain_posts::migrations_cli::handle_args` with
  `["down"]`.
- `status` — forward to `domain_posts::migrations_cli::handle_args` with
  `["status"]`.
- `--list` — forward to `domain_posts::migrations_cli::handle_args` with
  `["--list"]`.
- `--help` — print usage, exit 0.

**Alternatives considered.**
- (a) **Reimplement the CLI in the gateway with `clap` / `argh`.** *Rejected*:
  the existing `handle_args` covers the verbs; adding a parsing dep is
  unnecessary ceremony.
- (b) **Spawn the gateway as a child process from the `migrate` compose
  service.** *Rejected*: doubles the runtime complexity.
- (c) **Use the Docker `HEALTHCHECK` + `until` polling pattern.** *Rejected*:
  changes operator UX.

**Consequences.** `apps/api/domain_posts/src/main.rs` is deleted.
`domain_posts::migrations_cli::handle_args` retains its `up` and `--list`
arms; new `down` and `status` arms are added in the same function (the
gateway forwards arguments). The gateway inherits the existing error
envelope (`Result<(), String>`), converts to `ExitCode`.

**Contracts.** Public CLI surface (matches parent change's design.md Decision 2):

| Invocation                       | Behaviour                                                  |
|----------------------------------|------------------------------------------------------------|
| `my-cms-api`                     | Boot HTTP server (existing).                               |
| `my-cms-api migrate up`          | Apply pending migrations, exit 0/1.                        |
| `my-cms-api migrate down`        | Revert last migration, exit 0/1.                           |
| `my-cms-api migrate status`      | Print applied/pending state, exit 0.                       |
| `my-cms-api migrate --list`      | Print migration ids to stdout, exit 0.                      |
| `my-cms-api migrate --help`      | Print usage, exit 0.                                       |
| `my-cms-api migrate <unknown>`   | Print usage to stderr, exit 1.                             |

**Migration / rollout.** No database change. Existing operators that
pinned `/app/domain_posts migrate up` in compose overrides must update to
`/app/my-cms-api migrate up`. Slice 3 retargets the docker-compose migrate
service; external overrides need a one-line edit per override.

**Verification.** Module-level tests in `gateway::migrate_cli::tests`.
Testcontainer integration test runs `migrate up` against a fresh PG and
asserts the four migrations are recorded.

### Decision 3: Keep boot-time auto-migrations

**Driver.** The current gateway runs the orchestrator unconditionally at
boot (`apps/api/gateway/src/main.rs:109`). Retiring this behaviour would
be a behavioural break for any operator who relies on the gateway
self-healing. The Docker Swarm compose file already enforces
`my-cms-api` depends_on `migrate` (docker-compose.yaml:120-122), so the
operator path is closed-circuit.

**Decision.** Keep auto-migrations at boot AND expose
`my-cms-api migrate <verb>`. Both paths share the same orchestrator. The
CLI subcommand path exits without binding the listener; the boot path
runs the orchestrator then continues to bind.

**Alternatives considered.**
- (a) **Disable auto-migrations at boot.** *Rejected*: a missed
  `migrate` service (compose override) would crash the gateway on a stale
  schema.
- (b) **Gate auto-migrations behind an env flag (`MIGRATE_AT_BOOT=0`).**
  *Considered but deferred*: adds config surface without a current need.

**Consequences.** Both paths are idempotent (sea-orm-migration tracks
applied migrations). The first migration to apply on a fresh database is
either via the `migrate` compose service (production) or via the boot path
(local dev without compose).

**Verification.** Testcontainer integration test runs `migrate up` and
then boots the gateway against the same database; the second run is a
no-op (no pending migrations) and the gateway starts successfully.

### Decision 4: `migrations_cli` down + status implementation

**Driver.** The coordinator resolved the parent change's open question
#2: ship all four CLI verbs (`up`, `down`, `status`, `--list`). The
current `domain_posts::migrations_cli::handle_args` supports only `up`
and `--list`.

**Decision.** Extend `handle_args` to recognise three new verbs:

```rust
match args.first().map(|s| s.as_str()) {
    Some("--list") | None => { /* existing --list or up behaviour */ }
    Some("down") => {
        let conn = connect_database().await?;
        Migrator::down(&conn, None).await?;
    }
    Some("status") => {
        let conn = connect_database().await?;
        match Migrator::get_migration_status(&conn).await {
            Ok(status) => { /* print applied/pending */ }
            Err(_) => { /* sea-orm-migration 1.1.x does not expose this API; print stub */ }
        }
    }
    _ => { /* unknown verb → error */ }
}
```

The `MigratorTrait::get_migration_status` API was introduced in
sea-orm-migration ≥ 1.1.x. If unavailable in the locked 1.1.20 version,
the `status` arm prints a stable stub:

```
m20240409_151952_release_100 — applied/pending: unknown (status unavailable in sea-orm-migration 1.1.20)
...
```

…and exits 0. The stub is documented in the spec scenario as
"migration status printed, exit 0" regardless of source.

**Alternatives considered.**
- (a) **Defer `down` + `status` to a follow-up change.** *Rejected*:
  coordinator's instruction is to ship all four verbs.
- (b) **Implement `down` + `status` as a query against the
  `sea_orm_migration` tracking table directly.** *Rejected*: the table
  name and column layout are sea-orm-migration internals; the public
  `MigratorTrait` API is the stable surface.

**Consequences.** `migrations_cli::handle_args` grows by ~30 lines. The
stub behaviour for `status` is version-specific and documented in the
module's doc-comment.

**Verification.** Module-level tests cover each verb; an ignored
`#[ignore]` test documents the live behaviour against a testcontainer PG.

### Decision 5: Delete the `domain_posts` standalone binary

**Driver.** After the CLI is moved to the gateway, the `domain_posts`
`[[bin]]` has no remaining consumer.

**Decision.** Delete `apps/api/domain_posts/src/main.rs` (176 lines).
Remove the `[[bin]]` block (lines 13-15) from
`apps/api/domain_posts/Cargo.toml`. Update the `description` (line 5) to
remove "and a standalone bin". The `[lib]` block stays. The crate
compiles as a library only.

**Alternatives considered.**
- (a) **Keep the bin as a shim that forwards to the gateway via
  `std::process::Command`.** *Rejected*: ships a deprecated binary; the
  `purge-legacy-cms-and-application-core` change removed the prior
  `migration` bin for the same reason.

**Consequences.** `cargo build --workspace` produces one binary
(`my-cms-api`). Operators that invoked `cargo run -p domain_posts`
locally now use `cargo run -p gateway`. Documented in the parent
change's `design.md` Decision 7.

**Verification.** `ls apps/api/domain_posts/src/main.rs` returns
"No such file or directory"; `cargo build --release -p domain_posts`
exits 0 (lib only).

### Decision 6: Test strategy

**Driver.** Per AGENTS.md §"Verify Before Commit" the verification gate
is `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy`.
The repo's existing pattern uses PostgreSQL testcontainers + wiremock.

**Decision.** Three test layers:

1. **Module-level tests** (no testcontainer):
   - `domain_interface::lib::tests` — `domain_service_run_migrations_default_is_ok`
     (constructs a stub `DomainService` and asserts the default returns `Ok(())`).
   - `gateway::migrate_cli::tests` — `parse_up`, `parse_down`,
     `parse_status`, `parse_list`, `parse_help`, `parse_unknown_verb_exits_one`,
     `missing_database_url_exits_one`. Stub the database connection by
     injecting a `DatabaseConnection` factory function pointer (test seam).
   - `domain_posts::migrations_cli::tests` — `handle_args_down_forwards_to_migrator`,
     `handle_args_status_forwards_to_migrator`, `handle_args_list_still_works`,
     `handle_args_up_still_works`.
2. **Testcontainer integration test** (uses `test_helpers::setup_test_space`):
   - `gateway::orchestrator::tests` — `runs_post_migrations_and_skips_empty_services`:
     boots a fresh PG, calls `run_orchestrator` against the registered
     services, asserts the four migrations are applied.
   - Idempotency test: runs the orchestrator twice and asserts the
     second run is a no-op.
   - `gateway::migrate_cli::tests::ignored` — `migrate_up_against_testcontainer_applies_four_migrations`
     (ignored by default; runs with `--ignored` against a local Docker
     daemon).
3. **Wiremock external-contract tests** (already present):
   - `domain_user` GoTrue contract.
   - `domain_media` Supabase Storage contract.

**Alternatives considered.**
- (a) **Live-service tests.** *Rejected*: per AGENTS.md, deterministic
  seams preferred over live Supabase / OpenAI.
- (b) **Skip the orchestrator test, rely on manual smoke.** *Rejected*:
  the orchestrator's topological sort and dispatch change is the most
  material behaviour.

**Verification.** All three layers run in `cargo test --workspace`. The
gateway-level tests use `apps/api/test_helpers` directly (no live
services). The ignored testcontainer test runs in CI when
`TESTCONTAINERS_RUNTIME` is set.

## Risks / Trade-offs

**[Risk]** Removing the `domain_posts` `[[bin]]` breaks any local
developer workflow that uses `cargo run -p domain_posts` to boot a
standalone post-domain service. → **Mitigation:** the replacement is
`cargo run -p gateway`, which composes every registered domain —
strictly more functionality. Documented in the parent change's
`proposal.md` Impact.

**[Risk]** The `sea-orm-migration` 1.1.20 API may not expose
`MigratorTrait::get_migration_status`. → **Mitigation:** Decision 4
specifies a stable stub fallback; the `status` arm exits 0 in both the
real and stub cases. Documented in the spec scenario.

**[Risk]** The orchestrator's topological sort does not currently honour
`MigrationDescriptor::depends_on` — it sorts by `id` and dedupes by `id`.
With zero current descriptors that declare `depends_on`, the change is
behaviour-preserving today. → **Mitigation:** the spec scenario
"Descriptor dependency order is respected" makes the contract explicit
for future domains. The orchestrator's topological sort is a small
follow-up if a future domain adds a dependent migration.

**[Risk]** The active `purge-legacy-cms-and-application-core` change
(41/47 tasks) is mid-flight. If that change lands first with a different
operator-CLI surface than expected, this change's CLI dispatch may need
a rebase. → **Mitigation:** the two changes touch non-overlapping Rust
files. Coordinate via the software-engineer review step before merging.

**[Risk]** Keeping auto-migrations at boot duplicates the `migrate`
compose service. In a hot-restart scenario (gateway restarted without
re-running `migrate`) the boot path is the safety net. → **Mitigation:**
both paths are idempotent; the additional cost is one `SELECT` against
the migration tracking table per boot.

## Migration Plan

### Code
1. Add `DomainService::run_migrations` default impl
   (`apps/api/domain_interface/src/lib.rs`). Verify
   `cargo check -p domain_interface`. **Independent.**
2. Override `run_migrations` on `DomainPostService`
   (`apps/api/domain_posts/src/service.rs`). Verify
   `cargo check -p domain_posts`. **Depends on 1.**
3. Extend `domain_posts::migrations_cli::handle_args` with `down` and
   `status` arms. Verify `cargo test -p domain_posts --lib migrations_cli`.
   **Depends on 1.**
4. Refactor `gateway::run_orchestrator` to use the new trait method.
   Remove the hard-coded `m2024` / `m2026` arm. Verify
   `cargo test -p gateway --lib main`. **Depends on 2.**
5. Add `gateway::migrate_cli` module + the `migrate` subcommand dispatch
   at the top of `gateway::main`. Verify
   `cargo run -p gateway -- migrate --help` and
   `cargo run -p gateway -- migrate --list`. **Depends on 3, 4.**
6. Delete `apps/api/domain_posts/src/main.rs` and the `[[bin]]` block.
   Verify `cargo build --release -p domain_posts` succeeds (lib only).
   **Depends on 5.**
7. Run the full verification gate (task 7).

### Deployment
None. This slice does not change the container image or any operator-facing
command beyond the `migrate` verb surface. Slice 3 retargets the docker
image and the docker-compose migrate service.

### Rollback
Single-commit rollback: revert the merge commit.
`gateway::run_orchestrator` reverts to the hard-coded dispatch (with
the original silent-warn behaviour). `domain_posts::[[bin]]` is
restored. The CLI subcommand surface disappears. No data loss; the
four migration identities are unchanged.

### Order of operations (suggested commit chain)
1. Add `DomainService::run_migrations` default + stub test.
2. Override on `DomainPostService`.
3. Extend `migrations_cli::handle_args` with `down` + `status`.
4. Refactor `run_orchestrator` + remove hard-coded arm.
5. Add `migrate_cli` module + subcommand dispatch.
6. Delete `domain_posts` `[[bin]]` + `src/main.rs`.
7. Run the full verification gate.

## Open Questions

1. **Should the orchestrator's topological sort honour `depends_on`
   now, or wait for a domain that actually declares a dependency?**
   *Current default:* keep `sort_by_key(|d| d.id)` and `dedup_by_key(|d| d.id)`.
   A topological sort is a small follow-up if a future domain adds a
   dependent migration. **No action in this slice.**

2. **Should `MIGRATE_AT_BOOT=0` become a configurable flag?** *Current
   default:* auto-migrate at boot unconditionally. The flag is a trivial
   follow-up if a hot-restart scenario emerges. **No action.**

3. **Should `domain_auth`'s placeholder bin
   (`apps/api/domain_auth/src/main.rs` prints "not implemented yet") be
   removed at the same time?** *Current default:* leave for a separate
   change; out of scope here. **No action.**
