# KL-02 Known limitations: WU-02

- **ID:** KL-02 (WU-02)
- **Stage:** Develop / build-and-verify, unit WU-02.
- **Status:** Open items carried to later units or to the release-candidate gate. WU-01 items stay in [../WU-01/known-limitations.md](../WU-01/known-limitations.md). Its L-01 (Docker unavailable) is closed: Docker was restored and the full suite now runs.

| # | Limitation | Impact | Owner or next step |
|---|---|---|---|
| L-01 | A job row can be INSERTed directly as `running`, `incomplete` or `failed`, because the transition trigger guards UPDATE only | Skipping `queued` is possible through raw SQL | WU-03: `DbJobStore` inserts only `queued`, covered by its store tests. A DB-level INSERT guard would need an owner decision. |
| L-02 | The statement-level audit guard rejects every UPDATE, DELETE or TRUNCATE on `backup_audit`, even one matching zero rows. That includes `INSERT ... ON CONFLICT DO UPDATE`. | Audit writers must use plain INSERT or `ON CONFLICT DO NOTHING` | WU-03 (`DbAuditSink`) and F-33 (retention). Documented in the 000002 module doc. |
| L-03 | `backup_allocations.job_id` FK has no ON DELETE clause (NO ACTION, as the contract states) | The retention purge cannot delete a job that still has allocation rows | WU-06 / F-33: delete or null allocation rows first, or a later owner decision for `ON DELETE SET NULL` |
| L-04 | Revert takes `SHARE ROW EXCLUSIVE` locks on `backup_audit`, `backup_allocations`, `backup_jobs` | A revert blocks backup admissions while it runs. The lock itself has no test (it hardens a tested refusal). | Accepted. Operators revert only with backup disabled. |
| L-05 | Triggers and CHECKs stop application DML only. The role that runs the migrations owns the tables and can `ALTER TABLE ... DISABLE TRIGGER` or drop constraints. | Append-only is not tamper-proof against the owner or a superuser (T-08 residual) | Record next to RK-13. True append-only needs a separate owner role, an owner decision outside U1. |
| L-06 | `migrate status` / `migrate down` in the gateway CLI cover posts only and print "status unavailable" | Operators cannot see backup (or SEO) migration status through the CLI | WU-10 (RK-08) |
| L-07 | Entities (`src/entities/*`) not added | none yet | WU-03 |
| L-08 | One red test was amended after build (D-16, an ordering defect in the `columns()` helper), and one test was added after build (`idempotency_scope_ignores_request_fingerprint`) | Both lack classic red-first evidence | Both proven by mutation (green report section 3). The amendment compares the same exact set. |
| L-09 | Two mutations are not caught by a dedicated test: the trigger allowing running to succeeded, and the jobs trigger re-created without `DROP TRIGGER IF EXISTS` | The first is still rejected by the U1 CHECK (defence in depth). The second is unreachable by any revert path. | Accepted, recorded |
| L-10 | Mutation runs marked "agent" in the green report were run by subagents; the conductor re-ran only the test-after mutations | Agent-reported evidence, logs kept in `evidence/mutation-wu02/` | Accepted, labelled in the report |
| L-11 | Not composed into the gateway | WU-02 changes nothing in the running API or its startup migrations | WU-10 |

## Rollback

The backup crate is still not referenced by the gateway, so its migrations never run in the API. To roll back, revert the WU-02 files. On any database where someone ran the backup migrator manually, `migrations_cli::revert` refuses while audit rows or live allocations exist. Otherwise it drops every backup table except `backup_audit`, which is kept by design.
