# Cross-Unit Traceability — Backup and Restore

No Unit DAG exists for this express workflow; all coverage is stage-level.

## Verdict: FAIL — uncovered functional requirements and NFRs

| ID | Status | Owning stage | Evidence/target |
|---|---|---|---|
| FR1.1 | GAP | Code Generation | Full-instance PostgreSQL export not implemented; including all Supabase-managed schemas/metadata is unverified. |
| FR1.2 | GAP | Code Generation | No configured object-storage inventory/copy or S3 transfer adapter. |
| FR1.3 | GAP | Code Generation | No durable backup outcome/location record. |
| FR1.4 | GAP | Code Generation | No completed archive or end-to-end integrity verification. |
| FR2.1 | GAP | Code Generation | Restore disabled pending a safe complete Supabase database recovery process. |
| FR2.2 | GAP | Code Generation | Object restore disabled. |
| FR2.3 | GAP | Code Generation | No durable restore result record. |
| NFR1 | OK | Code Generation | Operator-only CLI surface: `apps/api/gateway/src/backup_cli.rs`; no application endpoint. |
| NFR2 | GAP | Code Generation | No archive creation/integrity verification or successful backup path. |
| NFR3 | GAP | Code Generation | Manifest model indicates best-effort, but no completed backup record exists. |

The sensor traceability result reported `pass: false` with the nine uncovered IDs above. This result is retained; uncovered requirements are not represented as implemented.
