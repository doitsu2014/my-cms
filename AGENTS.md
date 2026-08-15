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
| Implementation & code execution  | **Project agents** | `software-engineer` (Codex) / `coder` (OpenCode) drive TDD, subagent dispatch, code review, and the verification gate |

> **OpenSpec** owns *what* and *why*. **Project agents** (`software-engineer` / `coder`) own *how* (the actual coding).

## Prompt Routing & Agent Dispatch

The primary agent (Codex or OpenCode) is the coordinator and must classify every incoming prompt, decide which agent owns the work, and dispatch with enough context for the subagent to act without inventing scope. This section defines the routing rules so the team can apply them consistently.

### 1. Classify the dominant intent

Detect the dominant intent behind the prompt first, then pick the primary agent and any secondary collaborators.

| Intent                          | Primary agent                                       | Secondary collaborators                                         | Cue words / signals |
|---------------------------------|-----------------------------------------------------|-----------------------------------------------------------------|---------------------|
| Explore / clarify               | `product-owner`                                     | `software-architect` (feasibility), `product-designer` (UX)     | "explore", "what should we", "do we need", "should we", "investigate" |
| Propose / kick off a change     | `product-owner`                                     | —                                                               | "propose", "draft a proposal", "kick off", "let's change", "add feature" |
| Design UX / design language     | `product-designer`                                  | `product-owner` (scope clarifier)                               | "design UX", "responsive", "screen for", "design tokens", "accessibility", "extract our design language" |
| Map / design API                | `software-architect`                                | —                                                               | "map API", "design API", "architecture for", "API contract", "endpoint shape" |
| Specs / design / tasks          | `software-architect`                                | `product-designer` (integrate UX brief)                         | "write specs", "draft design", "break down tasks", "openspec-continue", "fast-forward" |
| Implement / execute change      | `software-engineer` (Codex) or `coder` (OpenCode)   | —                                                               | "implement", "build", "code", "fix bug", "apply change", "execute tasks" |
| Release / deploy                | `release-engineer`                                  | `software-engineer` (implementation and operational-impact evidence) | "release", "deploy", "rollout", "rollback", "post-deploy", "deployment readiness" |
| Verify / archive                | `software-engineer` / `coder`                       | `product-owner` (final sign-off)                                | "verify", "archive", "sync specs", "finalize", "wrap up" |
| Fast fix / hot-patch            | `coder` (Fast Fix mode, only when eligible)         | —                                                               | "fast fix", "fast implement", "hotfix", "typo", "config tweak", single-file change |
| Debug / investigate failure     | `coder` (OpenCode) or `software-engineer` (Codex)   | —                                                               | "debug", "why is", "reproduce", "investigate test failure" |
| Question / explain              | Primary agent answers directly (no dispatch)        | —                                                               | "what is", "how does", "explain", "where is", "what does X mean" |
| Update this file / docs         | Primary agent edits directly                        | —                                                               | "update AGENTS.md", "rewrite docs", "document this" |

When the prompt mixes intents (e.g. "design and implement"), split it into ordered tasks and let the primary agent run them in sequence. Do not bundle multiple artifacts under a single dispatch.

### 2. Apply routing rules

1. **Skill-driven default.** If the user explicitly names an OpenSpec skill (`openspec-explore`, `openspec-propose`, `openspec-continue`, etc.), honor the skill's intent before re-classifying. The skill is the routing contract.
2. **Phase-aware override.** When a change is mid-flight in `openspec/changes/<name>/`, use the artifact state (proposal / specs / design / tasks) to pick the next owner rather than the user's literal wording. For example, a prompt like "implement X" during an unarchived change routes to whichever agent owns the next unchecked artifact.
3. **Single primary owner.** Pick exactly one primary agent per dispatch; secondary collaborators join on demand, never co-own the artifact.
4. **Serialise writes.** If the dispatch touches a shared artifact (`design.md`, `tasks.md`, an OpenSpec file), the primary agent owns the write; collaborators contribute content but do not edit concurrently.
5. **Ambiguity → ask.** If intent is unclear or two intents compete, ask one concise question before dispatching. Never guess between ambiguous changes.
6. **Out-of-scope redirect.** If a subagent discovers the request belongs to another role, it must return the issue to the owning agent rather than silently expanding scope.

### Fast Fix eligibility

Use Fast Fix / Fast Implement only when **every** condition holds: the change
touches one non-generated repository file; changes at most 40 non-generated
lines; preserves observable product and operator behavior; does not affect
authentication, authorization, API contracts, schema, migrations,
dependencies, deployment manifests, secrets, generated files, or active
OpenSpec artifacts; and has a focused verification command. If any condition
fails or scope is uncertain, route the work through the normal intent and
OpenSpec workflow; do not bypass the required owner, review, or verification
gate.

### 3. Dispatch contract

