---
id: ideate
name: Ideate
order: 2
agents:
  - solution-explorer
  - decision-facilitator
stages:
  - option-generation
  - direction-selection
gate: solution-direction
---

# Ideate

Explore alternatives and select a solution direction grounded in the approved analysis.

## Entry

- Analyze exit criteria are met and its approval is recorded.

## Exit

- At least one feasible option and meaningful alternatives are documented.
- Trade-offs, scope, and a recommended direction are explicit.
- The project owner approves the selected direction before implementation planning.

On rejection, revisit options and trade-offs. Do not enter Develop.

## Stages

1. **option-generation** — `solution-explorer`. Produces solution options and a trade-off matrix.
2. **direction-selection** — `decision-facilitator`. Produces a recommended direction, a decision record, a scope boundary, and an architecture design. Write the design with [the arc42 and ADR template](../templates/architecture-design.md). This stage is the phase gate.

## Activities

- Generate a small set of genuinely different options from the approved analysis.
- Compare cost, risk, scope, and what each option leaves out.
- Recommend one direction and write the decision so a later agent can follow it without the design meeting.
- Stop at the solution-direction gate until the project owner chooses.

## Questions

- Which options are feasible under the recorded constraints?
- What is in scope, out of scope, and deferred?
- Which decision is approved, and what would require reopening it?

## Tracking

Record the approved direction in the active intent folder. Do not start implementation while `state.json` still shows Ideate in progress.
