# Requirements Analysis Questions

## Q1. [Operations engineer] Backup schedule and retention
How should backups be initiated and retained? This determines the operational workflow and storage growth.

A. Run backups only when an administrator starts them; retain them until manually deleted
B. Run backups on a schedule; retain the most recent 30 days
C. Run backups on a schedule; retain the most recent 90 days
D. Support both scheduled and administrator-triggered backups; retain 30 days
X. Other (please specify)

[Answer]: A. Run backups only when an administrator starts them; retain them until manually deleted

## Q2. [Product manager] Restore access and target
Who can restore a backup, and where should it restore? This affects authorization and protection against accidental replacement of live data.

A. Only an operator with server access; restore into the same environment, replacing its current data
B. An authenticated administrator through the application; restore into the same environment, replacing its current data after explicit confirmation
C. Only an operator with server access; restore into a separate empty environment for verification or recovery
D. An authenticated administrator through the application; restore into a separate empty environment for verification or recovery
X. Other (please specify)

[Answer]: A. Only an operator with server access; restore into the same environment, replacing its current data

## Q3. [Developer] Backup consistency and verification
PostgreSQL and Supabase Storage are separate persistence systems. What should a backup guarantee when content changes during capture, and how should a completed backup be checked?

A. Best-effort capture; record completion status and verify archive integrity
B. Prevent writes during capture; verify archive integrity and that database and object files are present
C. Use a point-in-time database snapshot and capture referenced storage objects consistently; verify archive integrity and database/object manifest completeness
D. Other (please specify)
X. Other (please specify)

[Answer]: A. Best-effort capture; record completion status and verify archive integrity

## Q4. [Developer] PostgreSQL backup boundary
The project uses Supabase PostgreSQL, which includes application data and platform-managed schemas. Which database contents must the backup include?

A. Every database and schema in the Supabase PostgreSQL instance, including Supabase-managed schemas and metadata
B. All My-CMS application databases and schemas, including application migrations and required extensions, but excluding Supabase platform-internal schemas and metadata
C. Only the My-CMS application database and its application-owned schemas; exclude other databases and platform schemas
D. Other (please specify)
X. Other (please specify)

[Answer]: A. Every database and schema in the Supabase PostgreSQL instance, including Supabase-managed schemas and metadata

## Consolidated Summary Confirmation

- Manual operator-triggered backups to S3 bucket `backup-system`; retain until manually deleted.
- Restore is restricted to an operator with server access and replaces covered data in the same environment.
- Backup captures PostgreSQL and configured image/blob storage on a best-effort basis; completion status and archive integrity are checked.
- Include every database and schema in the Supabase PostgreSQL instance, including Supabase-managed schemas and metadata.
- Current unresolved details include encryption/key policy, restore safeguards, exact operator interface, backup naming/audit record retention, and whether S3 provisioning is pre-existing.

Does this all look correct before I generate the requirements artifact?

Looks correct
Request changes

[Answer]: Looks correct
