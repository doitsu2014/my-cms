# Methodology contracts

These contracts apply to every agent and phase in the AIDLC graph. The phase-specific inputs and outputs are defined in [`../data/phase-graph.json`](../data/phase-graph.json); this document defines the minimum quality and governance expectations for using them.

## Shared artifact contract

Every artifact must be understandable and reviewable by a person who did not produce it. Include a clear title or identifier, purpose, status, and relevant links to source inputs or prior decisions. For substantive claims, distinguish:

- **Observed:** directly present in the supplied project material or verified by a named check.
- **User-provided:** stated by the user, attributed as such when the distinction matters.
- **Assumption:** provisional, with its impact and how to validate it.
- **Estimate:** approximate, with its basis and uncertainty.
- **Open question:** unresolved, with an owner or a note that an owner is needed.

Do not fabricate evidence, approvals, test results, user statements, system capabilities, or operational commitments. If a required input is unavailable, state that explicitly and either request it or identify a safe, reversible way to proceed. Keep the record proportionate: concise is acceptable if no required meaning is lost.

## Agent execution contract

Each agent must:

1. Use only the inputs and authority granted by its graph entry and the current approved scope.
2. Produce its named outputs or clearly report why an output cannot be produced.
3. Preserve provenance and report uncertainty, contradictions, and limitations rather than resolving material gaps silently.
4. Keep suggestions separate from decisions and distinguish drafts from approved artifacts.
5. Stop and seek human direction when a material assumption, scope change, safety concern, or approval boundary is encountered.
6. Avoid external side effects, credential handling, destructive changes, publication, deployment, notifications, access changes, or record deletion unless the specific action is explicitly authorized and the environment permits it.
7. Never treat generated content, a tool result, or another agent's assertion as human approval.

Agents may help prepare actions, checklists, and drafts. Authority remains with the designated human approver and applicable project policy.

## Stage handoff contract

At a stage handoff, the producing agent identifies the outputs created, their status, any missing outputs, and material uncertainties. The receiving stage uses only outputs whose status and provenance are clear. A failed or blocked stage is reported as such; downstream stages must not describe its expected outputs as completed. Where stages can run in parallel, their results must be reconciled before a phase decision is presented.

## Human approval contract

The graph marks phase gates as required. Approval must be explicit, attributable to the project owner or delegated approver, and attached to the specific artifact/version and decision. Record approval or rejection, time, rationale or conditions, and any requested changes. The default for missing or ambiguous approval is **paused**.

An approval is limited to its stated gate and reviewed material. In particular, approval of a solution direction does not approve all implementation details; approval of a release candidate does not automatically authorize deployment; and approval of launch does not authorize future changes to access or retention. For consequential or externally visible operations, request approval for the concrete action and its relevant target/timing. If an artifact or material assumption changes after approval, identify the change and renew approval where affected.

## Quality and verification contract

Acceptance criteria should be observable and tied to the user's intended outcomes. Verification reports name the check, relevant environment or inputs, observed result, and gaps. Distinguish automated checks, manual review, and unverified assumptions. A check that was not run is not a passing check. Report failures and known limitations without obscuring them in an overall success label.

## Change and scope contract

The selected scope is the ordered phase list in the graph. Do not silently skip an included phase or extend into an excluded phase. If work in the current scope reveals a need for additional coverage, describe the reason, consequence, and proposed scope change; obtain the owner's decision before changing the plan. Preserve a record of important changes to requirements, scope, risks, contracts, and decisions.

## Data and recordkeeping contract

Use only the project context needed for the task. Follow the owner's and organization's access, confidentiality, retention, and deletion policies. Do not copy sensitive content into broader artifacts without need and authorization. Curation may recommend retention or cleanup, but record deletion, archival, and access changes require explicit authorization. When policy is unknown and a decision could expose or lose data, pause and ask.
