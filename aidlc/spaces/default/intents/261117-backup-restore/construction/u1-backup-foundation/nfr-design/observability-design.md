# Thiết kế quan sát hệ thống — u1-backup-foundation

## Event and metric pipeline

Reuse gateway's existing structured logging/tracing facade after source verification. U1 emits local events by default; do not enable OTLP or any external exporter merely because a config name exists. External export requires explicit operator configuration and verified privacy/transport policy.

### Structured event fields

Allowed: `correlation_id`, `job_id`, `actor_id` (audit only, not metric label), configured source instance ID/label (restricted audit), operation, from/to status, safe reason code, component enum, duration, byte count, digest, retry count, cleanup result. Values are validated/normalized; display labels should not contain secrets.

Forbidden: endpoint, URL with credentials, tokens, password, raw idempotency key, SQL/query text, raw DB rows, object path/content, archive contents, raw upstream body or exception string.

### Metrics

- `backup_job_total{status,reason_code}`; bounded enum labels only.
- `backup_job_duration_seconds{outcome}` and component duration with bounded component names.
- `backup_staging_bytes_total`, `backup_cleanup_total{result}`, `backup_limit_rejection_total{limit_kind}`.
- No actor, instance-specific secret, job ID, object key or user data labels.
- Progress is returned only when actual completed/total counters exist; otherwise UI status is indeterminate/text only.

### Audit

Audit rows record actor/source/job/time/digest/outcome/reason code. Pre-side-effect audit failure rejects job execution. Post-failure audit persistence is retried boundedly and reported as an operational error; it cannot change a failed/incomplete job into success.

### Alerting / SLO

No production availability or RPO/RTO SLO is set for localhost U1. Before real-data/remote operation, approve expiry cleanup objective, deletion-failure alert threshold, job duration/size budgets and recovery escalation. Local logs and metrics must remain useful without exporter configured.
