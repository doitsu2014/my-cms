# WU-10 failing tests (code manifest)

Author: parallel worktree worker (model `sonnet`, commit `a0a5709`) plus one conductor-added test (marked). Gateway tests live in `apps/api/gateway/src/main.rs` (`mod tests`) unless noted. "Red" = failed on an assertion against the stub in the red run (`red.log`, `red-docker.log`). "Guard" = already passed on the red run because the behaviour already held (kept as regression guards).

| Test | File | TS / ref | Asserts | Red / guard |
| --- | --- | --- | --- | --- |
| `ts55_oversized_headers_get_431_before_auth_and_never_reach_handler` | gateway/src/main.rs | TS-55, F-38, T-13 | Unauthenticated and authenticated requests over CFG-18 get 431 (not 401) with the crate error body (`HeaderTooLarge`, `HEADER_TOO_LARGE`); handler never called | Red (401 != 431) |
| `ts55_header_limit_boundary_is_exact` | gateway/src/main.rs | TS-55 | Exactly at the limit goes on to auth (401); one byte over is 431 | Red (401 != 431) |
| `ts55_slow_authenticated_request_gets_408_with_crate_error_body` | gateway/src/main.rs | TS-55, F-38 | Admin request whose inner handler outlives CFG-17 gets 408 `RequestTimeout`/`REQUEST_TIMEOUT` before the handler finishes | Red (200 != 408) |
| `ts55_layers_read_cfg17_and_cfg18_from_backup_config` | gateway/src/main.rs | TS-55, CFG-17, CFG-18 | `IngressLimits::from_backup_state` reads 7 s / 9000 from an enabled config; disabled state uses owner-approved 30 s / 16384 | Red (30 s/16384 != 7 s/9000) |
| `ts55_unauthenticated_request_within_limits_gets_401` | gateway/src/main.rs | TS-55 | Within limits, no credential: 401, handler not called | Guard |
| `ts55_authenticated_request_within_limits_reaches_handler` | gateway/src/main.rs | TS-55 | Admin request inside both limits reaches the handler (200) | Guard |
| `ts55_non_backup_routes_are_unaffected_by_ingress_layers` | gateway/src/main.rs | TS-55, T-13 | Public, protected and ordinary administrator routes: no header limit, no timeout | Guard |
| `ts08b_non_loopback_bind_forces_backup_disabled` | gateway/src/main.rs | TS-08b, F-39, T-04, AC-005 S1 | Bind hosts `0.0.0.0`, `::`, `10.0.0.5`, `192.168.1.20`, `example.com`, `localhost`, `""` give `NON_LOOPBACK_BIND`; `127.0.0.1`, `127.1.2.3`, `::1` give enabled | Red (None != NON_LOOPBACK_BIND) |
| `ts08b_effective_bind_host_is_what_the_config_sees` | gateway/src/main.rs | TS-08b | Bind value wins over a `HOST` in the lookup; `effective_bind_host` defaults to `127.0.0.1` | Red |
| `ts03_backup_routes_deny_unauthorized_tokens_before_handler` | gateway/src/main.rs | TS-03, AC-001 S2, AC-005 S1, T-01 | Missing/expired/forged -> 401, writer/non-admin -> 403 on `GET /backup/instances`, `GET`/`POST /backup/jobs`, `GET /backup/jobs/{id}`; instance list absent; zero handler calls; admin control reaches handler | Guard (administrator layer already denied) |
| `ts07_backup_routes_carry_no_cors_headers` | gateway/src/main.rs | TS-07, T-04 | Preflight and cross-origin read carry no `access-control-*` header | Guard |
| `ts07_cookie_only_mutation_is_not_authorized` | gateway/src/main.rs | TS-07, RK-07 | Valid admin token only in a Cookie (4 cookie names) on `POST /backup/jobs` gives 401, no handler call | Guard |
| `backup_runtime_starts_after_migrations_and_composed_api_is_ready` (Docker, ignored) | gateway/src/main.rs | F-37, TS-07 (415 part), RK-15 backup part | Migrations via the orchestrator record versions in `seaql_migrations_backup`; `startup_health` builds and starts the runtime; `/backup/status` state `ready`; `/backup/instances` 200; `POST /backup/jobs` with `text/plain` or no content type gives 415 `UNSUPPORTED_MEDIA_TYPE` | Red (`runtime().is_some()` false) |
| `scheduler_step_ticks_and_recovers_after_outage` (Docker, ignored) | gateway/src/main.rs | F-37 | Tick keeps admissions open; after `database_outage` a scheduler step reopens them | Red (`runtime()` None) |
| `backup_service_registers_under_administrator_ingress` | gateway/src/main.rs | F-38 | Registrations use `Mount::AdministratorIngress`, prefix `/backup` | Red (Administrator != AdministratorIngress) |
| `manifest_with_five_services_returns_five_entries` (updated to 6) | gateway/src/main.rs | WU-10 Done when | Six entries, includes `domain-backup` (name kept for history) | Red (5 != 6) |
| `runtime_start_failure_leaves_backup_not_ready_without_aborting_gateway` | gateway/src/main.rs | F-37 | Namespace lock held elsewhere: `startup_health` is Ok, API answers 503 `BACKUP_NOT_READY` | Guard (no runtime yet gives the same 503) |
| `disabled_backup_does_not_start_a_runtime` | gateway/src/main.rs | F-37 | Disabled config builds no runtime | Guard |
| `readme_env_section_lists_every_backup_key_with_default_or_required_value` | gateway/src/main.rs | WU-10 Done when, CFG-17/18 | Every `BACKUP_*` key read by `domain/config.rs` has a README table row with a default or required value; CFG-17 row has `30` and "owner-approved", CFG-18 row has `16384` and "owner-approved"; legacy CLI keys mentioned | Red (no row for `BACKUP_ALLOC_MAX_BYTES`) |
| `migrate_list_chains_backup_identities_after_seo` | gateway/src/main.rs | RK-08 | `all_migration_identities` ends with backup ids, seo ids precede them | Red |
| `all_migration_identities_include_seo_after_posts` (updated) | gateway/src/migrate_cli.rs | RK-08 | seo after posts, last id is the second backup migration | Red (last was the seo id) |
| `migrate_usage_documents_down_status_posts_only_limit` | gateway/src/main.rs | RK-08 | USAGE says down/status cover the posts migrator only and names `seaql_migrations_backup` | Red |
| `mount_serializes_to_lowercase`, `mount_round_trips_through_json` (extended) | domain_interface/src/lib.rs | F-38 | New `AdministratorIngress` variant serializes as `"administratoringress"` and round-trips | Guard (variant added with its tests) |
| `ingress_errors_map_to_408_and_431_with_the_crate_body_shape` | domain_backup/src/domain/response.rs | F-38 | Ingress error bodies map to 408/431, fixed reason codes, fresh correlation ids | Written with the helper, not red |

Existing tests that must stay green and were re-run: `compose_routers_supplies_cookie_extractor_to_domain_routes`, `compose_routers_traces_health_requests_without_changing_response`, `compose_routers_traces_rejected_protected_requests_without_calling_handler` (all passed in the red and green runs).

## Added by the conductor during integration

| Test | File | TS / ref | Asserts | Red / guard |
| --- | --- | --- | --- | --- |
| `f37_started_service_runs_a_created_job_to_incomplete` (Docker, ignored) | domain_backup/src/infra/tests/http_api.rs | F-37, F-16, AC-002 S1 | Started `DomainBackupService` + its composed routes: a created job (202 `queued`) is run by `worker_step()` (returns 1) to `incomplete` / `REQUIRED_COMPONENTS_UNAVAILABLE`, `download_eligible` false; a second step returns 0 | Red (`worker_step` 0 != 1) |

Reason: the worker draft listed "no job dispatcher" as out of scope, but FD 7.1 ("worker spawn failure at runtime start") places spawning the F-16 worker in production runtime start, which WU-10 composes (F-37). Without it every production job would sit `queued` until the sweeper failed it with `DISPATCH_FAILED`.
