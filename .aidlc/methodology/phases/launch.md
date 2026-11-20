---
id: launch
name: Launch
order: 4
agents:
  - release-coordinator
  - operations-handoff
stages:
  - release-readiness
  - operational-handoff
gate: launch-and-handoff
---

# Launch

Coordinate a controlled release, communication, and the initial operational handoff.

## Entry

- Develop exit criteria are met and the release candidate is approved.

## Exit

- Release status and deployment evidence are recorded.
- Monitoring, support ownership, and the rollback path are confirmed.
- The project owner approves the launch outcome and operational handoff.

On rejection, pause or roll back according to the approved plan. Resolve the concerns before declaring launch complete.

## Stages

1. **release-readiness** — `release-coordinator`. Produces a release checklist, a deployment and rollback plan, and a communications draft.
2. **operational-handoff** — `operations-handoff`. Produces a launch record, monitoring and support ownership, and a handoff record. This stage is the phase gate.

## Activities

- Prepare the release checklist, communications draft, monitoring owner, and rollback path from the approved release candidate.
- Run only the release checks that cannot exist before a release, such as smoke and health checks.
- Ask for explicit approval before deployment, publication, notification, or any irreversible action.
- Record the launch result and who accepted operational ownership.

## Questions

- Is the integrated release safe to run and safe to undo?
- Who owns monitoring and support after release?
- What exact action is proposed, and what approval does it have?

## Tracking

Launch progress stays on the same intent. Completing Develop does not mark Launch complete.
