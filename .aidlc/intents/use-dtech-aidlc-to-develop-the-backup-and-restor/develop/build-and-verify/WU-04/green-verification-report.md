# GVR-04 Green verification report: WU-04

- **ID:** GVR-04 (WU-04)
- **Stage:** Develop / build-and-verify, unit WU-04 of WU-01..WU-15. Not the release-candidate gate.
- **Status:** WU-04 green on this host, 2026-11-22.
- **Inputs:**
  - [red-tests/WU-04](../../red-tests/WU-04/failing-tests.md)
  - [implementation.md](implementation.md)
  - [ids.json](../../implementation-design/ids.json)
  - [acceptance-criteria](../../../analyze/problem-framing/acceptance-criteria.md)
  - [quality-requirements](../../../analyze/context-and-risk/quality-requirements.md)
- **Logs:**
  - [green-wu04-cargo-test.log](../evidence/green-wu04-cargo-test.log)
  - [red-wu04-delete-fail-closed.log](../evidence/red-wu04-delete-fail-closed.log)
  - [mutation-wu04-conductor.log](../evidence/mutation-wu04-conductor.log)

Provenance:
- Results are Observed from the conductor's own runs on 2026-11-22 (Windows 11, cargo 1.100.0, Docker 29.7.2, `postgres:15-alpine`).
- Mutation rows marked "agent" were run and reported by the mutation subagent. That agent checked each edit applied before testing; its scripts are in the session scratchpad.
- Rows marked "conductor" were run by the conductor, with the log linked above.

## 1. Red then green, same tests

- **Red** (2026-11-22, [log](../../red-tests/evidence/red-wu04-cargo-test.log)): 62 new tests failed on assertions on Windows.
- **Green** (from `apps/api`):
  - `cargo test -p domain_backup --no-fail-fast`: exit 0, **82 passed, 0 failed, 105 ignored**.
  - `-- --ignored`: exit 0, **105 passed, 0 failed** (80 s).
- **Red tests unchanged:** every file under `src/infra/tests/` was byte-identical to the red snapshot through build, fix and the mutation pass. The conductor then added two tests, and a diff shows 0 removed lines (+25 and +24 added). No test was changed, skipped or ignored.
- **Earlier units:** WU-01..WU-03 code and tests are byte-identical.
- **Build-stage red-first:** `delete_fails_closed_when_the_namespace_directories_are_missing` was written after review and run before the fix. It failed with `left: Ok(AlreadyAbsent), right: Err(Unavailable)` ([log](../evidence/red-wu04-delete-fail-closed.log)), then passed after the one-line fix.
- **Identifiers:** `dtech-aidlc ids check` passes, with 0 unregistered.

## 2. Results per test level

| Level | Result |
|---|---|
| Unit and filesystem (no Docker) | 82 passed. The 4 file-symlink tests skip on this host (os error 1314: no symlink privilege). The 4 `cfg(unix)` permission tests are not compiled on Windows. |
| Integration (DB, PostgreSQL 15) | 105 passed |
| Multi-process | The two-process lock tests passed: a child process is refused while the parent holds the lock, and the parent is refused until the child exits. |
| Concurrency | Concurrent and forced-interleaving budget registrations never exceed 1 GiB. Racing `mark_written`/`release` has one winner. |
| End to end, performance | not run (not in scope) |
| Security | Verified on this host: T-11 path and link safety (directory junctions), T-12, T-13 (budgets, bounded inventory) and T-08 (audit in the same transaction). T-14 Unix permissions and file-symlink handling are not verifiable on Windows; CI (Ubuntu) must run them. The Windows ACL gap is the owner-accepted residual. |
| Static | `cargo fmt -- --check` exit 0. `cargo clippy --all-targets` exit 0, with no warning in `domain_backup`. |

## 3. Mutation proof

