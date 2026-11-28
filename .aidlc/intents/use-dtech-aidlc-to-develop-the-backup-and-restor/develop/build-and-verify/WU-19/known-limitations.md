# KL-19 Known limitations: WU-19

- **ID:** KL-19 (WU-19)
- **Stage:** Develop / build-and-verify.

1. **Sweep cadence and deadline backstop.** The periodic driver is deliberately not implemented here; `run_tick` is one scheduler iteration and the gateway timer loop/shutdown composition belongs to WU-10. The 900-second sweep interval may add up to 15 minutes to the worker's 10-minute deadline if a worker is stuck (design estimate, not a measured latency).
2. **Audit refusal bound.** A healthy-connection audit refusal is counted per failed sweep; admissions close after three consecutive refusals. A successful sweep resets the counter. Runtime recovery/probe wiring belongs to composition; tests inject port failures rather than killing PostgreSQL mid-transaction.
3. **Truncated inventory.** A truncated discover-only scan degrades runtime state but does not itself close admissions, per the WU-19 test decision. Startup and recovery reconciliation fail closed and do not open admissions on truncation.
4. **Platform coverage.** Verified on Windows 11 with Docker PostgreSQL 15 only. Unix filesystem permission behavior and other host-specific locking behavior were not run here; see WU-04/TS-31 residuals.
5. **Retention boundary.** WU-19 verifies the injected retention purge, audit retention, exclusions and idempotency, but does not assert just-before/exact-at `retain_until` boundary behavior. The WU-06 retention adapter's eligibility query is `retain_until < now`; a focused boundary test remains a follow-up.
6. **Orphan grace.** Sweeper persists discoveries but does not unlink; an `orphan_discovered` allocation past grace is claimable by the cleaner. The cleaner boundary is verified in WU-18; this unit does not introduce a separate promotion step.
7. **Second mutation evidence artifact.** The initial durable mutation record reports 3 survivors; tests were then added and the three mutants rerun successfully. The second-pass results were observed, but the temporary scratchpad containing that small rerun output is no longer available. The permanent report names the three passing killer tests and does not claim a separate second-pass JSON artifact.
8. **Out of scope.** Full gateway timer/shutdown composition (WU-10), web UI, deployment, real-data staging, restore execution, and production readiness were not implemented or authorized by WU-19.
