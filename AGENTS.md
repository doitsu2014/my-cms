# My-CMS Agent Workflow (Lean SDLC)

## Project
Headless CMS — Rust (Axum + SeaORM) backend, React (DaisyUI + TipTap) frontend, Supabase (PostgreSQL + pgvector + Storage) platform.

## Tooling Split

The SDLC combines two complementary toolchains:

| Concern                         | Tool            | Why                                                                |
|---------------------------------|-----------------|--------------------------------------------------------------------|
| Requirements & spec design      | **OpenSpec**    | Versioned, testable capability specs; machine-checkable artifacts  |
| Proposal → design → task docs   | **OpenSpec**    | Standardized `proposal.md` / `specs/` / `design.md` / `tasks.md`   |
| Archive & spec sync             | **OpenSpec**    | Syncs delta specs into canonical `openspec/specs/<capability>/`    |
| Implementation & code execution  | **Superpowers** | Battle-tested execution skills (TDD, subagents, code review)       |

> **OpenSpec** owns *what* and *why*. **Superpowers** owns *how* (the actual coding).

## SDLC Phases

```
┌──────────────────────┐     ┌──────────────────────┐     ┌──────────────────────┐
│ 1. EXPLORE           │ ──▶ │ 2. PROPOSE & DESIGN  │ ──▶ │ 3. IMPLEMENT         │
│                      │     │                      │     │                      │
│ Agent: product-owner │     │ Agents:              │     │ Agent: coder         │
│ Skill:               │     │  product-owner       │     │ Skills (Superpowers):│
│  openspec-explore    │     │   (proposal)         │     │  executing-plans     │
│  (+ brainstorming    │     │  software-architect  │     │  subagent-driven-    │
│     for free-form)   │     │   (specs, design,    │     │   development        │
│                      │     │    tasks)            │     │  test-driven-        │
│                      │     │ Skill:               │     │   development        │
│                      │     │  openspec-propose    │     │  requesting-code-    │
│                      │     │  openspec-new        │     │   review             │
│                      │     │  openspec-continue   │     │  verification-       │
│                      │     │  openspec-ff-change  │     │   before-completion  │
└──────────────────────┘     └──────────────────────┘     └──────────────────────┘
                                                                     │
                                                                     ▼
                                                            ┌──────────────────────┐
                                                            │ 4. VERIFY & ARCHIVE  │
                                                            │                      │
                                                            │ Skills (OpenSpec):   │
                                                            │  openspec-verify-    │
                                                            │   change             │
                                                            │  openspec-sync-specs │
                                                            │  openspec-archive-   │
                                                            │   change             │
                                                            │ Skill (Superpowers): │
                                                            │  finishing-a-        │
                                                            │   development-branch │
                                                            └──────────────────────┘
```

## Phase Details

### Phase 1: Explore Requirements
**Agents:** `product-owner` (requirements & user intent) + `software-architect` (technical & architecture feasibility)
**Primary skill:** `openspec-explore` (+ optional `brainstorming` for free-form idea capture)

- Enter explore mode and investigate the problem space
- Read the codebase, map integration points, surface hidden complexity
- Check `openspec list --json` for any active change that may be relevant
- Optionally use `brainstorming` (Superpowers) for unstructured idea generation
- **`product-owner`** focuses on *what* the user needs — requirements, user stories, scope, success criteria, impact
- **`software-architect`** focuses on *how feasible it is* — current architecture, affected layers (API/Application Core/DB), library & framework fit, perf/security/data-model implications, alternative approaches
- **No code is written in this phase.** Specs may be drafted in conversation but not saved
- When thinking crystallizes, offer to create a change. The proposing agent (product-owner for product changes, software-architect for technical/architecture changes such as refactors, cross-cutting concerns, platform upgrades, or pattern shifts) drafts the proposal in Phase 2

### Phase 2: Propose & Design (OpenSpec-driven)
**Agents:** `product-owner` (proposal) + `software-architect` (specs, design, tasks)
**Primary skills:** `openspec-propose` *or* `openspec-new` + `openspec-continue` *or* `openspec-ff-change`

