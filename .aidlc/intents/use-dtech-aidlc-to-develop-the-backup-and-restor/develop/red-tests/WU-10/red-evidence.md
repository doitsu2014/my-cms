# WU-10 red evidence

Worktree: `/Users/doitsu2014/workspaces/doitsu-technology/my-cms/.claude/worktrees/wf_963bdb52-c92-1`, run from `apps/api`. The worktree was created from `main` (`6f0ed65`), which lacks WU-01..WU-12; it was reset to the intent branch `doitsu2014/migrate-exported-documents-to` (`ffd2519`, identical `apps/api` to the main checkout) before any work. Docker was available (`docker info` ok).

## Commands and counts

1. `cargo test -p gateway --no-fail-fast` (stubs compiled; tests written first). Log: `evidence/red.log`.
   Result: `28 passed; 12 failed; 4 ignored`.
2. `cargo test -p gateway --no-fail-fast -- --ignored --test-threads=2 tests::`. Log: `evidence/red-docker.log`.
   Result: `2 passed; 2 failed` (the two failures are the new Docker tests; the two passes are the pre-existing `migration_tests`).

The stubs (what made the tests compile): `Mount::AdministratorIngress` variant; `manifest` taking the backup service but still returning five entries; `IngressLimits::from_backup_state` returning the owner-approved constants; `build_backup_config` ignoring the bind host; `compose_routers_with_limits` treating `AdministratorIngress` like `Administrator` (no ingress layers); `DomainBackupService::runtime()` returning `None` and `scheduler_step()` doing nothing; service still registering `Mount::Administrator`.

## Failing tests with verbatim assertion output

| Test | Output |
| --- | --- |
| `ts55_oversized_headers_get_431_before_auth_and_never_reach_handler` | `assertion left == right failed: GET /backup/instances unauthenticated with oversized headers` `left: 401` `right: 431` |
| `ts55_header_limit_boundary_is_exact` | `assertion left == right failed: one byte over the limit` `left: 401` `right: 431` |
| `ts55_slow_authenticated_request_gets_408_with_crate_error_body` | `assertion left == right failed` `left: 200` `right: 408` |
| `ts55_layers_read_cfg17_and_cfg18_from_backup_config` | `left: IngressLimits { request_timeout: 30s, max_header_bytes: 16384 }` `right: IngressLimits { request_timeout: 7s, max_header_bytes: 9000 }` |
| `ts08b_non_loopback_bind_forces_backup_disabled` | `assertion left == right failed: bind host "0.0.0.0" must disable backup` `left: None` `right: Some("NON_LOOPBACK_BIND")` |
| `ts08b_effective_bind_host_is_what_the_config_sees` | `left: None` `right: Some("NON_LOOPBACK_BIND")` |
| `backup_service_registers_under_administrator_ingress` | `left: Administrator` `right: AdministratorIngress` |
| `manifest_with_five_services_returns_five_entries` | `left: 5` `right: 6` |
| `migrate_list_chains_backup_identities_after_seo` | `backup ids must come last: ["m20240409_151952_release_100", ..., "m20260822_000001_seo_head_assets"]` |
| `all_migration_identities_include_seo_after_posts` | `left: Some("m20260822_000001_seo_head_assets")` `right: Some("m20261122_000002_backup_terminal_guards")` |
| `migrate_usage_documents_down_status_posts_only_limit` | `USAGE must state that down/status cover the posts migrator only: Usage: my-cms-api migrate <verb> ...` |
| `readme_env_section_lists_every_backup_key_with_default_or_required_value` | `README has no table row for 'BACKUP_ALLOC_MAX_BYTES'` |
| `backup_runtime_starts_after_migrations_and_composed_api_is_ready` (Docker) | panicked at the assertion `enabled backup builds a runtime`; the backup migrations had already applied and been recorded in `seaql_migrations_backup` |
| `scheduler_step_ticks_and_recovers_after_outage` (Docker) | panicked `runtime` (`backup.runtime()` is None because `startup_health` builds nothing) |

## Why these are behavioural red

Each test compiled and ran against a stub, then failed because the observable behaviour was absent or wrong: a 401 where 431 is required, a 200 where 408 is required, a missing disabled state, an `Administrator` mount, five manifest entries, a missing identity or doc text, and a runtime that was never built. None is a compile, import, setup or formatting failure; `cargo fmt --all` was run on the tests before the red snapshot.

## Guards (passed in the red run because the behaviour already held)

`ts03_backup_routes_deny_unauthorized_tokens_before_handler`, `ts07_backup_routes_carry_no_cors_headers`, `ts07_cookie_only_mutation_is_not_authorized`, `ts55_unauthenticated_request_within_limits_gets_401`, `ts55_authenticated_request_within_limits_reaches_handler`, `ts55_non_backup_routes_are_unaffected_by_ingress_layers`, `runtime_start_failure_leaves_backup_not_ready_without_aborting_gateway`, `disabled_backup_does_not_start_a_runtime`, and the extended `domain_interface` `Mount` tests. They hold because the existing administrator layer already denies bad tokens and the gateway has no CORS layer or cookie auth. They are recorded as guards, not red.

TS-03 and TS-07 therefore have no behavioural red run: the composed router already satisfied them once the backup crate was wired in. The 415 half of TS-07 is the Docker test, which was red until the runtime started.

## 431-before-401 and 408 validation (F-38 feasibility)

Validated by the green run: with the layers ordered timeout, header-size, administrator auth, routes, an oversized unauthenticated request gets 431 and an in-limit unauthenticated request gets 401, with zero handler calls; a slow authenticated request gets 408 before the handler returns and the handler future is cancelled. Header budget is name bytes plus value bytes of every header; the boundary test pins it exactly. Slow header delivery (slowloris) is handled by hyper before the router and is not covered (T-13 residual).

## Conductor-added dispatch test (main tree, after integrating the worker commit)

Command (from `apps/api`): `cargo test -p domain_backup f37_started_service -- --ignored`. Log: `evidence/red-dispatch-docker.log`. Result: `0 passed; 1 failed`.

State: `DomainBackupService::worker_step()` existed as a stub returning `0`; everything else was the integrated WU-10 code.

```
thread 'infra::tests::http_api::f37_started_service_runs_a_created_job_to_incomplete' panicked at domain_backup/src/infra/tests/http_api.rs:1157:5:
assertion `left == right` failed: the worker runs the queued job
  left: 0
 right: 1
```

Behavioural: the runtime started (`Ready`), the composed API accepted the job (202, `queued`), and the assertion failed only because nothing ran the queued job.

## Owner decisions in force (recorded 2026-11-28, Duc Tran, this session)

- CFG-17 `BACKUP_REQUEST_TIMEOUT_SECS` = 30; CFG-18 `BACKUP_MAX_HEADER_BYTES` = 16384 (owner-approved values; still required when enabled).
- RK-15: full gateway boot beside posts/seo (TS-34 part, TS-41 runtime part) descoped as an explicit residual; no seo/posts changes.
- RK-16: U1 stays off `main` until a separate release decision.
