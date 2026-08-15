## Why

The documented My-CMS agent workflow no longer matches the domain-oriented API
layout and leaves routine operating decisions implicit. This creates avoidable
handoff errors: contributors can navigate to retired paths, classify a
substantive change as a fast fix, or finish a change without a named deploy
and rollback owner.

## What Changes

- Align the repository architecture guidance with the current `apps/api/domain_*`
  services and `apps/api/gateway` composition root.
- Define a bounded Fast Fix eligibility rule and its escalation path to the
  normal OpenSpec workflow.
- Give the Product Designer read-only code-review-graph access and require
  evidence-based graph use only for cross-cutting design-system work.
- Add a dedicated Release Engineer role that owns deployment readiness,
  rollout/rollback plans, post-deploy verification, and release handoff within
  the existing Phase 3/4 workflow.
- Increase read-only exploration capacity to four concurrent workers with
  explicit checkpoints and one-writer safeguards.

## Capabilities

### New Capabilities

- `agent-workflow-governance`: Defines accurate, bounded, evidence-driven,
  and operationally owned agent-workflow behavior.

### Modified Capabilities

- None.

## Impact

- Affected governance/configuration: `AGENTS.md`, `.codex/config.toml`, and
  `.codex/agents/product-designer.toml`, `.codex/agents/software-engineer.toml`,
  and new `.codex/agents/release-engineer.toml`.
- No product API, database schema, generated SeaORM entity, frontend runtime,
  external dependency, or deployment manifest changes are introduced.
- Existing active OpenSpec changes remain untouched; their future execution
  inherits the clarified workflow after this change is implemented.
