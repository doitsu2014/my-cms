# Contract Summary — Backup & Restore

## Contract scope and evidence

This proposal formalizes contracts for the four approved Units defined in [`unit-of-work.md`](../units-generation/unit-of-work.md) and their DAG and integration boundaries in [`unit-of-work-dependency.md`](../units-generation/unit-of-work-dependency.md), plus the admin HTTP surface consumed by `apps/web`. The component catalogue and requirements further inform the contracts: [`components.md`](../domain-design/components.md) establishes ownership and adapter boundaries, while [`requirements.md`](../requirements-analysis/requirements.md) defines authorization, integrity, preflight and recovery constraints. Synchronous admin HTTP is specified with OpenAPI; internal same-process boundaries are described as Rust interface/type contracts and are not split into network services. The implementation details below are proposed contracts, not claims about existing routes. The initial scope is localhost-only, configured self-hosted Supabase instances, and no production restore.

Source data remains owned by current My-CMS domains, GoTrue and Supabase Storage. Backup-specific jobs, package metadata, orchestration and recovery state belong to BackupRestore/BackupPersistence. Restore is replace-in-scope and saga/recovery-based; no cross-system atomicity is claimed.

## Contracts table

| # | Provider Unit | Consumer | Mechanism | Owner |
|---|---|---|---|---|
| C1 | U1 `u1-backup-foundation` | External: `apps/web` admin client | HTTPS/JSON HTTP API (OpenAPI) | U1 owns API spec; web client consumes it |
| C2 | U1 `u1-backup-foundation` | U2 `u2-complete-backup` | Internal Rust traits/types over persisted job and package metadata | U1 owns the interface; U2 implements/consumes the extension points |
| C3 | U2 `u2-complete-backup` | U3 `u3-restore-preflight` | Versioned ZIP package + JSON manifest and component digests | U2 owns package-format spec; U3 consumes/validates it |
| C4 | U3 `u3-restore-preflight` | U4 `u4-restore-execution` | Internal Rust typed preflight result bound to digest, target and compatibility snapshot | U3 owns the preflight result contract; U4 consumes it |
| C5 | U1–U4 | External: configured PostgreSQL, GoTrue/Auth and Supabase Storage adapters | Internal Rust ports/adapters; provider-specific protocols remain behind adapter ports | Each adapter-owning Unit owns its port and capability result; external systems remain authoritative |

## C1 — Admin HTTP API

The API requires an authenticated administrator on every operation. Instance identifiers resolve only through server-side configured allowlists. The API never accepts arbitrary endpoints, connection strings, credentials or service-role keys. All responses use safe DTOs; errors must not contain secrets, raw database rows, object contents or unsanitized upstream payloads.

