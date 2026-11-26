# KL-02 Known limitations: WU-02

- **Stage:** Develop / build-and-verify, unit WU-02.
- **Status:** Existing limitations retained; verification refreshed 2026-11-24. No WU-02 source or test changes were made.

| # | Limitation | Impact | Owner or next step |
|---|---|---|---|
| L-01 | A job row can be inserted directly as `running`, `incomplete` or `failed`; the transition trigger guards UPDATE only. | Raw SQL can bypass the normal queued admission edge. | WU-03 store inserts only `queued`; any DB INSERT guard requires an owner decision. |
| L-02 | Statement-level audit guard rejects UPDATE, DELETE and TRUNCATE on `backup_audit`, even when no row matches. | Audit writers must use plain INSERT or `ON CONFLICT DO NOTHING`. | WU-03 `DbAuditSink` and F-33 retention path. |
| L-03 | `backup_allocations.job_id` FK uses default NO ACTION. | Retention cannot delete a job before dependent allocations are removed. | WU-06 retention purge must delete eligible allocation rows before jobs, or later owner decision for `ON DELETE SET NULL`. |
| L-04 | Guarded down migration takes `SHARE ROW EXCLUSIVE` locks. | Revert blocks backup admissions while running. | Operators revert only with backup disabled. |
| L-05 | DB triggers/checks protect against application DML, not table owner or superuser. | A privileged role can disable triggers/drop constraints. | Record as T-08 residual; tamper-proof append-only requires separate owner-role design outside U1. |
| L-06 | Gateway `migrate status` / `migrate down` cover posts only. | Backup migration status is not exposed through that CLI. | WU-10 (RK-08). |
| L-07 | WU-02 does not add entities. | No schema-entity mapping in this unit. | WU-03. |
| L-08 | Two test changes followed the original red run: D-16 test helper amendment and `idempotency_scope_ignores_request_fingerprint` addition. | Neither has classic red-first evidence. | Historical green report records both and mutation evidence. |
| L-09 | Two mutations lack a dedicated catching assertion: trigger allows running→succeeded, and jobs trigger creation without `DROP TRIGGER IF EXISTS`. | First is still rejected by U1 CHECK; second is unreachable along documented revert paths. | Accepted and recorded in historical green report. |
| L-10 | Some historical mutation results were produced by delegated agents. | Those are agent-reported rather than conductor-rerun evidence. | Logs retained under `evidence/mutation-wu02/`; historical green report labels provenance. |
| L-11 | Backup crate is not composed into gateway. | WU-02 migrations are not run by API startup. | WU-10. |
| L-12 | Current broader crate run has 13 lifecycle failures (TS-20/21/22/23/25/26/27a/29/30/32/42/56/65). | Current `domain_backup` is not green; failures are in later-unit WU-06/WU-16 lifecycle shells/tests, not `migrations::tests`. | Keep WU-02 report limited to its migration scope; resolve through the appropriate lifecycle work unit, not by changing WU-02 migrations. |
| L-13 | Current focused Clippy check exits 0 with warnings in later-unit stores and lifecycle shell types. | Warning debt in current worktree remains. | Track with the corresponding WU-06/WU-16 implementation; not a WU-02 migration regression. |

## Rollback

The backup crate is not registered in the gateway, so WU-02 migrations are not applied by the running API. Revert the WU-02 files to roll back. If a backup migration was manually applied, `migrations_cli::revert` refuses while audit rows or non-deleted allocations exist; otherwise it retains `backup_audit` by design.
