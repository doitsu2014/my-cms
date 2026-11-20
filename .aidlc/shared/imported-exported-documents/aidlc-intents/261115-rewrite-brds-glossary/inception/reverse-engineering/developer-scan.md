## Developer Code Scan Results

### Scan Coverage
- **Analyzed deeply**:
  - `apps/api/` — Cargo workspace membership and package manifests; gateway composition, domain service contract and registrations, authentication middleware, post translation flow, GraphQL contribution, and service-level entry points.
  - `apps/web/` — package manifest, application routing, GraphQL client/auth link, and TipTap editor implementation relevant to the shared authoring experience.
  - `apps/ducth-dev-website/` — package manifest, SSR entry and production server, route composition, and shared prose consumption.
  - `packages/editor-prose/` — package manifest, ArticleProse integration, Mermaid enhancement lifecycle, and tests discovered by directory inventory.
  - `deployments/docker-swarm/` — app-stack composition and README; `deployments/k8s/` — chart and manifest inventory.
  - `.github/workflows/` — CI workflow and image/release publishing workflow inventory.
  - Root `README.md`, `.gitignore`, `apps/web/.gitignore`, and `apps/ducth-dev-website/.gitignore`.
- **Skimmed only**:
  - Remaining source modules and tests within `apps/api/`, `apps/web/`, `apps/ducth-dev-website/`, and `packages/editor-prose/`; package-level directory listings were examined, but not every source file was read.
  - Remaining deployment definitions in `deployments/`, documentation outside the root README, and remaining active workflow files in `.github/`.
  - Test and fixture files: locations and framework/config evidence were inventoried, but suites were not executed or exhaustively read.
- **Left out**:
  - `.claude/`, `.aidlc/`, and the `aidlc/` workflow workspace (AI-DLC installation/workspace, not application source).
  - Ignored and generated/dependency/cache content: `target/`, `debug/`, `dist/`, `node_modules/`, `.pnpm-store/`, `.idea/`, `.worktrees/`, local secrets/env files, Docker volumes, coverage outputs, and test-LLVM reports. These were not opened.

### Packages Found
- `gateway` — API composition-root binary — Rust — builds the sole shipped `my-cms-api`; merges domain routes and applies shared concerns.
- `domain_interface` — library/contract — Rust — defines `DomainService`, shared context, route/mount and migration descriptors, actor identity, and errors.
- `domain_posts` — domain library — Rust — posts, categories, tags, translations, GraphQL integration, persistence entities, and canonical post migrations.
- `domain_auth` — infrastructure/domain library — Rust — Supabase JWT validation and role middleware; no domain-owned routes or migrations evident in its service implementation.
- `domain_media` — domain library — Rust — media and bucket APIs, Supabase Storage adapter, configuration, and caches.
- `domain_user` — domain library — Rust — administrator user operations and Supabase GoTrue admin adapter.
- `domain_seo` — domain library — Rust — validated SEO head-asset storage, administrator CRUD, public delivery, and its migration.
- `test_helpers` — test-support library — Rust — present in the API workspace; detailed source not deeply analyzed.
- `admin_side` — frontend application — TypeScript/React — CMS administration and authoring.
- `ducth-dev-website` — frontend/SSR application — TypeScript/React with Node/Express production server — localized public reader.
- `editor-prose` — shared frontend library — TypeScript/React — common article-prose styles and Mermaid rendering used by both frontends.

### Build System
- **Type**: Cargo workspace under `apps/api/`; pnpm-managed independent frontend packages; Docker Compose and Helm for deployments; GitHub Actions for CI and image/release publishing.
- **Config Files**: `apps/api/Cargo.toml`; crate manifests in `apps/api/{gateway,domain_interface,domain_posts,domain_auth,domain_media,domain_user,domain_seo,test_helpers}/Cargo.toml`; `apps/web/package.json`; `apps/ducth-dev-website/package.json`; `packages/editor-prose/package.json`; frontend TypeScript, Rsbuild, and Vitest configs; `deployments/docker-swarm/` Compose files; `deployments/k8s/charts/`; `.github/workflows/`.
- **Build Dependencies**: `gateway` path-depends on the five domain crates and `domain_interface`; domain crates implement the `domain_interface` contract. Frontends and public site depend on local `packages/editor-prose`. Both frontends use React 19, Rsbuild, Tailwind CSS 4 and Apollo; the public site also uses Express SSR and i18next. API runs on Axum/Tokio/SeaORM with async-graphql/Seaography. Compose builds API from `apps/api` and frontends from app-scoped contexts with a named `editor-prose` context.

### APIs Discovered
- HTTP REST — `apps/api/domain_posts/src/api/` and `handlers/` — routes for content, categories, tags, translation and translation jobs; route inventory is distributed across registration modules and was not fully enumerated here.
- GraphQL HTTP — `apps/api/domain_posts/src/api/` and `domain/graphql.rs` — public immutable `/posts/graphql/immutable` and authenticated mutable `/posts/graphql/mutable`; schema contribution includes seven historical entities (see `domain/graphql.rs`).
- HTTP REST — `apps/api/domain_user/` — `POST /users`, `GET /users`, `GET /users/:id`, `PUT /users/:id`, `DELETE /users/:id`, and `POST /users/:id/reset-password` (project knowledge `aidlc/spaces/default/knowledge/aidlc-shared/auth-users-media.md`; service source confirms domain responsibility).
- HTTP REST — `apps/api/domain_media/` — media and bucket operations; handlers/API modules inventoried, but full route/method list not read.
- HTTP REST — `apps/api/domain_seo/` — administrator head-asset CRUD and public enabled-asset delivery; route registration evidence in `service.rs` and test.
- HTTP — gateway health routes `/`, `/health`, and `/healthz`, registered in `apps/api/gateway/src/main.rs`.
- HTTP — public website routes under `/:lang` include home, categories, category detail, post detail, and about; `apps/ducth-dev-website/src/AppContent.tsx`.
- External integrations — Supabase GoTrue/JWT and Admin API, Supabase Storage, PostgreSQL/pgvector, OpenAI chat completions and embeddings (see `apps/api/domain_posts/Cargo.toml` and translation handler). Exact external payload contracts were not fully scanned.

