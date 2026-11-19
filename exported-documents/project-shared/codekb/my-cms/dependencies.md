# Dependencies

## Internal Component Dependencies

- `gateway` composes `domain_interface` and the domain crates: `domain_posts`, `domain_auth`, `domain_media`, `domain_user`, and `domain_seo`.
- Domain crates depend on the shared `domain_interface` contract.
- `domain_posts` owns its persistence entities and migrations; the gateway dispatches shared application startup/migration pathways.
- `test_helpers` provides backend test support.
- `apps/web` and `apps/ducth-dev-website` consume the local `packages/editor-prose` package.

This graph is manifest/workspace-level and representative; the scan did not extract every source-file import edge or check for all cycles.

## External Dependency Groups

| Group | Examples / evidence | Purpose |
|---|---|---|
| Rust web/runtime | Axum, Tokio | HTTP service and async runtime |
| Persistence | SeaORM and PostgreSQL ecosystem | Relational data access |
| Frontend runtime | React, React Router, Apollo Client | Admin and reader UI/API access |
| Public server | Express, OpenTelemetry | Server-side rendering/support and telemetry |
| Admin integrations | Supabase JS, TipTap | Supabase interaction and rich-text authoring |
| Platform services | Supabase PostgreSQL, Storage, GoTrue | Data, object storage, identity |
| Translation | OpenAI | Translation and embedding integrations |
| Tooling | pnpm, Rsbuild, Vite/Vitest, ESLint, Prettier | Workspace build, tests, lint, formatting |

Exact versions and transitive dependency relationships are recorded in Cargo and pnpm lockfiles, not reproduced exhaustively here.

## Backup/Restore Dependency Risks
The target feature depends on supported export/import mechanisms for PostgreSQL/Supabase-managed schemas, GoTrue/Auth, and Storage object bytes/metadata. Package format, checksums, compatibility rules, private temporary storage, approved restore tooling, and recovery-point capabilities must be validated against supported platform versions. Uploaded package content must not supply executable restore commands.

## Limitations
No complete file-level adjacency list, vulnerability audit, or outdated-version determination was produced. No tests or build tools were run.
