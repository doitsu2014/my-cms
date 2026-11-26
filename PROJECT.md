# Project

Standing knowledge for this repository. Humans edit this file. The tool never overwrites it.

Last reviewed: 2026-11-22. Facts below were checked against the repository and the 2026-11-22 baseline run. Source code wins when it disagrees with this file or with `docs/`.

## Purpose and users

My-CMS is a self-hosted, API-first publishing platform (MIT OR Apache-2.0).

- **Writers and administrators** use the React admin console (`apps/web`) to write posts, manage categories, media, users and SEO head assets.
- **Readers** use the public website (`apps/ducth-dev-website`), which reads the public GraphQL API.
- **API clients** call the REST and GraphQL endpoints of the `my-cms-api` gateway.
- **The owner/operator** runs everything on a self-hosted Supabase stack (PostgreSQL + pgvector, GoTrue, Storage) behind Traefik.

Work in progress: backup and restore (intent `use-dtech-aidlc-to-develop-the-backup-and-restor`). Only the U1 slice is approved: localhost, synthetic fixtures, administrator-only, database slice, no restore.

## Domain glossary

- **Gateway**: `apps/api/gateway`, the `my-cms-api` binary. It is the composition root only: it builds shared state, mounts domain routes, applies auth layers once and runs domain migrations. Feature logic does not live here.
- **Domain / DomainService**: a crate under `apps/api/domain_*` that owns its routes, state, entities, migrations and health behaviour, and implements the `domain_interface` contract.
- **Route groups (`Mount`)**: `Public` (anyone), `Protected` (writer or administrator JWT), `Administrator` (administrator JWT only).
- **Roles**: Supabase app roles in the JWT `app_metadata.roles`: `my-headless-cms-writer` and `my-headless-cms-administrator`.
- **editor-prose**: `packages/editor-prose`, the shared rendered-article contract used by both the admin and the reader.
- **Intent**: one dtech-aidlc unit of work, recorded under `.aidlc/intents/<id>/`.
- **Work unit (WU-nn)**: one red-tests then build-and-verify loop inside Develop, listed in the intent's `work-units.md` and `work-units.json`.
- **Backup job statuses**: `queued`, `running`, `succeeded`, `incomplete`, `failed`, `recovery-required`. U1 can only reach `incomplete` or `failed`.

## Architecture overview

- **API** (`apps/api`, Rust Cargo workspace; Axum 0.8, Tokio, SeaORM 1.1, async-graphql 7):
  - `gateway`: composition root, `migrate` and `backup` CLI subcommands.
  - `domain_interface`: `DomainService`, `RouteRegistration { mount, router, prefix }`, `Mount`.
  - `domain_posts`: posts, categories, tags, GraphQL, AI translation (OpenAI), canonical migrations.
  - `domain_auth`: Supabase JWT validation and role gates.
  - `domain_media`: media and bucket APIs over Supabase Storage.
  - `domain_user`: user administration over the Supabase Admin API.
  - `domain_seo`: SEO head assets.
  - `domain_backup`: new (backup/restore WU-01). Config, status enum and guards only. Not yet composed into the gateway; that happens in WU-10.
  - `test_helpers`: testcontainers PostgreSQL helpers.
- **Admin** (`apps/web`): React 19, TypeScript, Rsbuild, DaisyUI 5, Tailwind CSS 4, TipTap, Apollo Client, Vitest. Dev server on `localhost:3002`.
- **Reader** (`apps/ducth-dev-website`): React 19, Rsbuild, React Router, i18next, Express production server. `localhost:3001`.
- **Platform** (`deployments/docker-swarm`): three Compose stacks (Supabase, apps, Traefik) on the external `supabase_network`. API on `localhost:8989`, Jaeger `16686`, Traefik dashboard `8080`. Helm charts in `deployments/k8s`.
- **Data ownership**: one shared PostgreSQL connection does not mean shared tables. Each domain owns its schema and migrations; cross-domain work needs an explicit design first.

Details: `docs/architecture/software-architecture.md` and `.aidlc/knowledge/_codebase-map.md`.

## Conventions

- **New capability**:
  - Add it as a new domain crate with a `DomainService` and register it in the gateway manifest.
  - Never import a domain's handlers or entities into the gateway.