Every dispatched subagent receives four mandatory fields plus an optional skills bundle:

- **Goal** — what the subagent is asked to deliver, in one sentence.
- **Context** — relevant artifacts (proposal/specs/design/tasks), commands already run, and decisions made upstream.
- **Constraints** — scope boundaries, must-not-touch files, non-goals, verification command expectations.
- **Done when** — observable acceptance criteria and the verification command(s) the subagent must run before reporting back.
- **Skills** *(optional)* — the OpenSpec skill(s) to load (e.g. `openspec-propose`, `openspec-apply-change`) or graph queries to run.

### 4. Worked examples

| Prompt                                                | Detected intent | Primary agent                                       | Secondary                                |
|-------------------------------------------------------|-----------------|-----------------------------------------------------|------------------------------------------|
| "Should we add dark mode?"                            | Explore         | `product-owner`                                     | `software-architect` (feasibility)       |
| "Propose a redesign of the post editor"               | Propose         | `product-owner`                                     | `product-designer`, `software-architect` |
| "Design UX for change-X"                              | Design UX       | `product-designer`                                  | `software-architect` (integrate)         |
| "Map API architecture for media uploads"              | Map API         | `software-architect`                                | —                                        |
| "Write specs and tasks for change-X"                  | Specs/tasks     | `software-architect`                                | `product-designer` (UX brief)            |
| "Implement change rename-legacy-foo"                  | Implement       | `software-engineer` (Codex) / `coder` (OpenCode)     | —                                        |
| "Fast fix typo on the about page"                     | Fast fix        | `coder` (Fast Fix)                                  | —                                        |
| "Why is auth middleware failing tests?"               | Debug           | `coder` / `software-engineer`                       | —                                        |
| "Verify and archive change-X"                         | Verify/archive  | `software-engineer` / `coder`                       | `product-owner` (sign-off)               |
| "Where do we register domain routes?"                 | Question        | Primary agent (no dispatch)                         | —                                        |

> The "Key Commands / Workflow" section below is a compact alias of this routing table. Keep both in sync when adding new intents.

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
                                                            │ Project agents:      │
                                                            │  release-engineer    │
                                                            │  + branch wrap-up    │
                                                            └──────────────────────┘
