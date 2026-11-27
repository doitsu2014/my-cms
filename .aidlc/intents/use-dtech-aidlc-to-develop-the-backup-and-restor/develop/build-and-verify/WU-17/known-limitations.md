# KL-17 Known limitations: WU-17

- **ID:** KL-17 (WU-17)
- **Stage:** Develop / build-and-verify, unit WU-17.

1. **The reconciler unlinks nothing.** The audited claim, the unlink and the audited `deleted` commit belong to the cleaner (WU-18). The full TS-20, TS-21 and TS-22 audit sequence (`delete_claimed` before the unlink, `deleted` after it) is therefore not verified yet. WU-18 must extend those scenarios.
2. **Not wired.** No runtime calls `advance_epoch`, `reconcile` or `discover_orphans` yet (WU-19). "Admissions stay closed on failure" is shown as the returned error only.
3. **A stale `deleting` row always goes through `release_pending`.** IFC 6.1a lists "older epoch, file absent: `deleted`". The reconciler returns the row to `release_pending` in both cases and leaves the audited unlink and `deleted` commit to the cleaner. The end state is the same; the audit trail has one extra `delete_claimed`.
4. **Two audit field choices the design does not fix.** `reconcile.fenced` is written with outcome `success`; `ownership.ambiguous` with outcome `failure`, no job id, the allocation id only when the name is a canonical key, and a fresh UUID as correlation id otherwise. The fence row carries operation `reconciliation`. The owner should confirm both.
5. **Incident noise.** Every full reconcile re-raises `ownership.ambiguous` for each ambiguous entry, including the file a `cleanup_failed` allocation leaves behind. The design asks for an incident, not for deduplication; the sweep scan does not raise incidents.
6. **A running owner of any epoch keeps its allocation.** If the fence of an old-epoch job is lost to a concurrent writer, the allocation is skipped until the next run. No allocation of a job that may still be running is released.
7. **Zero-byte orphan.** An empty unowned file is persisted with `bytes_reserved` 1, because the column requires a positive value. No test covers an empty file.
8. **Equivalent mutant.** The "owner is incomplete" check in the missing-file branch has no killing test because the other owner states are unreachable in U1.
9. **`list_deadline_expired` is still inert.** It belongs to WU-19.
10. **Platform.** Verified on Windows only.