- **Migrations**:
  - Each domain owns its own and runs them through the gateway (`my-cms-api migrate status|up`).
  - Posts owns sea-orm's default `seaql_migrations` table. Every other domain must override `migration_table_name()` with its own table (`seaql_migrations_seo`, `seaql_migrations_backup`).
  - Sharing the default table breaks startup: `Migrator::up` refuses to run when the table holds versions it does not know. This was RK-15, confirmed and fixed for SEO on 2026-11-22 (gateway tests `migration_tests::*`).
- **Tests**:
  - Rust tests live inline in `#[cfg(test)] mod tests`.
  - DB-backed tests use testcontainers through `test_helpers`, so they need Docker.
  - In new crates, mark DB tests `#[ignore = "requires Docker for PostgreSQL testcontainer"]` (the `domain_seo` precedent) and run them with `-- --ignored`.
  - `domain_posts` handler tests are not ignored, so they fail without Docker.
  - Config tests inject a key lookup; do not mutate process environment in tests.
- **Delivery**:
  - Behavioural changes go RED, then GREEN, then REFACTOR. Record the failing run before writing production code.
  - Run `cargo fmt` on new tests before taking the red snapshot.
  - Never change a test to fit code written first.
  - One writer per shared file or intent artifact.
  - Handoffs state goal, constraints, done-when, evidence, decisions, risks and the next owner.
- **Fast fix** (no intent needed):
  - One non-generated file, at most 40 lines.
  - No behaviour, auth, API, schema, dependency, deployment, secret or generated change.
  - Must include a focused check.
- **Workflow tool**: structured work uses the dtech-aidlc CLI (`/dtech-aidlc` in Claude Code). The CLI owns `.aidlc/state.json`, intent `state.json` and `audit.jsonl`; never edit them by hand.
- **Artifacts**: when asked to create an artifact for this project, use an Orca artifact. Do not publish it through Claude's Artifact tool.
- **Error responses**: existing domains return a camelCase envelope (`domain_posts/src/domain/response.rs`). The backup domain's bare snake_case body is a recorded deviation (D-11).
- **Docs**: feature docs stay close to user-visible behaviour and name source paths. Change records go in a dtech-aidlc intent, not a separate spec tree.

## Constraints and non-negotiables

- **Pushing to `main` releases images.** `release-my-cms-image.yml` (`apps/api/**`), `release-my-cms-admin-image.yml` (`apps/web/**`, `packages/editor-prose/**`) and `release-my-cms-ducth-dev-website-image.yml` build and publish on push to `main`. Merging is a release decision.
- **Backup/restore U1 stays on its feature branch until the Launch phase** (owner decision, 2026-11-22). Do not merge it to `main` during Develop.
- **Backup/restore U1 must not:**
  - enable real-data staging, upload or preflight acceptance, restore execution, downloads or production use;
  - let any job reach `succeeded` or `recovery-required`;
  - change the owner-approved bounds: 24 h expiry, 1 h grace, 15 min sweep, 3 retries with 1 min backoff, 10 min deadline, 256 MiB / 1 GiB, 1 job, 30 days retention, owner escalation.
- **Backup endpoints** are restricted to `my-headless-cms-administrator` and must bind to loopback only.
- **Environment-changing commands** (deployments, runtime configuration, reset scripts) need explicit owner authorization. Their changes include rollout, a rollback trigger and post-deploy checks.
- **`apps/api/take-coverage.sh`** runs `rustup override set stable` and `cargo clean`. Do not run it as part of a check.
- **Secrets** live only in local `.env` files copied from `.env.example`. Never put secrets, tokens or database contents into code, logs, audit records or artifacts.

## Commands

Run backend commands from `apps/api` and frontend commands from the app directory.

| Check | Command |
|---|---|
| Rust tests | `cargo test --no-fail-fast` |
| Rust DB tests (Docker) | `cargo test --no-fail-fast -- --ignored` |
| One crate | `cargo test -p <crate> --no-fail-fast` (add `-- --ignored` for its DB tests) |
| Format | `cargo fmt -- --check` |
| Lint | `cargo clippy --all-targets` |
| Workspace build | `cargo check --workspace` |
| Migrations | `cargo run -p gateway --bin my-cms-api -- migrate up` / `migrate status` |
| Admin tests | `pnpm test` (in `apps/web`) |
| Admin build | `pnpm build` (in `apps/web`) |
| Admin lint | `pnpm lint` (in `apps/web`) |
| Admin typecheck | `pnpm exec tsc --noEmit` (in `apps/web`) |
| Reader | `pnpm test` / `pnpm build` (in `apps/ducth-dev-website`) |