- Run `openspec new change "<kebab-case-name>"` to scaffold the change under `openspec/changes/`
- A change contains four artifacts, created in dependency order:
  1. **`proposal.md`** — *product-owner* drafts Why, What Changes, Capabilities, Impact
  2. **`specs/<capability>/spec.md`** — *software-architect* writes testable `### Requirement` + `#### Scenario` blocks (WHEN/THEN/AND)
  3. **`design.md`** — *software-architect* captures Context, Goals/Non-Goals, Decisions, architecture
  4. **`tasks.md`** — *software-architect* breaks work into numbered `- [ ]` checkboxes
- Use `openspec instructions <artifact> --change "<name>" --json` to get templates & rules for each artifact
- For small changes, `openspec-propose` or `openspec-ff-change` generates all four artifacts in one go
- For larger changes, step through them with `openspec-new` + `openspec-continue <name>` to review each artifact
- Re-run `openspec status --change "<name>" --json` between artifacts to track `applyRequires` readiness
- Stop when status reports all `applyRequires` artifacts `done` → ready for implementation

### Phase 3: Implement (Superpowers-driven)
**Agent:** `coder`
**Primary skills:** `executing-plans` + `subagent-driven-development` + `test-driven-development` + `requesting-code-review` + `verification-before-completion`

- Read the OpenSpec change artifacts from `openspec/changes/<name>/` (proposal, specs, design, **tasks.md**)
- Use `executing-plans` to walk through `tasks.md` checkboxes step by step
- For independent tasks, dispatch subagents in parallel (`subagent-driven-development`)
- Follow RED-GREEN-REFACTOR for every behavioral change (`test-driven-development`)
- Request a code review between task groups (`requesting-code-review`)
- Before claiming done, run `verification-before-completion`:
  - `cargo check`
  - `cargo test`
  - `cargo fmt -- --check`
  - `cargo clippy`
  - `pnpm build` (in `apps/web/`)
- Mark each task complete in `tasks.md` (`- [ ]` → `- [x]`) immediately after it passes verification

> **Note:** The OpenSpec `openspec-apply-change` skill is available as a fallback if you want OpenSpec to drive task execution. By default, the project prefers Superpowers `executing-plans` for the actual coding loop.

### Phase 4: Verify & Archive
**Agent:** `coder` (verify + sync) → `product-owner` (final archive approval)
**Primary skills (OpenSpec):** `openspec-verify-change` → `openspec-sync-specs` → `openspec-archive-change`
**Plus (Superpowers):** `finishing-a-development-branch`

1. **Verify** — Run `openspec-verify-change <name>` to check Completeness (tasks, spec coverage), Correctness (requirement ↔ implementation mapping), and Coherence (design adherence, pattern consistency). Fix all `CRITICAL` issues; review `WARNING` issues.
2. **Sync specs** — Run `openspec-sync-specs <name>` to merge delta specs from `openspec/changes/<name>/specs/` into the canonical `openspec/specs/<capability>/spec.md`. This is agent-driven and idempotent.
3. **Archive** — Run `openspec-archive-change <name>`. The change moves to `openspec/changes/archive/YYYY-MM-DD-<name>/` and becomes part of the project's decision history.
4. **Wrap up branch** — Use `finishing-a-development-branch` (Superpowers) to present options: merge, PR, keep, or discard. Never force-push; respect protected branches.
5. **Final verification** — `verification-before-completion` once more on the merged result.

## Agent Quick Reference

| Agent                | Phase      | Mode(s)      | Primary skills                                                              | Outputs (under `openspec/changes/<name>/`)                                            |
|----------------------|------------|--------------|-----------------------------------------------------------------------------|---------------------------------------------------------------------------------------|
| `product-owner`      | 1, 2, 4    | Always OpenSpec | `openspec-explore`, `openspec-propose`, `openspec-new-change`, `brainstorming` (optional) | Explored result + **`proposal.md`** (Why, What Changes, Capabilities, Impact) — final sign-off |
| `software-architect` | 1, 2        | Always OpenSpec | `openspec-new-change`, `openspec-continue-change`, `openspec-ff-change`      | **`specs/<capability>/spec.md`** (Requirement/Spec), **`design.md`** (Architecture Design), **`tasks.md`** (implementation checklist) |
| `coder`              | 3, 4        | **Normal** + **Fast Fix/Fast Implement** (see below) | Normal → `executing-plans`, `subagent-driven-development`, `test-driven-development`, `requesting-code-review`, `verification-before-completion`, `finishing-a-development-branch` · Fast Fix → `verification-before-completion`, `systematic-debugging`, `test-driven-development` (only if behavioral) | Implementation, tests, verification, branch wrap-up; Normal mode also drives `openspec-verify-change` → `openspec-sync-specs` → `openspec-archive-change` |