```yaml
openapi: 3.1.0
info:
  title: My-CMS Backup and Restore Admin API
  version: 1.0.0
  description: Proposed localhost admin contract; routes and DTOs require implementation validation.
servers:
  - url: /api/admin/backup-restore
security:
  - bearerAuth: []
tags:
  - name: Instances
  - name: Backup jobs
  - name: Restore jobs
paths:
  /instances:
    get:
      operationId: listConfiguredInstances
      tags: [Instances]
      responses:
        '200':
          description: Configured and authorized instance choices; contains no credentials or endpoints.
          content:
            application/json:
              schema:
                type: object
                required: [items]
                properties:
                  items:
                    type: array
                    items: { $ref: '#/components/schemas/InstanceChoice' }
        '401': { $ref: '#/components/responses/Unauthorized' }
        '403': { $ref: '#/components/responses/Forbidden' }
        '500': { $ref: '#/components/responses/SafeError' }
  /backups:
    post:
      operationId: createBackupJob
      tags: [Backup jobs]
      description: Creates a durable job and returns without waiting for package creation.
      parameters:
        - $ref: '#/components/parameters/IdempotencyKey'
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              required: [sourceInstanceId]
              additionalProperties: false
              properties:
                sourceInstanceId: { type: string, minLength: 1, maxLength: 128 }
      responses:
        '202':
          description: Job accepted.
          content:
            application/json:
              schema: { $ref: '#/components/schemas/JobAccepted' }
        '400': { $ref: '#/components/responses/SafeError' }
        '401': { $ref: '#/components/responses/Unauthorized' }
        '403': { $ref: '#/components/responses/Forbidden' }
        '409': { $ref: '#/components/responses/Conflict' }
        '422': { $ref: '#/components/responses/SafeError' }
        '500': { $ref: '#/components/responses/SafeError' }
  /jobs/{jobId}:
    get:
      operationId: getJob
      tags: [Backup jobs, Restore jobs]
      parameters:
        - in: path
          name: jobId
          required: true
          schema: { type: string, format: uuid }
      responses:
        '200':
          description: Authorized safe job projection.
          content:
            application/json:
              schema: { $ref: '#/components/schemas/JobStatus' }
        '401': { $ref: '#/components/responses/Unauthorized' }
        '403': { $ref: '#/components/responses/Forbidden' }
        '404': { $ref: '#/components/responses/SafeError' }
        '500': { $ref: '#/components/responses/SafeError' }
  /packages/{packageId}/download:
    get:
      operationId: downloadVerifiedPackage
      tags: [Backup jobs]
      parameters:
        - in: path
          name: packageId
          required: true
          schema: { type: string, format: uuid }
      responses:
        '200':
          description: Package bytes only for a succeeded job with verified integrity.
          headers:
            Content-Disposition: { schema: { type: string } }
            X-Content-SHA256: { schema: { type: string } }
          content:
            application/zip:
              schema: { type: string, format: binary }
        '401': { $ref: '#/components/responses/Unauthorized' }
        '403': { $ref: '#/components/responses/Forbidden' }
        '404': { $ref: '#/components/responses/SafeError' }
        '409': { $ref: '#/components/responses/Conflict' }
        '500': { $ref: '#/components/responses/SafeError' }
  /restore-uploads:
    post:
      operationId: uploadRestorePackage
      tags: [Restore jobs]
      parameters:
        - $ref: '#/components/parameters/IdempotencyKey'
      requestBody:
        required: true
        content:
          multipart/form-data:
            schema:
              type: object
              required: [targetInstanceId, file]
              additionalProperties: false
              properties:
                targetInstanceId: { type: string, minLength: 1, maxLength: 128 }
                file: { type: string, format: binary }
      responses:
        '202':
          description: Bounded private upload accepted for preflight; target is unchanged.
          content:
            application/json:
              schema: { $ref: '#/components/schemas/RestoreUploadAccepted' }
        '400': { $ref: '#/components/responses/SafeError' }
        '401': { $ref: '#/components/responses/Unauthorized' }
        '403': { $ref: '#/components/responses/Forbidden' }
        '413': { $ref: '#/components/responses/SafeError' }
        '422': { $ref: '#/components/responses/SafeError' }
        '500': { $ref: '#/components/responses/SafeError' }
  /restore-preflights/{preflightId}:
    get:
      operationId: getRestorePreflight
      tags: [Restore jobs]
      parameters:
        - in: path
          name: preflightId
          required: true
          schema: { type: string, format: uuid }
      responses:
        '200':
          description: Safe preflight result, including compatibility and replace impact; no target mutation has occurred.
          content:
            application/json:
              schema: { $ref: '#/components/schemas/PreflightResult' }
        '401': { $ref: '#/components/responses/Unauthorized' }
        '403': { $ref: '#/components/responses/Forbidden' }
        '404': { $ref: '#/components/responses/SafeError' }
        '500': { $ref: '#/components/responses/SafeError' }
  /restore-preflights/{preflightId}/confirm:
    post:
      operationId: confirmRestore
      tags: [Restore jobs]
      description: Creates a durable restore job only after successful preflight, fresh target/digest binding, and explicit administrator confirmation.
      parameters:
        - in: path
          name: preflightId
          required: true
          schema: { type: string, format: uuid }
        - $ref: '#/components/parameters/IdempotencyKey'
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              required: [confirmed, expectedTargetInstanceId, expectedPackageSha256]
              additionalProperties: false
              properties:
                confirmed: { const: true }
                expectedTargetInstanceId: { type: string, minLength: 1, maxLength: 128 }
                expectedPackageSha256: { type: string, pattern: '^[a-fA-F0-9]{64}$' }
      responses:
        '202':
          description: Restore job accepted after recovery-point check; execution is asynchronous.
          content:
            application/json:
              schema: { $ref: '#/components/schemas/JobAccepted' }
        '400': { $ref: '#/components/responses/SafeError' }
        '401': { $ref: '#/components/responses/Unauthorized' }
        '403': { $ref: '#/components/responses/Forbidden' }
        '409': { $ref: '#/components/responses/Conflict' }
        '422': { $ref: '#/components/responses/SafeError' }
        '500': { $ref: '#/components/responses/SafeError' }
components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
  parameters:
    IdempotencyKey:
      in: header
      name: Idempotency-Key
      required: true
      schema: { type: string, minLength: 16, maxLength: 128 }
      description: Client-generated opaque key; server binds it to actor, operation and canonical request digest.
  responses:
    Unauthorized:
      description: Authentication required.
      content:
        application/json:
          schema: { $ref: '#/components/schemas/SafeError' }
    Forbidden:
      description: Administrator role or resource authorization denied.
      content:
        application/json:
          schema: { $ref: '#/components/schemas/SafeError' }
    Conflict:
      description: Conflicting job, idempotency-key reuse with a different request, stale preflight, or target lock.
      content:
        application/json:
          schema: { $ref: '#/components/schemas/SafeError' }
    SafeError:
      description: Safe error projection; never includes secrets or raw upstream payloads.
      content:
        application/json:
          schema: { $ref: '#/components/schemas/SafeError' }
  schemas:
    InstanceChoice:
      type: object
      required: [id, label, environment]
      additionalProperties: false
      properties:
        id: { type: string }
        label: { type: string }
        environment: { type: string }
    JobAccepted:
      type: object
      required: [jobId, status]
      additionalProperties: false
      properties:
        jobId: { type: string, format: uuid }
        status: { type: string, enum: [queued] }
    RestoreUploadAccepted:
      type: object
      required: [preflightId, status]
      additionalProperties: false
      properties:
        preflightId: { type: string, format: uuid }
        status: { type: string, enum: [queued, running] }
    JobStatus:
      type: object
      required: [jobId, operation, status, createdAt]
      additionalProperties: false
      properties:
        jobId: { type: string, format: uuid }
        operation: { type: string, enum: [backup, restore] }
        status: { type: string, enum: [queued, running, succeeded, failed, recovery-required] }
        sourceLabel: { type: [string, 'null'] }
        targetLabel: { type: [string, 'null'] }
        progress:
          oneOf:
            - type: 'null'
            - type: object
              required: [completed, total]
              properties:
                completed: { type: integer, minimum: 0 }
                total: { type: integer, minimum: 0 }
        createdAt: { type: string, format: date-time }
        completedAt: { type: [string, 'null'], format: date-time }
        error: { oneOf: [{ type: 'null' }, { $ref: '#/components/schemas/SafeError' }] }
        recoveryState: { type: string, enum: [not-applicable, pending, rollback-complete, recovery-required] }
    PreflightResult:
      type: object
      required: [preflightId, packageSha256, targetInstanceId, compatible, targetMutationOccurred, replaceImpact, components]
      additionalProperties: false
      properties:
        preflightId: { type: string, format: uuid }
        packageSha256: { type: string, pattern: '^[a-fA-F0-9]{64}$' }
        targetInstanceId: { type: string }
        sourceLabel: { type: string }
        compatible: { type: boolean }
        targetMutationOccurred: { const: false }
        replaceImpact: { type: string }
        recoveryPointAvailable: { type: boolean }
        components:
          type: array
          items:
            type: object
            required: [name, status]
            properties:
              name: { type: string }
              status: { type: string, enum: [supported, unsupported, missing, invalid] }
              digest: { type: [string, 'null'] }
              bytes: { type: [integer, 'null'], minimum: 0 }
        issues: { type: array, items: { type: string } }
    SafeError:
      type: object
      required: [code, message, correlationId]
      additionalProperties: false
      properties:
        code: { type: string }
        message: { type: string }
        correlationId: { type: string }
```