```

## Team Orchestration Contract

The primary agent remains the coordinator and owns the final synthesis. For every delegated task, provide **Goal**, **Context**, **Constraints**, and **Done when** so the subagent can work without inventing scope.

- Use the named agents for their narrow roles; do not send implementation work to PO/PD/SA, product decisions to SE, or operational release work to SE. The SE supplies implementation verification and operational-impact evidence to the Release Engineer, which owns the release finding and handoff.
- For independent read-only discovery, code-path mapping, test-gap review, and risk analysis, the coordinator may use up to four workers. Record an `update_plan` checkpoint before dispatch and another after synthesizing their results. Serialize all shared edits.
- Enforce a single writer for every artifact, configuration, and source file at a time. In Phase 2 the default sequence is PO proposal → PD UX/design brief → SA specs/design/tasks.
- Give each delegated agent a bounded question and expected output. Wait for all requested results, then return a distilled synthesis rather than raw logs.
- Preserve traceability across handoffs: product outcome → UX behavior → requirement/scenario → architecture decision → task → test/verification.
- Every handoff reports: goal, evidence inspected, decisions, artifacts changed, verification, assumptions/risks, open questions, and next owner.
- If an agent discovers a decision outside its authority, it returns the issue to the owning role instead of silently expanding scope.

## Phase Details

### Phase 1: Explore Requirements
**Agents:** `product-owner` (requirements & user intent) + `product-designer` (UX & visual direction) + `software-architect` (technical & architecture feasibility)
**Primary skills:** `openspec-explore` + `map-my-cms-api-architecture` for API work

- Enter explore mode and investigate the problem space
- For up to four independent read-only exploration questions, record an `update_plan` checkpoint before dispatch; synthesize the findings, record a second checkpoint, and only then assign a single writer for any resulting artifact, configuration, or source edit.
- Read the codebase, map integration points, surface hidden complexity
- Check `openspec list --json` for any active change that may be relevant
- Optionally capture unstructured ideas in conversation before formalizing them as proposal scope
- **`product-owner`** focuses on *what* the user needs — requirements, user stories, scope, success criteria, impact
- **`product-designer`** focuses on *how the product should feel and work* — responsive information architecture, interaction flows, accessibility, visual language, and reusable frontend patterns
- **`software-architect`** focuses on *how feasible it is* — current gateway/domain architecture, affected layers, library & framework fit, perf/security/data-model implications, alternative approaches
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

### Phase 3: Implement (Agent-driven)
**Agent:** `software-engineer` / SE in Codex (primary) or `coder` in OpenCode
**How it works:** The executing agent walks `tasks.md`, dispatches subagents for independent work, applies TDD for behavioral changes, requests code review between task groups, and runs the full verification gate before marking anything done.

- Read the OpenSpec change artifacts from `openspec/changes/<name>/` (proposal, specs, design, **tasks.md**)
- Walk through `tasks.md` checkboxes step by step
- For independent tasks, dispatch subagents in parallel
- Follow RED-GREEN-REFACTOR for every behavioral change (TDD)
- Request a code review between task groups
- For a change affecting `deployments/`, runtime configuration, or an operational contract, the SE prepares implementation verification and operational-impact evidence for the Release Engineer. The Release Engineer owns deployment readiness, rollout and rollback planning, post-deploy verification, and release handoff.
- The Release Engineer requires explicit user or release authorization before environment-changing production commands. Without it, it reports the release-ready plan, rollback trigger, and next approving owner; it never represents the change as production-deployed.
- Before claiming done, run the full verification gate:
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

> **Note:** The OpenSpec `openspec-apply-change` skill is available as a fallback if you want OpenSpec to drive task execution. By default, the `software-engineer` / `coder` project agent drives the coding loop directly.

### Phase 4: Verify & Archive
**Agents:** `software-engineer` / `coder` (verify + sync) → `release-engineer` (operational release readiness and handoff) → `product-owner` (final archive approval)
**Primary skills (OpenSpec):** `openspec-verify-change` → `openspec-sync-specs` → `openspec-archive-change`
**Plus (project agent):** branch wrap-up (merge / PR / keep / discard)

1. **Implementation handoff** — The SE supplies implementation verification and operational-impact evidence to the Release Engineer for every approved change.
2. **Operational readiness** — The Release Engineer records either deployment readiness, rollout/rollback, post-deploy verification, and release handoff for an operationally affected change, or that no deployment action is required. Environment-changing production commands require explicit user or release authorization.
3. **Verify** — Run `openspec-verify-change <name>` to check Completeness (tasks, spec coverage), Correctness (requirement ↔ implementation mapping), and Coherence (design adherence, pattern consistency). Fix all `CRITICAL` issues; review `WARNING` issues.
4. **Sync specs** — Run `openspec-sync-specs <name>` to merge delta specs from `openspec/changes/<name>/specs/` into the canonical `openspec/specs/<capability>/spec.md`. This is agent-driven and idempotent.
5. **Archive** — Run `openspec-archive-change <name>`. The change moves to `openspec/changes/archive/YYYY-MM-DD-<name>/` and becomes part of the project's decision history.
6. **Wrap up branch** — The executing agent presents options: merge, PR, keep, or discard. Never force-push; respect protected branches.
7. **Final verification** — Run the full verification gate once more on the merged result.

## Agent Quick Reference

> Routing rules live in **Prompt Routing & Agent Dispatch** (above). This table is the per-agent skill/output summary that the routing rules refer to.

| Agent                | Phase      | Mode(s)      | Primary skills                                                              | Primary outputs                                                                        |
|----------------------|------------|--------------|-----------------------------------------------------------------------------|---------------------------------------------------------------------------------------|
| `product-owner`      | 1, 2, 4    | OpenCode agent or Codex project agent (`.codex/agents/product-owner.toml`) | `openspec-explore`, `openspec-propose`, `openspec-new-change` | Explored result + **`proposal.md`** (Why, What Changes, Capabilities, Impact) — final sign-off |
| `product-designer`   | 1, 2       | Codex project agent (`.codex/agents/product-designer.toml`) | Responsive UX, information architecture, design language, accessibility, UI component guidance | Screen specifications, responsive behavior, interaction states, design tokens, and implementation-ready design guidance |
| `software-architect` | 1, 2        | Always OpenSpec | `map-my-cms-api-architecture`, `design-my-cms-api-change`, `openspec-new-change`, `openspec-continue-change`, `openspec-ff-change` | Source-backed architecture map, **`specs/<capability>/spec.md`**, **`design.md`**, and **`tasks.md`** |
| `coder`              | 3, 4        | **Normal** + **Fast Fix/Fast Implement** (see below) | Normal → walk `tasks.md`, dispatch subagents in parallel, apply TDD, request code review between task groups, run the full verification gate, wrap up branch · Fast Fix → run the full verification gate, apply TDD only if the change is behavioral | Implementation, tests, verification, branch wrap-up; Normal mode also drives `openspec-verify-change` → `openspec-sync-specs` → `openspec-archive-change` |
| `software-engineer` | 3, 4        | Codex project agent (`.codex/agents/software-engineer.toml`) | Walk `tasks.md`, dispatch subagents in parallel, apply TDD, code-review-graph, run the full verification gate, prepare operational-impact evidence | Implementation, tests, graph impact review, operational-impact evidence for Release Engineer, verification, branch wrap-up |
| `release-engineer` | 3, 4        | Codex project agent (`.codex/agents/release-engineer.toml`) | Assess deployment readiness, plan rollout/rollback and post-deploy verification, enforce authorization boundary | Release-ready plan or no deployment action finding, release handoff, and post-deploy verification result when authorized |

### Codex Agent Team Definition

Codex loads the project-scoped team from `.codex/agents/` when the repository is trusted. Use `product-owner` (PO) for exploration and proposals, `product-designer` (PD) for responsive UX and design language, `software-architect` (SA) for proposal/spec/design/task review, `software-engineer` (SE) for implementation, and `release-engineer` (RE) for operational release work. PD, SA, and SE are configured with the `code-review-graph` MCP server in their agent files. PD uses it only for read-only discovery; the graph gates above are mandatory for SA and SE.

Codex loads the repository OpenSpec workflow from `.agents/skills/openspec/SKILL.md`. It is the Codex-native equivalent of the `.opencode/skills/openspec-*` skills and maps multi-step tracking to `update_plan` while preserving the same OpenSpec CLI lifecycle.

For API architecture work, the SA loads `.agents/skills/map-my-cms-api-architecture/SKILL.md` to reconstruct current behavior from `apps/api`, then `.agents/skills/design-my-cms-api-change/SKILL.md` to turn that evidence into contracts, decisions, migrations, tasks, and verification. The source-derived references are navigation baselines and must be revalidated against current code.

### Coder modes

- **Normal** — default when an active OpenSpec change has `tasks.md` ready. Read `openspec/changes/<name>/`, execute with TDD, request code review, run the verification gate, finish.
- **Fast Fix / Fast Implement** — permitted only when all Fast Fix eligibility conditions above hold. It is never a shortcut for a behavior, auth, API, schema, migration, dependency, deployment, secret, generated-file, or active OpenSpec change. Run and report focused verification. Any failed or uncertain condition escalates to the normal intent and OpenSpec workflow.

## Key Commands / Workflow

> Compact alias of the **Prompt Routing & Agent Dispatch** table (above). The full table includes cue words, secondary collaborators, and dispatch contracts; this block is the cheat-sheet version.

```
"Let's explore <feature>"          → product-owner uses openspec-explore
"Propose <feature>"                → product-owner uses openspec-propose
"Design UX for <change>"           → product-designer audits UI and produces a responsive design brief
"Extract our design language"      → product-designer derives tokens, patterns, states, and usage rules
"Map API architecture for X"       → software-architect uses map-my-cms-api-architecture
"Design API change X"              → software-architect maps current source, then uses design-my-cms-api-change
"Write specs/design/tasks for X"   → software-architect uses openspec-continue
"Implement <change-name>"          → software-engineer in Codex or coder in OpenCode executes tasks.md
"Release <change-name>"            → release-engineer assesses readiness and plans rollout/rollback; explicit authorization is required before production commands
"Verify and archive <change>"      → software-engineer/coder runs verify → sync → archive
"Fast fix <request>"               → coder only if every Fast Fix eligibility condition holds; otherwise normal OpenSpec workflow
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

