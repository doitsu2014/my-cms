# My-CMS Agent Workflow (Lean SDLC)

## Project
Headless CMS — Rust (Axum + SeaORM) backend, React (DaisyUI + TipTap) frontend, Supabase (PostgreSQL + pgvector + Storage) platform.

## SDLC Phases

```
┌──────────────────────┐     ┌──────────────────────┐     ┌──────────────────────┐
│ 1. EXPLORE           │ ──▶ │ 2. PROPOSE           │ ──▶ │ 3. IMPLEMENT         │
│                      │     │                      │     │                      │
│ Agent: product-owner │     │ Agents:              │     │ Agent: coder         │
│ Skill: brainstorming │     │ product-owner        │     │ Skill: executing-    │
│                      │     │ (proposal)           │     │ plans                │
│                      │     │                      │     │                      │
│                      │     │ software-architect   │     │ + subagent-driven    │
│                      │     │ (design + tasks)     │     │ + TDD                │
│                      │     │ Skill: writing-plans │     │ + code review        │
└──────────────────────┘     └──────────────────────┘     └──────────────────────┘
                                                                     │
                                                                     ▼
                                                            ┌──────────────────────┐
                                                            │ 4. ARCHIVE           │
                                                            │ Wrap up change       │
                                                            │ Skill: finishing-    │
                                                            │ a-development-branch │
                                                            └──────────────────────┘
```

## Phase Details

### Phase 1: Gather Requirements
**Agent:** `product-owner`
**Skill:** `brainstorming`

- Explore ideas, investigate problems, clarify requirements
- Read existing specs in `docs/superpowers/specs/` and codebase
- Produce a design document at `docs/superpowers/specs/YYYY-MM-DD-feature-name.md`
- No code written — thinking phase only

### Phase 2: Design + Tasks
**Agents:** `product-owner` (proposal) + `software-architect` (design + tasks)
**Skill:** `writing-plans`

- `product-owner` writes the proposal: what & why, user stories, acceptance criteria
- `software-architect` reads the proposal, designs architecture, writes the implementation plan
- Plan output: `docs/superpowers/plans/YYYY-MM-DD-feature-name.md`
- Plan format: detailed tasks with exact file paths, `- [ ]` checkboxes, verification steps

### Phase 3: Implement
**Agent:** `coder`
**Skills:** `executing-plans` + `subagent-driven-development` + `test-driven-development` + `requesting-code-review`

- Reads the plan from `docs/superpowers/plans/`
- Executes tasks step by step (executing-plans)
- Spawns subagents for parallel independent tasks (subagent-driven-development)
- Follows RED-GREEN-REFACTOR cycle (test-driven-development)
- Requests code review between task groups (requesting-code-review)
- Runs `cargo check` → `cargo test` to verify
- Marks tasks `[x]` as completed

### Phase 4: Archive
**Skill:** `finishing-a-development-branch` + `verification-before-completion`

- Verifies all tests pass
- Presents options: merge, PR, keep, or discard
- Archives completed specs and plans into `docs/superpowers/` for permanent reference
- Final documents live at:
  - `docs/superpowers/specs/YYYY-MM-DD-feature-name.md` (design, user stories, acceptance criteria)
  - `docs/superpowers/plans/YYYY-MM-DD-feature-name.md` (architecture, tasks, verification results)

## Agent Quick Reference

| Agent | Skill | Phase | Responsibility |
|-------|-------|-------|----------------|
| `product-owner` | `brainstorming` | 1, 2 | Requirements, user stories, proposal |
| `software-architect` | `writing-plans` | 2 | System design, API contracts, DB schema, task breakdown |
| `coder` | `executing-plans` | 3 | Implementation, unit tests, verification |

## Key Commands / Workflow

```
"Let's brainstorm <feature>"       → product-owner uses brainstorming
"Write a plan for <feature>"       → software-architect uses writing-plans
"Execute the plan"                 → coder uses executing-plans
```

## Document Convention

```
docs/superpowers/
├── specs/                          # Design documents (from brainstorming)
│   └── YYYY-MM-DD-feature-name.md  # Preserved as permanent record after archive
└── plans/                          # Implementation plans (from writing-plans)
    └── YYYY-MM-DD-feature-name.md  # Preserved as permanent record after archive
```

After Phase 4 (Archive), the spec and plan are retained in `docs/superpowers/` for future reference by other agents and developers.

---

## Project Structure

```
my-cms/
├── services/                          # Rust backend
│   ├── src/
│   │   ├── api/                       # API layer (Axum routes + handlers)
│   │   │   ├── category/              # Category CRUD
│   │   │   ├── post/                  # Post CRUD + AI translate
│   │   │   ├── tag/                   # Tag management
│   │   │   ├── media/                 # Media upload/serve
│   │   │   ├── public/                # Public endpoints
│   │   │   ├── graphql/               # GraphQL endpoint
│   │   │   └── administrator/         # Admin operations
│   │   ├── common/                    # Shared utilities, auth middleware
│   │   ├── presentation_models/       # API request/response DTOs
│   │   └── lib.rs                     # AppState definition
│   ├── application_core/              # Business logic layer
│   │   └── src/
│   │       ├── commands/              # Command handlers (business logic)
│   │       │   ├── category/
│   │       │   ├── post/
│   │       │   ├── tag/
│   │       │   ├── media/
│   │       │   └── ai/               # AI translation (3-tier lookup)
│   │       ├── entities/              # SeaORM entities (auto-generated)
│   │       └── common/               # AppError, domain utils
│   ├── migration/                     # Database migrations (SeaORM)
│   └── test_helpers/                  # Test utilities
├── frontend/                          # React frontend
│   └── src/
│       ├── app/admin/                 # Admin pages (layout, dashboard, CRUD)
│       ├── components/                # Reusable UI components
│       ├── domains/                   # Domain type definitions
│       ├── models/                    # API request/response models
│       ├── schemas/                   # Zod validation schemas
│       ├── auth/                      # Auth context + Supabase client
│       ├── config/                    # Runtime config, API utilities
│       └── infrastructure/            # GraphQL client, auth utilities
├── docs/superpowers/                  # Design docs + implementation plans
│   ├── specs/                         # Feature specifications
│   └── plans/                         # Implementation plans
├── docker-compose.yml                 # Local dev stack (Supabase + API + Frontend + Jaeger)
└── AGENTS.md                          # This file — SDLC workflow + conventions
```

---

## Rust Backend Conventions

### Architecture: Strictly Layered

```
API Layer (services/src/api/)        — HTTP routing, serialization, auth extraction
        │
        ▼
Application Core (application_core/) — Business logic, command handlers
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
- **Never manually edit** entity files in `application_core/src/entities/`
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
pnpm build                  # verify frontend builds (in frontend/)
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
| Infra | Docker Compose (local), Helm (prod) |
