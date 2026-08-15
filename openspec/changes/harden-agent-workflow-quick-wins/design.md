## Context

This is a governance-only change. Repository evidence shows that the current
API workspace contains `apps/api/domain_auth`, `domain_interface`,
`domain_media`, `domain_posts`, `domain_user`, and `gateway`, while
`AGENTS.md` still presents retired `apps/api/src`, `application_core`, and
`migration` directories as current. The active
`wire-all-domains-and-collapse-to-gateway-binary` change also treats the
gateway as the composition root.

The current `.codex/config.toml` caps concurrent threads at two. The Product
Designer agent configuration has no graph-server entry; the Software Architect
and Software Engineer configurations each do. `AGENTS.md` names Fast Fix but
only describes it as a small change, and assigns no explicit deployment
lifecycle owner.

Graph evidence: `get_minimal_context(task="propose-agent-workflow-quick-wins")`
reported medium repository risk driven by unrelated editor test gaps. A focused
review context for `AGENTS.md`, `.codex/config.toml`, and the PD/SE agent
configurations found zero changed or impacted code nodes and no impacted flows.
This confirms that the change is confined to agent orchestration and does not
alter runtime callers, imports, or API flows.

## Goals / Non-Goals

**Goals:**

- Make repository navigation guidance match the active API architecture.
- Make Fast Fix eligibility deterministic and safely escalatable.
- Enable evidence-backed Product Designer discovery without granting a
  graph-writing workflow.
- Make rollout and rollback accountability explicit without creating a new
  release role.
- Allow faster independent discovery while retaining planning checkpoints and
  single-writer protection.

**Non-Goals:**

- Modify product behavior, API contracts, Rust code, database schema, SeaORM
  entities, frontend runtime code, deployment manifests, or external services.
- Add a new security-reviewer, QA, release-engineer, observability owner, or
  third-party plugin.
- Authorize or perform a production deployment.
- Change OpenSpec artifact ownership or the PO → PD → SA → SE phase sequence.

## Decisions

### 1. Treat agent workflow as one new governance capability

Create the `agent-workflow-governance` delta capability rather than modifying
an API or website capability. The affected contracts describe how project
agents classify work, access repository evidence, and hand off releases; no
existing canonical capability owns those behaviors.

Alternatives considered:

- Amend `domain-api-cutover`: rejected because it owns runtime gateway behavior,
  not contributor workflow.
- Make undocumented one-off edits: rejected because the five outcomes are
  behaviorally coupled and need traceable implementation verification.

### 2. Use the current domain/gateway layout as the only current API map

Update `AGENTS.md` project structure and layered-architecture guidance to
identify `gateway` as the composition root and `domain_*` crates as owned
surfaces. Retired paths may appear only as clearly marked historical context.
Do not duplicate domain internals exhaustively; link responsibility by layer so
the document remains accurate as individual domains evolve.

Alternative considered: retain the legacy tree as a conceptual three-layer
diagram. Rejected because literal paths in the current guide are actionable
navigation instructions and presently point agents to absent locations.

### 3. Adopt an all-conditions Fast Fix guardrail

Add an explicit conjunction of eligibility conditions: one non-generated file,
at most 40 non-generated changed lines, no observable behavior or operator
behavior change, no sensitive or operational surface, no dependency/generated
file/active-OpenSpec change, and a focused verification command. A failed or
uncertain condition routes the request through the normal classifier and its
owning workflow.

Alternative considered: a discretionary "small change" rule. Rejected because
scope estimates vary between agents and offer no auditable reason to bypass
OpenSpec and the normal review gate.

### 4. Give PD a project-local graph server with instruction-enforced read-only use

Add the existing `code-review-graph` MCP server definition to
`.codex/agents/product-designer.toml`. Add PD instructions limiting graph use
to discovery and explicitly forbidding graph mutation, indexing, embeddings,
or wiki generation. For cross-cutting UI/design-system work, require the same
`get_minimal_context` first-call pattern used by SA/SE, then report the graph
evidence in the design brief.

The existing project configuration does not expose a per-agent MCP allowlist
or server-side read-only mode. Therefore the restriction is enforced through
agent instructions and reviewable configuration, not a transport-level access
control guarantee.

Alternative considered: omit PD graph access and rely on source search. Rejected
because graph-backed UI consumer and flow discovery helps prevent one-off token
or component recommendations. A separate read-only proxy was rejected as
unnecessary infrastructure for this quick win.

### 5. Fold deploy ownership into the Software Engineer role

Update `AGENTS.md` and `.codex/agents/software-engineer.toml` so SE owns
deployment readiness, rollout/rollback documentation, post-deploy verification
planning, and release handoff when operational surfaces change. SE may execute
a deployment only with explicit user or release authorization; otherwise it
reports readiness and the next owner. No release-engineer role is added.

