# WU-09 failing tests (code manifest)

- **Unit:** WU-09 Disabled capabilities, download denial, previews (F-11..F-14; IF-05..IF-10).
- **Owned TS:** TS-09, TS-10, TS-43, TS-53, TS-64 (TS-60 extended to the preview routes).
- **Author:** parallel worktree worker (model `sonnet`), integrated and checked by the conductor. Worker commit `79fe2e8` on `worktree-wf_6fc7eae6-d0c-1`.

| Test | File | TS | Asserts |
|---|---|---|---|
| `ts53_catalog_trusts_exactly_three_synthetic_fixtures_by_exact_id` | `domain/fixtures.rs` | TS-53 | catalog holds exactly `fixture-valid`, `fixture-incompatible`, `fixture-corrupt`, matched by exact id |
| `ts09_download_is_503_not_denied_while_backup_is_disabled_or_not_ready` | `handlers/download_denied/tests.rs` | TS-09 | IF-05 answers 503 `BACKUP_DISABLED` / `BACKUP_NOT_READY` before any denial (IFC 2.1 precedence) |
| `ts09_download_role_and_host_checks_precede_everything` | `handlers/download_denied/tests.rs` | TS-09 | role and Host 403s come first (guard on WU-08 middleware) |
| `ts09_disabled_capabilities_return_403_whatever_the_backup_state` | `handlers/disabled_capability/tests.rs` | TS-09 | IF-06..IF-08 return 403 `CAPABILITY_DISABLED` when disabled, not ready or ready |
| `ts09_disabled_capabilities_never_poll_the_request_body` | `handlers/disabled_capability/tests.rs` | TS-09 | body-polling spy never polled, even with a 1 GiB `Content-Length` |
| `ts09_role_host_and_origin_checks_precede_capability_disabled` | `handlers/disabled_capability/tests.rs` | TS-09 | role/Host/Origin 403s precede `CAPABILITY_DISABLED` (guard) |
| `ts09_capability_denial_attempts_audit_and_audit_failure_still_denies` | `handlers/disabled_capability/tests.rs` | TS-09 | one `capability.denied` audit attempt; audit failure still returns 403 |
| `ts09_denied_capabilities_create_no_files_and_record_one_denied_metric_each` | `handlers/disabled_capability/tests.rs` | TS-09 | no files, no ledger calls, one `Denied/Forbidden` metric per request |
| `ts10_runtime_composition_holds_only_the_fixture_adapter` | `handlers/disabled_capability/tests.rs` | TS-10 | only the synthetic fixture adapter is composed (guard) |
| `ts53_preflight_preview_returns_the_canned_result_for_each_trusted_fixture` | `handlers/preview/tests.rs` | TS-53 | IF-09 returns the canned labelled synthetic result per trusted fixture |
| `ts64_restore_confirmation_preview_carries_the_identification_fields` | `handlers/preview/tests.rs` | TS-64 | IF-10 returns fixture package, target instance label and data in scope |
| `ts64_request_supplied_target_values_are_rejected_not_echoed` | `handlers/preview/tests.rs` | TS-64 | client-supplied target fields are 400 and never echoed |
| `ts43_confirm_and_cancel_previews_enqueue_nothing` | `handlers/preview/tests.rs` | TS-43 | confirm and cancel decisions enqueue no job and touch no store |
| `ts53_unknown_fixture_ids_paths_and_uploads_are_rejected` | `handlers/preview/tests.rs` | TS-53 | unknown ids, paths and upload shapes are 400 |
| `ts60_preview_routes_enforce_media_type_and_the_16_kib_body_cap` | `handlers/preview/tests.rs` | TS-60, TS-62 | 415 on non-JSON, 413 over 16 KiB on IF-09/IF-10 |
| `ts09_preview_precedence_matrix` | `handlers/preview/tests.rs` | TS-09 | disabled / not ready / role / Host precedence for previews |
| `ts09_download_is_denied_for_every_status_and_changes_nothing` | `infra/tests/capabilities_api.rs` (Docker) | TS-09 | 403 `DOWNLOAD_DENIED` for every job status; job, allocation and file state unchanged |
| `ts09_download_unknown_foreign_and_malformed_ids_are_404_and_unaudited` | `infra/tests/capabilities_api.rs` (Docker) | TS-09 | unknown, foreign and malformed ids are the same 404 with no audit row |
| `ts09_download_and_capability_denials_survive_an_audit_failure` | `infra/tests/capabilities_api.rs` (Docker) | TS-09 | denials are returned when the audit write fails |
| `ts09_disabled_capabilities_leave_no_ledger_rows_or_files` | `infra/tests/capabilities_api.rs` (Docker) | TS-09 | no `backup_allocations` rows and no files after IF-06..IF-08 |
| `ts10_client_flags_cannot_select_a_real_adapter` | `infra/tests/capabilities_api.rs` (Docker) | TS-10 | request flags cannot select a real adapter (guard) |

Test support: `handlers/disabled_capability/test_support.rs` (in-memory spies, body-polling spy, request builders); dev-dependency `futures-core` (already in `Cargo.lock`).
