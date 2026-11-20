---
id: develop
name: Develop
order: 3
agents:
  - implementation-planner
  - builder-verifier
stages:
  - implementation-design
  - red-tests
  - build-and-verify
gate: release-candidate
---

# Develop

Turn the approved direction into a validated, reviewable implementation and release candidate.

## Entry

- Ideate exit criteria are met and the chosen direction is approved.

## Exit

- The implementation matches the approved scope and contracts.
- Verification evidence, known limitations, and deployment or rollback notes are recorded.
- The project owner approves the release candidate before launch activities.

On rejection, address the findings and rerun the relevant verification. Do not enter Launch.

## Stages

1. **implementation-design** — `implementation-planner`. Produces an implementation plan, interfaces and contracts, and a functional design. Write the design with [the IEEE 1016 template](../templates/functional-design.md). Trace every function to an Analyze acceptance-criterion identifier.
2. **red-tests** — `builder-verifier`. Produce failing tests from the acceptance criteria before production code. Record the red run.
3. **build-and-verify** — `builder-verifier`. Write only enough implementation to turn those tests green. Produce the implementation, the green verification report, and known limitations. This stage is the phase gate.

## Activities

- Turn the approved direction into tasks, interfaces, and a functional design before coding.
- Follow red, then green. Write the test, run it, and keep the observed failure. Do not start production code for a behavior whose test has not failed for the intended reason.
- Implement only the approved scope, and only enough to make the existing red test pass.
- Run the same tests again and record the green result, including any check that is still failing.
- Present a release candidate with known limitations and a rollback note.

## Questions

- Does the implementation match the approved scope and contracts?
- Which checks were run, and what did they show?
- What remains unverified or out of scope?

## Tracking

Keep implementation notes and verification evidence with the intent. A stage is not complete in `state.json` until `orchestrate report` records it.