## Environment prerequisites

- **Rust toolchain**: cargo/rustc 1.100.0 on the dev host. CI uses nightly on Ubuntu.
- **Node and pnpm**: node v24.18.0 and pnpm 10.30.3 on the dev host. Run `pnpm install --frozen-lockfile` in `apps/web` when `node_modules` is missing.
- **Docker** is required for full verification: the testcontainers tests, migration checks and the local stack. It is declared as the dtech-aidlc prerequisite `docker` (`docker info`) for red-tests and build-and-verify. When Docker is unavailable those checks are reported "not run", never passed.
- **Testcontainers gotchas**:
  - `test_helpers::setup_test_space_with_pgvector` can report ready during PostgreSQL's init restart, so connect with a short retry.
  - `ContainerAsyncPostgresEx::get_database_connection()` also applies the posts migrations as a side effect, so tests that need an empty database must open their own connection.
  - Pin test databases to production's major version: PostgreSQL 15 (`supabase/postgres:15.x`), not the testcontainers default 11.
- **Windows dev host**:
  - Tests that create links use junctions (`mklink /J`).
  - Unix-only checks such as directory mode 0700 compile out on Windows and need CI or a Linux host.

## Known baseline failures

Recorded 2026-11-22 on the Windows dev host with Docker running. The latest full results are in the backup intent's `develop/baseline.md`.

- **Rust**: none. `cargo test --no-fail-fast` gives 290 passed, 0 failed, 39 ignored, and all 39 ignored tests pass with `-- --ignored`. Without Docker the 8 `domain_posts` handler tests fail (testcontainers).
- **Clippy**: exit 0, with existing warnings in other crates (for example `domain_auth` type complexity, `domain_media` Debug impls).
- **Admin lint** (`pnpm lint`): fails before any change, with an ESLint flat-config error for the `react-hooks` plugin.
- **Admin typecheck** (`pnpm exec tsc --noEmit`): fails before any change, with 23 errors in the rich-text editor and the SEO head-asset form.
- **Admin tests and build**: 49/49 tests pass; build passes.
- **Reader**: not baselined yet.

## Testing and release

- **Regression rule**: a test that passed at baseline must still pass. The failing set may not grow. New lint or typecheck errors are not allowed in touched files.
- **CI** (`ci-my-cms.yml`) runs on pull requests to `main`:
  - It runs `cargo test --all --all-features --no-fail-fast` on Ubuntu with nightly Rust and grcov coverage (Codecov/Coveralls).
  - It does not pass `--ignored`, so the DB-backed tests do not run in CI.
  - It does not run the web apps' tests, lint or build.
- **Release**: merging to `main` builds and publishes the API, admin and reader images (path-filtered). `publish-github-release-images.yml` runs on a published GitHub release.

## Deeper docs

Registered in `aidlc.knowledge.json` (list them with `dtech-aidlc knowledge sources`):

- `README.md`: overview, local run, verify commands.
- `docs/architecture/software-architecture.md`: composition, route and authorization model, extension rules.
- `docs/development/agent-team.md`: delivery rules (RED-GREEN, one writer, fast fix).
- `docs/features/backup-restore.md`, `docs/features/ai-translation.md`, `docs/features/editor-prose-contract.md`: feature requirements and contracts.
- `docs/postman_collection/`: REST and GraphQL request examples.
- `deployments/docker-swarm/README.md`: local stack runbook.
- `.aidlc/knowledge/_codebase-map.md`: generated codebase map.

**Known documentation drift (Observed 2026-11-22).** These docs still describe the earlier AWS AI-DLC setup:

- `docs/development/agent-team.md` calls AWS AI-DLC "the only harness".
- `README.md`, `docs/README.md` and the architecture doc point to `AGENTS.md` and `aidlc/spaces/default/...`.
- None of those paths exist in this checkout. The current workflow is dtech-aidlc with records under `.aidlc/`.

Treat those references as stale until the docs are updated.
