# WU-06 known limitations

- **Unit:** WU-06 — Allocation reconciliation and retention store contracts
- **Status:** Store/adapter behavior is implemented. WU-06 focused database tests passed in runs when Docker/PostgreSQL was available; later replays became environmentally blocked.

## Out of scope / deferred

- Runtime startup/recovery, epoch advancement and job fencing, deadline selection, inventory orchestration, cleanup/retries, missing-file full-scan audit, truncated-inventory fail-closed handling, filesystem reconciliation, and sweeper integration remain WU-16 responsibilities. No files under `apps/api/domain_backup/src/lifecycle/**` are listed as WU-06 implementation changes.
- This WU covers only store/adapter portions: TS-42 purge adapter, TS-44b allocation CAS audit persistence, TS-56 allocation LEFT JOIN inventory, and TS-65 retention rollback. It does not claim all lifecycle portions of those TS cases.
- The schema FK rejects a non-NULL `backup_allocations.job_id` referencing a missing job; TS-56 proves NULL-owner orphan retention and live owner joins, not a referentially-invalid non-NULL owner.

## Verification caveats

- The declared Docker prerequisite passed initially, and focused PostgreSQL 15 runs passed. Later Docker Desktop became unable to start; `docker info` exited 1 and subsequent testcontainer launches timed out before reaching assertions. Successful focused output excerpts are under `evidence/observed-green-wu06-*.log`; the later environment failures are recorded separately in `evidence/green-wu06-*.log`.
- Full normal tests still report 13 WU-16-owned lifecycle failures. The explicit ignored workspace run reports 3 WU-16-owned JobStore recovery/deadline failures. Identities and baseline comparison are in `regression-report.md`; no WU-06-owned test failed in the recorded full runs.
- The full workspace is not green, so this is not a verified release candidate. No baseline-passing test identity was among the current failures.
- Unix-only permission modes and Windows owner-only ACL verification are not exercised by this WU; the separate accepted synthetic-only T-14 residual is recorded in earlier work-unit artifacts.
