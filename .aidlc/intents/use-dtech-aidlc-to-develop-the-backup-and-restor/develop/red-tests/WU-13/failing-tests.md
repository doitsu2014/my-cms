# WU-13 failing tests (code manifest)

| Test | File | TS | Asserts | Red? |
|---|---|---|---|---|
| ts46_renders_the_instance_label_and_environment_label | apps/web/src/app/admin/backup/components/instance-selector.test.tsx | TS-46 | radio named by label; label and environment text present | red |
| ts46_marks_the_selected_instance_and_reports_selection | same | TS-46 | onSelect(id) on click; checked reflects selectedId | red |
| ts38_empty_list_offers_nothing_to_select_and_says_so | same | TS-38 | no radio; "no backup instance is available" | red |
| ts38_unavailable_list_offers_nothing_to_select_and_a_retry | same | TS-38 | no radio; alert; retry callback | red |
| ts38_loading_state_offers_nothing_to_select | same | TS-38 | no radio; status | red |
| renders server strings only as text | same | T-03 hardening | hostile label stays text, no img element | red |
| ts52_sends_a_fresh_crypto_random_uuid_idempotency_key_for_every_submit | .../components/job-create-panel.test.tsx | TS-52 panel | two submits send two distinct randomUUID keys with token and instance id | red |
| ts52_double_submit_guard_sends_one_request_while_one_is_in_flight | same | TS-52 panel | 3 rapid clicks, one request, button disabled | red |
| ts52_submit_without_a_selected_instance_sends_nothing | same | TS-52 / TS-38 | disabled, no request with null instance | red |
| reports the created job to the caller | same | TS-52 | onCreated(job, replayed) | red |
| shows fixed failure copy, never server strings, and allows a retry with a new key | same | TS-52 | fixed alert copy, re-enabled, new key | red |
| lists instances with the caller token and renders label and environment | apps/web/src/app/admin/backup/page.test.tsx | TS-46 | listBackupInstances(token), label/env, h1 | red |
| ts38_empty_instance_list_disables_submission | same | TS-38 | empty list: submit disabled, no create call | red |
| ts38_unavailable_instance_list_disables_submission_and_can_be_retried | same | TS-38 | 503 list: alert, disabled; retry enables | red |
| ts38_submission_is_disabled_while_the_list_is_loading | same | TS-38 | disabled during load | red |
| selects the single listed instance and creates a job for it with a fresh key | same | TS-52 | auto-select, create(token, id, uuid), reference shown | red |
| ts35_only_shows_backup_navigation_to_administrators | apps/web/src/app/admin/components/left-menu.test.tsx | TS-35 | Backup link absent for non-admin, present with /admin/backup href for admin | red |
| ts35_backup_route_renders_for_administrators_inside_the_admin_layout | apps/web/src/App.test.tsx | TS-35 | admin sees page inside layout | red |
| ts35_backup_route_shows_access_denied_to_non_administrators | same | TS-35 | writer sees access denied, not page | red |
| ts35_backup_route_redirects_unauthenticated_visitors_to_login | same | TS-35 | unauthenticated goes to login | red |

Guards (already passing in red run): "only shows SEO navigation to administrators" (existing left-menu test, unchanged).
TS-35 is UX only; not evidence for server-side authorization.

Author: parallel worktree worker (model `sonnet`, commit `fedfc30`), integrated and checked by the conductor.
