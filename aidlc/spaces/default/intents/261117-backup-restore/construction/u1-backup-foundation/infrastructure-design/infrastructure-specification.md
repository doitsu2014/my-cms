# Infrastructure Specification — u1-backup-foundation

## Scope and deployment

This design implements the U1 security posture in [`security-design.md`](../nfr-design/security-design.md), performance budgets in [`performance-design.md`](../nfr-design/performance-design.md), bounded topology in [`scalability-design.md`](../nfr-design/scalability-design.md), recovery/cleanup behavior in [`reliability-design.md`](../nfr-design/reliability-design.md), event boundaries in [`observability-design.md`](../nfr-design/observability-design.md), and logical ownership in [`logical-components.md`](../nfr-design/logical-components.md). The application boundary follows [`components.md`](../../../inception/domain-design/components.md), and U1 workflows follow [`functional-spec.md`](../../functional-design/functional-spec.md).

U1 is developed and verified on localhost. No new cloud account, AWS resource, production target, deployment environment, or release step is approved. Existing My-CMS gateway and local Supabase topology remain the environment. This is a no-new-provisioning design; it documents local prerequisites and gates only.

### Deployment

| Facet | Choice | Rationale |
|---|---|---|
| Compute model | Existing `my-cms-api` gateway process; bounded local worker task | Fits approved U1 localhost slice; Tokio task is not treated as a durable queue or restart guarantee. |
| Networking topology | Existing localhost/Docker network and gateway entrypoint; no new ingress/VPC/LB | No remote deployment is in scope. Admin/API, database and staging connections retain TLS requirements; no unapproved plaintext exception. |
| Storage strategy | Existing configured database for job metadata; staging requires a private, encrypted, finite, cleanup-capable storage boundary. No new object-store provider is selected in U1. | Prevents unsupported AWS or object-store assumption. Real-data staging stays disabled unless TLS, encryption-at-rest, bounds/expiry and crash-safe cleanup are all configured and verified. |
| Environments | Local development/test only. No dev/staging/prod deployment resources provisioned by this Unit. | Team affirmed localhost-only/no release task; promotion remains unresolved and outside this scope. |
| IaC approach | No IaC changes for U1. Reuse existing local Compose/configuration as found in source. | No new infrastructure is approved; exact compose/runtime details must be source-checked before operation. |
| Resource sizing | No fixed capacity claim. Finite byte/time/temp/concurrency settings are mandatory runtime configuration; missing/invalid values disable data writes. | No measured dataset, throughput or numeric capacity target supplied. Benchmark before selecting values. |

### Infrastructure services

| Service | Role | Configuration | Notes |
|---|---|---|---|
| Existing gateway / `my-cms-api` | application host | Existing local app configuration; server-side instance resolver; bounded worker settings | No new service/container. Actual route and TLS behavior must be validated against source. |
| Existing PostgreSQL | database | Existing gateway DB connection; TLS as required; domain-owned migrations/metadata ownership to be verified | Store job/idempotency/audit metadata only; do not transfer ownership of CMS source tables. |
| Existing local Supabase Storage / private staging boundary | staging storage | Must be private, TLS-protected and encrypted at rest; finite limits, expiry and crash-safe orphan cleanup required | Exact storage provider is not chosen. If existing local storage cannot meet controls, do not stage real data. |
| Existing server-side configuration/secret source | secrets/config | Opaque configured instance IDs to browser; credentials never returned; validate required values before job accepts data | Exact secret provider/config source not yet verified. |
| External telemetry/export service | monitoring | Disabled by default; can be enabled only after config/privacy/TLS verification | Gateway OTLP behavior is documented as unverified; no external exporter is assumed. |

### Shared infrastructure

No shared resource is provisioned by U1. Package object storage, retention, deletion, encryption key ownership, and cross-Unit sharing are deferred until the policy and owner are confirmed. U1 must not create a shared S3 bucket or a new database/schema by assumption.

## Infrastructure prerequisites and gates

1. Verify current gateway, DB and local Supabase configuration paths and TLS behavior; keep real-data path disabled where required TLS cannot be met.
2. Verify a private staging adapter with encryption-at-rest and least privilege; no public object URL.
3. Require finite staging bytes, temp disk, worker concurrency, operation/job deadlines and expiry configuration. Missing/invalid required configuration refuses before first data byte.
4. Implement/test durable metadata plus orphan discovery and cleanup after process restart. Cleanup failure is observable and blocks further real-data staging; no absolute expiry guarantee is claimed while storage is unavailable.
5. Verify the existing source/build scripts before setting the separately approved verification command. CI workflow `.github/workflows/ci-my-cms.yml` is documented as temporarily disabled; this Unit does not claim CI success or change the workflow.
6. No production deployment, cloud resources, pipeline, IAM policy or cost estimate is applicable to the approved localhost scope.