### HTTP failure, timeout, and retry rules

- Stable `code` values distinguish authentication/authorization, invalid input, unsupported capability, integrity/compatibility failure, configured-limit failure, conflict, dependency failure, timeout, and internal failure. Human-readable `message` is safe and localized by the client; `correlationId`/job ID supports support and audit lookup.
- The API request deadline covers validation and durable job creation only. Long-running capture/restore uses configured per-component and whole-job deadlines; values are deployment configuration and remain open until design validates safe limits.
- Retry only transient failures and only for operations with an idempotency contract. `Idempotency-Key` is bound to actor, operation, and request digest; same-key/same-request returns the original job, while same-key/different-request returns `409`.
- Do not automatically retry non-idempotent target mutations. Restore failures enter rollback or `recovery-required`; a new attempt requires explicit recovery-state evaluation and a fresh confirmation where the target/package binding changed.
- Unauthorized access and resource-ownership failures return safe errors and do not disclose whether an inaccessible job/package exists beyond the agreed not-found policy.

## C2 — Internal job and backup capability contracts

U1 defines internal Rust interfaces and stable domain types; these are in-process boundaries, not HTTP endpoints:

- `BackupJobRepository`: create idempotent job, fetch authorized projection, perform guarded state transition, claim/heartbeat/complete lease, and acquire/release target lock. Every transition is conditional on expected current state/version.
- `BackupExecutor`: accept a persisted job identity and a server-resolved configuration reference; it never accepts client credentials/endpoints. It returns typed per-component outcomes, inventory and digest references, not raw sensitive content in status DTOs.
- `BackupCapability`: each source adapter declares supported operation, instance/version constraints and a safe capability result. Unsupported required capability fails the job; optional capability may be excluded only when the versioned manifest explicitly records the exclusion and policy permits it.
- `JobStatus` projection: `queued | running | succeeded | failed | recovery-required`; only `succeeded` is downloadable and requires all required components and integrity checks.
- U1 owns the baseline; U2 adds supported Auth and Storage implementations without weakening U1's state and authorization invariants.

