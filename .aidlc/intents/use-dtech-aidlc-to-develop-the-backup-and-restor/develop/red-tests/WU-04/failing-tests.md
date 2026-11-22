# FT-04 Failing tests (code manifest): WU-04

- **ID:** FT-04 (WU-04)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-04 of WU-01..WU-15.
- **Status:** red run recorded 2026-11-22: PostgreSQL 15 via Docker, Windows host. A Linux container confirmed the Unix-only tests.
- **Inputs:**
  - [work-units](../../implementation-design/work-units.md) WU-04
  - [ids.json](../../implementation-design/ids.json)
  - [test-strategy](../../implementation-design/test-strategy.md) TS-19, TS-23 (classification), TS-30 (budget), TS-31, TS-32 (lock), TS-56
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) sections 6.1, 6.2, 8 and 9
  - [functional-design](../../implementation-design/functional-design.md) FN-21..FN-24 (legacy F-21..F-24) and section 4.4
- **Evidence:** [red-evidence.md](red-evidence.md); raw log [../evidence/red-wu04-cargo-test.log](../evidence/red-wu04-cargo-test.log).

Provenance:
- **Observed:** results from the conductor's run on 2026-11-22.
- **Identifiers:** registry and Analyze only; `dtech-aidlc ids check` reports none unregistered.
- **Mappings:** AC is AC-003 S3 unless stated. T and FN are as the test-strategy and the fixer's table assign them.

## Files

Production shells and tests live under `apps/api/domain_backup/src/`.

| Path | Change | Shell behaviour |
|---|---|---|
| `ports/allocation_store.rs` | new | `AllocationStore` trait with `register`, `get`, `mark_written`, `release`, `claim_for_delete`, `mark_deleted`, `record_cleanup_failure`, `insert_orphan` and `due_for_cleanup`. Also `Allocation`, `AllocationState`, `AllocationLimits`, `AllocationError`. |
| `ports/fixture_storage.rs` | new | `FixtureStorage` trait (`write`, `delete`, `inventory(max_entries)`, classification), plus `StorageKey` and `Ownership`. |
| `ports/namespace_lock.rs` | new | `NamespaceLock` trait and its guard. |
| `infra/db_allocation_store.rs` | new | Logic-free stub: reports success, writes no rows, stamps wall-clock time. |
| `infra/fs_fixture_storage.rs` | new | Logic-free stub: writes nothing; `inventory` returns empty with `truncated = false`. |
| `infra/fs_namespace_lock.rs` | new | Logic-free stub: the lock always succeeds. |
| `infra/tests/{allocation_store,fs_fixture_storage,fs_namespace_lock,alloc_support,fs_support}.rs` | new | Tests and helpers. `fs_support` provides a link-safe temp-dir guard, a junction/symlink maker, and `FakeLedger`/`DownLedger`. |
| `ports/mod.rs`, `infra/mod.rs`, `infra/tests/mod.rs` | modified | Module wiring only. |

No `Cargo.toml`/`Cargo.lock` change, and no WU-01..WU-03 code or test change. That includes `support.rs`, which was not touched.

## Tests

- **Commands:** from `apps/api`, `cargo test -p domain_backup --no-fail-fast`, then the same with `-- --ignored`.
- **Docker:** `allocation_store::` tests carry `#[ignore = "requires Docker for PostgreSQL testcontainer"]`. The filesystem and lock tests run without Docker.
- **Prefix:** all tests are under `infra::tests::`.

### `fs_fixture_storage` (no Docker): 26 red, 3 not red on this host

| Test | TS | T | FN | Red reason |
|---|---|---|---|---|
| `storage_key_is_derived_from_the_allocation_id_only` | TS-19 | T-11 | FN-22 | shell key not derived |
| `storage_key_parse_refuses_every_non_opaque_name` | TS-19 | T-11 | FN-22 | shell accepts |
| `open_creates_root_and_alloc_directories` | unit-own | T-11 | FN-22 | nothing created |
| `open_refuses_a_relative_root`, `open_refuses_a_root_that_is_a_file`, `open_refuses_a_root_that_is_a_link`, `open_refuses_an_alloc_directory_that_is_a_link` | TS-19 | T-11 | FN-22 | shell accepts |
| `write_without_a_registered_allocation_is_refused_and_writes_nothing` | unit-own (register-before-write) | T-11, T-12 | FN-21, FN-22 | shell writes "ok" |
| `write_is_refused_for_every_state_except_registered` | unit-own | T-12 | FN-21, FN-22 | shell accepts |
| `write_creates_exactly_the_opaque_key_file` | unit-own | T-11 | FN-22 | no file |
| `write_beyond_the_reserved_bytes_is_refused_without_a_partial_file` | unit-own | T-13 | FN-22 | shell accepts |
| `write_never_overwrites_an_existing_file` | unit-own | T-11 | FN-22 | shell accepts |
| `write_fails_closed_when_the_ledger_is_unavailable` | unit-own | T-12 | FN-21 | shell accepts |
| `write_never_goes_through_a_link_at_the_key_path` | TS-19 | T-11 | FN-22 | shell accepts (weak test: any create-new refuses) |
| `write_refuses_when_alloc_is_swapped_for_a_link_after_open` | TS-19 | T-11 | FN-22 | shell accepts |
| `delete_removes_a_regular_file_and_a_missing_file_is_already_absent` | unit-own | T-11 | FN-22 | nothing removed |
| `delete_leaves_every_other_entry_on_disk` | TS-19 | T-11 | FN-22 | target not removed |
| `delete_never_removes_a_case_folded_forged_file` | TS-19 | T-11 | FN-22 | target not removed |
| `delete_refuses_a_directory_at_the_key_path` | TS-19, TS-23 | T-11 | FN-22 | shell accepts |
| `delete_never_follows_or_removes_a_link` | TS-19 | T-11 | FN-22 | shell accepts |
| `inventory_never_exceeds_max_entries_and_flags_truncation`, `inventory_bound_holds_for_a_large_namespace` | TS-56 | T-13 | FN-22 | empty, `truncated` false |
| `inventory_reports_kinds_and_never_follows_links` | TS-19 | T-11 | FN-22 | empty |
| `classify_forged_and_non_opaque_names_as_ambiguous`, `classify_non_regular_entries_as_ambiguous_even_with_a_valid_name`, `classify_valid_opaque_name_follows_the_ledger_table` | TS-19, TS-23 | T-11, T-12 | FN-22 | wrong classification |
| `write_never_goes_through_a_file_symlink_to_an_existing_file`, `write_never_goes_through_a_dangling_symlink`, `delete_never_follows_or_removes_a_file_symlink` | TS-19 | T-11 | FN-22 | **not red on Windows.** They skip: creating a file symlink needs a privilege this host lacks (os error 1314). Red on Linux. |
| `open_creates_private_directories_on_unix`, `open_refuses_a_group_or_world_accessible_root_on_unix`, `open_refuses_a_group_or_world_accessible_alloc_directory_on_unix`, `written_file_is_private_on_unix` | TS-31 | T-14 | FN-22 | `cfg(unix)`: not compiled on Windows. Red on Linux (builder-reported container run). |

