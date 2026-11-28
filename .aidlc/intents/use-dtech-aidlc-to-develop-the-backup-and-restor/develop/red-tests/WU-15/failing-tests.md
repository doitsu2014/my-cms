# WU-15 failing-tests (code manifest)

Branch commit: `3213b77 WU-15 worker` on top of `8ceb568` (WU-01..WU-14). No application code changed.

| Test | File | TS | Asserts | Red or guard |
|---|---|---|---|---|
| `ts58_feature_doc_lists_open_items_as_open_and_synthetic_only` | apps/api/domain_backup/src/infra/tests/verification_sweep.rs | TS-58 | docs/features/backup-restore.md lists Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention, non-owner guards (16 KiB), "synthetic-only", BACKUP_ENABLED and README pointer | RED (assertion: "docs/features/backup-restore.md must list Q-04 as open"), green after the doc update |
| `ts58_implementation_plan_lists_open_items_as_open` | same | TS-58 | plan section 3.1 lists the same open items, audit retention and non-owner guards (read-only) | guard (plan already listed them). REMOVED at integration by the conductor: a product test must not read an intent artifact under `.aidlc/` (it would break when the intent is archived). The same check is recorded as a one-off evidence run: `build-and-verify/WU-15/evidence/ts58-implementation-plan-check.log` |
| `ts39_job_creation_latency_is_measured_without_a_target` | same (Docker, ignored) | TS-39 | 30 sequential creates through started service routes, each 202 `queued`, job run to terminal by `worker_step`; prints min/median/p95/max; no target asserted | guard / measurement (characterization) |
| `ts40_concurrency_and_temporary_bytes_stay_bounded_under_load` | same (Docker, ignored) | TS-40 | 5 rounds x (12 concurrent creates + 4 concurrent `worker_step`): only 202 or 409 `CONCURRENCY_LIMIT`; each accepted job run exactly once; sampled active jobs <= 1; reserved bytes and bytes on disk within CFG-11/12 (256 MiB, 1 GiB) | guard (characterization) |
| `ts41_admin_creates_job_ends_incomplete_and_download_is_denied` | same (Docker, ignored) | TS-41 API part (also TS-09, TS-11, TS-58 API labels) | create 202 -> worker -> `incomplete`, `synthetic` true, `download_eligible` false, never `succeeded`; `GET /backup/jobs/{id}/download` -> 403 `DOWNLOAD_DENIED`, no Location/Content-Disposition/Content-Range, id not echoed; status `synthetic_only` and instances `synthetic` true | guard (characterization) |
| `positive control: reports a canary placed in a js asset` | apps/web/scripts/bundle-canary-scan.test.mjs | TS-17 full | scanner reports the canary and the relative file | RED against stub scanner (`expected [] to deeply equal [ ObjectContaining{...} ]`) |
| `positive control: reports a canary in html, css, json and map files` | same | TS-17 full | scanner finds four canaries in four file types | RED against stub scanner |
| `declares canaries for BACKUP_*, VITE_ and PUBLIC_ build variables` | same | TS-17 full | canary set covers the three env families, all `CANARY-` prefixed | guard (stub already exported the constant) |
| `negative control: a clean output yields no hits` | same | TS-17 full | clean dist gives no hits | guard (stub returns [] by construction) |
| `pnpm scan:bundle-canary` (script, not a vitest test) | apps/web/scripts/bundle-canary-scan.mjs | TS-17 full | real `pnpm build` with 6 canary env vars; dist scan clean; positive control A (canary injected into a copy of the real dist) detected; positive control B (build whose source reads a `PUBLIC_*` variable) detected | evidence run, green first time |

Existing tests cited (not changed): `http_api::ts45_instances_and_status_return_the_configured_safe_contract`, `http_api::ts17_store_outage_paths_are_safe_and_status_stays_200_degraded` (TS-17 API canary), WU-14 `job-detail.test.tsx` `ts12a_*` (no download control).

Test-support change: `config`, `test_domain_context`, `admin`, `request`, `body` in `infra/tests/http_api.rs` became `pub(super)` so the new file reuses them.

Author: worktree worker (model `sonnet`, commit `3213b77` on base `8ceb568`), integrated and checked by the conductor.
