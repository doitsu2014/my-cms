# WU-08 green verification report

- **Status:** Partial; the work unit is not complete. Standalone API tests pass, while the service-created API state has no ready runtime, so production reads/create remain not-ready. No gateway composition change was made.
- **Environment:** Windows 11; cargo/rustc 1.100.0; Docker 29.7.2 available.
- **Baseline:** Reused unchanged from `develop/baseline.md` (2026-11-22).

## Checks observed

| Check | Result |
|---|---|
| `cargo check -p domain_backup` | Pass for current source/service declaration; does not demonstrate a ready runtime or composed-service behavior. |
| `cargo test -q -p domain_backup --no-fail-fast` | Pass: 141 passed, 0 failed, 202 ignored. |
| `cargo test -p domain_backup -- --ignored` | Pass: 202 passed, 0 failed with Docker/PostgreSQL. |
| `cargo test -p domain_backup infra::tests::http_api:: -- --ignored --nocapture` | Pass: 3 standalone REST tests (TS-04/TS-49, TS-45/status, TS-63 invalid UUID). |
| `cargo fmt --all -- --check` | Pass. |
| `cargo clippy -p domain_backup --all-targets --message-format=short` | Pass. |
| `cargo check --workspace` | Pass. Existing warnings in sibling crates and the Rust future-incompatibility notice were observed. |
| `cargo test --workspace --no-fail-fast` | Pass: 399 passed, 0 failed, 207 ignored. `domain_backup`: 141 passed, 202 ignored. Baseline's 290 normal tests had no failures; all normal tests remain passing. |
| `cargo test --workspace --no-fail-fast -- --ignored` | Pass: 207 ignored tests passed, 0 failed. `domain_backup`: 202 passed. Baseline's 39/39 ignored tests remain green. |
| `dtech-aidlc --project D:/orca/my-cms/makara ids check --stage build-and-verify` | `ok: true`; no unregistered identifiers. |

## Trace table

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-01 | AC-001 | FN-02 | `infra::tests::http_api::ts45_instances_and_status_return_the_configured_safe_contract` | Passed standalone |
| US-05 | AC-005 | FN-03 | `domain::policy::tests::ts59_requires_admin_then_loopback_host_and_exact_post_origin` | Passed at policy unit level |
| US-05 | AC-005 | FN-04 | `domain::policy::tests::ts05b_claim_policy_rechecks_only_the_registered_instance` | Passed |
| US-02 | AC-002 | FN-09 | `infra::tests::http_api::ts04_foreign_and_unknown_ids_are_the_same_404_and_ts49_is_safe`; list validation in TS-45 test | Passed standalone |
| US-02 | AC-002 | FN-10 | `handlers::job_response::tests::ts12b_projection_refuses_success_and_recovery_required` and two TS-49 tests | Passed |
| US-02 | AC-002 | FN-36 | `handlers::status::tests::ts27b_disabled_state_and_closed_disabled_reasons_are_projected`; ready status in TS-45 test | Partial; durable cleanup/escalation counts remain |
| US-05 | AC-005 | FN-35 | safe error assertions in HTTP tests | Partial; complete extractor matrix remains |
| US-05 | AC-005 | FN-05..FN-08 | existing WU-07 parser/acceptance suite | Not fully verified through WU-08 route adapter |
| US-05 | AC-005 | FN-37 | package/workspace check | Build passes; production runtime composition is not verified |

No unimplemented WU-08 criteria are claimed as passing.
