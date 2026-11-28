# WU-10 green verification report

- **Status:** Green on the integrated main tree (WU-08, WU-09, WU-12 commits + WU-10), 2026-11-28, macOS, Docker 29.9.3.

| Check (from `apps/api`) | Result | Evidence |
|---|---|---|
| `cargo test -p gateway --no-fail-fast` | 40 passed, 0 failed, 4 ignored | `evidence/green-wu10-gateway.log` |
| `cargo test --workspace --no-fail-fast` | 439 passed, 0 failed, 216 ignored | `evidence/regression-wu10-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` | 216 passed, 0 failed (includes the gateway Docker tests and `f37_started_service_runs_a_created_job_to_incomplete`) | `evidence/regression-wu10-ignored.log` |
| `cargo fmt --all -- --check` | exit 0 | `evidence/green-wu10-fmt.log` |
| `cargo clippy -p gateway -p domain_backup -p domain_interface --all-targets` | exit 0; only pre-existing warnings (`backup_cli.rs`, existing `migrate_cli.rs` test) | `evidence/green-wu10-clippy.log` |
| `cargo check --workspace` | exit 0 | `evidence/green-wu10-check.log` |
| `dtech-aidlc ids check --stage build-and-verify` | `ok: true` | `evidence/green-wu10-ids.log` |

Worker-side worktree logs are kept as `evidence/worker-*.log` (gateway 40/0; workspace 439/0; Docker 215/0 on the third run, two earlier runs lost 1-2 unrelated domain_backup tests to container start-up failures under load).

## Done-when trace

| Criterion | Evidence | Result |
|---|---|---|
| RK-16 owner decision on migration gating recorded; no merge to main | implementation.md "Owner decisions" | recorded (keep off main) |
| RK-15 runtime-start condition resolved or descoped | owner disposition: descoped residual (known-limitations item 1); backup runtime start after migrations tested (`backup_runtime_starts_after_migrations_and_composed_api_is_ready`) | descoped as residual |
| Owner values for CFG-17/CFG-18 recorded | 30 s / 16384 in README and implementation.md; `ts55_layers_read_cfg17_and_cfg18_from_backup_config` | pass |
| F-38 layer feasibility: 431 before 401, 408 on slow inner processing | `ts55_oversized_headers_get_431_before_auth_and_never_reach_handler`, `ts55_header_limit_boundary_is_exact`, `ts55_slow_authenticated_request_gets_408_with_crate_error_body` | pass |
| TS-03, TS-07, TS-08b, TS-55 green | tests listed in `red-tests/WU-10/failing-tests.md` | pass |
| README env section lists every `BACKUP_*` key with default | `readme_env_section_lists_every_backup_key_with_default_or_required_value` | pass |
| Manifest test updated to 6; `compose_routers_*` re-run green | `manifest_with_five_services_returns_five_entries` (asserts 6), three `compose_routers_*` tests | pass |
| RK-05 unused `cors_layer` / `body_limit_layer` checked by grep | `red-tests/WU-10/evidence/rk05-grep.log` (defined only in `domain_posts/src/domain/layers.rs:14,33`, never applied) | recorded |
| RK-07 cookie-only authorization of mutations tested | `ts07_cookie_only_mutation_is_not_authorized` | pass |
| RK-08 backup identities chained into `all_migration_identities`, down/status limit documented | `migrate_list_chains_backup_identities_after_seo`, `all_migration_identities_include_seo_after_posts`, `migrate_usage_documents_down_status_posts_only_limit` | pass |
| Runtime started after migrations and worker driven (F-37, F-16) | `backup_runtime_starts_after_migrations_and_composed_api_is_ready`, `scheduler_step_ticks_and_recovers_after_outage`, `f37_started_service_runs_a_created_job_to_incomplete` | pass |
