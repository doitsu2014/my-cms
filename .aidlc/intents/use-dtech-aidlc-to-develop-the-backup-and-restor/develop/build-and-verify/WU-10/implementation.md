# WU-10 implementation

- **Status:** Complete against the WU-10 "Done when" with the owner decisions below (attempt 1, 2026-11-28).
- **Author:** parallel worktree worker (model `sonnet`, commit `a0a5709`, based on `ffd2519`), integrated into the main tree by the conductor, who added the worker driver (section "Conductor addition").



## Files changed

- `apps/api/domain_interface/src/lib.rs`: new `Mount::AdministratorIngress` variant (serde lowercase) and extended serde tests. Grep confirmed the only exhaustive `match` on `Mount` is in `compose_routers`.
- `apps/api/gateway/Cargo.toml`: `domain_backup` dependency; `jsonwebtoken` dev-dependency (same version as `domain_auth`) for the token-minting tests. `apps/api/Cargo.lock` follows.
- `apps/api/gateway/src/main.rs`:
  - `manifest(media, user, backup)` now returns six services (`DomainBackupService` added last).
  - `IngressLimits` (`request_timeout`, `max_header_bytes`), `IngressLimits::OWNER_APPROVED` (30 s, 16384) and `from_backup_state` (reads CFG-17 and CFG-18 from an enabled config; a disabled backup is still wrapped, with the owner-approved values).
  - `compose_routers_with_limits`: `Mount::AdministratorIngress` registrations merge into a separate `backup_admin` router wrapped, outermost to innermost, by `backup_request_timeout` (CFG-17, 408), `backup_header_size_limit` (CFG-18, 431), the administrator auth layer, then the routes; merged with the public, protected and ordinary administrator routers before cookie manager and otel layers. `compose_routers(services, ctx)` remains as a test-only wrapper with the owner-approved limits, so the three existing `compose_routers_*` tests are untouched.
  - `effective_bind_host` and `build_backup_config(bind_host, lookup)`: the listener's bind value, not a second read of `HOST`, drives the F-39 guard (TS-08b). `main` computes the bind host once, builds the backup config from it, and binds the same value.
  - `main` keeps a clone of the backup service (clones share the runtime slot), and after `startup_health` spawns `spawn_backup_scheduler` (wakes `scheduler_step` every 60 s) when a runtime exists.
  - Tests for TS-03, TS-07, TS-08b, TS-55, the wiring checks, README and migration-identity checks, plus two Docker tests.
- `apps/api/gateway/src/migrate_cli.rs`: backup identities chained after seo in `all_migration_identities` (RK-08); USAGE and module docs state that `down` and `status` cover the posts migrator only; `migrate up` builds the manifest with the backup service; the existing identity test updated to the new order.
- `apps/api/gateway/src/migration_tests.rs`: `connect_untouched` and `recorded_versions` made `pub(crate)` for reuse (test-support only).
- `apps/api/domain_backup/src/service.rs` (minimal): `build_runtime(conn, config)` builds the production `Runtime` (real stores, `FsFixtureStorage`, `FsNamespaceLock`, bounds from config); `DomainBackupService` now holds a shared `OnceLock<Arc<Runtime>>`; `startup_health` builds and starts the runtime after migrations and never fails the gateway (a failed start leaves `BACKUP_NOT_READY`); `register_routes` uses `Mount::AdministratorIngress` and hands the runtime to `BackupApiState`; `runtime()` accessor and `scheduler_step()` (recovery when admissions are closed, otherwise the due tick).
- `apps/api/domain_backup/src/domain/response.rs` (minimal): `ingress_request_timeout()` (408 `REQUEST_TIMEOUT`) and `ingress_header_too_large()` (431 `HEADER_TOO_LARGE`) so the gateway layers use the crate error body; one unit test.
- `apps/api/domain_backup/Cargo.toml`: `tracing` moved from dev-dependencies to dependencies (warnings from the service).
- `README.md`: new "Backup and restore (U1) configuration" section: every `BACKUP_*` key with its default or required value, CFG-17 `30` and CFG-18 `16384` recorded as owner-approved, separation from the legacy CLI keys, the migration-table note and the `down|status` limit.

## F, IF, TS covered

