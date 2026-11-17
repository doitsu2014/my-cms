# Unit of Work — Backup & Restore

## Scope and decomposition rationale

Units are coarse-grained and organized as independently testable vertical slices, consistent with the team's preference for an end-to-end slice before expansion. This document defines dependency topology only; Delivery Planning selects the economic execution order.

Version 1 targets localhost with configured Supabase self-hosted instances. It has no production restore/release unit. Source entities stay under current My-CMS/Auth/Storage ownership; Units build the Backup & Restore orchestration and its adapters/interfaces.

## Units

| Unit ID | Directory | Kind | Description | Complexity | Deployment model |
|---|---|---|---|---|---|
| U1 | `u1-backup-foundation` | service | Establish server-side configured instance boundary, admin API, backup job/package metadata, idempotency and status; prove a minimal localhost vertical slice that creates and lists a backup job. | L | Embedded in `my-cms-api` gateway; not separately deployed. |
| U2 | `u2-backup-package` | library | Capture allowed My-CMS/Auth/Storage components through adapters, assemble a versioned ZIP/manifest/checksums, store/download only verified packages; exercise complete backup journey on localhost. | XL | Library/module embedded in gateway/worker process. |
| U3 | `u3-restore-preflight` | library | Upload ZIP, enforce archive limits, verify manifest/checksums and instance compatibility, then present non-mutating preflight to admin. | L | Library/module embedded in gateway. |
| U4 | `u4-restore-local` | service | Confirm target, run local-only restore state machine with per-component checkpoints/recovery, validate result and expose job status; production target is excluded. | XL | Embedded in `my-cms-api` gateway and its localhost task runtime. |
| U5 | `u5-admin-backup-ui` | ui | Deliver two-tab admin page for create/download and upload/preflight/confirm/local restore, job history/details, warnings and accessible states. | L | Existing `apps/web` application. |
| U6 | `u6-backup-audit-hardening` | library | Complete audit redaction, retention/cleanup and configured security limits; ensure local workflows cannot target production. | M | Embedded in gateway and existing admin UI as applicable. |

## Unit responsibilities and constraints

### U1 — `u1-backup-foundation`

- Own server-side instance IDs/labels, admin-only API boundary and durable metadata contract.
- Define backup/restore job lifecycle statuses and idempotency key semantics.
- Persist job/package metadata through `BackupPersistence`; no credentials in browser payloads.
- Bootstrap minimal vertical slice: configured local instance appears; admin starts a job; job ID and status persist and render.
- Does not yet claim a complete backup archive or destructive restore.

### U2 — `u2-backup-package`

- Add ZIP writer, manifest, per-component inventory and cryptographic checksum.
- Add narrow source adapters without taking ownership of source tables/entities.
- Validate self-hosted Supabase/PostgreSQL compatibility and enumerate Storage objects with pagination.
- Report best-effort consistency; fail rather than claim a complete package when required data/Auth/object enumeration fails.
- Save package in configured object store; actual retention, size and at-rest protection configuration must be decided before real data use.

### U3 — `u3-restore-preflight`

- Upload package privately and inspect it as untrusted input.
- Reject traversal, symlink, duplicate/conflicting entries, archive bombs, unsupported versions, checksum errors and out-of-policy sizes.
- Validate source/target compatibility; no target mutation during upload/preflight.
- Return a preflight result with replace scope, consistency caveat and warnings.

### U4 — `u4-restore-local`

- Require explicit target confirmation and a verified recovery point before modifying target.
- Run per-component saga/state transitions; never assume a transaction spans database/Auth/Storage.
- On failure, roll back or mark recovery-required; success only after post-restore validation.
- Hard reject production instance labels in the first-release local-only configuration.
- Use local Tokio task runtime as the initial assumption; Functional Design must decide leases, restart behavior and persistence synchronization.

### U5 — `u5-admin-backup-ui`

- Implement two tabs, instance selection, create/upload, preflight, destructive confirmation, job history/details and safe errors.
- Warn that downloaded backup packages contain sensitive data and the admin must protect the downloaded file.
- Use existing localization, component/theme conventions and target WCAG 2.1 AA.
- Do not implement production restore UI/target in the first release.

### U6 — `u6-backup-audit-hardening`

- Redact tokens, passwords, credentials, object contents and database rows from audit/log/metrics.
- Ensure audit identifies actor, configured source/target, package/job digest and outcome.
- Configure upload/extraction/concurrency/timeout/retention limits; exact numeric thresholds remain design decisions.
- Ensure clean staging cleanup and no production targeting in localhost-only scope.

## Interface boundaries

- `BackupAdminApi` accepts only configured instance identifiers and authenticated admin operations.
- `BackupRestore` owns job state machine and use-case coordination.
- `BackupWorker` runs bounded long jobs in the gateway's local runtime and resumes/fails safely according to state persisted via `BackupPersistence`.
- `DatabaseBackupAdapter`, `AuthBackupAdapter`, and `StorageBackupAdapter` own integration details but not source data entities.
- `ArchiveIntegrity` owns ZIP safety, manifest format and checksum verification.
- `apps/web` calls only the protected API and never connects directly to database/Auth/object store.

## Out of scope for Units

- Restore to production or automatic deployment/release.
- Supabase hosted platforms or unconfigured arbitrary instances.
- Moving CMS/Auth/Storage source-data ownership to BackupRestore.
- Custom cross-instance ID/URL/bucket mapping.
- Claiming atomic point-in-time backup across DB/Auth/Storage.
- Executing restore scripts included in ZIP.
