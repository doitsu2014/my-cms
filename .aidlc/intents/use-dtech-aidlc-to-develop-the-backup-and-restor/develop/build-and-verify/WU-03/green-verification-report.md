# GVR-03 Green verification report: WU-03

- **ID:** GVR-03 (WU-03)
- **Stage:** Develop / build-and-verify, unit WU-03 of WU-01..WU-15. Not the release-candidate gate.
- **Status:** WU-03 green on this host, 2026-11-22.
- **Inputs:**
  - [red-tests/WU-03](../../red-tests/WU-03/failing-tests.md)
  - [implementation.md](implementation.md)
  - [ids.json](../../implementation-design/ids.json)
  - [acceptance-criteria](../../../analyze/problem-framing/acceptance-criteria.md)
  - [quality-requirements](../../../analyze/context-and-risk/quality-requirements.md)
- **Logs:**
  - [green-wu03-cargo-test.log](../evidence/green-wu03-cargo-test.log)
  - [mutation-wu03-claim.log](../evidence/mutation-wu03-claim.log)
  - [mutation-wu03-test-after.log](../evidence/mutation-wu03-test-after.log)

Provenance: Observed from the conductor's own runs on 2026-11-22 (Windows 11, cargo 1.100.0, Docker 29.7.2, `postgres:15-alpine`). Mutation rows marked "agent" were run and reported by the mutation subagent. Rows marked "conductor" were re-run or added by the conductor; the logs are linked above.

## 1. Red then green, same tests

- **Red** (2026-11-22, [log](../../red-tests/evidence/red-wu03-cargo-test.log)): 45 new tests failed on assertions against the logic-free shell (4 unit, 41 DB).
- **Green** (from `apps/api`):
  - `cargo test -p domain_backup --no-fail-fast`: exit 0, **41 passed, 0 failed, 77 ignored**.
  - `-- --ignored`: exit 0, **77 passed, 0 failed** (61 s).
- **Red tests unchanged:**
  - Every test file was byte-identical to the red snapshot through build, fix and the mutation pass.
  - The conductor then appended two test-after tests (section 3); a `diff` shows 0 removed lines and 30 + 36 added lines in two files.
  - WU-01 and WU-02 code and tests are byte-identical to before.
  - No test was changed, skipped or ignored.
- **Identifiers:** `dtech-aidlc ids check` passes, with 0 unregistered.

## 2. Results per test level

| Level | Result |
|---|---|
| Unit (no Docker) | 41 passed (37 earlier, 4 new) |
| Integration (DB, PostgreSQL 15) | 77 passed (34 earlier, 41 red-stage, 2 test-after) |
| Concurrency | Deterministic interleavings (barriers and lock-wait polling, no fixed sleeps). Claim double-win, late-writer overwrite and racing terminal transitions all passed. |
| End to end | Not run. The store is not composed into the API yet (WU-08/WU-10). |
| Performance | Not run (not in scope) |
| Security | T-08 (audit-before-effect in one transaction) and T-09 (no DB text in errors; digests only, never raw keys) are verified at store level. T-07 and T-12 are verified for concurrency. |
| Static | `cargo fmt -- --check` exit 0. `cargo clippy --all-targets` exit 0, with no warning in `domain_backup`. |

## 3. Mutation proof

