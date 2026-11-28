# KL-07 Known limitations: WU-07

- **ID:** KL-07 (WU-07)
- **Stage:** Develop / build-and-verify, unit WU-07.

1. **No HTTP surface.** `parse_create_request` and `CreateJobHandler::create` take plain values. Header extraction, the Host/Origin and role checks, the `Idempotent-Replay: true` response header and the JSON error bodies belong to WU-08 and WU-10. `CreateJobError::status()` and `reason_code()` record the mapping of IFC section 2.5 but are not exercised through HTTP.
2. **Replay skips the instance-ownership check (FN-04).** A replay returns the stored job. WU-08 must run its authorization on the replayed job as the contract states ("existing job after F-04").
3. **Same-transaction audit is a [D] mechanism.** The design marks it as awaiting plan-gate confirmation (ADR-005). The tests prove the audit row is written on the acceptance transaction and rolls back with it.
4. **One re-read.** After a unique violation the handler re-reads the idempotency row once. The tests cover same-key races (same and different body) and distinct-key races; a violation with no idempotency row visible is answered `CONCURRENCY_LIMIT` as the contract specifies. No test forces a third writer to commit between the rollback and the re-read.
5. **Epoch is a constructor argument.** The handler does not read the durable epoch itself; the runtime must pass the current one (WU-19).
6. **Platform.** Verified on Windows only.
7. **Cargo.lock.** One added line (`sha2 0.10.9` under `domain_backup`); no new crate version enters the lockfile.