### Coder modes

- **Normal** — default when an active OpenSpec change has `tasks.md` ready. Read `openspec/changes/<name>/`, load `executing-plans`, execute with TDD, request code review, verify, finish.
- **Fast Fix / Fast Implement** — for small changes (typos, config tweaks, single-file refactors, hot-fixes). No `brainstorming`, no OpenSpec scaffolding, no plan. Follow existing patterns, verify, report. Triggered by an explicit "fast" / "fast fix" / "fast implement" cue, OR inferred when the change is clearly trivial.

## Key Commands / Workflow

```
"Let's explore <feature>"          → product-owner uses openspec-explore
"Propose <feature>"                → product-owner uses openspec-propose
"Write specs/design/tasks for X"   → software-architect uses openspec-continue
"Implement <change-name>"          → coder uses executing-plans (reads tasks.md)
"Verify and archive <change>"      → coder uses openspec-verify-change → sync → archive
```

**Quick CLI reference:**

```bash
# OpenSpec — spec/design lifecycle
openspec new change "<kebab-name>"        # scaffold a change
openspec list                              # list active changes
openspec status --change "<name>" --json   # artifact readiness
openspec instructions <artifact> --change "<name>" --json   # template + rules
openspec verify --change "<name>"          # completeness + correctness check
openspec sync --change "<name>"            # delta specs → main specs
openspec archive "<name>"                  # move to archive/YYYY-MM-DD-<name>/

# Cargo / pnpm — verification gate
cargo check && cargo test && cargo fmt -- --check && cargo clippy
pnpm --dir apps/web build
```

## Document Convention

OpenSpec owns the spec & decision artifacts. Superpowers is invoked for execution but does not own any document.

```
openspec/
├── config.yaml                                # OpenSpec project config (schema: spec-driven)
├── specs/                                     # Canonical capability specs (synced source of truth)
│   └── <capability>/
│       └── spec.md                            # Synced from delta specs after archive
├── changes/                                   # Active and archived changes
│   ├── <change-name>/                         # Active change
│   │   ├── proposal.md                        # Why + What Changes + Capabilities + Impact
│   │   ├── design.md                          # Context, Goals/Non-Goals, Decisions
│   │   ├── specs/
│   │   │   └── <capability>/spec.md           # Delta spec (ADDED/MODIFIED/REMOVED/RENAMED)
│   │   └── tasks.md                           # Numbered `- [ ]` implementation checklist
│   └── archive/
│       └── YYYY-MM-DD-<change-name>/          # Archived change — permanent record
```

**Lifecycle of a change:**

```
openspec new change "<name>"            ──▶  openspec/changes/<name>/
openspec-ff-change / -propose / -continue   │  proposal.md → specs/ → design.md → tasks.md
                                            │
executing-plans (Superpowers)           ──▶  │  tasks.md checkboxes ticked off
                                            │
openspec-verify-change                   ──▶  │  Completeness + Correctness + Coherence report
openspec-sync-specs                      ──▶  │  delta specs → openspec/specs/<capability>/
openspec-archive-change                  ──▶  openspec/changes/archive/YYYY-MM-DD-<name>/
```

> **Legacy `docs/superpowers/`** holds pre-OpenSpec historical artifacts. New work uses `openspec/` only. Do not add new files under `docs/superpowers/`.

---

## Project Structure

