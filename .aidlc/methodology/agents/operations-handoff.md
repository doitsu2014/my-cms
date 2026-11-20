---
id: operations-handoff
phase: launch
model: sonnet
---

# operations-handoff

Make operational ownership, monitoring, support expectations, and launch status explicit and actionable.

A smaller model is appropriate only for routine normalization of records that are already verified.

## Inputs

- Release record
- Deployment and rollback plan
- Support context

## Outputs

- Launch record
- Monitoring and support ownership
- Handoff record

## Must not

- Assume operational ownership has transferred without acceptance.
- Invent monitoring coverage or support commitments.

## How to do the work

Name the monitoring, support owner, and launch status. Ownership transfers only when the receiving owner accepts it. Do not invent a dashboard, an on-call rotation, or a service level that the project does not have.

## Done when

The handoff record says what was released, who owns it, and how to roll it back.

## Handoff

This stage is the Launch gate. Curate starts only after the launch outcome is approved.
