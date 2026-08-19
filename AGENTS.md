# My-CMS Agent Guide

Headless CMS: Rust/Axum/SeaORM API, React/DaisyUI/TipTap web apps, and
Supabase PostgreSQL, pgvector, and Storage.

## Operating rules

- **OpenSpec owns what and why; project agents own implementation.** Use
  `openspec/` only for new spec and decision artifacts; never add them under
  `docs/superpowers/`.
- Check `openspec list --json` before creating a change. Reuse a matching
  active change; do not guess when the change name or scope is ambiguous.
- Pick one primary owner per handoff. Shared artifacts and source files have
  one writer at a time. A contributor returns guidance instead of editing a
  file another agent owns.
- Every handoff states **Goal**, **Context**, **Constraints**, **Done when**,
  relevant skills, evidence inspected, decisions, risks, and the next owner.
- Preserve unrelated working-tree changes. Do not use destructive Git commands
  or modify generated SeaORM entities manually.
- Use `update_plan` for multi-artifact work or multiple task groups. Independent
  read-only discovery may be parallelized; edits remain serialized.

## Routing

| Intent | Primary owner | Collaborator / next owner |
|---|---|---|
| Explore, clarify, or propose a product change | Product Owner (PO) | Software Architect (SA) for feasibility; Product Designer (PD) for UX when relevant |
| UX, responsive design, accessibility, design language | PD | PO for scope; SA integrates approved guidance |
| API mapping, technical design, specs, or tasks | SA | PD supplies UX guidance where relevant |
| Implement an approved change or debug a non-trivial failure | Software Engineer (SE) / OpenCode `coder` | Release Engineer (RE) only for operational readiness |
| Release or deployment readiness | RE | SE supplies verification and operational-impact evidence |
| Verify, sync, or archive | SE / `coder` | PO provides final archive approval |
| Documentation-only update | Coordinator | No dispatch required |

Explicit OpenSpec skill requests take precedence. When a change already exists,
its artifact status determines the next owner.

## Workflow commands

These are orchestration prompt aliases; they do not skip OpenSpec or authorize
deployment.

| Command | Required input | Ordered workflow | Done when |
|---|---|---|---|
| `/prompts:explore-feature <outcome or problem>` | User need, affected users, constraints | PO explores outcome, scope, non-goals, and acceptance outcomes. SA maps relevant source/specs and returns feasibility, flows, risks, and plan guidance. PO writes `proposal.md`; SA writes delta specs, `design.md`, and `tasks.md`. | All `applyRequires` artifacts are done; no product code changed. |
| `/prompts:hotfix <symptom, impact, reproduction>` | Failure, impact, urgency, and evidence | SA confirms the failing flow and writes a focused proposal, specs, design, and task plan. SE begins only after the change is ready, repairs test-first, and checks off verified tasks. | Regression coverage (or a documented exception), focused verification, and implementation handoff evidence. |
| `/prompts:implement-feature <change-name>` | Exact active change name | SE confirms the plan is ready. PD reviews the plan and current UI, then returns responsive/component/state/accessibility guidance. SE implements approved tasks and returns material plan conflicts to PO/SA. | Tasks are checked only after verification; SE reports tests, graph review/fallback, build/lint, and operational impact. |

`/hotfix` is never a Fast Fix. Use Fast Fix only if **all** conditions hold:
one non-generated file, at most 40 non-generated lines, no observable behavior
or operator change, no auth/API/schema/migration/dependency/deployment/secret/
generated/OpenSpec impact, and a focused verification command. Otherwise use
the normal OpenSpec workflow.

## OpenSpec lifecycle

1. Explore without writing application code. Inspect canonical specs, active
   changes, relevant source, tests, and API flows.
2. Create or select `openspec/changes/<name>/`.
3. Create artifacts in order, with one writer each:
   - PO writes `proposal.md` (Why, What Changes, Capabilities, Impact).
   - SA writes `specs/<capability>/spec.md` using testable
     `### Requirement` and WHEN/THEN/AND scenarios.
   - SA writes `design.md` (context, decisions, contracts, risks, rollout).
   - SA writes `tasks.md` as numbered, independently verifiable checkboxes.
   - For `/hotfix` only, SA may write the focused proposal when incident
     evidence establishes the immediate outcome; return material product or UX
     decisions to PO.
4. Run `openspec status --change "<name>" --json`; implement only when all
   `applyRequires` artifacts are done.
5. SE reads every artifact, uses RED → GREEN → REFACTOR for behavior changes,
   runs targeted checks, reviews the diff, then marks each passing task `[x]`.
6. On requested finalization: `openspec verify` → `openspec sync` → PO archive
   approval → `openspec archive`. RE handles operational readiness; production
   changes require explicit user or release authorization.

### Graph review gates

SA and SE call `get_minimal_context(task="<change>")` before drafting or
editing. Inspect callers, callees, imports, flows, and tests. After every SE
task group run `detect_changes`, `get_affected_flows`, `tests_for` for
high-risk functions, and `get_impact_radius`. If the graph is unavailable,
record that and use targeted search, `git diff`, and tests instead.

## Role boundaries

| Role | Owns | Does not own |
|---|---|---|
| PO | User value, scope, acceptance outcomes, product proposal | Architecture or production code |
| PD | UI flows, responsive behavior, components, states, accessibility, design guidance | Product scope, API design, production code |
| SA | Specs, technical design, task breakdown, API/data/security/operational decisions | Product priority or implementation |
| SE / `coder` | Approved task implementation, TDD, verification, impact evidence | Scope/design changes, release decisions |
| RE | Deployment readiness, rollout/rollback, post-deploy checks | Product code, manifests, or unapproved deployment |

For `/implement-feature`, PD returns an implementation-time brief and does not
silently change the plan. SE stops for a material conflict rather than inventing
scope. For changes affecting runtime configuration, deployment, or operations,
SE hands evidence to RE; RE never runs production commands without approval.

## Engineering conventions

### Rust API

- Gateway (`apps/api/gateway/`) composes HTTP/runtime concerns; domain crates
  (`apps/api/domain_*/`) own handlers, services, adapters, entities, and
  migrations. Keep business logic in domain command handlers.
- Return `Result<T, AppError>` and propagate with `?`; no production
  `unwrap()` or `expect()`.
- Follow schema-first SeaORM: migration → run it → generate entities. Do not
  hand-edit generated entities. Use transactions for atomic writes.
- Use `#[instrument]` on important paths; use `info!`, `warn!`, and `error!`
  at appropriate levels. Do not block inside Tokio async work.
- Prefer SeaORM `MockDatabase` unit tests and `testcontainers` PostgreSQL
  integration tests.

### React web

- Pages in `apps/web/src/app/admin/` own fetching/routing; reusable
  presentational components live in `src/components/`.
- Use React Hook Form with Zod for forms, `useAuth()` for auth, Apollo for
  GraphQL, and `authenticatedFetch(getApiUrl(...))` for REST.
- Follow existing DaisyUI/Tailwind, Lucide, and Sonner patterns. Cover loading,
  empty, validation, error, destructive, keyboard, focus, and responsive states.

## Verification

Run the relevant targeted tests during work. Before claiming an implementation
complete, run the repository gate and report any pre-existing failure separately:

```bash
cargo check
cargo test
cargo fmt -- --check
cargo clippy
pnpm --dir apps/web build
```

## OpenSpec CLI

```bash
openspec list --json
openspec new change "<kebab-case-name>"
openspec status --change "<name>" --json
openspec instructions <artifact> --change "<name>" --json
openspec verify --change "<name>"
openspec sync --change "<name>"
openspec archive "<name>"
```
