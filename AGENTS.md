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
│ Agents:              │     │ Agents (serial):     │     │ Agent:               │
│  product-owner       │     │  product-owner       │     │  software-engineer   │
│  product-designer    │     │   (proposal)         │     │                      │
│  software-architect  │     │  product-designer    │     │ TDD + graph review + │
│                      │     │   (UX brief)         │     │ focused review + full│
│ Skill:               │     │  software-architect  │     │ verification         │
│  openspec-explore    │     │   (specs/design/     │     │                      │
│                      │     │    tasks)            │     │                      │
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

## Team Orchestration Contract

The primary agent remains the coordinator and owns the final synthesis. For every delegated task, provide **Goal**, **Context**, **Constraints**, and **Done when** so the subagent can work without inventing scope.

- Use the named agents for their narrow roles; do not send implementation work to PO/PD/SA or product decisions to SE.
- Parallelize independent read-heavy work such as discovery, code-path mapping, test-gap review, and risk analysis. Serialize write-heavy work.
- Enforce one writer per artifact or source file at a time. In Phase 2 the default sequence is PO proposal → PD UX/design brief → SA specs/design/tasks.
- Give each delegated agent a bounded question and expected output. Wait for all requested results, then return a distilled synthesis rather than raw logs.
- Preserve traceability across handoffs: product outcome → UX behavior → requirement/scenario → architecture decision → task → test/verification.
- Every handoff reports: goal, evidence inspected, decisions, artifacts changed, verification, assumptions/risks, open questions, and next owner.
- If an agent discovers a decision outside its authority, it returns the issue to the owning role instead of silently expanding scope.

## Phase Details

### Phase 1: Explore Requirements
**Agents:** `product-owner` (requirements & user intent) + `product-designer` (UX & visual direction) + `software-architect` (technical & architecture feasibility)
**Primary skills:** `openspec-explore` + `map-my-cms-api-architecture` for API work (+ optional `brainstorming` for free-form idea capture)

- Enter explore mode and investigate the problem space
- Read the codebase, map integration points, surface hidden complexity
- Check `openspec list --json` for any active change that may be relevant
- Optionally use `brainstorming` (Superpowers) for unstructured idea generation
- **`product-owner`** focuses on *what* the user needs — requirements, user stories, scope, success criteria, impact
- **`product-designer`** focuses on *how the product should feel and work* — responsive information architecture, interaction flows, accessibility, visual language, and reusable frontend patterns
- **`software-architect`** focuses on *how feasible it is* — current architecture, affected layers (API/Application Core/DB), library & framework fit, perf/security/data-model implications, alternative approaches
- **No code is written in this phase.** Specs may be drafted in conversation but not saved
- When thinking crystallizes, offer to create a change. The proposing agent (product-owner for product changes, software-architect for technical/architecture changes such as refactors, cross-cutting concerns, platform upgrades, or pattern shifts) drafts the proposal in Phase 2

### Phase 2: Propose & Design (OpenSpec-driven)
**Agents:** `product-owner` (proposal) + `product-designer` (UX/visual design) + `software-architect` / SA (specs, design, tasks)
**Primary skills:** `openspec-propose` *or* `openspec-new` + `openspec-continue` *or* `openspec-ff-change`; SA uses `map-my-cms-api-architecture` → `design-my-cms-api-change` for API changes

- Default to a serial handoff: the PO finalizes product scope, the PD returns an implementation-ready UX/design brief, and the SA integrates it with technical decisions. Do not let multiple agents edit `design.md` concurrently.
- Run `openspec new change "<kebab-case-name>"` to scaffold the change under `openspec/changes/`
- A change contains four artifacts, created in dependency order:
  1. **`proposal.md`** — *product-owner* drafts Why, What Changes, Capabilities, Impact
  2. **`specs/<capability>/spec.md`** — *software-architect* writes testable `### Requirement` + `#### Scenario` blocks (WHEN/THEN/AND)
  3. **`design.md`** — *software-architect* integrates the *product-designer* brief with technical architecture, constraints, and decisions
  4. **`tasks.md`** — *software-architect* breaks work into numbered `- [ ]` checkboxes
