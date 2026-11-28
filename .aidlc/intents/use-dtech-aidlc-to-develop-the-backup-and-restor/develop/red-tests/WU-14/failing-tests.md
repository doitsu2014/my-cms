# WU-14 failing tests (code manifest)

All paths under `apps/web/src/`. Names are the vitest test titles (prefix shows the TS id). G = guard (passed in the red run because the behaviour already held or the stub trivially satisfied it).

| Test (title or pattern) | File | TS | Asserts | Red |
|---|---|---|---|---|
| ts12a_renders_{queued,running,succeeded,failed,recovery-required,incomplete,unknown}_as_text_with_no_download_control (7) | app/admin/backup/components/job-status-badge.test.tsx | TS-12a | each status incl. succeeded/recovery-required/unknown renders its text; no button, link or [download] | red |
| ts12a_unknown_status_shows_the_safe_fallback_and_never_a_success_label | job-status-badge.test.tsx | TS-12a | "Unknown status" shown, no "succeeded" | red |
| ts37_status_is_text_and_the_icon_is_hidden_from_assistive_technology | job-status-badge.test.tsx | TS-37 | text label visible, svg aria-hidden | red |
| labels an incomplete job as synthetic, database-only and not restorable when detailed | job-status-badge.test.tsx | TS-12a | incomplete note when detailed | red |
| omits the incomplete description unless detailed | job-status-badge.test.tsx | TS-12a | no note by default | G (null stub) |
| ts12a_status_{six values}_parses_renders_and_offers_no_download (6) | components/job-detail.test.tsx | TS-12a | zod parse then render each status, no download button/link/[download], "Download is not available" text | red |
| ts12a_an_unrecognised_status_renders_the_safe_fallback_with_no_download | job-detail.test.tsx | TS-12a | server value "archived" becomes unknown and renders "Unknown status" | red |
| shows the incomplete outcome, reason, components and measured progress as text | job-detail.test.tsx | TS-12a/TS-37 | reason, component list, progress, correlation, instance | red |
| says progress is not measured ... | job-detail.test.tsx | TS-12a | null progress and empty components copy | red |
| ts18_html_bearing_server_strings_render_escaped_and_never_as_markup | job-detail.test.tsx | TS-18 | canary in 5 server fields renders as escaped text; no img, no onerror, innerHTML has &lt;img | red |
| renders a labelled table with each job status as text | components/job-list.test.tsx | TS-12a/TS-37 | table named "Backup jobs", status text, column headers | red |
| ts12a_every_status_row_renders_without_any_download_control | job-list.test.tsx | TS-12a | 7 statuses, 8 rows, no download control | red |
| ts37_a_job_is_selected_with_the_keyboard_and_marks_the_current_one | job-list.test.tsx | TS-37 | tab order, Enter selects, aria-current | red |
| ts37_announces_loading_empty_and_polling_states_through_a_polite_live_region | job-list.test.tsx | TS-37 | role=status aria-live=polite text for each state | red |
| shows an alert for a request failure ... fixed copy | job-list.test.tsx | TS-36/TS-37 | alert while retrying, table kept | red |
| ts36_shows_an_error_banner_and_a_manual_refresh_when_unknown_status_polling_is_exhausted | job-list.test.tsx | TS-36 | exhausted alert and Refresh button | red |
| ts18_html_bearing_server_strings_render_escaped | job-list.test.tsx | TS-18 | script canary in id and label is escaped | red |
| ts50_escalation_visible_names_the_project_owner_and_cleanup_exhaustion | components/backup-status-banner.test.tsx | TS-50 | degraded, "Escalate to the project owner", cleanup.exhausted count | red |
| ts50_escalation_visible_for_ownership_incidents_alone | banner test | TS-50 | ownership_incidents text and owner | red |
| ts50_ready_without_escalation_shows_a_status_and_no_escalation_text | banner test | TS-50 | ready, role=status, no owner text, no alert | red |
| ts50_not_ready_state_is_announced_without_escalation | banner test | TS-50 | not_ready text | red |
| ts50_database_unhealthy_is_stated_in_text | banner test | TS-50 | db_healthy false text and owner | red |
| ts50_disabled_with_reason_{NOT_CONFIGURED,NON_LOOPBACK_BIND,INVALID_CONFIG}_... (3) | banner test | TS-50 | disabled text, per-reason copy, project owner named | red |
| ts50_disabled_with_a_null_reason_still_renders_a_generic_disabled_message | banner test | TS-50 | null reason generic copy | red |
| ts50_schema_rejects_a_disabled_reason_outside_the_closed_set... | banner test | TS-50 | zod closed set (WU-12 schema) | G |
| shows fixed copy while loading and when the status request failed | banner test | TS-50 | loading status, error alert | red |
| ts18_html_bearing_owner_string_renders_escaped | banner test | TS-18 | owner canary escaped | red |
| ts51_is_labelled_synthetic_and_shows_upload_as_unavailable | components/synthetic-preview-panel.test.tsx | TS-51 | heading, "no real package", disabled upload, no download/confirm | red |
| ts51_preflight_names_the_fixture_package_target_instance_and_data_in_scope | panel test | TS-51 | fixture_label, target label, environment, data_scope, result, impact summary | red |
| ts51_the_selected_fixture_is_the_one_sent | panel test | TS-51 | fixture id passed to IF-09 | red |
| ts51_confirm_posts_the_decision_and_shows_executed_false_with_the_identification_fields | panel test | TS-51 | IF-10 confirm call, message, "not executed" | red |
| ts51_cancel_posts_cancel_and_confirms_nothing_changed | panel test | TS-51 | IF-10 cancel call, cancelled copy | red |
| ts51_an_incompatible_or_corrupt_package_cannot_be_confirmed | panel test | TS-51 | no confirm button, "cannot be restored" | red |
| ts51_a_failed_request_shows_fixed_copy_and_never_the_server_string | panel test | TS-51 | fixed alert copy | red |
| ts51_a_second_click_while_a_request_is_in_flight_sends_one_request | panel test | TS-51 | in-flight guard | red |
| ts37_controls_have_accessible_names_and_work_from_the_keyboard | panel test | TS-37 | label, tab order, Enter, live region | red |
| ts18_html_bearing_server_strings_render_escaped | panel test | TS-18 | canary in 4 IF-09 fields escaped | red |
| ts36_declares_the_interface_contract_constants | hooks/use-backup-jobs.test.ts | TS-36 | 2000, 30000, 10 | G (constants in stub) |
| ts36_loads_once_on_mount_with_the_caller_token | use-backup-jobs.test.ts | TS-36 | initial load | red |
| ts36_polls_every_2s_while_any_job_is_non_terminal | use-backup-jobs.test.ts | TS-36 | 1999 ms no call, 2000 ms call (fake timers) | red |
| ts36_stops_polling_when_every_job_is_terminal | use-backup-jobs.test.ts | TS-36 | no calls after terminal | red |
| ts36_does_not_poll_at_all_when_the_first_list_is_all_terminal_or_empty | use-backup-jobs.test.ts | TS-36 | one call only | red |
| ts36_backs_off_by_doubling_from_2s_to_a_30s_ceiling_on_request_errors | use-backup-jobs.test.ts | TS-36 | 2,4,8,16,30,30 s | red |
| ts36_resets_the_backoff_after_a_success | use-backup-jobs.test.ts | TS-36 | backoff restarts at 2 s | red |
| ts36_keeps_the_last_jobs_while_a_poll_fails | use-backup-jobs.test.ts | TS-36 | jobs kept, phase error | red |
| ts36_treats_unknown_as_non_terminal_for_at_most_10_polls_then_stops_with_exhausted | use-backup-jobs.test.ts | TS-36 | exactly 10 calls then stop, exhausted | red |
| ts36_an_unknown_status_that_resolves_to_terminal_stops_normally_without_exhaustion | use-backup-jobs.test.ts | TS-36 | no false exhaustion | red |
| ts36_cleans_up_on_unmount_so_no_timer_or_request_survives | use-backup-jobs.test.ts | TS-36 | getTimerCount 0 after unmount | red |
| ts36_an_in_flight_response_after_unmount_is_ignored_and_schedules_nothing | use-backup-jobs.test.ts | TS-36 | late response ignored | G (stub never calls the API; passes trivially) |
| ts36_refresh_reloads_immediately_and_restarts_polling_after_a_stop | use-backup-jobs.test.ts | TS-36 | refresh | red |
| ts36_a_token_change_restarts_the_load_with_the_new_token | use-backup-jobs.test.ts | TS-36 | token change | red |
| loads the IF-11 status once on mount / reports an error phase / refresh reloads | hooks/use-backup-status.test.ts | TS-50 support | status hook | red (3) |
| ignores a response that arrives after unmount | use-backup-status.test.ts | TS-50 support | late response ignored | G (stub) |
| posts only fixture_id to the preflight path ... | domains/backup.test.ts | TS-51 support | IF-09 client | red |
| posts fixture_id and decision to the restore-confirmation path | domains/backup.test.ts | TS-51 support | IF-10 client | red |
| rejects a preview body that does not match the contract and surfaces typed errors | domains/backup.test.ts | TS-51 support | zod validation, typed error | red |
| closes the fixture id, result, executed and decision sets in the schemas | domains/backup.test.ts | TS-51 support | schema closure (schemas were written complete before the red run) | G |
| renders the status banner, the job list and the synthetic preview panel (page.status.test.tsx) | page.status.test.tsx | TS-50/TS-51 wiring | page composition | red |
| shows the escalation banner ... / selecting a job shows its detail ... / refreshes the job list and status after a job is created / keeps the page usable when the jobs and status requests fail | page.status.test.tsx | TS-12a/TS-36/TS-50 wiring | page wiring | red (4) |

Pre-existing WU-12/WU-13 tests (33 in backup.test.ts before WU-14, plus page.test.tsx, instance-selector, job-create-panel) are not WU-14 tests; they passed throughout.

Author: worktree worker (model `sonnet`, commit `9dec63a` on base `0021087`), integrated and checked by the conductor.
