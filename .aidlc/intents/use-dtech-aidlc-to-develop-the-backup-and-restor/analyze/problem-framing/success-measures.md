# Success Measures

## Framing-level outcomes
- **SM-01 — Scope clarity:** The approved work scope distinguishes U1 localhost/database-only behavior from the broader target backup/restore feature; no unsupported completeness, consistency, or production-readiness claim is made.
- **SM-02 — Safe U1 behavior:** Any U1 database-only job ends `incomplete` or `failed`, never `succeeded`; incomplete artifacts cannot be downloaded; real-data staging remains disabled until finite limits and durable crash cleanup/reconciliation are proven (`docs/features/backup-restore.md`, §§10.1–10.2).
- **SM-03 — Unblocked design:** Before code generation resumes, persistence/migration ownership, durable staging cleanup/reconciliation, and the public/internal terminal-state mapping are explicitly resolved (`docs/features/backup-restore.md`, §12).
- **SM-04 — Verifiable requirements:** Each in-scope behavior has observable Given/When/Then criteria and a linked user story in `acceptance-criteria.md` and `user-stories.md`.

## Broader target outcomes (not claims of U1 delivery)
The long-term target is a server-authorized admin UI for selecting configured source/target instances, creating and verifying a complete package, safely preflighting uploads, requiring explicit restore confirmation, and reporting verified outcomes without partial-success claims (`docs/features/backup-restore.md`, §§1–7). Full package success requires database, supported Auth/GoTrue, and Storage components; do not claim point-in-time consistency unless proven (`§§2.1, 6`).

## Production restore readiness gate
Production restore remains disabled until all applicable conditions in `docs/features/backup-restore.md` §7 are verified and approved: supported schema/Auth recovery; Storage objects and metadata recovery; published consistency/session limits; encryption, key management, upload/package bounds and retention; tested recovery-point/rollback/failure handling; a successful representative restore drill with owner-approved recovery targets; and any required second-person approval. These are readiness measures, not evidence that the conditions currently pass.

## Measures pending owner decisions
Numeric package, workload, RPO/RTO, SLO, retention, and drill thresholds have not been approved. Do not invent defaults or treat unmeasured progress as a percentage. Resolve the questions in `docs/features/backup-restore.md` §§8 and 10 before using such values as acceptance targets.