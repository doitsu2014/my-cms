# CI/CD and Verification — u1-backup-foundation

## Scope

This CI/test guidance follows the resource/verification budgets in [`performance-design.md`](../nfr-design/performance-design.md), security gates in [`security-design.md`](../nfr-design/security-design.md), bounded test topology in [`scalability-design.md`](../nfr-design/scalability-design.md), failure evidence in [`reliability-design.md`](../nfr-design/reliability-design.md), safe event requirements in [`observability-design.md`](../nfr-design/observability-design.md), and component boundaries in [`logical-components.md`](../nfr-design/logical-components.md), building on [`components.md`](../../../inception/domain-design/components.md) and [`functional-spec.md`](../../functional-design/functional-spec.md).

No deployment/release pipeline is created for U1: the approved scope is localhost-only with no release step. The existing API workflow `.github/workflows/ci-my-cms.yml` declares tests and coverage but is labeled “TEMPORARY DISABLED”; do not represent it as an active gate and do not re-enable or alter it as part of this design without an explicit request/approval.

## Local verification flow

1. Inspect current scripts and tool availability before selecting the human-authorized Construction Verification Command.
2. Run layer tests after each layer (team `test-after` posture), and the full Unit verification at its checkpoint.
3. Include test and coverage evidence for the 80% line-coverage target; if the current coverage mechanism cannot run locally, report the exact limitation and do not claim coverage or CI green.
4. Keep the command selection as a separate human decision at the checkpoint. This document does not choose or authorize a command.

## Pipeline stages / gates

| Stage | Gate | Current status |
|---|---|---|
| Lint/static analysis | Existing project config, if part of approved verification | To verify against current scripts. |
| Build/API compile | Existing Rust workspace scripts | Documented local commands exist; not run/verified by this design stage. |
| Unit/integration tests | Existing Rust and web tests | Documented in README, but actual relevant coverage for U1 must be checked during build. |
| Coverage | Target 80% per team/scope posture | Workflow recipe exists but CI workflow is temporarily disabled; active enforcement not established. |
| Deploy/release | None for U1 | Out of scope: localhost-only; no promotion/deployment action. |

## Promotion and rollback

No staging/production promotion is defined for this Unit. A future request to move beyond localhost requires a separate deployment decision, active CI/coverage gate, TLS/secret/storage design, rollback procedure and smoke test before operation. No deploy or rollback command is provided here because no deployment target is approved.