### Frameworks & Libraries
- Rust 2021; Axum `0.8.9`, Tokio `1.52.3`, SeaORM `1.1.20`, SeaORM migration `1.1.20`, async-graphql `7.2.1`, Seaography `1.1.4` (from Cargo manifests).
- React `19.1.1`, TypeScript `5.9.2`, Rsbuild `1.5.6`, Tailwind CSS 4, DaisyUI 5, Apollo Client 3.x, TipTap `3.14.0` (admin package manifest).
- Public site: Express `5.2.1`, React Router `7.11.0`, i18next `25.7.4`, OpenTelemetry Node SDK `0.212.0` (package manifest).
- Shared prose: Mermaid `11.17.2`, React peer dependency `>=18` (package manifest).
- Operations: Docker Compose, Traefik, self-hosted Supabase, Jaeger, Helm, and GitHub Actions (root README and deployment/workflow files).

### Test Coverage
- **Test Directories**: Rust tests are colocated in crate source and `apps/api/test_helpers`; frontend tests are colocated and under `apps/web/src` and `apps/ducth-dev-website/src`; shared package tests under `packages/editor-prose/src/__tests__/`.
- **Test Frameworks**: Rust built-in test harness with Tokio test support and `wiremock`; frontend Vitest and Testing Library. The SEO crate manifest also declares `testcontainers` and PostgreSQL test modules.
- **Coverage Config**: API CI workflow `.github/workflows/ci-my-cms.yml` contains grcov/LLVM coverage instrumentation and uploads to Coveralls and Codecov. Frontend Vitest configs exist; coverage configuration not confirmed in this scan.

### Code Quality Indicators
- **Linting**: Frontend package scripts include ESLint and Prettier; public website has `typecheck`; Rust CI workflow invokes Cargo tests, while root README documents `cargo fmt -- --check` and `cargo clippy`.
- **CI/CD**: `.github/workflows/ci-my-cms.yml`; separate workflows publish API, admin, public site, and GitHub Release images. A CI file contains a prominent `TEMPORARY DISABLED` comment; actual repository event execution was not tested. Deployment notes in current project knowledge state publishers deploy no workloads.
- **Documentation**: Root README, deployment README, `docs/`, and requirements/architecture knowledge under `aidlc/spaces/default/knowledge/aidlc-shared/` exist. Project knowledge notes that the old OpenSpec tree was deliberately removed; do not recreate it.
- **Source-quality observations**: `apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs` is 1,144 lines, well beyond the guide's >500-line technical-debt signal. The auth package still has a `src/main.rs` stub; current project knowledge explicitly says it is not a deployment surface. Historical transition/standalone references remain in comments, including `apps/api/domain_posts/src/handlers/post/translate/mod.rs` and `src/migrations/mod.rs`.

### Technical Debt Signals
- `apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs` is a >500-line file containing lookup, provider calls, chunking, persistence, and background job lifecycle responsibilities.
- `apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs` includes comments describing a non-atomic delete-then-insert retranslation window (roughly lines 432–437); a failed insert after delete could lose the prior translation. Paragraph validation occurs before saving, but this remaining persistence window is explicitly noted in source.
- `apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs` retains placeholder/disabled handler tests (roughly lines 1130–1143); comments say end-to-end coverage is in `apps/api/test_helpers`, which was not deeply examined.
- `.github/workflows/ci-my-cms.yml` labels itself temporarily disabled (line 3); verify intended CI status before relying on it as an active gate.
- `apps/api/domain_auth/src/main.rs` is a stub binary. Project contract says do not expand or deploy it; the gateway is the sole API binary.
- Historical “transition” and obsolete migration-shim comments in post-domain source may diverge from current architecture docs; comments are not evidence of separate current deployment surfaces.
- No tests, builds, or programs were run per instruction. Findings about behavior are from static source and project knowledge, not execution verification.

## Handoff Summary
- **Intent-relevant finding**: The active intent is to rewrite project BRDs and glossary in the shared knowledge folder. The authoritative requirements location is `aidlc/spaces/default/knowledge/aidlc-shared/`; its `spec-migration-index.md` maps the migrated source contract to backend, auth/media, posts/translation, admin/editor, public-site, and deployment knowledge. The index states the docs/source/knowledge precedence and warns not to treat archived changes or unchecked task boxes as implementation proof. For technical grounding, the primary cross-cutting references are `docs/architecture/software-architecture.md`, `docs/features/ai-translation.md`, and `docs/features/editor-prose-contract.md`, cross-checked against source as listed above.
- **Risks / follow-up**: This is a breadth-limited static scan despite full-repository snapshot breadth `./`: only principal entry points, package manifests, and selected high-signal components were read deeply, with substantial application/deployment source skimmed. Do not treat the endpoint inventory, exhaustive dependency graph, or detailed test coverage as complete. Before synthesizing requirements/glossary claims, cross-check the intent-specific requirements against canonical knowledge and actual source, especially the translation path and the source files listed in the knowledge map. The repository has only `apps/api/Cargo.toml` (no root `Cargo.toml`). No tests/builds were run. Snapshot commit supplied for this handoff: `039a5815c074a351c4ae56517932dc9793c187a0`.