| Guard | Result | By |
|---|---|---|
| Register-before-write, state and reservation checks in `write` | caught | `write_fails_closed_when_the_ledger_is_unavailable`, `write_is_refused_for_every_state_except_registered`, `write_beyond_the_reserved_bytes_is_refused_without_a_partial_file` (agent) |
| 256 MiB and 1 GiB bounds; advisory-lock serialisation; total counts every state except deleted | caught | `register_enforces_the_256_mib_per_allocation_budget`, `register_enforces_the_1_gib_total_budget_durably`, `forced_interleaving_never_exceeds_the_total_budget`, `total_budget_counts_every_state_except_deleted` (agent) |
| Audit in the same transaction and rollback on audit failure (register, claim, delete, orphan) | caught | `register_writes_allocation_registered_audit_in_the_same_transaction`, `register_is_refused_and_rolled_back_when_audit_fails`, `claim_for_delete_audits_in_the_same_transaction_and_is_conditional`, `insert_orphan_audits_in_the_same_transaction_and_fails_closed`, `mark_deleted_commits_with_audit_and_has_no_shortcut` (agent) |
| `mark_written` state filter, `release` state filter, claim due/backoff filters, epoch stamping, retry limit 3, orphan idempotency | caught | `mark_written_is_a_compare_and_set`, `release_moves_registered_and_written_to_release_pending`, `claim_for_delete_waits_for_the_backoff_to_pass`, `claim_for_delete_stamps_the_current_epoch`, `cleanup_failures_retry_with_backoff_then_exhaust_after_three`, `insert_orphan_twice_keeps_the_first_grace` (agent) |
| **`mark_written` version compare-and-set** | **caught after test-after** | The agent found that dropping or weakening the version check was not caught. The new `mark_written_with_a_wrong_version_on_a_registered_row_loses` fails for both edits (conductor) |
| Storage key parse; directory links in `open`/write; overwrite (both guards removed); delete exact-name regular file only; case-folded twin survives; inventory bound and `truncated`; inventory never follows links; classification table | caught | `storage_key_parse_refuses_every_non_opaque_name`, `open_refuses_an_alloc_directory_that_is_a_link`, `write_refuses_when_alloc_is_swapped_for_a_link_after_open`, `write_never_overwrites_an_existing_file`, `delete_refuses_a_directory_at_the_key_path`, `delete_never_follows_or_removes_a_link`, `delete_never_removes_a_case_folded_forged_file`, `inventory_never_exceeds_max_entries_and_flags_truncation`, `inventory_reports_kinds_and_never_follows_links`, `classify_valid_opaque_name_follows_the_ledger_table` (agent) |
| **`delete` fails closed on a missing namespace** | **caught** | Reverting the fix to `Ok(AlreadyAbsent)` fails `delete_fails_closed_when_the_namespace_directories_are_missing` (conductor). The conductor's first attempt edited the wrong line (outside `delete`); it is logged as void and was redone correctly. |
| Lock exclusivity; lock path is a sibling; lock path keeps the whole name | caught | `second_holder_is_refused_and_drop_releases_the_lock`, `lock_held_by_parent_refuses_a_second_process`, `lock_path_is_a_sibling_of_the_root`, `lock_path_appends_to_the_whole_root_name` (agent) |
| `create_new` alone, or the key-path pre-check alone | not independently caught | Each covers the other. Only removing both fails (`write_never_overwrites_an_existing_file`). Isolating `create_new` needs a concurrent check-then-create test (known limitation). |
| `is_link` clause in the directory check | equivalent mutant | Redundant with `!is_dir()` under `symlink_metadata` |
| Write pre-check following a file link; lock link handling (pre-open, post-open, reparse flag) | not verifiable on this host | Covered only by the file-symlink tests, which skip on Windows. CI or Linux must run them before the release-candidate gate. |

**Result:** every guard that is verifiable on this host and required by the contract has a test that fails when the guard is removed.

## 4. Quality requirements

