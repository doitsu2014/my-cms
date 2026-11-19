# Thiết kế bảo mật — u1-backup-foundation

## Security boundary

U1 là localhost-only nhưng xử lý database có thể nhạy cảm. Không nới TLS hoặc access-control requirement theo suy luận rằng loopback “đủ an toàn”. Cho tới khi đúng TLS, private staging, encryption-at-rest và cleanup/reconciliation được cấu hình và kiểm chứng, chỉ synthetic fixtures được ghi vào staging; real-data path disabled.

Nguồn: [`security-requirements.md`](../nfr-requirements/security-requirements.md), [`functional-spec.md`](../../functional-design/functional-spec.md), [`rules.md`](../../functional-design/rules.md), [`contract-summary.md`](../../../inception/contract-design/contract-summary.md), upstream [`requirements.md`](../../../inception/requirements-analysis/requirements.md).

## Authentication and authorization

- Tận dụng middleware/session/admin role của gateway hiện hữu; exact JWT role claim, cookie/CSRF mechanism cần source verification trước Code Generation.
- Enforce admin role server-side tại mọi route: instance list, create job, status read, package/staging references.
- Mỗi resource lookup kiểm tra owner/access policy; ID không phải capability. Với request không authorized, trả safe denial theo contract, không leak resource existence beyond agreed policy.
- Resolve `sourceInstanceId` từ allowlist server-side; request schema chấp nhận opaque ID, không nhận URL/connection string/secret/key.
- Role/resource check xảy ra trước DB content access, archive I/O hoặc mutation.

## Transport and trust boundaries

| Hop | Security requirement | U1 default/guard |
|---|---|---|
| Admin browser → API | TLS per NFR1.12 and C1 (HTTPS/JSON) contract | Nếu localhost gateway không có TLS-compatible path thì real-data mode bị disable; không thêm HTTP loopback exception. |
| API → PostgreSQL | TLS per requirement | Require approved encrypted connection configuration; no plaintext remote fallback. |
| API/worker → private staging/object store | TLS in transit + encryption at rest | Require private endpoint, validated TLS identity, encryption-at-rest policy and least-privilege identity; missing config refuses before write. |
| Worker → configuration/secret provider | Server-only trusted boundary | Credentials resolved server-side just-in-time; never serialized to job record or frontend DTO. |

Certificate trust, local dev CA/proxy provisioning and secret-provider implementation are not chosen here; NFR Design/Code Generation must verify existing repository mechanisms and keep real-data access disabled until they satisfy the contract.

## Staging confidentiality and cleanup

- Define a `PrivateStagingPort` behind an adapter; only the worker/service identity may write/read/delete. No public object URL or `private_storage_ref` leaves the server boundary.
- Before first byte: validate finite max bytes, temp budget, expiry, encryption-at-rest and durable cleanup capability. Config is typed, fails on absent/invalid/overflow values, and has no unlimited default.
- Use an explicit artifact lifecycle: `ALLOCATING → WRITING → INCOMPLETE → DELETING → DELETED`, with `DELETE_FAILED` observable and bounded retries. A manifest/metadata record is committed with an immutable artifact ID and expected size/digest; an object created before metadata commit is discoverable as an orphan by deterministic namespace and age policy.
- Startup/reconciliation worker scans only the U1 staging namespace, joins records to objects, identifies unreferenced/expired artifacts, and deletes through the same private port. It is idempotent and resumes after restart. It never promotes artifacts to complete.
- If delete remains unavailable after retry budget, emit security event/metric without sensitive identifiers, retain a safe incident reference, alert/require operator action, and keep real-data staging disabled for affected scope. The design does not claim an absolute deletion-time guarantee while storage is unavailable; configured expiry is an objective with observable failure/escalation.
- Do not use process-shutdown hooks as sole cleanup. A crash-safe store-native lifecycle rule may be an additional defense only after configured and verified.

## Input and archive controls

- Parse strict request DTOs with unknown fields rejected; reject endpoint, arbitrary path, config override and credentials.
- Validate source ID, idempotency key size/format, request digest, and actor scope before persistence.
- Archive paths are relative canonical paths; reject traversal, absolute paths, symlinks, duplicate/conflicting names, excessive expansion, entries, depth and byte/time budgets before extraction.
- U1 staging uses an incomplete label and cannot reach complete-package download handlers. Content is never executed.

## Secrets, errors and audit

- Secret sources remain gateway configuration/secret mechanisms; redact at source, not only at UI.
- Safe error DTO contains stable code, safe localized message key and correlation/job ID. Raw SQL, HTTP response bodies, DB rows and filesystem secrets do not cross the boundary.
- Audit records actor, configured source ID, job ID, timestamp, outcome, digest if available, and reason code. It excludes raw idempotency keys and payloads.
- If required audit persistence fails before data read/staging side effect, reject the operation. Audit write after terminal operation failure is retried boundedly and surfaced as an operational failure; never report successful backup.

## Security verification

- Integration tests for unauthenticated, non-admin, wrong-owner, guessed IDs, disabled instance and unknown request fields.
- Secret canary scan for response, logs, audit, metrics, test errors and browser bundles.
- Storage policy tests for private access, at-rest encryption configuration, TLS validation and no public URLs.
- Failure injection at every staging lifecycle boundary including crash after bytes but before metadata, metadata before terminal transition, restart, expiry and delete failure.
- Archive adversarial tests for traversal/symlink/conflict/bomb/limit cases; no target mutation and no code execution.
- Negative gate: real-data mode refuses when any TLS, secret, finite-limit, expiry, durable-cleanup or recovery requirement is missing/unverified.