| Guard | Result | By |
|---|---|---|
| CAS version filter, status filter, version bump (transition and claim) | caught | `a_stale_version_or_status_loses_and_changes_nothing`, `each_legal_edge_applies_bumps_the_version_and_stamps_times`, the claim tests (agent) |
| Illegal-edge refusal | caught | `edges_outside_the_u1_graph_are_refused_without_changes`, `terminal_rows_are_immutable_through_the_store` (agent). Without the Rust check the WU-02 DB trigger still blocks terminal rows, but the error variant changes, and the test catches that. |
| Unique-violation mapping, PK clash, idempotency row, accept on caller's transaction | caught | `concurrent_accepts_for_one_instance_create_exactly_one_job`, `an_idempotency_key_collision_is_a_unique_violation_not_a_persistence_error`, `duplicate_job_id_is_a_persistence_error_not_a_concurrency_conflict`, `accept_is_part_of_the_callers_transaction` (agent) |
| Audit on the store's transaction; audit failure refuses and rolls back; Rejected vs Unavailable mapping | caught | `a_transition_hands_its_own_uncommitted_transaction_to_the_audit_sink`, `an_audit_failure_refuses_the_transition`, `an_audit_outage_is_a_store_outage_for_transitions_and_claims`, `an_audit_row_written_before_the_sink_fails_rolls_back_with_the_claim` (agent) |
| `classify_audit_error` fail-closed; sink plain INSERT on given transaction; event, outcome and reason spellings; `Digest` validation | caught | `connection_level_and_unclassifiable_failures_are_an_outage`, `a_row_recorded_on_the_callers_transaction_is_visible_only_after_commit`, `event_kinds_map_to_the_closed_event_set`, `reason_codes_map_to_the_ifc_10_1_spellings`, `a_digest_is_only_a_sha256_hex_string` (agent) |
| **Claim double-win** (naive read-then-update, no lock, no outer status recheck) | **caught by 4 tests** | `exactly_one_of_n_concurrent_claims_wins`, `concurrent_claims_never_claim_a_job_twice`, `a_claim_blocked_on_a_row_taken_by_another_transaction_does_not_claim_it_again`, `a_claim_and_a_queued_failure_race_to_exactly_one_winner` (conductor). The agent had reported this guard "not caught", but its edit had not applied: its anchor text did not match the source. |
| Claim without `SKIP LOCKED` (blocking `FOR UPDATE`), or without a row lock but with the outer status recheck | not caught, by design | Both variants still never double-claim: the outer `status = 'queued'` recheck prevents it. They can only make a claim wait behind another transaction. FN-16 requires a conditional claim with concurrency 1, not non-blocking behaviour, so no test pins `SKIP LOCKED`. Recorded as known limitation L-07. (conductor) |
| Outer status filter alone removed | not caught, redundant | The inner filter and the lock cover it. Dropping the inner filter is caught. (agent) |
| `accept` digest check (fixer hardening) | caught after test-after | `accept_refuses_digests_that_are_not_sha256_hex_without_writing`: removing the whole check, or only the fingerprint half, fails (conductor) |
| SQLSTATE allow-list (fixer hardening) | caught after test-after | `sqlstates_split_into_statement_rejections_and_fail_closed_outages` raises real server errors (22001, 23505, 42P01 and P0001 are Rejected; 53100, 25006, 57014, 40001 and XX000 are Unavailable). Treating every code as a rejection fails, and so does dropping class P0 (conductor). |

**Result:** every guard that the contract requires has a test that fails when the guard is removed. The two uncaught claim variants are behaviour-preserving with respect to the contract.

## 4. Quality requirements

| QR | Status after WU-03 |
|---|---|
| A-01 Job idempotency | Partial. The store inserts the idempotency row atomically with the job and reports any unique violation as a conflict. Replay and key-reuse decisions come in WU-07. |
| P-05 Concurrent job limits | Store level verified: a second active job for an instance is refused durably, including under concurrent creates. |
| A-03 Crash recovery | Partial. The CAS and the "late worker cannot overwrite a terminal result" rule are verified. Reconciliation and fencing come in WU-06. |
| PR-04 Audit content safety | Partial. Digests only (validated). Audit errors and store errors carry no DB text. Projection and canary tests come in WU-08/WU-15. |
| O-02 Safe error projection | Partial. Store and audit errors are free of DB text. HTTP projection is WU-08. |
| Others | Not in WU-03 scope; not run. |

## 5. Codebase map