| QR | Status after WU-04 |
|---|---|
| A-05 Staging expiry enforcement | Partial. Expiry and orphan grace are stored per row, and claims wait for them. The sweeper and cleaner (WU-06) enforce them. |
| P-05 Concurrent job limits | Unchanged (store level, WU-03) |
| PR-05 Staging data protection | Partial. Opaque keys, no link following, exact-name deletes, a private directory and file on Unix (Linux/CI to verify), and a lock outside the namespace. Windows ACL is the owner-accepted residual. |
| A-03 Crash recovery | Partial. Register-before-write, the epoch on claim, idempotent orphan insert and bounded retries are in place. Reconciliation is WU-06. |
| PR-04 Audit content safety | Allocation events carry ids only. Errors carry no SQL or path text. |
| Others | not in WU-04 scope |

## 5. Codebase map

Not updated in this unit; the crate is not yet composed into the gateway (WU-10). This is the same decision as for WU-01..WU-03.

## 6. Trace table

One row per acceptance criterion, cumulative over WU-01..WU-04.

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-01, US-05 | AC-001 Instance selection and authorization | FN-01 | WU-01 config tests | config part passed; rest not run |
| US-02, US-06 | AC-002 S1 Job status is observable | FN-15, FN-16 | WU-03 store tests | store level passed |
| US-02, US-06 | AC-002 S2 Required capture fails | FN-15, FN-23 | WU-03 CAS tests; `due_for_cleanup_returns_exactly_the_due_rows`, `rows_are_stamped_from_the_injected_clock` | store level passed |
| US-02, US-06 | AC-002 S3 U1 database-only result | FN-15 | WU-01..WU-03 guard tests | passed |
| US-03 | AC-003 S1 Invalid or incompatible package | none (WU-09) | none yet | not run |
| US-03 | AC-003 S2 Unsafe archive | none (deferred) | none | not run (deferred) |
| US-02, US-03, US-04, US-06 | AC-003 S3 U1 real-data staging is not enabled | FN-21, FN-22, FN-23, FN-24 | `the_ledger_row_exists_before_the_first_byte_is_written`, `write_without_a_registered_allocation_is_refused_and_writes_nothing`, `register_enforces_the_1_gib_total_budget_durably`, `storage_key_parse_refuses_every_non_opaque_name`, `delete_fails_closed_when_the_namespace_directories_are_missing`, `inventory_never_exceeds_max_entries_and_flags_truncation`, `lock_held_by_parent_refuses_a_second_process` | passed (synthetic-fixture storage, bounded and private) |
| US-04 | AC-004 S1 Confirmation describes impact | none yet (WU-09) | none yet | not run |
| US-04 | AC-004 S2 Restore is cancelled | none yet (WU-09) | none yet | not run |
| US-04 | AC-004 S3 Restore failure | none (restore not authorized) | none | not run (deferred) |
| US-05, US-06 | AC-005 S1 Unauthorized operation or resource access | FN-39, FN-01 | WU-01..WU-03 tests | partial; authorization not run |
| US-05, US-06 | AC-005 S2 Audit records a sensitive operation | FN-21, FN-34 | `register_writes_allocation_registered_audit_in_the_same_transaction`, `claim_for_delete_audits_in_the_same_transaction_and_is_conditional`, `insert_orphan_audits_in_the_same_transaction_and_fails_closed`, WU-03 audit tests | passed for job and allocation events |
| US-05, US-06 | AC-005 S3 Audit store unavailable before effect | FN-21, FN-34 | `register_is_refused_and_rolled_back_when_audit_fails`, `claim_and_delete_commit_are_refused_when_audit_fails`, `register_fails_closed_when_the_ledger_is_unreachable`, WU-03 audit-failure tests | passed at store level |
| US-02, US-03, US-04 | AC-006 S1 Production restore readiness | none (production not authorized) | none | not run (out of scope) |
| US-02, US-03, US-04 | AC-006 S2 Unresolved U1 design blockers | FN-01, FN-21 | `defaults_match_owner_values`, the WU-04 budget tests (owner-approved Q-05 bounds) | passed |
