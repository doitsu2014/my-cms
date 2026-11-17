# Unit of Work Story Map — Backup & Restore

The map assigns each story to the Unit whose acceptance criteria it implements. Requirements NFRs are mapped separately in traceability where they have explicit owner Units. This is capability coverage, not an economic sequence.

| Story ID | Unit ID | Directory | Implemented coverage |
|---|---|---|---|
| US1.1 | U1 | u1-backup-foundation | Configured source/target labels and admin-only instance selection |
| US1.2 | U2 | u2-backup-package | Backup job execution, DB/Auth/Storage capture, ZIP, manifest, checksum, best-effort declaration |
| US1.3 | U2; U5; U6 | u2-backup-package; u5-admin-backup-ui; u6-backup-audit-hardening | Verified package download, long-term object-store metadata/policy, sensitive-file warning and retention/security checks |
| US2.1 | U3; U5 | u3-restore-preflight; u5-admin-backup-ui | Upload, archive validation, integrity/version/compatibility checks, read-only preflight UI |
| US2.2 | U4; U5 | u4-restore-local; u5-admin-backup-ui | Destructive confirmation, recovery point, saga restore, post-validation and failed recovery UI |
| US3.1 | U1; U4; U5 | u1-backup-foundation; u4-restore-local; u5-admin-backup-ui | Durable job history/status, per-component outcome and failure/recovery UI |
| US3.2 | U1; U5; U6 | u1-backup-foundation; u5-admin-backup-ui; u6-backup-audit-hardening | Audit metadata, sensitive-data redaction and security verification |

## Cross-cutting acceptance mapping

- Exact duplicate-submit/idempotency for starting backup is owned by U1 and exercised in U2's end-to-end backup flow.
- Accessibility and localization behavior is implemented in U5 and verified against its source stories/requirements; NFR6 and broader quality policy are verified in Build and Test, not duplicated as standalone functionality here.
- Each mapped Unit's functional implementation must retain its story's full acceptance criteria; cross-cutting entries identify additional implementing Units, not reduced scope.