## C3 — Backup package and manifest contract

U2 owns package format and compatibility rules; U3 verifies these before restore. The archive is untrusted data. Entry names are relative normalized paths; absolute paths, traversal, symlinks, duplicate/conflicting paths, unsupported compression, excessive size/count/depth/ratio, and undeclared entries are rejected. Package content is never executed.

```yaml
format: my-cms-backup
format_version: 1
backup_id: 00000000-0000-0000-0000-000000000000
source:
  configured_instance_id: source-local
  display_label: Local source
created_at: '2026-01-01T00:00:00Z'
tool_version: '1.0.0'
consistency:
  mode: best-effort
  statement: Components may represent different capture times; this is not a point-in-time snapshot.
  captured_at: '2026-01-01T00:00:00Z'
components:
  - name: cms_database
    required: true
    status: complete
    path: components/cms_database.bin
    bytes: 0
    item_count: 0
    sha256: '0000000000000000000000000000000000000000000000000000000000000000'
  - name: auth
    required: true
    status: complete
    path: components/auth.json
    bytes: 0
    item_count: 0
    sha256: '0000000000000000000000000000000000000000000000000000000000000000'
  - name: storage
    required: true
    status: complete
    path: components/storage/
    bytes: 0
    item_count: 0
    sha256: '0000000000000000000000000000000000000000000000000000000000000000'
exclusions: []
```

The block is a schema example, not permission to claim Auth/database/Storage export capability. IDs/labels contain no secrets. Exact database encoding, Auth representation and Storage metadata fields remain unresolved until adapter capability is verified. Manifest and archive component digests are checked against actual bytes; job completion additionally checks required-component status, inventory/counts and configured bounds. Any missing required component or digest mismatch prevents `succeeded` and download. Future incompatible formats require a new `format_version` and explicit preflight compatibility handling.

## C4 — Preflight-to-restore handoff

U3 returns a typed `PreflightResult` that binds at least:

- package ID and verified package SHA-256;
- exact target configured-instance ID and captured configuration/version fingerprint;
- manifest format and component inventory, required/optional status and verification results;
- compatibility verdict and bounded, safe issue codes;
- replace-in-scope impact summary;
- recovery-point availability/evidence identifier and expiry;
- immutable preflight ID and creation/expiry times.

