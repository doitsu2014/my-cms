# Codebase map

- Status: draft
- Mapped on: 2026-11-21
- Mapped at commit: `b588ed469b2cbcd9ba8f3dfc39d6afc4cdcfe755`
- Mapped by: AI-DLC analysis (generated draft; owner review required)
- Sources consulted: `README.md`, `docs` (`docs/features/backup-restore.md`, `docs/architecture/software-architecture.md`, `docs/development/agent-team.md`), `PROJECT.md`, `apps/api/Cargo.toml`, `apps/api/gateway/Cargo.toml`, `apps/api/gateway/src/main.rs`, `apps/api/gateway/src/backup_cli.rs`, `apps/web/package.json`

## 1. System summary

- **Observed** (`README.md`, `docs/architecture/software-architecture.md`): My-CMS is a self-hosted publishing platform with a Rust API gateway, admin UI, public reader, PostgreSQL/Supabase Auth, and Supabase Storage.
- **Observed** (`apps/api/gateway/src/main.rs`): Gateway composes domain services; backup CLI is dispatched before starting the HTTP listener.

## 2. Components

| Name | Path | Responsibility | Depends on |
|---|---|---|---|
| API workspace | `apps/api/` | Rust workspace for gateway and domain crates | Rust/Cargo, PostgreSQL/Supabase |
| Gateway | `apps/api/gateway/` | Axum composition root, migration CLI, backup CLI | Domain crates, shared database/config |
| Posts domain | `apps/api/domain_posts/` | CMS content and GraphQL | PostgreSQL |
| Auth domain | `apps/api/domain_auth/` | JWT verification | Supabase Auth |
| Media domain | `apps/api/domain_media/` | Media and bucket operations | Supabase Storage |
| User domain | `apps/api/domain_user/` | User administration | Supabase GoTrue |
| Admin web | `apps/web/` | React/TypeScript administrative UI | API, Supabase auth |
| Reader website | `apps/ducth-dev-website/` | Public content reader | API GraphQL |

## 3. Entry points

- **Observed** (`apps/api/gateway/src/main.rs`): `my-cms-api` starts HTTP serving by default; `migrate` and `backup` subcommands are dispatched separately.
- **Observed** (`apps/api/gateway/src/backup_cli.rs`): backup CLI exposes create/list/restore argument handling. Create/list currently fail because the S3 transfer adapter is not configured; restore fails closed because full Supabase recovery is unverified.
- **Observed** (`apps/web/package.json`): admin application scripts include build, lint, and Vitest test commands.

## 4. External dependencies and integrations

- **Observed** (`README.md`, `docs/architecture/software-architecture.md`): PostgreSQL/Supabase, GoTrue, Supabase Storage, Docker Compose, and OpenAI integration for translation.
- **Observed** (`apps/api/gateway/Cargo.toml`): gateway uses Axum, Tokio, SeaORM, serde, and sha2 among its dependencies. No S3 SDK dependency is declared in this manifest.

## 5. Data

- **Observed** (`README.md`, `docs/architecture/software-architecture.md`): CMS relational data is in PostgreSQL; media bytes and bucket policy are in Supabase Storage; user identity is managed by GoTrue.
- **Observed** (`apps/api/gateway/src/backup_cli.rs`): backup manifest model refers to optional database archive and object entries, but this does not establish a complete export/import implementation.

## 6. Conventions

- **Observed** (`docs/architecture/software-architecture.md`): feature logic and data ownership belong in domains; the gateway is a composition root. Domain migrations/entities remain domain-owned.
- **Observed** (`docs/development/agent-team.md`): behavioral changes use RED–GREEN–REFACTOR; operational changes include rollout/rollback/post-deploy checks; one writer per shared artifact; test-after is required.
- **Observed** (`PROJECT.md`): project file is a human-owned placeholder; no additional project-specific conventions are recorded there.

## 7. Tests

- **Observed** (`apps/api/Cargo.toml`, `apps/api/gateway/Cargo.toml`): Rust Cargo workspace; gateway has inline unit tests in `backup_cli.rs`.
- **Observed** (`apps/web/package.json`): admin tests use Vitest; exact whole-suite command is `pnpm test` from `apps/web`.
- **Attempted:** `cargo test -p gateway` from the project root; could not run because `cargo` is not recognized in this environment. No duration or baseline test result available. Frontend tests were not run because they are not needed to establish this map and no feature implementation has begun.
- **Gap:** Rust toolchain availability and current suite baseline remain unverified.

## 8. Build and run

- **Observed** (`README.md`): local environment is organized as Docker Compose stacks under `deployments/docker-swarm`; API development commands run from `apps/api`.
- **Observed** (`apps/web/package.json`): web build script is `pnpm build` from `apps/web`.
- **Gap:** Build commands were not run; runtime configuration and required local secrets were not inspected.

## 9. Known risks and technical debt

- **Observed** (`docs/features/backup-restore.md` §§9, 12): backup/restore is incomplete; previous U1 stopped before app source changes due unresolved persistence/cleanup, migration ownership, and job-state contract issues.
- **Observed** (`apps/api/gateway/src/backup_cli.rs`): restore deliberately refuses to run; backup create/list fail without an S3 adapter.
- **Observed** (`docs/features/backup-restore.md` §§7–8, 10): supported Supabase/Auth/Storage recovery scope, consistency, security/retention limits, rollback, and RPO/RTO require decisions and verification.

## 10. Gaps

- Full source-level walkthrough of every domain and frontend route was not performed; this is a task-focused map for backup/restore.
- Registered architecture images were not consulted because their textual architecture counterpart was available and adequate for this map.
- The `docs` registered source was consulted through the directly relevant feature, architecture, and development documents; unrelated docs were not read.
- Rust test command was blocked by missing `cargo`; actual suite baseline remains unknown.
- No database, secrets, deployment environment, or live Supabase instance was inspected.
