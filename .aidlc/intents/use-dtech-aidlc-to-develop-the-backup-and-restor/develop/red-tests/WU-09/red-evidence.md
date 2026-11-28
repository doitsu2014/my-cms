# WU-09 red evidence

- **Environment:** macOS Darwin 25.6.0, cargo 1.100.0, Docker 29.9.3; worker worktree `.claude/worktrees/wf_6fc7eae6-d0c-1` based on `36361ae`.
- **Red state:** the six IF-05..IF-10 routes were registered with placeholder handlers returning `501 {"stub":true}`, and the fixture catalog returned an empty list. Everything compiled; failures are assertions on status codes, bodies and catalog contents, not build or setup errors.

## Runs (from `apps/api`)

| Command | Result | Log |
|---|---|---|
| `cargo test -p domain_backup --no-fail-fast --lib -- ts09 ts10 ts43 ts53 ts60_preview ts64` | 3 passed, 13 failed | `evidence/wu09-red-default.log` |
| same filter with `-- --ignored` (Docker) | 1 passed, 4 failed | `evidence/wu09-red-ignored.log` |

## Verbatim excerpts

```
ts53_catalog_trusts_exactly_three_synthetic_fixtures_by_exact_id
assertion `left == right` failed
  left: []
 right: ["fixture-valid", "fixture-incompatible", "fixture-corrupt"]

ts09_disabled_capabilities_never_poll_the_request_body
assertion `left == right` failed: disabled /backup/packages content-length 1073741824: {"stub":true}
  left: 501
 right: 403

ts09_download_is_503_not_denied_while_backup_is_disabled_or_not_ready
assertion `left == right` failed: disabled <uuid>: {"stub":true}
  left: 501
 right: 503

ts64_request_supplied_target_values_are_rejected_not_echoed
assertion `left == right` failed: /backup/previews/preflight target_instance_id: {"stub":true}
  left: 501
 right: 400

ts09_download_unknown_foreign_and_malformed_ids_are_404_and_unaudited (Docker)
assertion `left == right` failed: <uuid>: {"stub":true}
  left: 501
 right: 404
```

## Tests already passing at red (guards, not red evidence)

- `ts09_download_role_and_host_checks_precede_everything` and `ts09_role_host_and_origin_checks_precede_capability_disabled`: the WU-08 authorization middleware already rejects role/Host/Origin before any handler runs.
- `ts10_runtime_composition_holds_only_the_fixture_adapter` and `ts10_client_flags_cannot_select_a_real_adapter`: no real adapter exists to select (TS-10 guards that this stays true).

These four tests guard behaviour that already held and are not counted as red. Every other owned test failed on behaviour.