- Use `openspec instructions <artifact> --change "<name>" --json` to get templates & rules for each artifact
- For small changes, `openspec-propose` or `openspec-ff-change` generates all four artifacts in one go
- For larger changes, step through them with `openspec-new` + `openspec-continue <name>` to review each artifact
- Re-run `openspec status --change "<name>" --json` between artifacts to track `applyRequires` readiness
- Stop when status reports all `applyRequires` artifacts `done` → ready for implementation

#### Code-review graph gate (SA)

The SA must inject the `code-review-graph` MCP workflow before finalizing the proposal, specs, or design:

1. Start with `get_minimal_context(task="<change>")`.
2. Inspect the affected architecture, communities, callers/callees, imports, and flows.
3. Use those findings to validate integration points, risk, test coverage, and the task breakdown.

If the graph server is unavailable, record the limitation and use repository inspection instead; never fabricate graph findings.

### Phase 3: Implement (Superpowers-driven)
**Agent:** `coder` in OpenCode or `software-engineer` / SE in Codex
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

#### Code-review graph gate (SE)

The SE must inject the `code-review-graph` MCP workflow before implementation and after each task group:

1. Before editing, call `get_minimal_context(task="<change>")` and inspect affected callers, callees, imports, communities, and flows.
2. After each task group, run `detect_changes`, `get_affected_flows`, `tests_for` for high-risk functions, and `get_impact_radius`.
3. Resolve material findings or document why a finding is not applicable before continuing.

If the graph server is unavailable, record the limitation and substitute `git diff` plus the repository verification gate.

> **Note:** The OpenSpec `openspec-apply-change` skill is available as a fallback if you want OpenSpec to drive task execution. By default, the project prefers Superpowers `executing-plans` for the actual coding loop.

### Phase 4: Verify & Archive
**Agent:** `coder` in OpenCode or `software-engineer` in Codex (verify + sync) → `product-owner` (final archive approval)
**Primary skills (OpenSpec):** `openspec-verify-change` → `openspec-sync-specs` → `openspec-archive-change`
**Plus (Superpowers):** `finishing-a-development-branch`

1. **Verify** — Run `openspec-verify-change <name>` to check Completeness (tasks, spec coverage), Correctness (requirement ↔ implementation mapping), and Coherence (design adherence, pattern consistency). Fix all `CRITICAL` issues; review `WARNING` issues.
2. **Sync specs** — Run `openspec-sync-specs <name>` to merge delta specs from `openspec/changes/<name>/specs/` into the canonical `openspec/specs/<capability>/spec.md`. This is agent-driven and idempotent.
3. **Archive** — Run `openspec-archive-change <name>`. The change moves to `openspec/changes/archive/YYYY-MM-DD-<name>/` and becomes part of the project's decision history.
4. **Wrap up branch** — Use `finishing-a-development-branch` (Superpowers) to present options: merge, PR, keep, or discard. Never force-push; respect protected branches.
5. **Final verification** — `verification-before-completion` once more on the merged result.

## Agent Quick Reference

| Agent                | Phase      | Mode(s)      | Primary skills                                                              | Primary outputs                                                                        |
|----------------------|------------|--------------|-----------------------------------------------------------------------------|---------------------------------------------------------------------------------------|
| `product-owner`      | 1, 2, 4    | OpenCode agent or Codex project agent (`.codex/agents/product-owner.toml`) | `openspec-explore`, `openspec-propose`, `openspec-new-change`, `brainstorming` (optional) | Explored result + **`proposal.md`** (Why, What Changes, Capabilities, Impact) — final sign-off |
| `product-designer`   | 1, 2       | Codex project agent (`.codex/agents/product-designer.toml`) | Responsive UX, information architecture, design language, accessibility, UI component guidance | Screen specifications, responsive behavior, interaction states, design tokens, and implementation-ready design guidance |
| `software-architect` | 1, 2        | Always OpenSpec | `map-my-cms-api-architecture`, `design-my-cms-api-change`, `openspec-new-change`, `openspec-continue-change`, `openspec-ff-change` | Source-backed architecture map, **`specs/<capability>/spec.md`**, **`design.md`**, and **`tasks.md`** |
| `coder`              | 3, 4        | **Normal** + **Fast Fix/Fast Implement** (see below) | Normal → `executing-plans`, `subagent-driven-development`, `test-driven-development`, `requesting-code-review`, `verification-before-completion`, `finishing-a-development-branch` · Fast Fix → `verification-before-completion`, `systematic-debugging`, `test-driven-development` (only if behavioral) | Implementation, tests, verification, branch wrap-up; Normal mode also drives `openspec-verify-change` → `openspec-sync-specs` → `openspec-archive-change` |
| `software-engineer` | 3, 4        | Codex project agent (`.codex/agents/software-engineer.toml`) | `executing-plans`, `subagent-driven-development`, `test-driven-development`, code-review-graph, `verification-before-completion` | Implementation, tests, graph impact review, verification, branch wrap-up |

