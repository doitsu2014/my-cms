# Monitoring Design — u1-backup-foundation

## Scope

This monitoring design implements the event/metric privacy model in [`observability-design.md`](../nfr-design/observability-design.md), the sensitive boundaries in [`security-design.md`](../nfr-design/security-design.md), the finite resource/operation budgets in [`performance-design.md`](../nfr-design/performance-design.md), the failure/reconciliation model in [`reliability-design.md`](../nfr-design/reliability-design.md), the bounded component topology in [`scalability-design.md`](../nfr-design/scalability-design.md), and the ownership/failure domains in [`logical-components.md`](../nfr-design/logical-components.md), based on [`components.md`](../../../inception/domain-design/components.md) and [`functional-spec.md`](../../functional-design/functional-spec.md).

Local-only diagnostics for U1. No external log aggregation, CloudWatch/X-Ray, OTLP or hosted dashboard is enabled by default. Gateway telemetry implementation is unverified and must be inspected before attaching a remote exporter.

## Metrics & KPIs

| Metric | Source | Threshold | Why it matters |
|---|---|---|---|
| `backup_job_total{status,reason_code}` | Durable job transitions | No numeric alert threshold yet; preserve every terminal outcome | Distinguishes queued/running/incomplete/failed without false success. |
| `backup_job_duration_seconds{outcome}` | Worker start/terminal timestamps | Collect baseline; threshold requires approved runtime config | Supports later timeout tuning; no SLA inferred. |
| `backup_staging_bytes_total` | Private staging adapter actual bytes | Enforce configured finite max before write | Prevents unbounded disk/storage use. |
| `backup_cleanup_total{result}` | Durable cleanup/reconciliation | Any terminal cleanup failure must surface as operational event; numeric alert window TBD | Detects orphaned sensitive data; real-data writes remain blocked if unresolved. |
| `backup_limit_rejection_total{limit_kind}` | Admission/config validation | Observe; no arbitrary threshold | Confirms fail-closed resource bounds. |
| `backup_capability_unavailable_total{component}` | Capability checks | U1 expected to identify Auth/Storage unavailable; not a success metric | Prevents incomplete slice from being misrepresented as full backup. |

Labels are bounded enums only. Never use actor, job ID, source label, object path, secret, or user data as metric labels.

## Alerts

| Alert | Condition | Severity | Routes to |
|---|---|---|---|
| Required security config absent | TLS/private/encryption/finite bound/cleanup config validation fails | Error surfaced to operator; severity policy owner to set | Local operator; do not start real-data job. |
| Staging cleanup unresolved | Delete/reconciliation failure after bounded retry or orphan remains | Security/operations incident; exact paging threshold to approve | Project/platform owner when identified; keep real-data staging disabled. |
| Job outcome unknown | Worker timeout/restart leaves running state without verified outcome | Non-success state and actionable safe event | Local operator/reconciliation path; no automatic success. |
| Audit persistence failure | Required audit write fails before side effect | Reject job start | Local operator; no data access begins. |

No numeric threshold, on-call route or SLO is invented for localhost. Before real-data use or remote deployment, owner must approve alert timing/escalation and cleanup objective.

## SLIs / SLOs

| SLI | SLO target | Measurement window |
|---|---|---|
| N/A for initial localhost U1 | No production availability/RPO/RTO objective approved | Not defined |

## Logs & tracing

- Emit structured local events with correlation ID, job ID, operation, state transition, safe reason code, component, duration and byte count where available.
- Filter/allowlist fields before serialization; never write credential, token, password, raw DB row, archive bytes, object path/content, raw idempotency key or raw upstream response.
- Instrument local spans across API → persistence → worker → adapter → staging using bounded attributes; no external exporter by default.
- If exporter later enabled, require TLS and privacy review and verify gateway support; configuration missing/invalid leaves exporter off without changing normal app behavior.