OpenSpec owns the spec & decision artifacts. Project agents implement but do not own any document.

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
software-engineer / coder executes     ──▶  │  tasks.md checkboxes ticked off
                                            │
openspec-verify-change                   ──▶  │  Completeness + Correctness + Coherence report
openspec-sync-specs                      ──▶  │  delta specs → openspec/specs/<capability>/
openspec-archive-change                  ──▶  openspec/changes/archive/YYYY-MM-DD-<name>/
```

> **Legacy `docs/superpowers/`** holds historical artifacts from an earlier workflow. New work uses `openspec/` only. Do not add new files under `docs/superpowers/`.

---

## Project Structure

```
my-cms/
├── apps/
│   ├── api/                           # Rust backend
│   │   ├── gateway/                    # API composition root and shared runtime wiring
│   │   ├── domain_*/                   # Current domain-owned services, adapters, handlers, entities, and migrations
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
Gateway composition root (apps/api/gateway/) — HTTP composition, runtime wiring, and cross-domain entry points
        │
        ▼
Domain crates (apps/api/domain_*/)    — domain-owned services, adapters, handlers, entities, and migrations
        │
        ▼
Persistence within each domain        — SeaORM entities, repositories, and schema-first migrations
```

**Rule:** Keep business logic in domain command handlers. Gateway handlers extract requests, delegate to the owning domain, and serialize responses.

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
- **Never manually edit** generated SeaORM entity files in a domain crate
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
| SDLC Skills | OpenSpec (spec, design, task lifecycle) + project agents (`software-engineer` / `coder`: TDD, subagent dispatch, code review, verification gate, branch wrap-up) |
| Infra | Docker Compose (local), Helm (prod) |
