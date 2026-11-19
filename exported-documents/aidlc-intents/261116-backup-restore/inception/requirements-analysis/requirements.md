# Requirements Analysis — Backup and Restore

## Intent analysis

Provide an operator-controlled way to back up and restore the system's persistent PostgreSQL data and image/blob storage to the S3 bucket `backup-system`. The selected policy is manual backup initiation, indefinite retention until manual deletion, operator-only restore to the same environment with replacement of current data, and best-effort capture with recorded completion status and archive-integrity verification.

## Sources

- Initial description: [desc] “Backup and Restore full system into S3 Storage in bucket backup-system. Backup includes Blob Storage of images blah blah, and Postgres Database of all database / schema this system use”
- Workflow-selected scope: [scope] express, Minimal depth and Minimal test strategy.
- Q1: [Q1] Manual-only backup; retain backups until manually deleted.
- Q2: [Q2] Restore is performed only by an operator with server access and replaces data in the same environment.
- Q3: [Q3] Capture is best-effort; completion status is recorded and archive integrity is verified.
- Q4: [Q4] Include every database and schema in the Supabase PostgreSQL instance, including Supabase-managed schemas and metadata.
- Brownfield context: [memory:M1] Code knowledge identifies PostgreSQL and Supabase Storage as separate persistence systems and does not verify an existing backup/restore facility.

## Functional requirements

### Backup creation

- **FR1 — Create a system backup.** The system shall provide an operator-invoked backup operation that stores a backup in S3 bucket `backup-system`.
  - **FR1.1 — Include the PostgreSQL instance.** Each completed backup shall contain every database and schema in the Supabase PostgreSQL instance, including Supabase-managed schemas and metadata.
  - **FR1.2 — Include image/blob objects.** Each completed backup shall contain the image/blob objects held in the system's configured object storage, together with sufficient object identifiers and metadata to associate them with the database export.
  - **FR1.3 — Record backup outcome.** Each backup attempt shall produce a durable outcome record identifying success or failure and the backup location; an incomplete attempt shall not be represented as a successful completed backup.
  - **FR1.4 — Verify archive integrity.** Before reporting a backup as successful, the system shall verify the produced backup archive's integrity and report verification failure as a failed backup.

### Restore

- **FR2 — Restore a selected backup.** An operator with server access shall be able to select an available backup in `backup-system` and restore it into the same environment, replacing the current PostgreSQL and image/blob data covered by FR1.
  - **FR2.1 — Restore database content.** A successful restore shall restore the PostgreSQL databases and schemas included in the selected backup.
  - **FR2.2 — Restore image/blob content.** A successful restore shall restore the image/blob objects included in the selected backup so their identifiers and metadata correspond to the restored database content.
  - **FR2.3 — Report restore outcome.** Each restore attempt shall report success or failure and identify the selected backup; a partial restore shall not be reported as a successful complete restore.

## Non-functional requirements

- **NFR1 — Operator-only restore access.** Restore operations shall be available only to an operator with server access; an unauthenticated application user shall not be able to initiate a restore through the application API.
- **NFR2 — Backup integrity.** A backup shall not be reported successful unless its archive-integrity check passes.
- **NFR3 — Best-effort consistency disclosure.** Backup completion records shall make clear that the selected capture policy is best-effort and does not guarantee a transactionally consistent point-in-time image across PostgreSQL and object storage.

## Constraints

- Backup destination is the S3 bucket named `backup-system`.
- Backups are initiated manually; scheduled backup execution is out of scope.
- Backups are retained until manually deleted; automatic time-based expiration is out of scope.
- Restore is operator-only and targets the same environment, replacing current data.
- Capture is best-effort; pausing writes and cross-store point-in-time consistency are not required by the selected policy.

## Assumptions

- **A1 — Target bucket and access are provisioned.** The S3 bucket `backup-system` exists and the runtime can obtain least-privilege credentials to read and write the required backup objects. Rationale: bucket provisioning and credential delivery were not specified; neither should be silently embedded in application code.
- **A2 — Restore overwrite is intentional.** An operator-selected restore replaces the current environment's covered data, as selected in Q2. Rationale: the same-environment replacement behavior is explicit, but the exact safeguard before destructive execution remains to be designed.

## Out of scope

- Scheduled backups, automatic retention expiry, and backup lifecycle policies beyond manual deletion.
- Restore into a different or newly provisioned environment.
- Guaranteed zero-downtime, write-paused, or transactionally consistent cross-store snapshots.
- Automatic provisioning of AWS accounts/buckets or credential management infrastructure.
- Backing up unrelated services or data outside the selected Supabase PostgreSQL instance and configured image/blob object storage.

## Open questions

- Should backups be encrypted client-side, rely on S3 server-side encryption, or both, and which key-management policy applies?
- What operator confirmation and pre-restore safeguards are required before replacing live data?
- Should backup and restore operations be exposed only as server-side commands, or is an authenticated administrative interface also desired for starting backups and browsing backup status?
- What backup naming, manual deletion, and audit-record retention policy is required?
