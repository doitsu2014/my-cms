## Developer Code Scan Results

### Scan Coverage
- **Analyzed deeply**:
  - `./` repository source/configuration inventory; top-level `.gitignore` and both frontend `.gitignore` files
  - `./README.md`
  - `./apps/api/Cargo.toml` and workspace member manifests for `domain_interface`, `domain_posts`, `domain_auth`, `domain_user`, `domain_seo`
  - `./apps/api/domain_interface/src/lib.rs`
  - `./apps/api/domain_media/src/lib.rs`
  - `./apps/api/gateway/src/main.rs` (full 761-line file, chunk-read; composition, startup, router policy and tests)
  - `./apps/api/domain_auth/src/lib.rs` (first 420 lines chunk-read; JWT middleware/validation and tests; remainder not reviewed)
  - `./apps/web/package.json`
  - `./apps/ducth-dev-website/package.json`
  - `./packages/editor-prose/package.json`
  - `./docs/architecture/software-architecture.md`
- **Skimmed only**:
  - `./apps/api/domain_posts/src/` (module tree only)
  - `./apps/api/domain_auth/src/` (module tree only)
  - `./apps/api/domain_user/src/` (module tree only)
  - `./apps/api/domain_seo/src/` (crate manifest only)
  - `./apps/web/src/`
  - `./apps/ducth-dev-website/src/`
  - `./packages/editor-prose/src/`
  - `./deployments/`, `./docs/` except architecture guide, `./scripts/`, `./.github/`
- **Left out**:
  - ignored paths per applicable `.gitignore`, including dependency folders, build output, local secrets, editor profiles/caches
  - `.claude/`, `.aidlc/`, `aidlc/` and generated AI-DLC integrations

### Packages Found
- `gateway` — binary/composition root — Rust — registers posts, auth, media, user and SEO domains and hosts the API
- `domain_interface` — contract library — Rust — shared `DomainService`, context, route, health and migration descriptors
- `domain_posts` — business domain library — Rust — posts, categories, tags, translations, GraphQL, entities and migrations
- `domain_auth` — domain library/binary — Rust — Supabase JWT authentication and role integration
- `domain_media` — business domain — Rust — media and bucket operations with Supabase Storage adapter
- `domain_user` — business domain — Rust — administrative user operations through GoTrue
- `domain_seo` — business domain — Rust — administrator-managed validated SEO head assets
- `test_helpers` — test support — Rust — shared test utilities
- `admin_side` — frontend application — TypeScript/React — editorial administration
- `ducth-dev-website` — frontend application — TypeScript/React — public reader
- `editor-prose` — shared frontend package — TypeScript/React — shared article rendering

### Build System
- **Type**: Cargo workspace (`apps/api`) plus pnpm/Rsbuild frontend applications.
- **Config Files**: `apps/api/Cargo.toml`, each crate manifest, frontend `package.json` and Rsbuild/TypeScript/Vitest configs.
- **Build Dependencies**: Gateway composes `domain_interface` and five domain crates; `domain_posts` depends on `domain_interface` and `domain_auth`; admin and reader consume local `editor-prose`.

### APIs Discovered
- REST over Axum for posts, categories, media, buckets, user administration and health; exact route count not enumerated.
- GraphQL immutable/public and mutable/authenticated at `/posts/graphql/immutable` and `/posts/graphql/mutable`.
- External Supabase GoTrue, Supabase Storage, PostgreSQL and OpenAI translation/embedding integrations.

### Frameworks & Libraries
- Rust, Axum 0.8, Tokio, SeaORM 1.1, async-graphql 7.
- React 19, TypeScript, Rsbuild, Tailwind CSS 4, DaisyUI 5; Apollo Client and TipTap in admin.
- Supabase PostgreSQL/pgvector, GoTrue and Storage; OpenAI; Docker Compose, Traefik, Jaeger.

### Test Coverage
- **Test Directories**: Rust unit/integration tests are organized under crate source/test support; exact full inventory not verified.
- **Test Frameworks**: Rust tests, Vitest, Testing Library.
- **Coverage Config**: README has coverage-service badges; exact thresholds/config not inspected.

### Code Quality Indicators
- **Linting**: frontend ESLint configs and Rust fmt/clippy commands documented in README; config details not fully examined.
- **CI/CD**: GitHub Actions indicated by README API CI badge; workflow files not inspected.
- **Documentation**: detailed README and architecture guide; Rust contract has extensive API docs and crate roots document boundaries.

### Technical Debt Signals
- Backup spans PostgreSQL and Supabase Storage, which have separate ownership and failure boundaries. No cross-store atomicity is established by inspected sources.
- Implementation details for most domains, frontend behavior, deployments, and workflows remain shallow/unopened; endpoint enumeration and operational recovery cannot be asserted.

## Handoff Summary
- **Intent-relevant finding**: CMS data is in Supabase PostgreSQL and media bytes in Supabase Storage (`docs/architecture/software-architecture.md`, lines 110–119); these must both be included in backup/restore design.
- **Risks / follow-up**: Define consistency point, restore sequencing, permissions/secrets, encryption, retention, integrity verification, and partial-failure recovery. Candidate knowledge is bounded by the coverage above; it is not a claim of exhaustive deep reading of all source files.