Alternative considered: introduce a release-engineer agent. Rejected because
the repository currently has no dedicated release workflow or external release
control integration, and the responsibility fits Phase 3/4 SE execution.

### 6. Set four available threads, restrict four-way use to exploration

Set `.codex/config.toml` `max_concurrent_threads_per_session` to `4`. Update
`AGENTS.md` to permit up to four workers only for independent, read-only
exploration and analysis. Require an `update_plan` checkpoint before dispatch
and after synthesis; keep all artifact, configuration, and source edits under
one writer.

Alternative considered: retain the global two-thread cap. Rejected because it
serializes independent code-path, test-gap, risk, and UI-pattern discovery.
Allowing four unrestricted writers was rejected because it conflicts with the
project's shared-worktree and single-writer safeguards.

## Implementation Boundaries and Contracts

| Surface | Contract | Owner |
|---|---|---|
| `AGENTS.md` | Current architecture map, Fast Fix gate, exploration checkpoints, deployment role | SE implements; SA-provided tasks/specs govern |
| `.codex/config.toml` | Maximum of four available concurrent threads | SE implements |
| `.codex/agents/product-designer.toml` | Read-only graph discovery + evidence handoff | SE implements |
| `.codex/agents/software-engineer.toml` | Release readiness, rollout/rollback, and authorization boundary | SE implements |

There is no HTTP, data, database, storage, AI, authentication, or frontend
contract change. No migration or SeaORM entity generation is required.

## Security and Operations

- The PD graph server command matches the existing project-local graph server;
  it introduces no new endpoint, credential, or third-party dependency.
- PD instructions MUST prohibit mutation-oriented graph tools and avoid placing
  sensitive repository values in a design brief.
- Deployment ownership is procedural, not deployment authority. Explicit
  approval remains required before any environment-changing command.
- Documentation/configuration rollout takes effect when the repository revision
  is used in a trusted Codex session; no application restart or data backfill is
  required.

## Migration Plan

1. Update the four governance/configuration surfaces in one reviewed change.
2. Validate TOML syntax and static contracts with focused searches and diff
   checks.
3. Validate the OpenSpec change and hand it to SE for implementation.
4. After merge, new sessions load the adjusted agent definitions; active agents
   continue under their already-loaded instructions until restarted.

**Rollback:** revert only the four governance/configuration files if the agent
runtime rejects the TOML or the four-thread setting causes coordination issues.
No runtime traffic, data, or schema rollback is required.

## Risks / Trade-offs

- [Instruction-enforced PD read-only rule] → Review the agent config for the
  explicit prohibited operations; do not represent it as server-enforced RBAC.
- [Four available threads can increase coordination pressure] → Limit four-way
  work to independent reads, mandate plan checkpoints, and preserve one writer
  per target.
- [Fast Fix may delay genuinely urgent behavioral fixes] → An ineligible change
  is escalated, not blocked; the normal owner can still prioritize an urgent
  OpenSpec change.
- [SE deploy ownership may be mistaken for deployment authority] → State the
  explicit authorization prerequisite in both workflow and SE instructions.
- [Architecture guide can drift again] → Use focused path searches in the
  implementation verification and update it whenever a future API topology
  change lands.
- [OpenSpec CLI command drift] → The installed CLI exposes `validate`, not the
  `verify` command currently documented elsewhere; this change's tasks use the
  supported strict `validate` command. Updating the broader CLI reference is
  intentionally deferred to avoid expanding this five-item quick-win scope.

## Verification Strategy and Traceability

| Proposal outcome | Requirement | Implementation task group | Verification |
|---|---|---|---|
| Accurate API layout | Current architecture guidance | 1 | `rg` rejects unlabelled retired paths and confirms domain/gateway paths |
| Safe shortcut | Bounded Fast Fix classification | 1 | focused Fast Fix eligibility search/review |
| PD evidence access | Evidence-bound Product Designer graph access | 2 | TOML parse plus config/instruction searches |
| Named operational owner | Deployment lifecycle ownership | 3 | role and authorization searches |
| Faster discovery | Controlled exploration concurrency | 4 | TOML parse and checkpoint/single-writer searches |

The installed OpenSpec CLI validates changes with
`openspec validate "harden-agent-workflow-quick-wins" --type change --strict --json`;
the final readiness check is
`openspec status --change "harden-agent-workflow-quick-wins" --json`.

## Open Questions

None. The scope deliberately excludes server-enforced read-only graph RBAC and
an independent release-engineer role; either can be proposed later if evidence
shows instruction-level controls or SE ownership are insufficient.
