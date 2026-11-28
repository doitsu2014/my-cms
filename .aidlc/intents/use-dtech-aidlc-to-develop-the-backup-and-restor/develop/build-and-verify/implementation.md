# Release-candidate implementation summary

- **Branch:** `doitsu2014/migrate-exported-documents-to` (kept off `main` per owner decision RK-16, 2026-11-28). Final commit for the sweep: `16fdfe0`.
- **Unit commits this session:** WU-08 `36361ae`, WU-09 `a0915ad`, WU-12 `ffd2519`, WU-10 `52c5875`, WU-13 `0021087`, WU-14 `8ceb568`, WU-15 `16fdfe0`. Earlier units (WU-01..WU-07, WU-11, WU-17..WU-19) were committed in earlier sessions.
- **Code areas:** `apps/api/domain_backup` (new domain), `apps/api/gateway` (mount, ingress layers, loopback guard, runtime/worker drivers, migration chaining), `apps/api/domain_interface` (`Mount::AdministratorIngress`), `apps/web/src/domains/backup.ts` and `apps/web/src/app/admin/backup/**`, `README.md` (BACKUP_* keys), `docs/features/backup-restore.md` (section 14), `docs/architecture/software-architecture.md`.
- **Deployment and rollback notes:**
  - Backup is off unless `BACKUP_ENABLED=true` with every required key (README "Backup and restore (U1) configuration"); a non-loopback `HOST` forces it off. The compose stack sets `HOST=0.0.0.0`, so backup stays disabled there.
  - Backup migrations are registered unconditionally: any image built from this branch creates `backup_*` tables on `migrate up` and on gateway start, enabled or not. This is why U1 must not merge to `main` (auto-release) until a separate release decision (RK-16).
  - Rollback limit (RK-08): `migrate down` and `migrate status` cover the posts migrator only; there is no CLI rollback for backup tables. Backup audit rows are append-only (TS-66). Rolling back means removing the code and dropping `backup_*` tables and `seaql_migrations_backup` manually.
  - Source and tests only in this develop phase; no deployment was performed.