`domain_backup` gains ports and infra, but it is still not composed into the gateway. The codebase map is not updated in this unit and will be updated in WU-10 (same decision as WU-01 and WU-02).

## 6. Trace table

One row per acceptance criterion, cumulative over WU-01..WU-03.

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-01, US-05 | AC-001 Instance selection and authorization | FN-01 (instance config) | WU-01 config tests. Listing and authorization come in WU-08/WU-13. | config part passed; the rest not run |
| US-02, US-06 | AC-002 S1 Job status is observable | FN-15, FN-16 | `accept_inserts_a_queued_job_and_its_idempotency_row`, `get_returns_the_stored_row_and_none_for_an_unknown_id`, `claim_next_claims_the_oldest_queued_job`, `concurrent_accepts_for_one_instance_create_exactly_one_job`, `exactly_one_of_n_concurrent_claims_wins` | store level passed; API and UI not run |
| US-02, US-06 | AC-002 S2 Required capture fails | FN-15 | `each_legal_edge_applies_bumps_the_version_and_stamps_times`, `a_late_worker_cannot_overwrite_a_terminal_result`, `racing_terminal_transitions_have_exactly_one_winner`, `a_stale_version_or_status_loses_and_changes_nothing` | store level passed; worker outcome (WU-05) not run |
| US-02, US-06 | AC-002 S3 U1 database-only result | FN-15 | WU-01/WU-02 guard tests; `terminal_rows_are_immutable_through_the_store`, `edges_outside_the_u1_graph_are_refused_without_changes` | passed |
| US-03 | AC-003 S1 Invalid or incompatible package | none (rejection previews in WU-09) | none yet | not run |
| US-03 | AC-003 S2 Unsafe archive | none (deferred) | none | not run (deferred) |
| US-02, US-03, US-04, US-06 | AC-003 S3 U1 real-data staging is not enabled | FN-01; allocation schema | WU-01 config tests; WU-02 allocation checks | passed |
| US-04 | AC-004 S1 Confirmation describes impact | none yet (WU-09) | none yet | not run |
| US-04 | AC-004 S2 Restore is cancelled | none yet (WU-09) | none yet | not run |
| US-04 | AC-004 S3 Restore failure | none (restore not authorized) | none | not run (deferred) |
| US-05, US-06 | AC-005 S1 Unauthorized operation or resource access | FN-39, FN-01; idempotency scope | WU-01 loopback and origin tests; WU-02 idempotency scope; `accept_refuses_digests_that_are_not_sha256_hex_without_writing` | guard, config, DB and store scope passed; authorization not run |
| US-05, US-06 | AC-005 S2 Audit records a sensitive operation | FN-34 | `a_transition_writes_one_job_audit_row_with_its_event_and_ids`, `a_transition_hands_its_own_uncommitted_transaction_to_the_audit_sink`, `a_row_recorded_on_the_callers_transaction_is_visible_only_after_commit`, `rolling_back_the_callers_transaction_leaves_no_audit_row`, `every_column_of_the_event_is_persisted`, `the_job_audit_row_excludes_free_text_stored_on_the_job` | passed for job events. `job.accepted` comes in WU-07. |
| US-05, US-06 | AC-005 S3 Audit store unavailable before effect | FN-34 | `an_audit_failure_refuses_the_transition`, `an_audit_failure_refuses_the_claim`, `an_audit_outage_is_a_store_outage_for_transitions_and_claims`, `sqlstates_split_into_statement_rejections_and_fail_closed_outages`, `connection_level_and_unclassifiable_failures_are_an_outage` | passed at store level. HTTP 503 projection comes in WU-08. |
| US-02, US-03, US-04 | AC-006 S1 Production restore readiness | none (production not authorized) | none | not run (out of scope) |
| US-02, US-03, US-04 | AC-006 S2 Unresolved U1 design blockers | FN-01, FN-37 | WU-01/WU-02 tests | passed |