### `fs_namespace_lock` (no Docker): 9 red, 2 not red

| Test | TS | T | FN | Red reason |
|---|---|---|---|---|
| `lock_path_is_a_sibling_of_the_root`, `lock_path_appends_to_the_whole_root_name`, `lock_path_for_a_root_with_a_trailing_separator_is_still_a_sibling` | TS-32 | T-11 | FN-24 | wrong path |
| `acquire_creates_the_sibling_lock_file_outside_the_namespace` | TS-32 | T-11, T-12 | FN-24 | no file |
| `second_holder_is_refused_and_drop_releases_the_lock` | TS-32 | T-12 | FN-24 | shell always grants |
| `lock_held_by_parent_refuses_a_second_process`, `lock_held_by_child_process_refuses_the_parent_until_it_exits` | TS-32 (two real processes) | T-12 | FN-24 | shell always grants |
| `a_relative_root_is_refused_with_invalid_root`, `a_directory_link_at_the_lock_path_is_refused_and_not_followed` | TS-32 | T-11 | FN-24 | shell accepts |
| `a_file_symlink_at_the_lock_path_is_refused_and_not_followed` | TS-32 | T-11 | FN-24 | **not red on Windows** (skips, os error 1314). Red on Linux. |
| `lock_child_probe` | n/a | n/a | n/a | inert helper that the two-process tests spawn; not a test |

### `allocation_store` (Docker): 27 red

| Test | TS | T | FN |
|---|---|---|---|
| `register_commits_a_registered_row_with_the_opaque_key`, `get_returns_the_committed_row`, `register_accepts_a_zero_byte_reservation`, `rows_are_stamped_from_the_injected_clock` | unit-own | T-12, T-13 | FN-21 |
| `register_writes_allocation_registered_audit_in_the_same_transaction`, `register_is_refused_and_rolled_back_when_audit_fails` | unit-own (IFC 6.3 same-transaction rule) | T-08 | FN-21 |
| `register_enforces_the_256_mib_per_allocation_budget`, `register_enforces_the_1_gib_total_budget_durably`, `total_budget_counts_every_state_except_deleted`, `concurrent_registrations_never_exceed_the_total_budget`, `forced_interleaving_never_exceeds_the_total_budget` | TS-30 (budget) | T-13 | FN-21 |
| `register_fails_closed_when_the_ledger_is_unreachable` | unit-own | T-12 | FN-21 |
| `the_ledger_row_exists_before_the_first_byte_is_written` | unit-own (register-before-write) | T-11, T-12 | FN-21, FN-22 |
| `mark_written_is_a_compare_and_set`, `release_moves_registered_and_written_to_release_pending`, `racing_mark_written_and_release_have_one_winner` | unit-own | T-12 | FN-23 |
| `claim_for_delete_audits_in_the_same_transaction_and_is_conditional`, `claim_for_delete_stamps_the_current_epoch`, `claim_for_delete_waits_for_the_backoff_to_pass`, `claim_and_delete_commit_are_refused_when_audit_fails`, `mark_deleted_commits_with_audit_and_has_no_shortcut` | unit-own | T-08, T-12 | FN-21 |
| `cleanup_failures_retry_with_backoff_then_exhaust_after_three`, `record_cleanup_failure_only_applies_to_a_deleting_row` | TS-30 (cleanup budget) | T-12, T-13 | FN-21 |
| `insert_orphan_persists_the_discovered_file_with_grace`, `insert_orphan_twice_keeps_the_first_grace`, `insert_orphan_audits_in_the_same_transaction_and_fails_closed`, `due_for_cleanup_returns_exactly_the_due_rows` | unit-own (TS-23 row shape) | T-12, T-13 | FN-23 |

**Totals (Observed on Windows):**
- **No-Docker run:** 46 passed (41 earlier, 4 symlink tests that skip on this host, 1 inert probe) and 35 failed (all new).
- **Docker run:** 77 passed (all earlier) and 27 failed (all new).
- **Red on Windows:** 62 new tests. Another 4 Unix-only and 4 file-symlink tests are red on Linux only.