```
my-cms/
├── apps/
│   ├── api/                           # Rust backend
│   │   ├── src/
│   │   │   ├── api/                   # API layer (Axum routes + handlers)
│   │   │   │   ├── category/          # Category CRUD
│   │   │   │   ├── post/              # Post CRUD + AI translate
│   │   │   │   ├── tag/               # Tag management
│   │   │   │   ├── media/             # Media upload/serve
│   │   │   │   ├── public/            # Public endpoints
│   │   │   │   ├── graphql/           # GraphQL endpoint
│   │   │   │   └── administrator/     # Admin operations
│   │   │   ├── common/                # Shared utilities, auth middleware
│   │   │   ├── presentation_models/   # API request/response DTOs
│   │   │   └── lib.rs                 # AppState definition
│   │   ├── application_core/          # Business logic layer
│   │   │   └── src/
│   │   │       ├── commands/          # Command handlers (business logic)
│   │   │       │   ├── category/
│   │   │       │   ├── post/
│   │   │       │   ├── tag/
│   │   │       │   ├── media/
│   │   │       │   └── ai/            # AI translation (3-tier lookup)
│   │   │       ├── entities/          # SeaORM entities (auto-generated)
│   │   │       └── common/            # AppError, domain utils
│   │   ├── migration/                 # Database migrations (SeaORM)
│   │   └── test_helpers/              # Test utilities
│   └── web/                           # React frontend
│       └── src/
│           ├── app/admin/             # Admin pages (layout, dashboard, CRUD)
│           ├── components/            # Reusable UI components
│           ├── domains/               # Domain type definitions
│           ├── models/                # API request/response models
│           ├── schemas/               # Zod validation schemas
│           ├── auth/                  # Auth context + Supabase client
│           ├── config/                # Runtime config, API utilities
│           └── infrastructure/        # GraphQL client, auth utilities
├── openspec/                          # Spec & change management (OpenSpec)
│   ├── config.yaml
│   ├── specs/                         # Canonical capability specs (synced)
│   └── changes/                       # Active changes + archive
├── deployments/                        # Deployment configs (isolated from app source)
│   ├── docker-swarm/                   # Docker Compose local dev stack
│   │   ├── docker-compose.my-cms.yaml  # Apps stack (API + Web + Jaeger)
│   │   ├── docker-compose.supabase.yaml# Supabase stack
│   │   ├── .env.supabase               # Supabase env (gitignored, from .example)
│   │   ├── .env.supabase.example       # Supabase env template
│   │   ├── .env.my-cms                  # Apps env (gitignored, from .example)
│   │   ├── .env.my-cms.example          # Apps env template
│   │   ├── volumes/                     # Mounted configs (SQL, kong, pooler)
│   │   │   ├── db/                      # Postgres init scripts + data
│   │   │   ├── api/                     # Kong gateway config
│   │   │   ├── pooler/                  # Supavisor config
│   │   │   └── secrets/                 # Generated secrets (admin password)
│   │   ├── bootstrap.sh                # One-time network setup
│   │   ├── reset-apps.sh               # Reset / restart / rebuild apps
│   │   └── reset-supabase.sh           # Reset / restart Supabase
│   └── k8s/                            # Helm charts (production)
└── AGENTS.md                          # This file — SDLC workflow + conventions
```

---

## Rust Backend Conventions

### Architecture: Strictly Layered

```
API Layer (apps/api/src/api/)        — HTTP routing, serialization, auth extraction
        │
        ▼
Application Core (apps/api/application_core/) — Business logic, command handlers
        │
        ▼
Database Layer (entities/)           — SeaORM entities (auto-generated)
```

**Rule:** Never put business logic in API handlers. API handlers extract request, call command handler, return response.

### Command Pattern (mandatory)

```rust
pub trait CreateFooHandlerTrait {
    fn handle_create_foo(&self, req: CreateFooRequest)
        -> impl Future<Output = Result<Foo, AppError>>;
}

pub struct CreateFooHandler {
    pub db: Arc<DatabaseConnection>,
}

impl CreateFooHandlerTrait for CreateFooHandler {
    async fn handle_create_foo(&self, req: CreateFooRequest) -> Result<Foo, AppError> {
        // business logic
    }
}
```

### Error Handling
- Every fallible function returns `Result<T, AppError>`
- Use `?` to propagate — never `unwrap()` or `expect()` in production code
- Add error context with `.map_err(|e| AppError::Variant(format!(...)))?`

