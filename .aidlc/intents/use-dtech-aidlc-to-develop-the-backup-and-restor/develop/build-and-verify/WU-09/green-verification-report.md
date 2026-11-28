# WU-09 green verification report

- **Status:** Green on the integrated main tree (WU-08 commit `36361ae` + WU-09 changes), 2026-11-28, macOS, Docker 29.9.3.

| Check (from `apps/api`) | Result | Evidence |
|---|---|---|
| `cargo test -p domain_backup --no-fail-fast -- ts09 ts10 ts43 ts53 ts60_preview ts64` | 16 passed, 0 failed (5 Docker tests ignored here) | `evidence/green-wu09-focused.log` |
| `cargo test --workspace --no-fail-fast` | 420 passed, 0 failed, 213 ignored | `evidence/regression-wu09-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` | 213 passed, 0 failed (includes the 5 WU-09 Docker tests) | `evidence/regression-wu09-ignored.log` |
| `cargo fmt --all -- --check` | exit 0 | `evidence/green-wu09-fmt.log` |
| `cargo clippy -p domain_backup --all-targets` | exit 0, no domain_backup warnings | `evidence/green-wu09-clippy.log` |
| `cargo check --workspace` | exit 0 | `evidence/green-wu09-check.log` |
| `dtech-aidlc ids check --stage build-and-verify` | `ok: true` | `evidence/green-wu09-ids.log` |

Worker-side worktree runs (before integration): package normal 162/0, package Docker 208/0 (`evidence/wu09-full-default.log`, `evidence/wu09-full-ignored.log`, `evidence/wu09-green-*.log`).

## Done-when trace

| Criterion | Tests | Result |
|---|---|---|
| TS-09 incl. body-not-read spies | `ts09_disabled_capabilities_never_poll_the_request_body` and the other TS-09 tests in `failing-tests.md` | pass |
| No ledger rows or files created | `ts09_disabled_capabilities_leave_no_ledger_rows_or_files` (Docker), `ts09_denied_capabilities_create_no_files_and_record_one_denied_metric_each` | pass |
| Download 403 `DOWNLOAD_DENIED` for every status | `ts09_download_is_denied_for_every_status_and_changes_nothing` (Docker) | pass |
| TS-10 only the fixture adapter | `ts10_runtime_composition_holds_only_the_fixture_adapter`, `ts10_client_flags_cannot_select_a_real_adapter` | pass |
| TS-43 confirm and cancel enqueue nothing | `ts43_confirm_and_cancel_previews_enqueue_nothing` | pass |
| TS-53 trusted fixture ids only, no uploads/paths | `ts53_*` (3 tests) | pass |
| TS-64 identification fields, no echoed targets | `ts64_*` (2 tests) | pass |

Two earlier Docker runs at default parallelism each lost one unrelated test to container start-up failures (`evidence/regression-wu09-ignored-run1-infra-flake.log`, `-run2-infra-flake.log`); see known-limitations.md.
