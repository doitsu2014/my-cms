# Technology Stack

## Languages and Build Systems

| Area | Technology | Version evidence / notes |
|---|---|---|
| API/backend | Rust | Workspace manifests and lockfile declare versions; exact compiler version not recorded in this scan. |
| Backend build | Cargo workspace | `apps/api/Cargo.toml`; lockfile at `apps/api/Cargo.lock`. |
| Web applications/library | TypeScript and React | pnpm workspace. |
| Frontend build/test | pnpm, Rsbuild, Vite/Vitest | Per-app package manifests; pnpm 10.33.0 declared. |
| Deployment | Docker, Kubernetes, YAML and shell | Deployment directories inventoried; not executed. |

## Frameworks and Major Libraries

- Backend: Axum, SeaORM, Tokio, Serde ecosystem; exact full package/version catalogue belongs to Cargo manifests and lockfile.
- Frontend: React `^19.1.1`; React Router `^7.9.0` admin and `^7.11.0` public; Apollo Client `^3.12.6` / `^3.14.0`; TypeScript `^5.9.2`.
- Frontend tooling: Rsbuild `^1.5.6`, Vitest `^4.1.8`, ESLint and Prettier.
- Public server: Express `^5.2.1`; OpenTelemetry packages identified.
- Admin: TipTap `^3.14.0`, Supabase JS `^2.106.2`.
- Styling: Tailwind CSS `^4.0.12` admin and `^4.1.18` public.

## External Platform Integrations
Project architecture and manifests identify Supabase PostgreSQL, Storage, and GoTrue, plus OpenAI for translation/embeddings. Exact backup-export compatibility across Supabase hosting/version variants is not established by this scan.

## Evidence Limitations
Versions above are those explicitly surfaced in the scan handoff. This is not a complete dependency/version report; consult manifests and lockfiles for authoritative exhaustive versions. No builds or runtime checks were run.
