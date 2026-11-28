# WU-08 green verification report

- **Status:** Green. All WU-08 "Done when" tests pass on the final tree.
- **Environment:** macOS Darwin 25.6.0; cargo/rustc 1.100.0; Docker 29.9.3 (prerequisite `docker` check ok).
- **Baseline:** `develop/baseline.md` (2026-11-22, Windows) reused; macOS surfaced a platform test-fixture defect (temp dir behind a symlink), fixed in test code only (see implementation.md).

## Checks observed on the final source tree (2026-11-28, commands from `apps/api`)

| Check | Result | Evidence |
|---|---|---|
| `cargo test --workspace --no-fail-fast` | exit 0: 404 passed, 0 failed, 208 ignored | `evidence/regression-wu08-macos-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored` | exit 0: 208 passed, 0 failed | `evidence/regression-wu08-macos-ignored.log` |
| `cargo test -p domain_backup infra::tests::http_api:: -- --ignored` | 4 passed, 0 failed | `evidence/green-wu08-macos-http-api.log` |
| `cargo test -p domain_backup domain::policy::` | 3 passed (incl. RK-03 test) | observed in run; included in workspace normal log |
| `cargo fmt --all -- --check` | exit 0 | `evidence/green-wu08-macos-fmt.log` |
| `cargo clippy -p domain_backup --all-targets --message-format=short` | exit 0, no domain_backup warnings | `evidence/green-wu08-macos-clippy.log` |
| `cargo check --workspace` | exit 0 | `evidence/green-wu08-macos-check.log` |
| `dtech-aidlc ids check --stage build-and-verify` | `ok: true` | `evidence/green-wu08-macos-ids.log` |

## Done-when trace

| Criterion | Test(s) | Result |
|---|---|---|
| TS-04 unknown and foreign ids same 404 | `http_api::ts04_foreign_and_unknown_ids_are_the_same_404_and_ts49_is_safe` | pass |
| TS-05b claim-time policy re-check | `policy::ts05b_claim_policy_rechecks_only_the_registered_instance` | pass |
| TS-06 strict create payload, no side effects | `http_api::ts45_...` (unsafe/unknown fields, disallowed instance, durable counts unchanged) | pass |
| TS-12b / TS-49 safe projection, `download_eligible` false | `handlers::job_response` tests, `http_api::ts04_...` | pass |
| TS-17 (API outputs) | `http_api::ts45_...` (success, replay, validation, denial, conflict) and `http_api::ts17_store_outage_...` (outage); bodies, headers, tracing with positive control, metrics, status, all five `backup_*` tables | pass |
| TS-27b escalation, disabled state and each `disabled_reason` | `handlers::status::tests::ts27b_disabled_state_and_closed_disabled_reasons_are_projected`, `http_api::ts45_...` durable counts, `http_api::ts17_store_outage_...` degraded escalation | pass |
| TS-45 instance registry | `instance::tests::ts45_...`, `http_api::ts45_...` | pass |
| TS-57 closed metric label set, canary-free | `domain::metrics` tests, `http_api::ts45_...`, `http_api::ts17_store_outage_...` | pass |
| TS-59 role, Host, Origin precedence | `policy::ts59_...`, `http_api::ts45_...` | pass |
| TS-60 16 KiB cap, 413 | `http_api::ts45_...` (declared and streamed) | pass |
| TS-61 IF-03 limit/cursor 400 | `http_api::ts45_...` | pass |
| TS-62 415 on IF-02 | `http_api::ts45_...` (missing, `text/plain`) | pass |
| TS-63 malformed JSON, unknown field, bad type, non-UUID path, no echo | `http_api::ts45_...`, `http_api::ts63_...` | pass |
| RK-03 actor extraction and non-empty role | `Extension<AuthenticatedActor>` as in `domain_user/src/api/routes.rs:46`; `policy::rk03_role_reassertion_never_uses_an_empty_role_list` | pass |

Out of WU-08 scope and not claimed: production runtime start and gateway composition (WU-10), TS-17 bundle scan (WU-15), TS-62 for previews (WU-09). See known-limitations.md.
