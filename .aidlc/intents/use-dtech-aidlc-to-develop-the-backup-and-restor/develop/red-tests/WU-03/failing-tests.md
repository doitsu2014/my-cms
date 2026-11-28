# FT-03 Failing tests (code manifest): WU-03

- **ID:** FT-03 (WU-03)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-03 of WU-01..WU-15.
- **Status:** red run recorded on 2026-11-22, PostgreSQL 15 via Docker.
- **Inputs:**
  - [work-units](../../implementation-design/work-units.md) WU-03
  - [ids.json](../../implementation-design/ids.json)
  - [test-strategy](../../implementation-design/test-strategy.md): TS-14 (the TS-14a store-level part) and TS-69
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md): section 9 traits, the audit sections and 10.1 codes
  - [functional-design](../../implementation-design/functional-design.md): FN-15, FN-16 and FN-34 (legacy F-15, F-16 and F-34), and section 6.6
  - [WU-02 known limitations](../../build-and-verify/WU-02/known-limitations.md)
- **Evidence:** [red-evidence.md](red-evidence.md), raw log [../evidence/red-wu03-cargo-test.log](../evidence/red-wu03-cargo-test.log).

Provenance:
- **Results:** Observed from the conductor's own run on 2026-11-22.
- **Identifiers:** taken from `ids.json` and the Analyze artifacts; `dtech-aidlc ids check` reports none unregistered.
- **Threat (T) ids:** follow test-strategy.
- **"Unit-own":** tests the work unit requires that have no TS id of their own.

## Files

All new files are under `apps/api/domain_backup/`.

| Path | Change | Shell behaviour |
|---|---|---|
| `src/ports/{mod.rs,job_store.rs,audit.rs}` | new | Holds the `JobStore` trait (`accept`, `get`, `transition`, `claim_next`) and the `AuditSink` trait. Types: `NewJob` (no status field, so the store can only insert `queued`), `IdemRecord`, `JobRow`, `TransitionResult::{Applied, Lost}`, `StoreError`, `AuditEvent`, `AuditEventKind` (closed set of 16), `AuditOutcome`, `AuditError::{Rejected, Unavailable}`, `Digest`, `ReasonCode` (IFC 10.1) |
| `src/infra/{mod.rs,db_job_store.rs,db_audit_sink.rs}` | new | Logic-free stubs. `accept` returns `Created` without writing, `get` returns `None`, `transition` returns `Applied`, `claim_next` returns `None`, `record` returns `Ok(())`, `classify_audit_error` always returns `Rejected`, and the `as_str` mappings return `""`. |
| `src/infra/tests/{mod.rs,support.rs,job_store.rs,audit_sink.rs}` | new | The tests, plus support code: fixture over `test_db.rs`, `FailingAuditSink` (Rejected), `OutageAuditSink` (Unavailable), `WriteThenFailSink`, `TxProbeSink`, and lock-wait helpers |
| `src/lib.rs` | modified | Declares `pub mod infra` and `pub mod ports` |
| `Cargo.toml` and `apps/api/Cargo.lock` | modified | Add `uuid` (v4, serde), the same version as `domain_seo` |

Nothing outside `apps/api/domain_backup/**` and `Cargo.lock` was touched: no WU-01/WU-02 code or tests, no entities (the store uses raw SQL), no gateway and no web.

## Tests

Commands, run from `apps/api`: `cargo test -p domain_backup --no-fail-fast`, then the same with `-- --ignored`. DB tests are `#[ignore = "requires Docker for PostgreSQL testcontainer"]`. Race tests use `flavor = "multi_thread"`, barriers and lock-wait polling, with no fixed sleeps. Module prefix: `infra::tests::`.