- F-37 (service: runtime start after migrations, routes), F-38 (ingress layers, mount variant, composed router), F-39 (loopback wiring). IF: all REST composed (auth precedence, content type, no CORS).
- TS-03, TS-07, TS-08b, TS-55 green; `manifest_with_five_services_returns_five_entries` updated to six; the three `compose_routers_*` tests green.
- AC-001, AC-002 S1 (TS-55), AC-005 S1; T-01, T-04, T-13, T-17.

## Decisions

- Owner values (recorded 2026-11-28 by Duc Tran): CFG-17 `BACKUP_REQUEST_TIMEOUT_SECS` = 30, CFG-18 `BACKUP_MAX_HEADER_BYTES` = 16384. WU-01 config semantics unchanged (both stay required when enabled, otherwise `INVALID_CONFIG`). The gateway constants `IngressLimits::OWNER_APPROVED` are used only to wrap the routes when backup is disabled; no default is applied to the config. README documents both as owner-approved.
- RK-15 owner disposition (descope as residual): backup runtime start is wired after migrations and tested with the backup migrator and its own `seaql_migrations_backup` table (Docker test asserts the recorded versions). `domain_seo` and `domain_posts` untouched. The gateway-boot-beside-posts/seo evidence is a residual, see known-limitations.
- RK-16 owner decision: U1 stays off `main` until a separate release decision. No conditional-registration code: backup migrations are registered unconditionally and run on every `migrate up` and gateway start, enabled or not. README states this.
- RK-05: `grep -rn "cors_layer\|body_limit_layer" apps/api` finds only the definitions in `domain_posts/src/domain/layers.rs` (lines 14 and 33); neither is applied anywhere. Backup adds neither (see `rk05-grep.log`).
- RK-07: `ts07_cookie_only_mutation_is_not_authorized` proves a valid admin token in a cookie (four cookie names) never authorizes `POST /backup/jobs`; the auth layer reads `Authorization: Bearer` only.
- RK-08: backup identities are chained into `all_migration_identities`; `migrate down` and `status` still forward to the posts migrator only, documented in USAGE, the module docs and README.
- Layer mechanism: custom axum middlewares rather than tower-http (`TimeoutLayer` gives an empty 408 body; the crate error body is required). Header size is the sum of name and value bytes over all headers, which is reproducible in tests.
- The `Mount` variant (not a `RouteRegistration` field) as designed, so no domain's struct literal changed.
- Test fixtures use `std::fs::canonicalize(temp_dir())` so macOS `/var` does not trip the CFG-04 link check.
- `manifest_with_five_services_returns_five_entries` keeps its name (history) and now asserts six.

## Conductor addition: worker driver (F-16 in the F-37 composition)

- `domain_backup/src/service.rs`: `build_worker(conn, config, epoch)` builds the production `Worker` (real job/allocation stores, `FixtureCaptureAdapter`, `BackupPolicy` over the configured instance, deadline CFG-10, allocation TTL CFG-05). `startup_health` builds it after a successful `Runtime::start` using the runtime's epoch. `worker_step()` runs queued jobs one at a time while admissions are open (at most 16 per step); a refused claim ends the step and leaves the job queued.
- `gateway/src/main.rs`: `spawn_backup_scheduler` also spawns a worker driver that calls `worker_step()` every 2 s (`BACKUP_WORKER_PERIOD`), well inside the 600 s queued-age bound after which the sweeper fails an unclaimed job with `DISPATCH_FAILED`. A single driver task keeps concurrency at 1.
- Test: `f37_started_service_runs_a_created_job_to_incomplete` (Docker): red with a stub (`worker_step` 0 != 1), green after the change.
- Why: FD 7.1 names "worker spawn failure at runtime start" as a `DISPATCH_FAILED` trigger, so starting the worker belongs to production runtime start. Without it no production job could reach `incomplete` (AC-002 S1).

## Owner decisions (2026-11-28, Duc Tran, this session)

- CFG-17 `BACKUP_REQUEST_TIMEOUT_SECS` = 30 and CFG-18 `BACKUP_MAX_HEADER_BYTES` = 16384: owner-approved values, documented in README; config semantics unchanged (required when enabled, else `INVALID_CONFIG`).
- RK-15: descoped as residual (see known-limitations item 1); `domain_seo` and `domain_posts` untouched.
- RK-16: U1 stays off `main` until a separate release decision; no conditional-registration code.
