# Architecture design

**Purpose:** Design Option A's constrained backup/restore U1 workflow using the arc42 template and C4 view names. **Status:** Proposed; no approved direction or construction authorization is recorded. This design describes only the recommended direction, conditionally on its approval, not multiple competing implementations. Architectural decisions are authoritative in [decision-record.md](decision-record.md); section 9 is their reference index.

## 1. Introduction and goals

- **Purpose:** Validate the localhost admin workflow for a database-only slice with synthetic fixtures, durable jobs, explicit incompleteness and protected diagnostics. It is not a real backup/recovery implementation.
- **Quality goals:** Truthful outcomes; durable acceptance; server-side protection; bounded, observable fixture cleanup; accessible admin feedback. See [quality-requirements.md](../../analyze/context-and-risk/quality-requirements.md) and section 10 for interpretation limits.
- **Stakeholders:** System/product owner approves scope and bounds; architect approves contracts, domain/migration ownership and lifecycle; developers implement; QA establishes failure evidence; operations owns cleanup escalation; security reviews authentication/resource policy/protection; authorized administrators use the UI. Operations staff do not gain API access automatically: the approved administrator/resource matrix governs access.
- **Trace:** [US-01–06](../../analyze/problem-framing/user-stories.md), [AC-001–006](../../analyze/problem-framing/acceptance-criteria.md), [FEAT-01](../../../../../docs/features/backup-restore.md) §§1–6/10–13.

## 2. Constraints

