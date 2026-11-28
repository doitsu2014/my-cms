# User Stories

## Users
- System administrator: authorized operator of the admin Backup & Restore workflows.
- Operations staff: monitors jobs and handles failure/recovery procedures.
- CMS end users: indirectly affected by data protection and restore outcomes; do not directly use this feature.
- System/product owner: decides scope, safety policy, and readiness.

## Stories
### US-01 Select an allowed instance
- Story: As a system administrator, I want to select a configured instance that I am permitted to use, so that I operate on the intended environment.
- Scope: in-scope target behavior; exact supported environments remain open.
- Criteria: AC-001

### US-02 Create and monitor a backup
- Story: As a system administrator, I want to start and monitor a backup job and obtain only a complete, verified package, so that I can protect data without mistaking a partial capture for a usable backup.
- Scope: target behavior; U1 is localhost/database-only and must end `incomplete` or `failed`, with incomplete artifacts not downloadable.
- Criteria: AC-002, AC-006

### US-03 Upload and preflight a package
- Story: As a system administrator, I want to upload a package and review safe compatibility/integrity checks before restore, so that invalid input cannot alter the target.
- Scope: target behavior; U1 real-data staging remains disabled until durable finite-expiry cleanup/reconciliation is proven.
- Criteria: AC-003

### US-04 Confirm restore impact and observe outcome
- Story: As a system administrator, I want to explicitly confirm the target and replacement scope and receive a truthful final result, so that I can avoid accidental overwrite and respond to failures.
- Scope: target behavior; production restore is not authorized by this framing.
- Criteria: AC-004, AC-006

### US-05 Protect operations and sensitive data
- Story: As the system owner, I want server-side authorization, protected package/job access, and safe audit records, so that backup and restore do not expose instance data or credentials.
- Scope: in-scope target safety requirements.
- Criteria: AC-001, AC-005

### US-06 Diagnose job outcomes
- Story: As an administrator or operations staff member, I want job references, safe failure reasons, and recovery status, so that I can determine whether intervention is required.
- Scope: in-scope target behavior, within measured/available telemetry.
- Criteria: AC-002, AC-005

## Open decisions
See `docs/features/backup-restore.md` §§7–8, 10, and 12. In particular: supported Supabase hosting/version, schema and Auth scope, consistency, cross-instance mapping, finite package limits/storage/retention, production authorization, RPO/RTO, restore drills, persistence/migration ownership, cleanup/reconciliation, and the `incomplete` terminal-state contract mapping remain unresolved.