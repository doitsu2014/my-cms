# Thiết kế độ tin cậy — u1-backup-foundation

## Job state and truthful outcomes

- Persist job before returning accepted; state transitions use expected state/version and append audit evidence.
- U1 terminal status for DB-only slice is `incomplete` (private staging exists) or `failed`; `succeeded` is not available in this flow. Unknown adapter outcome, timeout, crash or checksum/config mismatch remains non-success.
- Retry only transient, idempotent operations with bounded attempts and jitter. Never blindly retry DB mutation or cleanup/delete without a stable idempotency/reference contract.
- Tokio task is an execution mechanism, not a durable queue. Startup reconciliation marks stale `running` jobs unknown/non-success and inspects staging metadata; automatic resume is off until side-effect idempotency and lease protocol are verified.

## Staging recovery

Artifact lifecycle record commits artifact ID, namespace, size/digest when possible, job ID, expiry and lifecycle state. Orphan detection covers object-created/metadata-not-committed windows using deterministic isolated namespace and conservative age policy. Cleanup is idempotent, bounded and restartable. `DELETE_FAILED` emits operational signal and keeps real-data staging disabled until recovery; never claim artifacts are deleted merely because expiry elapsed.

Staging encryption-at-rest, private access and TLS are required. If any required storage/recovery policy is absent, reject before writing data. There is no absolute deletion SLO while the storage service is unavailable; a measurable expiry objective and alert/escalation threshold must be approved before real data.

## Failure handling table

| Failure | Result | Recovery behavior |
|---|---|---|
| Job persistence fails before accept | No accepted job | Safe 5xx; no worker work. |
| Source config absent/invalid | `failed` | No adapter read or staging write; safe reason. |
| Worker timeout before known side effect | `failed` or `incomplete` based on committed staging record | Reconcile state; no success. |
| Timeout/connection loss after possible side effect | Non-success/unknown | Do not retry mutation; require reconciliation. |
| Missing required Auth/Storage | `incomplete` if database staging exists, else `failed` | No complete package or download. |
| Metadata write fails after bytes written | Non-success | Reconciliation discovers deterministic orphan; cleanup; real-data path stays disabled. |
| Cleanup fails | `DELETE_FAILED` observable | Bounded retry/escalation; no claim of deletion; disable new real-data writes for affected scope. |
| Audit fails before data access | Refuse job execution | No data side effect. |

## Health and checks

- Health/readiness semantics reuse gateway patterns only after source verification; do not add a deep dependency check that turns unrelated API readiness into failure.
- Readiness for the backup capability should reflect required config availability without disclosing secret state.
- Restore rollback/recovery, cancellation, durable lease and restart semantics are not claimed by U1; downstream Unit/design must define them before restore execution.
