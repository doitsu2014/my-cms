---
id: analyze
name: Analyze
order: 1
agents:
  - brief-analyst
  - context-risk-analyst
stages:
  - problem-framing
  - context-and-risk
gate: analysis-baseline
---

# Analyze

Establish the problem, context, constraints, stakeholders, and evidence before selecting a direction.

## Entry

- A project brief or explicit request exists.

## Exit

- The problem and desired outcomes are stated.
- Constraints, assumptions, risks, and open questions are recorded.
- The project owner approves the problem framing and analysis baseline.

On rejection, revise the analysis artifacts and resubmit. Do not enter Ideate.

## Stages

1. **problem-framing** — `brief-analyst`. Produces a problem statement, success measures, a stakeholder map, and acceptance criteria. Write the criteria in Given/When/Then using [the acceptance-criteria template](../templates/acceptance-criteria.md).
2. **context-and-risk** — `context-risk-analyst`. Produces constraints and assumptions, an evidence log, a risk register, and open questions. This stage is the phase gate.

## Activities

- Read the project request and any existing repository context before proposing a solution.
- Name the people affected, the current behavior, and the outcome that would count as success.
- Separate evidence, assumptions, constraints, and open questions.
- Record the task in `.aidlc/intents/<intent-id>/` and keep progress in that folder's `state.json`.

## Questions

- What problem is being solved, for whom, and why now?
- Which constraints and risks are known, and which claims still lack evidence?
- What must be true before a solution direction is allowed?

## Tracking

The active project is the directory where `dtech-aidlc` is run. The task text is `intent.md`. Stage status, the current stage, and approval progress are `state.json` in the same intent folder. `.aidlc/state.json` points at that active task.
