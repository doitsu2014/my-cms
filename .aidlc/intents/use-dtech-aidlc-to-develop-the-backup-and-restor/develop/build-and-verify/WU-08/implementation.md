# WU-08 implementation

- **Status:** Complete against the WU-08 "Done when" in `implementation-design/work-units.md` (attempt 4, 2026-11-28).
- **Scope:** WU-08 only. Gateway composition and runtime start (WU-10), disabled/download/preview handlers (WU-09), web units, the full TS-17 bundle scan (WU-15), schemas and migrations were not changed.
- **Identifiers:** Cites registered identifiers only; `dtech-aidlc ids check --stage build-and-verify` returned `ok: true`.

## Implemented and verified (carried from attempts 1-3, re-verified on this tree)

- `InstanceRegistry` / `BackupPolicy` (FN-02..FN-04): configured single local instance, safe capability projection, administrator re-assertion with the non-empty `ADMIN_ROLE` constant, loopback Host and configured POST Origin checks, claim-time instance re-check (TS-05b, TS-45, TS-59).
- `JobResponse` (FN-10): no actor/internal fields, `download_eligible` always false, forbidden U1 statuses refused, progress null unless measured (TS-12b, TS-49).
- `JobStore`/`DbJobStore` list (FN-09): instance-scoped, newest first, opaque cursor, bounded limit (TS-61); unknown and foreign ids are the same 404 (TS-04).
- Axum adapter for IF-01..IF-04 and IF-11 (FN-09, FN-35, FN-36): safe error bodies with `reason_code` + `correlation_id`, strict create parsing, accept/replay, 16 KiB body cap (TS-60), 415 on missing/wrong content type (TS-62), malformed JSON / unknown field / bad type / non-UUID path (TS-63), durable cleanup counts and escalation (TS-27b), disabled state with each closed `disabled_reason` (TS-27b).
- `MetricRecorder` (TS-57): accepted/refused/denied counters with the closed label set `job_kind`, `outcome`, `error_code` only.
- Actor extraction (RK-03): handlers take `Extension<AuthenticatedActor>`, the same extractor as `domain_user` (`api/routes.rs:46`) and `domain_media`.

## Changes in this attempt

| File | Change | Why |
|---|---|---|
| `apps/api/domain_backup/src/infra/tests/http_api.rs` | New `ts17_store_outage_paths_are_safe_and_status_stays_200_degraded` plus a `ready_runtime` helper | TS-17 (API outputs) requires the outage path in the canary scan; IF-11 "never 503" during a DB outage was untested. Ready runtime + API adapter on a disconnected store: status 200 `degraded`, `db_healthy=false`, `admissions_open=false`, `reconciliation`/`cleanup` null, escalation visible; list/get 503 `STORE_UNAVAILABLE` with correlation id; create 503 with no accepted body; refusal metric recorded with closed labels; tracing positive control detected; canary absent from bodies, headers, captured tracing, metrics and all five `backup_*` tables. |
| `apps/api/domain_backup/src/domain/policy.rs` | New `rk03_role_reassertion_never_uses_an_empty_role_list` | Implementation plan RK-03 asks for a unit test that an empty role list cannot be passed (`has_any_role(&[])` is true upstream). |
| `apps/api/domain_backup/src/lib.rs` | Test-only `test_temp_dir()` (resolves links on Unix, raw path on Windows) | On macOS the OS temp dir is behind `/var -> /private/var`; CFG-04 correctly rejects a link on any storage-root ancestor, so 18 normal and 4 Docker tests (WU-01 config, WU-08 instance/policy/HTTP) failed with `INVALID_CONFIG`. Prior runs were Windows-only. |
| `domain/config.rs`, `domain/instance.rs`, `domain/policy.rs`, `infra/tests/fs_support.rs` | `std::env::temp_dir()` -> `crate::test_temp_dir()` in test code only | Same platform fix. No production code path changed; CFG-04 validation is unchanged. `config.rs` and `fs_support.rs` belong to completed units WU-01/WU-04; the edit is confined to their `#[cfg(test)]` fixtures. |

## Boundary (owned elsewhere, not WU-08 "Done when")

- Production composition: `DomainBackupService::register_routes` builds `BackupApiState` without a started runtime, so production reads/create answer `BACKUP_NOT_READY` until WU-10 ("Gateway integration, runtime start"), which lists F-37 and owns runtime start after migrations, F-38 ingress layers, TS-03/TS-07/TS-08b/TS-55.
- TS-17 full run including the web bundle scan is WU-15.
- TS-62 for IF-09/IF-10 (previews) is WU-09.
