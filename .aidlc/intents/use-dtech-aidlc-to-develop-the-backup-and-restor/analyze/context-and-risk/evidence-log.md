# Evidence Log

## Purpose
This log records every source consulted during the context-risk analysis, what evidence was extracted, and at what revision.

## Sources Consulted

| Source ID | Path | Evidence Taken | Label | Commit |
|-----------|------|----------------|-------|--------|
| REQ-01 | `.aidlc/request.md` | Initial request: develop backup/restore feature based on feature spec | Observed | 9c7e0f3 |
| FEAT-01 | `docs/features/backup-restore.md` | Complete feature specification in Vietnamese: target scope (database/Auth/Storage ZIP), U1 constraints (localhost, database-only, must end incomplete/failed), business rules, user stories, acceptance criteria, NFRs, release conditions, open questions, U1 blockers (persistence/migration ownership, cleanup/reconciliation, status contract mismatch) | Observed | 9c7e0f3 |
| PROB-01 | `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/analyze/problem-framing/problem-statement.md` | Problem: admin UI workflow does not exist; CLI skeleton present but incomplete; prior U1 stopped without app source changes due to unresolved blockers | Observed | 9c7e0f3 |
| STORY-01 | `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/analyze/problem-framing/user-stories.md` | Six user stories (US-01 through US-06) covering instance selection, backup creation/monitoring, upload/preflight, restore confirmation, security, and diagnostics; open decisions list | Observed | 9c7e0f3 |
| AC-01 | `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/analyze/problem-framing/acceptance-criteria.md` | Five acceptance criteria (AC-001 through AC-006) with Given/When/Then scenarios; U1-specific constraints explicitly called out | Observed | 9c7e0f3 |
| STAKE-01 | `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/analyze/problem-framing/stakeholder-map.md` | Stakeholders: system admin, operations staff, CMS end users, system/product owner, dev/QA/security teams; ownership of open decisions unresolved | Observed | 9c7e0f3 |
| SUCCESS-01 | `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/analyze/problem-framing/success-measures.md` | Four framing-level success measures; production restore gate conditions; pending owner decisions on numeric thresholds | Observed | 9c7e0f3 |
| INTENT-01 | `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/intent.md` | Intent label and one-line description | Observed | 9c7e0f3 |
| MAP-01 | `.aidlc/knowledge/_codebase-map.md` | System architecture: Rust API gateway (Axum/Tokio/SeaORM), React admin UI, Supabase self-hosted (PostgreSQL/GoTrue/Storage); backup_cli.rs exists at apps/api/gateway/src/; create/list fail (no S3 adapter), restore refuses to run (Supabase recovery unverified); cargo unavailable in this environment | Observed | b588ed4 |
| BUILD-01 | File system inspection | `backup_cli.rs` file exists (6862 bytes, modified Nov 19); Cargo workspace structure at `apps/api/` with domain crates | Observed | current |
| COMMIT-01 | Git log inspection | Revision 9c7e0f3 verified to contain `docs/features/backup-restore.md` | Observed | 9c7e0f3 |

## Evidence Not Available

- **Test execution baseline:** Cargo toolchain not available in this environment per MAP-01; cannot execute `cargo test` to establish current pass/fail baseline
- **Runtime configuration:** Database credentials, Supabase instance configuration, secrets not inspected (out of scope for analysis gate)
- **Full codebase walkthrough:** Domain crate internals not read beyond what's mapped in MAP-01; focus limited to backup/restore surface area
- **Prior U1 code artifacts:** Previous code generation attempt made no app source changes per PROB-01; no implementation artifacts to inspect
- **Numeric requirements:** RPO/RTO targets, package size limits, retention policies, concurrency limits pending owner approval per FEAT-01 §§8, 10 and SUCCESS-01

## Source Language

The feature specification (FEAT-01) is authored in Vietnamese. Analysis artifacts (PROB-01 through SUCCESS-01) are in English. This evidence log and all gate documents are in English per workflow context.