U4 must revalidate that the package digest, target identity/configuration, compatibility snapshot and recovery-point evidence are still current before the first target mutation. Confirmation records the authenticated actor, timestamp, expected target ID and expected package digest. A stale/mismatched binding fails closed and requires new preflight/confirmation. No target mutation occurs during upload or preflight.

## C5 — Adapter ports and outcomes

U2/U4 own narrowly scoped adapter ports for database, Auth and Storage. Adapter calls receive only server-resolved connection/configuration references, cancellation/deadline context and the approved component scope. Each returns typed outcomes: supported/unsupported capability, exported/imported item counts, byte counts, digest/evidence references, safe error classification, and whether a side effect may have occurred. Raw object/database content stays in protected package/staging paths, not job status, audit, logs, metrics or error DTOs.

The adapter contract must expose operation idempotency and recovery semantics. A timeout after a mutation begins is an unknown outcome, not proof of failure-with-no-effect; orchestration marks recovery state and requires reconciliation. Domain ownership and platform APIs remain authoritative; no direct cross-domain table writes are implied.

## Contract ownership and evolution

- A provider Unit owns the normative specification for its provided boundary. Consumers may propose changes but do not unilaterally redefine provider behavior.
- OpenAPI, Rust interface contracts and the package manifest format are versioned independently. API versioning must not be confused with backup `format_version`.
- Additive fields are preferred. Consumers ignore unknown optional response/manifest fields while still rejecting unknown safety-critical semantics or unsupported required components. Defaults never silently broaden restore scope.
- Breaking changes require a new compatible version/format, provider and all affected consumer updates, and explicit compatibility tests before merging.
- Idempotency is scoped to actor + operation + canonical request digest. Job state changes use expected state/version to prevent duplicate or stale transitions.
- User-visible errors are stable safe codes; adapter-specific secrets and raw upstream errors are not part of the contract.

## Open questions

| Contract | Question | Blocks |
|---|---|---|
| C1 | Confirm actual admin route prefix, existing auth/session/CSRF conventions, and whether polling or explicit refresh is the supported job-status UX. | U1, U2, U3, U4 implementation; must be reconciled with existing gateway and `apps/web` contracts. |
| C1/C2 | Define authorized instance configuration IDs/labels and actor-to-resource access policy. | Real-data use; no arbitrary client endpoint is allowed. |
| C2/C3 | Which exact My-CMS schemas/tables and PostgreSQL dump/restore mechanism are approved, and what source/target version matrix is supported? | Complete backup, preflight and restore. |
| C2/C3 | Which GoTrue export/import API is supported, and what are password-hash, session/token invalidation and identity semantics? | Any claim of Auth backup/restore; must fail closed until verified. |
| C2/C3 | Which Storage buckets, object metadata, paging behavior, and per-object consistency checks are included? | Complete backup and restore correctness. |
| C1/C2/C3 | Which long-term object store, maximum upload/package/extracted bytes, entry/count/depth limits, retention/deletion policy, encryption-at-rest and key ownership apply? | Real-data backup/upload; required configuration absent means refusal. |
| C2/C4/C5 | How are durable worker leases, restart recovery, cancellation, retry budgets and unknown side-effect outcomes implemented with the existing gateway runtime? | Safe restart/retry and restore execution; Tokio process tasks alone do not imply durable recovery. |
| C4/C5 | What mechanism establishes and verifies a recovery point before target mutation, and what rollback evidence is sufficient across DB/Auth/Storage? | Restore execution; refuse if no verified recovery point or safe recovery plan. |
| C1 | What concrete API/job/component deadlines, concurrency limits and measurable RPO/RTO targets are approved? | Operational configuration and any performance/recovery claims; do not invent numeric commitments here. |
| C1 | Confirm whether UI screens are owned by a backend `service` Unit's functional design or require an explicit `ui` Unit/design artifact. | UI ownership and NFR7 traceability in Construction; accepted Units Generation review finding R-01 remains a risk. |
| C1/C2 | Resolve story/acceptance-criterion partitioning for stories assigned to multiple Units, so each Unit's Functional Design covers only its named AC subset and later Units complete the remainder. | Precise per-Unit test/functional-design traceability; accepted Units Generation review finding R-02 remains a risk. |