### Codex Agent Team Definition

Codex loads the project-scoped team from `.codex/agents/` when the repository is trusted. Use `product-owner` (PO) for exploration and proposals, `product-designer` (PD) for responsive UX and design language, `software-architect` (SA) for proposal/spec/design/task review, and `software-engineer` (SE) for implementation. SA and SE are configured with the `code-review-graph` MCP server in `.codex/config.toml` and their agent files. The graph gates above are mandatory for SA and SE.

Codex loads the repository OpenSpec workflow from `.agents/skills/openspec/SKILL.md`. It is the Codex-native equivalent of the `.opencode/skills/openspec-*` skills and maps multi-step tracking to `update_plan` while preserving the same OpenSpec CLI lifecycle.

For API architecture work, the SA loads `.agents/skills/map-my-cms-api-architecture/SKILL.md` to reconstruct current behavior from `apps/api`, then `.agents/skills/design-my-cms-api-change/SKILL.md` to turn that evidence into contracts, decisions, migrations, tasks, and verification. The source-derived references are navigation baselines and must be revalidated against current code.

### Coder modes

- **Normal** — default when an active OpenSpec change has `tasks.md` ready. Read `openspec/changes/<name>/`, load `executing-plans`, execute with TDD, request code review, verify, finish.
- **Fast Fix / Fast Implement** — for small changes (typos, config tweaks, single-file refactors, hot-fixes). No `brainstorming`, no OpenSpec scaffolding, no plan. Follow existing patterns, verify, report. Triggered by an explicit "fast" / "fast fix" / "fast implement" cue, OR inferred when the change is clearly trivial.

## Key Commands / Workflow

```
"Let's explore <feature>"          → product-owner uses openspec-explore
"Propose <feature>"                → product-owner uses openspec-propose
"Design UX for <change>"           → product-designer audits UI and produces a responsive design brief
"Extract our design language"      → product-designer derives tokens, patterns, states, and usage rules
"Map API architecture for X"       → software-architect uses map-my-cms-api-architecture
"Design API change X"              → software-architect maps current source, then uses design-my-cms-api-change
"Write specs/design/tasks for X"   → software-architect uses openspec-continue
"Implement <change-name>"          → software-engineer in Codex or coder in OpenCode executes tasks.md
"Verify and archive <change>"      → software-engineer/coder runs verify → sync → archive
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
│   │   ├── bootstrap.sh                # One-time network setup
│   │   ├── README.md                   # Quickstart + per-component entry points
│   │   ├── supabase/                   # Supabase stack (compose + env + reset + volumes)
│   │   │   ├── docker-compose.yaml
│   │   │   ├── docker-compose.expose.yaml   # optional override: expose ports directly
│   │   │   ├── .env / .env.example
│   │   │   ├── reset.sh
│   │   │   └── volumes/                # SQL init, Kong, Supavisor, secrets
│   │   ├── apps/                       # my-cms apps (API + Web + Jaeger)
│   │   │   ├── docker-compose.yaml
│   │   │   ├── .env / .env.example
│   │   │   └── reset.sh
│   │   └── traefik/                    # Reverse proxy (file-based routing)
│   │       ├── docker-compose.yaml
│   │       ├── .env.example            # CMS_HOST, CORS origins, Basic Auth
│   │       ├── reset.sh
│   │       └── dynamic/my-cms.yml      # Router/middleware/service definitions
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