### Database (SeaORM)
- **Schema-first**: Create migrations → run them → generate entities from DB
- **Never manually edit** entity files in `apps/api/application_core/src/entities/`
- Use `Arc<DatabaseConnection>` for shared DB access
- For transactions: `let txn = db.begin().await?; ... txn.commit().await?;`

### Async / Concurrency (Tokio)
- Use `JoinSet` for parallel operations: `let mut set = JoinSet::new(); set.spawn(async {...});`
- For fire-and-forget: `tokio::spawn(async move { ... })` with `Arc::clone()`
- Never block in async context (no `std::thread::sleep`, no blocking I/O)

### Testing
- **Unit tests**: SeaORM `MockDatabase` — `#[cfg(test)] mod tests` in handler file
- **Integration tests**: `testcontainers` for full PostgreSQL

### Tracing
- Use `#[instrument]` on important functions, skip large fields with `skip(field)`
- Log levels: `info!()` for state changes, `warn!()` for recoverable, `error!()` for failures

---

## React Frontend Conventions

### Component Architecture
- **Page components** (`src/app/admin/*/page.tsx`): data fetching, routing, pass data down
- **Presentational components** (`src/components/`): receive props, render UI, minimal state
- **Forms**: React Hook Form + Zod validation in `src/schemas/`

### State Management
- Local state: `useState` for component-specific
- Shared state: React Context (auth, config)
- URL state: `useSearchParams` for filters/pagination

### Data Fetching
- GraphQL: Apollo `useQuery` / `useMutation`
- REST: `authenticatedFetch(getApiUrl(path), token, options, keycloak?)`
- Auth context: `useAuth()` from `src/auth/AuthContext.tsx`

### UI (DaisyUI + Tailwind CSS 4)
- Buttons: `btn btn-primary`, `btn btn-ghost`, `btn btn-outline`
- Icons: Lucide React (`<Save className="w-5 h-5" />`)
- Toast: Sonner (`toast.success()`, `toast.error()`)
- Loading: `<span className="loading loading-spinner" />` or `skeleton` divs
- Cards: `<div className="card bg-base-100 shadow-xl"><div className="card-body">...`

### Forms
```tsx
const { register, handleSubmit, control, formState: { errors } } = useForm<Data>({
  resolver: zodResolver(schema),
});

// Controlled components (rich text, etc.):
<Controller name="content" control={control} render={({ field }) => <Editor {...field} />} />

// Dynamic arrays:
const { fields, append, remove } = useFieldArray({ control, name: "items" });
```

### Key Imports
```tsx
import { getApiUrl, authenticatedFetch } from '@/config/api.config';
import { useAuth } from '@/auth/AuthContext';
import { toast } from 'sonner';
import { Save, Edit, Trash2, Plus, X } from 'lucide-react';
```

### Routing (React Router v7)
```tsx
import { useNavigate, useParams, useSearchParams } from 'react-router-dom';
<Route path="/admin/categories" element={<CategoriesPage />} />
<Route path="/admin/categories/edit/:id" element={<EditCategoryPage />} />
```

---

## Verify Before Commit

```bash
cargo check                 # verify compilation
cargo test                  # verify tests pass
cargo fmt -- --check        # verify formatting
cargo clippy                # verify lint
pnpm --dir apps/web build   # verify frontend builds
```

## Tech Stack Reference

| Layer | Technology |
|-------|-----------|
| Backend | Rust, Axum 0.8, SeaORM 1.1, Tokio |
| Database | PostgreSQL 15+ (Supabase: pgvector, PostgREST, GoTrue) |
| Frontend | React 19, DaisyUI 5, Tailwind CSS 4, TipTap, rsbuild |
| API | REST + GraphQL (Seaography) |
| AI | OpenAI GPT, pgvector (3-tier lookup: DB→pgvector→OpenAI) |
| Media | Supabase Storage (S3-compatible) |
| Auth | Supabase GoTrue JWT (custom middleware) |
| Observability | OpenTelemetry + Jaeger |
| Spec Management | OpenSpec 1.4+ (capability specs + change workflow) |
| SDLC Skills | Superpowers (brainstorming, executing-plans, subagent-driven-development, test-driven-development, requesting-code-review, verification-before-completion, finishing-a-development-branch) |
| Infra | Docker Compose (local), Helm (prod) |