- **Technical:** C-01 localhost-only/database-only; every U1 terminal backup result is `incomplete` or `failed`, never `succeeded`, and no U1 download is allowed. C-02 prohibits real-data staging; this design uses synthetic fixtures and no real dump. C-03 prohibits production restore. C-04 existing CLI is incomplete and remains fail-closed. C-05 Rust test execution is unverified. C-06 prohibits inventing operating bounds. C-07 requires brownfield Rust/Axum/Tokio/SeaORM and React integration. See [constraints](../../analyze/context-and-risk/constraints-and-assumptions.md).
- **Organizational:** Q-01/Q-02/Q-03 proposals require explicit written owner/architect approval before construction; finite Q-05 inputs and current toolchain/baseline evidence are additional dependencies. Q-04 recovery remains unverified; Q-06 RPO/RTO remains owner-pending. Gate reporting belongs to the conductor.
- **Convention:** Follow [MAP-01 §6](../../../../knowledge/_codebase-map.md) and [existing architecture](../../../../../docs/architecture/software-architecture.md#rules-for-extending-the-system), not a gateway-owned feature exception. Use existing domain registration/migration/administrator mounts and frontend auth/API conventions. No global auth/cookie layers are duplicated inside the domain.
- **Not applicable:** Production topology, deployment/release procedure, high availability and multi-host worker coordination are not designed for U1 because they are excluded, not because the system is proven ready for them.

## 3. Context and scope

### C4 System Context

The existing system context is linked, not redefined: [MAP-01 §§1–5](../../../../knowledge/_codebase-map.md) and [architecture overview](../../../../../docs/architecture/software-architecture.md#architectural-position).

| Actor/system | Relationship to the U1 increment | Boundary |
| --- | --- | --- |
| Authorized administrator using admin UI | Lists eligible local instances, submits synthetic jobs, monitors and reviews safe diagnostics/previews. | Browser is untrusted; UI gating is convenience only. |
| CMS API modular monolith | Hosts backup domain, validates actor and resource policy, owns asynchronous fixture work and metadata. | Trusted application boundary, not a claim against a compromised host. |
| Supabase Auth | Existing login/JWT trust source. | Existing auth integration is reused and must be verified; no Auth recovery is added. |
| PostgreSQL metadata store | Stores backup-owned jobs, safe audit intent/outcomes, idempotency and allocation records. | Shared pool does not imply shared schema ownership. |
| Private local fixture storage | Holds only non-sensitive synthetic files under allocation control. | No public mapping, static serving or download. |
| Application database/Auth/Storage content | Future capture/recovery subjects only. | Not connected to a real capture/restore adapter in U1. |

**Out of scope:** Complete ZIPs, real upload/preflight/restore, production, arbitrary remote instances, CLI completion and cross-instance mapping. See [scope-boundary.md](scope-boundary.md) for full exclusions and acceptance accounting. Q-08 same-instance and Q-09 admin-first remain assumptions awaiting confirmation.

## 4. Solution strategy

Chosen recommendation and the reasons alternatives lost are in [recommended-direction.md](recommended-direction.md); decision details are in [decision-record.md](decision-record.md). The design applies those proposals through one feature-owned domain, one durable metadata authority, existing administrator route composition, and fixture-only adapters. Unsupported capabilities are refused server-side and clearly labeled in UI. B/C are not parallel build targets.

Implementation order after approvals: domain contract/persistence/state and RED tests; lifecycle fault-test harness; administrator API and failure contracts; UI integration and accessibility tests. Verify each layer before the next. No implementation begins while critical approvals or mandatory input bounds are missing.

## 5. Building block view

### C4 Container view

Reuse existing containers from [MAP-01 §2](../../../../knowledge/_codebase-map.md). No new deployable service is proposed. The increment spans `apps/web/`, the API process in `apps/api/`, PostgreSQL metadata, and private fixture files. Reader website is unchanged. Gateway remains composition-only.

### C4 Component view

All paths below are proposed additions unless labeled existing. Interfaces name responsibilities, not finalized route spelling or Rust signatures.

| Component / location | Responsibility | Interface and dependencies |
| --- | --- | --- |
| Backup admin pages, proposed under `apps/web/src/app/admin/` | Eligible-instance selector, synthetic job creation/status, safe diagnostics and labeled previews. | Existing router and `AdminOnlyRoute`; existing API URL/auth fetch helpers; domain-owned API contracts. |
| Backup API adapter, proposed `apps/api/domain_backup/` | Parse strict requests, expose allowed instances and authorized job commands/queries; reject real package operations. | Bare Axum routes through `RouteRegistration` with administrator mount; validated actor and application service. |
| Backup application service/state policy, same domain | Instance/resource authorization, capability policy, idempotency, audit-before-effect and guarded transitions. | Repository, server-config registry, fixture capture port, lifecycle/audit ports. |
| Job/idempotency/audit persistence, same domain | Durable acceptance and safe lifecycle evidence; canonical states, request fingerprints and scoped key digests. | SeaORM and domain migrations; transactional operations or reviewed audit protocol. |
| Local fixture worker, same domain | Claims queued work conditionally, invokes synthetic database slice, records component outcomes and terminal result. | Durable repository, finite execution configuration, fixture capture adapter. No real Auth/Storage/capture adapters in U1 composition. |
| Allocation ledger/reconciler/cleaner, same domain | Inventory reconciliation, ownership/fencing, expiry scheduling, bounded idempotent deletion and failure escalation. | Durable ledger, controlled namespace inventory/storage port, injected clock, approved budgets; ADR-004 protocol. |
| Existing gateway / domain interface | Builds dependencies, registers domain, collects migrations, mounts policy layers and binds process. | Existing `DomainService`, `DomainContext`, `MigrationDescriptor`, `Mount::Administrator`. No backup handler/entity import into gateway. |

Observed integration anchors: [domain interface](../../../../../apps/api/domain_interface/src/lib.rs), [gateway](../../../../../apps/api/gateway/src/main.rs), [frontend administrator gate](../../../../../apps/web/src/app/admin/components/admin-only-route.tsx), [API helpers](../../../../../apps/web/src/config/api.config.ts). The API helper resolves client transport URLs; it is not an instance target selector. Exact DTOs, endpoint names and adapter signatures must be finalized in implementation planning within the approved decisions.

## 6. Runtime view

### Main scenario: submit and monitor a synthetic backup job

1. Admin UI authenticates using the existing login path and requests eligible instances. Gateway validates actor/administrator before domain code reads configuration; the application service filters configured local instances by approved operation policy.
2. Submission includes only configured instance ID and idempotency metadata, not credentials/URLs. Domain rejects unknown or non-local instances, unauthorized actor/resource, unsupported operation and missing finite configuration before work.
3. Within the approved transaction/protocol, commit job `queued`, scoped idempotency digest/request fingerprint and safe audit intent before returning accepted. Same-key/same-body replay returns the authorized existing job; same key with changed body is rejected. Concurrent replay is controlled durably, not by a process-local map.
4. Worker conditionally claims the job using expected state/version. Sensitive synthetic effects require durable audit intent. Allocation follows ADR-004 before fixture files are created.
5. Synthetic database-slice execution records measured fixture counters only. Required Auth/Storage are unavailable. Normal constrained completion writes `incomplete`; step failure/timeout/unknown interrupted execution writes `failed` when persistence permits. No path writes U1 `succeeded`.
6. Every job read rechecks administrator and resource/instance policy. API returns stable ID, status, timestamps, safe reason, component capabilities and measured progress if available; UI explicitly states synthetic/database-only/not restorable. U1 download eligibility remains false regardless of file existence.

### Failure scenario: persistence, audit or execution failure

- Initial commit/audit fails: no accepted response, no synthetic capture/allocation, and no fabricated job reference. If commit outcome is uncertain, client retries use idempotency to recover the authoritative result.
- Scheduling after commit fails: retain the durable non-success job and audit a safe dispatch failure; conditionally mark failed. A persisted queued job is not evidence that it ran. Startup reconciliation resolves abandoned queued/running work, not automatic resumption.
- Adapter/deadline fails: persist a safe terminal failure and cleanup eligibility; if persistence is down, leave durable non-success evidence for restart reconciliation and report degraded health. Never infer success from a subprocess exit or file existence.
- Terminal transition races: compare-and-set state/version allows only the valid winner; loser does not replay effects or overwrite a terminal result. Retry only approved idempotent transient operations within finite budgets.

### Crash scenario: allocation and deletion

ADR-004 owns the protocol. Required fault cases include ledger commit before file creation, file creation before ledger update, worker interruption, orphan namespace entry, missing file, cleaner claim before unlink, unlink before final commit, retry exhaustion, active-worker/sweeper race and metadata DB outage. At restart fence old work before inventory/deletion. Startup reconciliation must complete safely before new admissions; DB outage disables allocation/deletion and requires visible intervention. Tests use a controllable clock and synthetic private files, not real backup content. Deletion may lag expiry during outages; no unconditional expiry SLA or automatic resume is promised.

### Disabled-capability scenario

Authorized requests to real upload/preflight/restore/download are denied before package body handling/storage/extraction or target mutation; unauthenticated/non-admin requests are denied before configuration/resource disclosure. Client-supplied `synthetic`, target or enablement fields cannot select a real adapter. Fixture previews use trusted predefined fixture identifiers, not uploaded files or a filesystem path. Confirmation/cancel previews neither enqueue restore nor call a mutation port.

## 7. Deployment view

- **Environments:** Development/testing localhost only. No production restore, deployment, launch or remote service enablement is authorized.
- **Runtime nodes:** Local browser/admin dev or static server; one API process with local Tokio fixture worker/reconciler; configured local PostgreSQL/Supabase services for auth and metadata; a private local fixture namespace. Use the repository's existing local run topology ([MAP-01 §8](../../../../knowledge/_codebase-map.md)); this is a logical view, not a claim those services are running.
- **Exposure:** Local-only backup endpoints require server-side environment/capability gating. If the shared API binds beyond loopback for unrelated features, backup operations must remain disabled or demonstrably restricted by a reviewed trusted ingress boundary. A localhost string, request Host header, client Origin or configured database name is not proof of local-only reachability or non-production target. The eligible server registry and deployment boundary both require tests.
- **Transport/secrets:** TLS on remote transport; the documented unencrypted loopback exception must remain local and must not migrate to remote deployments. Secrets resolve inside the server boundary and are never bundled into UI. Metadata connections follow existing least-privilege/config conventions; new privileges are reviewed explicitly.
- **Storage/process lifecycle:** Backup-owned namespace not under web static assets; controlled permissions and reviewed protection policy. Exclusive local process ownership/fencing is required by ADR-004, but its implementation is still a construction prerequisite. No overlapping processes sharing the namespace and no distributed coordination claim.
- **Startup/rollback:** Validate enabled configuration and schema, take namespace ownership, reconcile, then admit synthetic work. Domain migration validation follows existing orchestration; destructive rollback/drop is not assumed safe. Disable admissions/fixture worker on rollback and retain ledger/evidence for reviewed reconciliation rather than deleting metadata ahead of files.

## 8. Cross-cutting concepts

### Security

See [threat-model.md](threat-model.md). Reuse the existing administrator envelope, require handler-level resource policy, enforce server-side capability denial and safe audit-before-effect. Deny at API boundaries, not merely through disabled UI controls.

### Data

Proposed logical records (not final SQL):

| Record | Minimum safe metadata / invariants |
| --- | --- |
| Job | Opaque ID, actor reference, configured instance, operation, canonical state/version, timestamps, safe reason, synthetic mode and measured counters. No raw rows/objects/secrets. |
| Idempotency | Actor+operation-scoped digest, request fingerprint, job reference, approved finite validity/retention. No raw key; durable uniqueness and conflict handling. |
| Audit | Actor/instance/job/allocation reference as appropriate, time, operation, transition/outcome/reason and permitted digest. No raw data, credentials or upstream errors. |
| Allocation | Durable ID/job linkage, namespace-derived opaque storage key, expiry, owner/version, lifecycle state, cleanup attempts/next attempt and safe failure reason. Discovered orphans explicitly represented. |

Transactions and compare-and-set transitions protect acceptance, worker claims and deletion claims. Audit physical adapter and atomicity must be approved; sharing the DB pool does not prove these guarantees. Retention/cleanup of metadata and idempotency is owner-pending alongside fixture expiry. Schema ownership and stable migration identities follow existing patterns. No existing business tables are rewritten to make U1 look functional.

### Observability

Structured events carry safe correlation/job IDs, actor/configured-instance references where permitted, state changes, safe codes, timestamps and measured counters. Metrics labels use operation/component/status/code, never actor, email, raw path, object key or other high-cardinality identifiers. Job references belong in controlled logs/traces/API, not metric labels. Report capability-disabled, missing-limit, cleanup pending/exhausted and reconciliation/DB health visibly. No simulated percentages, secret-bearing upstream errors or numerical SLO claims. Alert destination/owner and escalation policy need operations approval.

### Configuration and accessibility

Validate all applicable finite bounds before enabled work; absent/invalid required config refuses the capability, never unbounded fallback. Real staging stays non-configurable-on for this U1 delivery. Use keyboard-operable forms, text status plus accessible announcements, explicit synthetic labels, safe error focus handling and no color-only status. Manual assistive-technology review remains required.

## 9. Architecture decisions

- [ADR-001 — Select an admin-first synthetic U1 workflow](decision-record.md#adr-001--select-an-admin-first-synthetic-u1-workflow) — proposed.
- [ADR-002 — Expose terminal incomplete without semantic projection](decision-record.md#adr-002--expose-terminal-incomplete-without-semantic-projection) — proposed.
- [ADR-003 — Assign backup state and migration ownership to a domain](decision-record.md#adr-003--assign-backup-state-and-migration-ownership-to-a-domain) — proposed.
- [ADR-004 — Use a durable allocation ledger and fail-closed reconciliation](decision-record.md#adr-004--use-a-durable-allocation-ledger-and-fail-closed-reconciliation) — proposed.
- [ADR-005 — Enforce authorization and durable audit before effects](decision-record.md#adr-005--enforce-authorization-and-durable-audit-before-effects) — proposed.

## 10. Quality requirements

See [quality-requirements.md](../../analyze/context-and-risk/quality-requirements.md), interpreted under C-01/C-02/C-06. Its example latency, dataset sizes, idempotency window and expiry/grace examples are not approved requirements. Its implied local concurrency value is not an approved default. A-02 generic failure wording does not override the normal U1 `incomplete` outcome. O-08's job-ID-in-metrics wording conflicts with §10.3's label restrictions: correlate via logs/traces instead. Its readiness summary cannot waive Q-03/Q-05/protection dependencies. These interpretations preserve the feature's stricter constraints without editing upstream analysis.

| Quality scenario | Verification measure / evidence required | Approval or evidence gap |
| --- | --- | --- |
| Unauthorized direct API and guessed job ID | No protected metadata returned; denied before configuration/job access; test actor, role and resource matrix. | Matrix approval and tests pending. |
| Replay/concurrent submission | One durable job per valid scoped key/request; conflicting request rejected; audit and acceptance failure cases exercised. | Finite window and transaction review pending. |
| Normal fixture result / failed step | Explicit `incomplete` / `failed`, zero U1 `succeeded`, no downloadable artifact across API/UI/serializers. | Q-01 approval, compatibility and tests pending. |
| Kill process during allocation/deletion | Restart inventory and ledger converge safely; active files protected, exhaustion/outages visible, measured cleanup lag. | Q-03 approval, finite Q-05 inputs and fault tests pending. |
| Real package or restore request | Rejection before package processing and zero target mutation, even bypassing UI. | API/gateway integration tests pending. |
| Secret-bearing synthetic adapter errors | Canary absent from response/bundle/log/audit/metric; stable safe code retained. | Tests pending. |
| Accessible monitoring | Keyboard and screen-reader status/error workflows usable; automated checks supplement manual review. | UI not implemented or tested here. |
| Synthetic job response/resource use | Measure observed latency, runtime and fixture resource usage against owner-approved limits only. | No numeric targets approved; no benchmark run. |

Current Rust baseline is unknown. FEAT-01 §12's historical failures are not a new run. Toolchain availability, Rust workspace tests from `apps/api`, `pnpm test` from `apps/web`, integration dependencies and CI/coverage feasibility must be established before construction. No tests, live capture, recovery or cleanup proof were executed for this design.

## 11. Risks and technical debt

[Risk register](../../analyze/context-and-risk/risk-register.md) remains authoritative. Principal residuals: R-01 unresolved approvals; R-02 cleanup proof/real-data gap; R-03 unsupported recovery; R-04 absent finite values; R-05 resource policy; R-06 enum compatibility; R-07 current tooling; R-08 same-instance assumption; R-09 ownership adoption; R-10 translation; R-11 production exclusion; R-12 unfinished CLI. A new ledger depends on database availability and cannot guarantee physical cleanup while the host/store is down. Enum expansion may break exhaustive consumers. A synthetic pilot may be mistaken for a recovery product unless every surface states its limitation. No risk is marked closed by this architecture.

## 12. Glossary

- **U1:** Localhost, synthetic database-slice workflow increment, not usable recovery.
- **Incomplete:** Terminal constrained outcome with required full-backup components absent; not downloadable.
- **Failed:** Execution/validation/timeout/interruption failure with safe diagnostic reference.
- **Allocation ledger:** Durable identity/lifecycle index committed before controlled fixture allocation.
- **Reconciliation:** Comparison of ledger, jobs and private namespace after interruption and during sweeps; not automatic job resume.
- **Preflight preview:** Trusted synthetic UI demonstration with no real package intake and no target mutation.
- **MAP-01 / FEAT-01:** Codebase map / backup-restore feature specification linked above.
- **C / Q / R / AC / US:** Constraint, open question, risk, acceptance criterion and user story identifiers from Analyze artifacts.
