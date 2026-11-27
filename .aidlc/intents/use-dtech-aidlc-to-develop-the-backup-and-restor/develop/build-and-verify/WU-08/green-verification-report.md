# WU-08 green verification report

- **Status:** Partial; WU-08 is not complete. Workspace regressions pass, but required WU-08 behavior and production composition remain incomplete.
- **Environment:** Windows 11; cargo/rustc 1.100.0; Docker 29.7.2.
- **Baseline:** Reused unchanged from `develop/baseline.md` (2026-11-22).

## Final checks observed on the current source tree (2026-11-27)

| Check | Result | Evidence |
|---|---|---|
| `cargo test -p domain_backup --no-fail-fast` | 142 passed, 0 failed, 202 ignored | package run observed; included in full workspace normal suite |
| `cargo test -p domain_backup --no-fail-fast -- --ignored` | 202 passed, 0 failed | package run observed; included in full workspace ignored suite |
| `cargo test -p domain_backup infra::tests::http_api:: -- --ignored --nocapture` | 3 passed, 0 failed | final focused HTTP run observed |
| `cargo test -p domain_backup handlers::job_response -- --nocapture` | 3 passed, 0 failed | `evidence/green-wu08-projection.log` |
| `cargo test -p domain_backup domain::metrics:: -- --nocapture` | 2 passed, 0 failed | focused unit run observed; also included in package/workspace suites |
| Projection mutation check | 4/4 mutants killed by focused tests | `evidence/green-wu08-mutations.json` |
| `cargo test --workspace --no-fail-fast` | 399 passed, 0 failed, 207 ignored | `evidence/regression-wu08-final-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored` | 207 passed, 0 failed; domain_backup 202 passed | `evidence/regression-wu08-final-ignored.log` |
| `cargo fmt --all -- --check` | pass | `evidence/green-wu08-fmt.log` |
| `cargo clippy -p domain_backup --all-targets --message-format=short` | exit 0 | `evidence/green-wu08-clippy.log` |
| `cargo check --workspace` | exit 0; sibling-crate warnings remain | `evidence/green-wu08-check-workspace.log` |
| `dtech-aidlc --project D:/orca/my-cms/makara ids check --stage build-and-verify` | `ok: true`; no unregistered IDs | `evidence/green-wu08-ids.log` |

## Verified contract coverage

- **TS-45 / FN-02:** configured single local instance and capability projection.
- **TS-59 / FN-03:** administrator role, loopback Host and exact configured POST Origin policy at the handler adapter boundary; gateway middleware order is WU-10.
- **TS-05b / FN-04:** claim-time configured-instance policy.
- **TS-04 / FN-09:** foreign/unknown job IDs are equivalent 404s; list is instance-scoped with bounded pagination.
- **TS-12b, TS-49 / FN-10:** forbidden U1 statuses refused, download ineligible, internal fields omitted, progress only when measured.
- **TS-06 / FN-05:** strict create payload rejects unknown/unsafe fields and disallowed instance without durable side effects; accepted create and replay persist one job, idempotency row and audit event. A sequential second-key conflict is checked against an active job; no simultaneous race is claimed.
- **TS-27b / FN-36:** IF-11 projects durable cleanup counts and escalation visibility.
- **TS-57 / FN-36:** route operations update enum-backed in-memory counters; status exposes exact attribute keys (`job_kind`, `outcome`, `error_code`) and values only from closed enums. Gateway telemetry exporter integration is not included.
- **TS-60, TS-62, TS-63:** declared and streamed over-limit bodies, missing/wrong JSON content type, malformed/unknown-field create bodies, invalid UUID, and bad/unknown list query inputs are exercised; not every extractor transport edge is covered.
- Raw idempotency key canary is asserted absent from responses and persisted audit/idempotency/job/allocation/epoch records in the create route test.

## Limits and unverified scope

WU-08 is not complete. TS-17 full API log/metrics canary scanning and gateway OTLP/exporter integration remain unverified. Reconciliation timestamps/results are simplified runtime projection fields. `DomainBackupService::register_routes` has no ready runtime; production read/create stay `BACKUP_NOT_READY` until WU-10 startup composition. Gateway auth ordering, effective bind, CFG-17/18 ingress, CORS/cookies and composed-router security are WU-10. See [implementation.md](implementation.md) and [known-limitations.md](known-limitations.md); no unimplemented criterion is claimed as passing.