| Test | TS | AC | T | FN | Red reason (Observed: all FAILED) |
|---|---|---|---|---|---|
| `job_store::accept_inserts_a_queued_job_and_its_idempotency_row` | unit-own | AC-002 S1 | T-07 | FN-15 | job row missing |
| `job_store::accept_is_part_of_the_callers_transaction` | unit-own | AC-005 S2 | T-08 | FN-34 | row not written on the caller's tx |
| `job_store::second_accept_conflicts_while_the_first_job_is_queued` | TS-14 (14a) | AC-002 S1 | T-07 | FN-15 | shell returns Ok |
| `job_store::second_accept_conflicts_while_a_job_is_running` | TS-14 (14a) | AC-002 S1 | T-07 | FN-15 | shell returns Ok |
| `job_store::accept_succeeds_after_the_active_job_is_terminal_and_for_other_instances` | TS-14 (14a) | AC-002 S1 | T-07 | FN-15 | job row missing |
| `job_store::conflicting_accept_leaves_no_job_or_idempotency_row` | TS-14 (14a) | AC-002 S1 | T-07 | FN-15 | shell returns Ok |
| `job_store::concurrent_accepts_for_one_instance_create_exactly_one_job` | TS-14 (14a) | AC-002 S1 | T-07 | FN-15 | 6 creates win |
| `job_store::duplicate_job_id_is_a_persistence_error_not_a_concurrency_conflict` | unit-own | AC-002 S1 | T-07 | FN-15 | shell returns Ok |
| `job_store::an_idempotency_key_collision_is_a_unique_violation_not_a_persistence_error` | unit-own | AC-002 S1 | T-07 | FN-15 | shell returns Ok |
| `job_store::get_returns_the_stored_row_and_none_for_an_unknown_id` | unit-own | AC-002 S1 | T-06 | FN-15 | `get` returns None |
| `job_store::each_legal_edge_applies_bumps_the_version_and_stamps_times` | unit-own | AC-002 S2, S3 | T-06 | FN-15 | row not updated |
| `job_store::a_stale_version_or_status_loses_and_changes_nothing` | unit-own | AC-002 S2 | T-12 | FN-15 | returns Applied |
| `job_store::edges_outside_the_u1_graph_are_refused_without_changes` | unit-own | AC-002 S3 | T-06 | FN-15 | no IllegalTransition |
| `job_store::terminal_rows_are_immutable_through_the_store` | unit-own | AC-002 S3 | T-06 | FN-15 | no IllegalTransition |
| `job_store::transition_of_an_unknown_job_is_not_found` | unit-own | AC-002 S2 | T-06 | FN-15 | returns Applied |
| `job_store::a_late_worker_cannot_overwrite_a_terminal_result` | unit-own | AC-002 S2 | T-12 | FN-15 | first transition not applied |
| `job_store::racing_terminal_transitions_have_exactly_one_winner` | unit-own | AC-002 S2 | T-12 | FN-15 | two winners |
| `job_store::a_claim_and_a_queued_failure_race_to_exactly_one_winner` | unit-own | AC-002 S1 | T-07, T-12 | FN-16 | no audit row, version not bumped |
| `job_store::a_transition_blocked_on_the_row_lock_loses_after_the_winner_commits` | unit-own | AC-002 S2 | T-12 | FN-15 | returns Applied |
| `job_store::omitting_components_keeps_the_stored_components` | unit-own | AC-002 S3 | T-06 | FN-15 | row not updated |
| `job_store::reason_codes_map_to_the_ifc_10_1_spellings` (no Docker) | unit-own | AC-002 S2 | T-06 | FN-15 | `as_str` is empty |
| `job_store::a_transition_writes_one_job_audit_row_with_its_event_and_ids` | unit-own (TS-44b forward) | AC-005 S2 | T-09 | FN-34 | no audit row |
| `job_store::a_transition_hands_its_own_uncommitted_transaction_to_the_audit_sink` | TS-69 | AC-005 S2 | T-08 | FN-34 | sink never called |
| `job_store::an_audit_failure_refuses_the_transition` | TS-69 | AC-005 S2 | T-08 | FN-34 | shell returns Ok (sink Rejected; expects AuditUnavailable) |
| `job_store::an_audit_row_written_before_the_sink_fails_rolls_back_with_the_transition` | TS-69 | AC-005 S2 | T-08 | FN-34 | shell returns Ok |
| `job_store::an_audit_outage_is_a_store_outage_for_transitions_and_claims` (conductor) | TS-69 | AC-005 S3 | T-08 | FN-34 | shell returns Ok (sink Unavailable; expects StoreUnavailable) |
| `job_store::the_job_audit_row_excludes_free_text_stored_on_the_job` | unit-own (TS-17 forward) | AC-005 S2 | T-09 | FN-34 | canary never reaches the job row |
| `job_store::claim_next_claims_the_oldest_queued_job` | unit-own | AC-002 S1 | T-07 | FN-16 | returns None |
| `job_store::claim_next_never_reclaims_a_running_or_terminal_job` | unit-own | AC-002 S1 | T-12 | FN-16 | returns None |
| `job_store::exactly_one_of_n_concurrent_claims_wins` | unit-own | AC-002 S1 | T-07, T-12 | FN-16 | no winner |
| `job_store::concurrent_claims_never_claim_a_job_twice` | unit-own | AC-002 S1 | T-07, T-12 | FN-16 | no claim wins |
| `job_store::a_claim_blocked_on_a_row_taken_by_another_transaction_does_not_claim_it_again` | unit-own | AC-002 S1 | T-12 | FN-16 | fresh job not claimed (only the positive half is red; the first half is structural) |
| `job_store::the_claim_hands_its_own_uncommitted_transaction_to_the_audit_sink` | TS-69 | AC-005 S2 | T-08 | FN-34 | returns None |
| `job_store::an_audit_failure_refuses_the_claim` | TS-69 | AC-005 S2 | T-08 | FN-34 | shell returns Ok(None) |
| `job_store::an_audit_row_written_before_the_sink_fails_rolls_back_with_the_claim` | TS-69 | AC-005 S2 | T-08 | FN-34 | shell returns Ok(None) |
| `audit_sink::a_row_recorded_on_the_callers_transaction_is_visible_only_after_commit` | TS-69 | AC-005 S2 | T-08 | FN-34 | no row in the tx |
| `audit_sink::rolling_back_the_callers_transaction_leaves_no_audit_row` | TS-69 | AC-005 S2 | T-08 | FN-34 | no row before rollback |
| `audit_sink::recording_without_a_transaction_commits_immediately` | unit-own | AC-005 S2 | T-08 | FN-34 | no row |
| `audit_sink::every_column_of_the_event_is_persisted` | unit-own (TS-44b forward) | AC-005 S2 | T-09 | FN-34 | no row |
| `audit_sink::every_event_kind_is_stored_with_its_closed_set_name` | unit-own | AC-005 S2 | T-09 | FN-34 | no rows |
| `audit_sink::recording_is_plain_insert_and_never_touches_existing_rows` | unit-own | AC-005 S2 | T-08 | FN-34 | no rows (cannot tell `ON CONFLICT DO NOTHING` from plain INSERT) |
| `audit_sink::an_audit_write_failure_is_reported_not_swallowed` | unit-own | AC-005 S2 | T-08 | FN-34 | shell returns Ok (expects Rejected: schema rejection on a healthy connection) |
| `audit_sink::connection_level_and_unclassifiable_failures_are_an_outage` (conductor, no Docker) | unit-own | AC-005 S3 | T-08 | FN-34 | classifier returns Rejected |
| `audit_sink::event_kinds_map_to_the_closed_event_set` (no Docker) | unit-own | AC-005 S2 | T-09 | FN-34 | `as_str` is empty |
| `audit_sink::a_digest_is_only_a_sha256_hex_string` (no Docker) | unit-own | AC-005 S2 | T-09 | FN-34 | shell accepts a raw value |

Totals (Observed):
- **No-Docker run:** 37 passed (all pre-existing), 4 failed (all new), 75 ignored.
- **Docker run:** 34 passed (all pre-existing), 41 failed (all new).

That makes 45 new tests, all red, and none passing.
