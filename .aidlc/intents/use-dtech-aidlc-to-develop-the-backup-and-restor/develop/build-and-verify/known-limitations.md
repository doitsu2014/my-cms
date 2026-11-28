# Release-candidate known limitations and residuals

Per-unit details live in each `WU-nn/known-limitations.md`. Consolidated:

1. **Not authorized in U1:** real data staging, upload or preflight acceptance, restore execution (AC-004 S3 deferred), download, production use, `succeeded` / `recovery-required` outcomes.
2. **RK-16 / release:** U1 stays off `main`; migrations register unconditionally (see implementation.md). Release is a separate owner decision.
3. **RK-15 residual (owner-descoped 2026-11-28):** gateway boot with posts, seo and backup migrators on one database (TS-34 "upgrade beside posts and seo", TS-41 runtime part) not exercised; `domain_seo` / `domain_posts` untouched.
4. **E2E-ENV-01:** browser verification blocked (TS-41 UI part); UI covered by component tests only.
5. **T-14 / TS-31:** owner-only permission check partial on Windows (owner-accepted synthetic-only residual); Unix checks ran on macOS.
6. **TS-37:** manual assistive-technology review not done; automated accessibility checks only.
7. **Open owner questions:** Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention, non-owner technical guards (16 KiB body, list 20/100, fixture size, 60 s probe), P-01..P-05 performance targets (TS-39/TS-40 numbers recorded without targets).
8. **CI (RK-09):** CI runs `cargo test` without `--ignored` and no web tests; Docker-backed backup tests do not run in CI; no green CI is claimed.
9. **Test harness:** Docker-backed suite needs `--test-threads=4` on this host; at full parallelism container start-up intermittently fails. Follow-up: retry or limit concurrency in `domain_backup/src/test_db.rs`.
10. **Tooling pre-existing:** `pnpm lint` cannot load its ESLint config; 23 `tsc` errors in untouched web files; clippy warnings in untouched crates.
11. **Runtime:** IF-11 `reconciliation.last_completed_at` always null; metrics in-memory only (no exporter); worker driver polls every 2 s; a failed runtime start is not retried until restart; slow header delivery handled by hyper (T-13 residual).
12. **Pending confirmations:** two owner decisions recorded by the CLI as "AI Orchestrator" (WU-17 scheduling 2026-11-26, WU-08 revisit 2026-11-27) await the owner's confirmation.
