# Acceptance Criteria

Criteria below frame the documented target and U1 constraints; they do not imply that unresolved capabilities are approved or implemented. U1-specific criteria are identified separately.

## AC-001 Instance selection and authorization
- Statement: The system shall expose only configured, permitted instances to authorized administrators.
- Rationale: Prevent operation on unintended environments and disclosure of instance configuration.
- Source: `docs/features/backup-restore.md` §§3, 5; US-01, US-05.
- Story: US-01, US-05

### Scenario: Authorized administrator has an eligible instance
- Given an authenticated administrator has at least one configured instance permitted for the operation
- When they open the relevant Backup & Restore workflow
- Then the UI lists the permitted instance with a recognizable environment label

### Scenario: Unauthorized instance listing
- Given the actor is not authorized as an administrator
- When they request the instance list directly
- Then the server denies access and does not disclose the list

## AC-002 Backup job visibility and truthful completion
- Statement: The system shall report backup job status and expose a package for download only when it is complete and verified for its declared scope.
- Rationale: Prevent incomplete or unverifiable data from being represented as restorable.
- Source: `docs/features/backup-restore.md` §§3, 5, 10.2; US-02.
- Story: US-02, US-06

### Scenario: Job status is observable
- Given an authorized administrator has started a backup job
- When they view that job
- Then the UI shows its reference and current status, with progress only when measured and safe error text when applicable

### Scenario: Required capture fails
- Given a component required by the declared backup scope cannot be fully captured or verified
- When the job reaches a terminal outcome
- Then the job is not reported as a successful complete backup and its artifact is not offered as a restorable package

### Scenario: U1 database-only result
- Given a U1 localhost job captures only the database slice and does not capture required Auth and Storage components
- When the job terminates
- Then its outcome is `incomplete` or `failed`, never `succeeded`, and its incomplete artifact is not downloadable

## AC-003 Upload and preflight are non-mutating
- Statement: The system shall reject unsafe or incompatible packages during preflight without changing target data.
- Rationale: Inspection must not trigger destructive restore effects.
- Source: `docs/features/backup-restore.md` §§3, 5; US-03.
- Story: US-03

### Scenario: Invalid or incompatible package
- Given an uploaded package is corrupt, unsupported, incompatible, missing required content, or has an invalid checksum
- When preflight evaluates it
- Then the system rejects it with a safe reason and leaves target data unchanged

### Scenario: Unsafe archive
- Given a package exceeds configured finite limits or contains an unsafe path, symlink, duplicate/conflicting entry, or archive-bomb condition
- When the system validates the package
- Then it rejects the package before restore and leaves target data unchanged

### Scenario: U1 real-data staging is not enabled
- Given durable finite-expiry cleanup and crash reconciliation for real-data staging have not been designed and tested
- When a U1 workflow would stage real backup/restore data
- Then staging and real-data restore remain disabled

## AC-004 Explicit restore authorization and safe outcome
- Statement: The system shall require explicit confirmation of the target and replacement impact before any restore side effect.
- Rationale: Restore replaces data and can cause irreversible loss.
- Source: `docs/features/backup-restore.md` §§2.3, 3, 5; US-04.
- Story: US-04

### Scenario: Confirmation describes impact
- Given preflight has passed for a package and target
- When the system asks for restore confirmation
- Then it identifies the package, target environment, and data in scope that will be replaced

### Scenario: Restore is cancelled
- Given the administrator has not confirmed or cancels
- When they leave the confirmation step
- Then no restore runs and target data remains unchanged

### Scenario: Restore failure
- Given a restore component or post-restore verification fails
- When the job ends
- Then the system does not report success and reports rollback status or a recovery-required/manual-intervention outcome

## AC-005 Protected operations, resources, and audit
- Statement: The system shall enforce authorization server-side and record safe audit metadata for sensitive operations.
- Rationale: Protect instance data, packages, and secrets from unauthorized use or disclosure.
- Source: `docs/features/backup-restore.md` §§5, 10.1; US-05.
- Story: US-05, US-06

### Scenario: Unauthorized operation or resource access
- Given an actor lacks administrator or resource-level authorization
- When they invoke an operation or request a job/package by identifier
- Then the server denies access without returning the protected resource

### Scenario: Audit records a sensitive operation
- Given a backup or restore operation is attempted and audit storage is available
- When the operation is audited
- Then the record includes approved actor/instance/job/time/outcome metadata but excludes credentials, tokens, passwords, raw idempotency keys, and database or object contents

### Scenario: Audit store is unavailable before a sensitive side effect
- Given audit storage is unavailable
- When a sensitive operation would begin
- Then the operation is refused before the side effect

## AC-006 Unresolved scope and readiness decisions
- Statement: The system owner shall resolve material scope and safety decisions before the affected capability is treated as approved for implementation or production use.
- Rationale: Avoid inventing platform guarantees or unsafe operating defaults.
- Source: `docs/features/backup-restore.md` §§7–8, 10, 12.
- Story: US-02, US-03, US-04

### Scenario: Production restore readiness
- Given production restore is proposed
- When readiness is assessed
- Then supported Auth/schema and Storage recovery, consistency limits, encryption/key handling, finite limits and retention, tested recovery/rollback, and an approved representative restore drill are verified, and any required second-person approval is applied

### Scenario: Unresolved U1 design blockers
- Given persistence/migration ownership, durable cleanup/reconciliation design, or terminal-state contract mapping remains unresolved
- When development is considered for resumption beyond analysis
- Then the unresolved blocker is explicitly surfaced and is not represented as decided