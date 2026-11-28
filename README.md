[![API CI](https://github.com/doitsu2014/my-cms/actions/workflows/ci-my-cms.yml/badge.svg)](https://github.com/doitsu2014/my-cms/actions/workflows/ci-my-cms.yml)
[![codecov](https://codecov.io/gh/doitsu2014/my-cms/branch/main/graph/badge.svg?token=7V6BYO0TJO)](https://codecov.io/gh/doitsu2014/my-cms)
[![Coverage Status](https://coveralls.io/repos/github/doitsu2014/my-cms/badge.svg?branch=main)](https://coveralls.io/github/doitsu2014/my-cms?branch=main)

![My-CMS architecture overview](docs/assets/my-cms-repository-thumbnail.jpg)

# My-CMS

My-CMS is a self-hosted, API-first publishing platform. It provides a Rust gateway API, a React administrative console, a public reader website, and a self-hosted Supabase platform for PostgreSQL, authentication, and object storage.

The current backend is a **pluggable domain-service workspace**. One deployable `my-cms-api` binary composes the posts, authentication, media, and user domains behind consistent HTTP, authorization, migration, and tracing boundaries.

## What is included

- **Content API** — REST endpoints for posts, categories, media, and users, plus public immutable and authenticated mutable GraphQL endpoints.
- **Editorial workflow** — a React 19 admin interface with DaisyUI, Tailwind CSS, TipTap, Apollo Client, Supabase authentication, and shared `editor-prose` rendering.
- **Public reader** — the `ducth-dev-website` React application reads the public GraphQL API and renders the same prose package used by the admin.
- **Domain-owned backend** — posts own content, categories, tags, translations, GraphQL contribution, and canonical migrations; media owns Supabase Storage access; users own Supabase Admin operations; auth owns JWT verification.
- **AI translation** — HTML-aware post translation uses OpenAI and can reuse cached or vector-similar results. See [AI translation](docs/features/ai-translation.md).
- **Self-hosted platform** — Docker Compose runs Supabase (PostgreSQL + pgvector, GoTrue, Storage, Kong, Studio, and supporting services), Traefik, Jaeger, the API, and both web apps.

## Architecture

```mermaid
flowchart TB
    Admin["Admin web app\nReact + Rsbuild"]
    Reader["Public reader\nducth-dev-website"]
    Client["API clients"]
    Proxy["Traefik\nlocal routing and edge concerns"]

    Admin --> Proxy
    Reader --> Proxy
    Client --> Proxy
    Proxy --> Gateway

    subgraph API["my-cms-api gateway binary (Axum)"]
        Gateway["Gateway composition root\nhealth endpoints • startup checks\nshared middleware • migration orchestration"]
        Contract["domain_interface\nDomainService contract\nDomainContext • route and migration descriptors"]
        Posts["domain_posts\nREST • GraphQL • categories • translation\nSeaORM entities and migrations"]
        Auth["domain_auth\nSupabase JWT validation\nwriter/admin authorization"]
        Media["domain_media\nmedia and bucket APIs\nSupabase Storage adapter"]
        Users["domain_user\nuser administration\nSupabase Admin adapter"]

        Gateway --> Contract
        Contract --> Posts
        Contract --> Auth
        Contract --> Media
        Contract --> Users
    end

    subgraph Supabase["Self-hosted Supabase platform"]
        Database[("PostgreSQL + pgvector")]
        GoTrue["GoTrue\nusers and JWTs"]
        Storage["Storage\nS3-compatible object API"]
    end

    Posts --> Database
    Posts --> OpenAI["OpenAI\ntranslation and embeddings"]
    Auth --> GoTrue
    Users --> GoTrue
    Media --> Storage
    Jaeger["Jaeger\nlocal observability service"]
```

### How requests are composed

At startup, the gateway creates one database connection and builds immutable and mutable GraphQL schemas. It asks each registered `DomainService` for route registrations, then merges them into three route groups:

| Route group | Who can use it | Examples |
| --- | --- | --- |
| Public | Anyone | `/health`, public media, `/posts/graphql/immutable` |
| Protected | CMS writers and administrators with a valid Supabase JWT | post and category CRUD, translation, `/posts/graphql/mutable`, media operations |
| Administrator | CMS administrators with a valid Supabase JWT | user management and media bucket management |

The gateway applies cross-cutting authentication and cookie handling once, rather than embedding them in every domain router. It also collects domain migration descriptors and runs their migrations against the shared connection. The operator CLI is available as `my-cms-api migrate up`, `down`, `status`, or `--list`.

For the detailed source map and API route topology, see [Software architecture](docs/architecture/software-architecture.md).

## Repository layout

```text
my-cms/
├── apps/
│   ├── api/                       Rust Cargo workspace
│   │   ├── gateway/               `my-cms-api` composition root and migration CLI
│   │   ├── domain_interface/      Stable contract for domain services
│   │   ├── domain_posts/          Content, GraphQL, AI translation, migrations
│   │   ├── domain_auth/           Supabase JWT authentication
│   │   ├── domain_media/          Media and bucket service
│   │   └── domain_user/           User administration service
│   ├── web/                       React administrative application
│   └── ducth-dev-website/         Public reader website
├── packages/editor-prose/         Shared rich-text presentation package
├── deployments/
│   ├── docker-swarm/              Local Supabase, apps, and Traefik Compose stacks
│   └── k8s/                       Helm charts
├── docs/                          Architecture and feature documentation
└── aidlc/                         AI-DLC method, intents, and project knowledge
```

## Technology

| Area | Current implementation |
| --- | --- |
| API | Rust, Axum 0.8, Tokio, SeaORM 1.1, async-graphql 7 |
| Backend structure | Gateway composition root plus `DomainService` implementations |
| Data | PostgreSQL + pgvector through self-hosted Supabase |
| Authentication | Supabase GoTrue JWTs, with writer and administrator role gates |
| Media | Supabase Storage API and image delivery endpoints |
| Admin | React 19, TypeScript, Rsbuild, DaisyUI 5, Tailwind CSS 4, TipTap, Apollo Client |
| Public web | React 19, Rsbuild, React Router, i18next, Express production server |
| Operations | Docker Compose, Traefik, Jaeger, GitHub Actions |

## Run locally

The local environment uses three independent Compose stacks connected through the external `supabase_network`: Supabase, application services, and Traefik. The reset scripts are intended for local development; a full reset removes the relevant local volumes.

```bash
cd deployments/docker-swarm

# One time: create the shared Docker network.
./bootstrap.sh

# Create local configuration files and set real development secrets.
cp supabase/.env.example supabase/.env
cp apps/.env.example apps/.env
cp traefik/.env.example traefik/.env

# Start Supabase and the reverse proxy, then the API, admin, reader, and Jaeger.
./supabase/reset.sh
./apps/reset.sh
```

The shared `POSTGRES_PASSWORD`, `JWT_SECRET`, `ANON_KEY`, and `SERVICE_ROLE_KEY` values in `supabase/.env` and `apps/.env` must match. See the [local Docker stack guide](deployments/docker-swarm/README.md) for prerequisites, restart-only commands, direct port exposure, and routing details.

| Local address | Service |
| --- | --- |
| `http://localhost:8989` | My-CMS API |
| `http://localhost:3002` | Admin web app |
| `http://localhost:3001` | Public reader website |
| `http://localhost:16686` | Jaeger UI |
| `http://localhost:8080` | Traefik dashboard |

## Develop and verify

Backend commands run from `apps/api`; frontend commands run from their application directory.

```bash
# API workspace
cd apps/api
cargo check
cargo test
cargo fmt -- --check
cargo clippy

# Inspect or apply domain-owned migrations through the gateway binary
cargo run -p gateway --bin my-cms-api -- migrate status
cargo run -p gateway --bin my-cms-api -- migrate up

# Admin frontend
pnpm --dir ../web test
pnpm --dir ../web build

# Public reader
pnpm --dir ../ducth-dev-website test
pnpm --dir ../ducth-dev-website build
```

## Backup and restore (U1) configuration

The backup slice is administrator-only, localhost-only and uses synthetic fixtures. It is off unless `BACKUP_ENABLED=true`, and it forces itself off (`NON_LOOPBACK_BIND`) when the gateway binds a non-loopback `HOST`; the Compose stack sets `HOST=0.0.0.0`, so it stays disabled there. A missing or invalid key never stops the CMS: backup is disabled with `INVALID_CONFIG` and `GET /backup/status` reports the reason. The `/backup/**` routes sit behind the administrator auth layer, a request timeout and a header-size limit (the two required values below). Bearer tokens are the only accepted credential.

Every key is read by `apps/api/domain_backup/src/domain/config.rs`. Keys marked fixed accept only the owner-approved value: any other value disables backup with `INVALID_CONFIG`.

| Key | Default or required value | Notes |
| --- | --- | --- |
| `BACKUP_ENABLED` | default: unset (disabled, `NOT_CONFIGURED`) | Must be exactly `true` to enable. |
| `BACKUP_INSTANCE_ID` | required when enabled | Stable id of the configured instance; no default. |
| `BACKUP_INSTANCE_LABEL` | required when enabled | Display label returned by the API; no default. |
| `BACKUP_STORAGE_ROOT` | required when enabled | Absolute, link-free, private (mode 0700) directory for synthetic fixtures. Not the legacy `BACKUP_LOCAL_ROOT`. |
| `BACKUP_ALLOWED_ORIGINS` | required when enabled | Comma-separated exact origins, for example `http://localhost:3002`; no wildcard. |
| `BACKUP_REQUEST_TIMEOUT_SECS` | required when enabled; owner-approved value `30` | Ingress timeout (CFG-17). The API answers 408 `REQUEST_TIMEOUT` past it. No built-in default: the key must be set, to `30` unless the owner approves another value. |
| `BACKUP_MAX_HEADER_BYTES` | required when enabled; owner-approved value `16384` | Header-size limit (CFG-18), name plus value bytes of all headers. The API answers 431 `HEADER_TOO_LARGE` over it, before authentication. No built-in default: set it to `16384` unless the owner approves another value. |
| `BACKUP_STAGING_EXPIRY_SECS` | default `86400` (fixed) | Staging expiry, 24 h. |
| `BACKUP_ORPHAN_GRACE_SECS` | default `3600` (fixed) | Orphan grace, 1 h. |
| `BACKUP_SWEEP_INTERVAL_SECS` | default `900` (fixed) | Sweep period, 15 min. |
| `BACKUP_CLEANUP_MAX_ATTEMPTS` | default `3` (fixed) | Cleanup retries. |
| `BACKUP_CLEANUP_BACKOFF_SECS` | default `60` (fixed) | Cleanup backoff, 1 min. |
| `BACKUP_JOB_DEADLINE_SECS` | default `600` (fixed) | Job deadline, 10 min. |
| `BACKUP_MAX_CONCURRENT_JOBS` | default `1` (fixed) | One queued or running job. |
| `BACKUP_ALLOC_MAX_BYTES` | default `268435456` (fixed) | 256 MiB per allocation. |
| `BACKUP_TOTAL_MAX_BYTES` | default `1073741824` (fixed) | 1 GiB across allocations. |
| `BACKUP_RETENTION_DAYS` | default `30` (fixed) | Audit and job retention. |
| `BACKUP_ESCALATION_OWNER` | default `project_owner` (fixed) | Escalation owner shown in status. |

The operator CLI `my-cms-api backup` is a separate, fail-closed surface with its own keys, `BACKUP_S3_ENDPOINT` and `BACKUP_LOCAL_ROOT`. They are unrelated to the keys above.

Backup tables use their own migration table, `seaql_migrations_backup`; the backup migrations run on every `migrate up` and gateway start, whether or not backup is enabled. `my-cms-api migrate down` and `migrate status` cover the posts migrator only; `migrate --list` prints the posts, SEO and backup identities in that order.

## Documentation and change process

- [Documentation index](docs/README.md) — entry point for current project documentation.
- [Software architecture](docs/architecture/software-architecture.md) — workspace, gateway, and domain ownership details.
- [AI translation](docs/features/ai-translation.md) — translation behavior, reuse strategy, and jobs.
- [Editor prose contract](docs/features/editor-prose-contract.md) — shared TipTap reader rendering across the admin and public website.
- [Agent workflow](docs/development/agent-team.md) — Claude Code and AI-DLC delivery rules.
- [Project knowledge](aidlc/spaces/default/knowledge/aidlc-shared/) — current requirements migrated from the removed spec tree.
- [Postman collection](docs/postman_collection/) — REST and GraphQL request examples.
- [Local Docker stack](deployments/docker-swarm/README.md) — Compose setup and operational commands.
- [Changelog](CHANGELOG.md) — released changes.

## License

Released under the terms of the [LICENSE](LICENSE) file (MIT OR Apache-2.0).
