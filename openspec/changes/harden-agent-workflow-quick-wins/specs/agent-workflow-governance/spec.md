## ADDED Requirements

### Requirement: Current architecture guidance
The agent workflow documentation SHALL identify `apps/api/gateway` as the API
composition root and `apps/api/domain_*` crates as the domain-owned service,
adapter, handler, and migration surfaces. It MUST NOT describe retired
`apps/api/src`, `apps/api/application_core`, or `apps/api/migration` paths as
current implementation locations, except in explicitly labelled historical
context.

#### Scenario: A contributor locates an API responsibility
- **WHEN** an agent uses the project-structure or layered-architecture guidance
- **THEN** it is directed to `apps/api/gateway` for composition and
  `apps/api/domain_*` for owned behavior
- **AND** it is not directed to a retired path as a current editing target

### Requirement: Bounded Fast Fix classification
The workflow SHALL permit Fast Fix / Fast Implement only when every eligibility
condition holds: the change modifies at most one non-generated repository file,
changes no more than 40 non-generated lines, preserves observable product and
operator behavior, does not affect authentication, authorization, API contracts,
schema, migrations, dependencies, deployment manifests, secrets, generated
files, or active OpenSpec artifacts, and has a focused verification command.

#### Scenario: Eligible editorial correction uses Fast Fix
- **WHEN** a user explicitly requests a fast fix for a one-file, 40-lines-or-
  fewer editorial correction with no behavior change
- **THEN** the implementer may use the Fast Fix workflow
- **AND** it runs and reports the focused verification command before handoff

#### Scenario: Ineligible request is escalated
- **WHEN** a proposed Fast Fix violates any eligibility condition or its scope
  cannot be confidently established
- **THEN** the coordinator SHALL route it through the normal intent and
  OpenSpec workflow
- **AND** it SHALL not bypass the required owner, review, or verification gate

### Requirement: Evidence-bound Product Designer graph access
The Product Designer SHALL have access to the project code-review-graph server
for read-only design discovery. For cross-cutting design-system or UX audits,
the Product Designer MUST start with `get_minimal_context` and may inspect only
read-only graph context needed to identify affected UI communities, imports,
callers, flows, and tests. The Product Designer MUST report that evidence and
MUST NOT invoke graph mutation, indexing, embedding, or write-generating tools.

#### Scenario: Cross-cutting UI audit uses graph evidence
- **WHEN** a Product Designer investigates a cross-cutting UI or design-system
  change
- **THEN** the brief distinguishes graph-backed observations from inference
- **AND** it identifies relevant UI consumers before recommending reusable
  component or token changes

#### Scenario: Graph access remains read-only
- **WHEN** Product Designer graph access is configured
- **THEN** its instructions permit read-only discovery tools only
- **AND** the instructions forbid graph rebuilding, embedding, wiki generation,
  or other graph-mutating operations

### Requirement: Dedicated Release Engineer role
The project SHALL define a `release-engineer` Codex agent and describe it in
the workflow routing, phase, and team references. The Release Engineer SHALL
own deployment readiness assessment, rollout and rollback plans, post-deploy
verification, and release handoff for approved operationally affected changes.
The Software Engineer SHALL provide implementation and verification evidence
to the Release Engineer but MUST NOT own the release lifecycle.

#### Scenario: Operationally affected change is handed to release engineering
- **WHEN** an approved change modifies `deployments/`, runtime configuration,
  or an operational contract
- **THEN** the Software Engineer supplies the implementation verification and
  operational-impact evidence to the Release Engineer
- **AND** the Release Engineer records the rollout steps, verification plan,
  rollback trigger, and release handoff state

#### Scenario: Non-operational change has an explicit no-release finding
- **WHEN** an approved change affects no deployment, runtime configuration, or
  operational contract
- **THEN** the Release Engineer records that no deployment action is required
- **AND** the Software Engineer still completes the normal implementation and
  OpenSpec handoff gates

### Requirement: Authorization-bounded release execution
The Release Engineer MUST NOT execute an environment-changing production
deployment autonomously. Before any such command, the Release Engineer SHALL
obtain and record explicit user or release authorization. Without authorization,
the Release Engineer SHALL report readiness, the planned rollout and rollback,
and the next approving owner; it MUST NOT represent the change as
production-deployed.

#### Scenario: Authorized production release proceeds
- **WHEN** a release plan is ready and explicit user or release authorization
  is recorded
- **THEN** the Release Engineer may execute the approved environment-changing
  deployment commands
- **AND** it records the post-deploy verification result and rollback outcome
  if triggered

#### Scenario: Authorization is absent
- **WHEN** a deployment plan is ready but explicit user or release authorization
  is absent
- **THEN** the Release Engineer does not run environment-changing deployment
  commands
- **AND** it reports the release-ready state and the next approving owner

### Requirement: Controlled exploration concurrency
The project configuration SHALL support up to four concurrent agent threads.
The workflow SHALL limit four-way parallelism to independent, read-only
exploration or analysis and require an `update_plan` checkpoint before
concurrent work begins and after results are synthesized. Artifact and source
edits MUST remain serialized under one owner.

#### Scenario: Parallel discovery is coordinated
- **WHEN** an exploration task is decomposed into independent read-only
  questions
- **THEN** the coordinator may run up to four agent threads
- **AND** it records a plan checkpoint before dispatch and after synthesis
- **AND** it returns a combined evidence-backed result before any shared write

#### Scenario: A shared artifact requires a single writer
- **WHEN** exploration results lead to edits of an OpenSpec artifact,
  configuration file, or source file
- **THEN** exactly one assigned owner writes each shared target at a time
- **AND** parallel workers return findings rather than concurrently editing it
