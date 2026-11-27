# KL-18 Known limitations: WU-18

- **ID:** KL-18 (WU-18)
- **Stage:** Develop / build-and-verify, unit WU-18.

1. **Reason code for an unlink failure.** The closed set has no storage-specific code, so a failed unlink is recorded as `ADAPTER_ERROR` in `last_reason_code` and on the `cleanup.exhausted` row. The owner should confirm or add a code.
2. **No `AuditSink` and no `Clock` in the cleaner.** The design text lists both. The allocation store already writes each cleaner audit row inside the state-change transaction, and `now` is a parameter. A sink in the cleaner would only allow a second, non-atomic write.
3. **`clean_due` stops at the first error.** Rows cleaned earlier in the batch are durable but not reported to the caller.
4. **The store's clock decides due-ness at the claim.** `claim_for_delete` compares `next_attempt_at` and the grace with the store's own clock, while the cleaner's early `NotDue` uses the `now` parameter. In production both are the same clock; the tests share one settable clock. A caller that passes a different `now` could see `Lost` where it expected a claim.
5. **A stuck `deleting` row waits for a restart.** If the `deleted` commit fails, the row stays `deleting` of the current epoch; `due_for_cleanup` does not list it, and only the next start-up reconciliation (WU-17) resets it. Until then the file is already gone and the row is visible as `deleting`.
6. **`cleanup_failed` is not retried or cleared.** By design (IFC 6.1a). The file stays on disk and the escalation stays visible; clearing is a manual database action.
7. **Lost claim.** No test forces a lost claim after a successful read; only the read-side `Lost` (unclaimable state, unknown id) and a lost `deleted` commit are tested.
8. **Removed shell tests.** `ts30` (clock arithmetic) was removed and `ts65` reduced to its purge half; WU-19 owns the replacement for the sweeper-side TS-30 boundaries and the purge refusal.
9. **Platform.** Verified on Windows only.
