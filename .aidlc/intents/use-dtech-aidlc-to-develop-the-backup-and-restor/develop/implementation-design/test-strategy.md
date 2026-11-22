# Test strategy (TST-01): Backup/Restore U1

- ID: TST-01
- Status: Draft — implementation-design stage, pending conductor review (revision 4, conductor reconciliation decisions C1..C13 applied)
- Scope: U1 only (localhost, one configured local instance, synthetic fixtures, administrator-only, jobs end `incomplete` or `failed`; `succeeded` and `recovery-required` are unreachable for U1 jobs, see TS-11).
- Written from the DESIGN SPINE supplied by the conductor. Shared IDs (F-xx, IF-xx, WU-nn, TS-xx, CFG-xx, T-xx, RK-xx) are not redefined. This file's numbering is canonical for TS-01..TS-69 (conductor decision C1; TS-54 reserved and unused). Split ids (for example TS-14a/TS-14b) and TS-45 and above are additions; every surviving id appears in the case table below (with AC, T, WU), the WU mapping, work-units.md and plan section 6. Deviation ids follow the plan: D-10 status enum handling, D-11 wire format, D-12 `hooks/` directory, D-13 web client placement.

Provenance labels: **[O]** Observed (source cited or relayed from research reports), **[UP]** User-provided (project owner, 2026-11-22), **[A]** Assumption, **[E]** Estimate, **[D]** Design decision proposed for the plan gate (not an approval), **[Q]** Open question.

No test described here has been written or run. No baseline exists. Nothing below claims a pass, a coverage figure or a green CI.

## Source inputs (relative links)

- [acceptance-criteria](../../analyze/problem-framing/acceptance-criteria.md) (re-read in revision 2; scenario names below are the file's own)
- [user-stories](../../analyze/problem-framing/user-stories.md)
- [quality-requirements](../../analyze/context-and-risk/quality-requirements.md)
- [risk-register](../../analyze/context-and-risk/risk-register.md)
- [open-questions](../../analyze/context-and-risk/open-questions.md)
- [decision-record](../../ideate/direction-selection/decision-record.md)
- [architecture-design](../../ideate/direction-selection/architecture-design.md)
- [threat-model](../../ideate/direction-selection/threat-model.md)
- [scope-boundary](../../ideate/direction-selection/scope-boundary.md)
- Repository context: `docs/features/backup-restore.md`, `docs/development/agent-team.md`, `README.md`, `.aidlc/knowledge/_codebase-map.md`.
- Methodology template: `test-strategy.md` (Levels, Criterion to level, Commands, Environments and data, Unverified).

## Owner decisions this strategy depends on [UP]

- Approval attribution (C13) [UP]: in solo scope the project owner is the sole approver and also acts as architect, operations and security approver for this intent; ADR and threat-model approvals below are recorded as "project owner (solo scope; owner also acts as architect/operations/security approver for this intent), 2026-11-22". No separate architect or security person approved anything.
- ADR-002, ADR-003, ADR-004 approved as proposed (ADR-002: add terminal `incomplete`, retaining existing values, so the status enum is NOT narrowed, see D-10 and TS-11); Q-05 finite bounds approved as exact values (CFG-05..CFG-14; CFG-15 escalation owner `project_owner`; defaults checked by TS-01; any CFG-05..CFG-14 value other than the approved default makes backup disabled with `disabled_reason` `INVALID_CONFIG`, checked by TS-02; DB CHECKs are safety ceilings only; changing a bound needs a new owner decision); administrator-only (`my-headless-cms-administrator`, writers excluded), admin-UI-first, one local instance; threat model T-01..T-14 accepted as mandatory construction requirements, each traced to a test below; final security evidence check at the release-candidate gate. T-15..T-17 deferred. Escalation to the project owner is a visible admin-UI status (tested by TS-50).
- Still open and not represented as decided: Q-04, Q-06, Q-07, Q-10, Q-11, Q-12 (test baseline), P-01..P-05 performance targets, audit retention, the non-owner technical guards (16 KiB body cap, list page size 20/100, fixed fixture size), CFG-04 fixture-storage location and protection approval (blocker for WU-04..WU-06), migration gating and release effect (RK-16: WU-02 and WU-10 hard stop, no merge to main until decided), the RK-15 runtime-start condition (WU-10), and owner values for CFG-17 and CFG-18 (ingress request timeout and header-size limit, no approved default, needed before WU-10, which builds the ingress layers and owns TS-55). Real-data staging, upload/preflight acceptance, restore execution, downloads and production use are NOT authorized.

## Levels

### Unit
- Applies: yes
- Why: state machine, config, policy, idempotency digest/fingerprint, path-safety and CLI fail-closed logic are pure and testable without Docker; web components and hooks need jsdom tests.
- Framework: Rust built-in `#[test]` and `#[tokio::test]` (inline `#[cfg(test)] mod tests` per file [O convention]); Vitest 4 with Testing Library and jsdom for web [O `apps/web/vitest.config.ts`].
- Location (exact paths, following existing conventions):
  - `apps/api/domain_backup/src/domain/{config,status,state_machine,idempotency,policy,instance,response,clock}.rs` (inline tests)
  - `apps/api/gateway/src/backup_cli.rs` (inline tests for `handle_args`), `apps/api/gateway/src/main.rs` (loopback guard, manifest count)
  - `apps/web/src/domains/backup.ts` (REST client and zod schemas, following the observed `apps/web/src/domains/*.ts` precedent; `infrastructure/das` is not used, D-13) with tests at `apps/web/src/domains/backup.test.ts`, `apps/web/src/app/admin/backup/page.test.tsx`, `apps/web/src/app/admin/backup/components/*.test.tsx`, `apps/web/src/app/admin/backup/hooks/*.test.ts`, `apps/web/src/app/admin/components/left-menu.test.tsx` (extended)
- Command: from `apps/api`: `cargo test -p domain_backup` and `cargo test -p gateway`; from `apps/web`: `pnpm test` (runs `vitest run` [O package.json]). Single file: `pnpm exec vitest run <path>`; single Rust test: `cargo test -p domain_backup <name>` [A standard cargo].
- Proves: no transition, write or projection can produce `succeeded` or `recovery-required` for a U1 job; owner defaults and per-key bounds validation (`INVALID_CONFIG`); key/digest correctness; UI renders status safely with no download control.
- Entry: WU red tests written and failing for the right reason; crate compiles (after WU-01). Exit: all unit tests of the WU green, no previously passing test regressed against baseline.

### Integration
- Applies: yes
- Why: CAS transitions, partial unique index, triggers, ledger budgets, atomic acceptance plus audit, crash windows and retention are DB behaviour that mocks cannot prove.
- Framework: `#[tokio::test]` with a crate-local fixture [D, aligned with implementation-plan WU-02]: `#[cfg(test)] async fn backup_test_db() -> DatabaseConnection` starts its own `testcontainers_modules::postgres::Postgres` in-crate as `domain_seo` does [O `apps/api/domain_seo/src/integration_tests.rs:15-31`, `domain_seo/Cargo.toml` dev-dependencies] and applies only `domain_backup::migrations::Migrator::up` (table `seaql_migrations_backup`). `test_helpers` is NOT used and NOT changed: it depends on `domain_posts`, runs posts' `Migrator::refresh` and is a dev-dependency of domain_media, domain_posts, domain_user and gateway [O `apps/api/test_helpers/Cargo.toml`, `src/lib.rs:3,68`], so a `domain_backup` dependency there would create a dev-dependency cycle and widen every consumer's build. Temp directories via `tempfile`; `FakeClock` for time.
- Docker gating mechanism [D]: cargo has no automatic Docker skip. Follow the `domain_seo` precedent [O `apps/api/domain_seo/src/integration_tests.rs:16`]: every DB-backed `domain_backup` test carries `#[ignore = "requires Docker for PostgreSQL testcontainer"]` and is run explicitly with `-- --ignored` when Docker is present. Without Docker those tests are reported "not run (Docker unavailable)", never as passed. Pure-logic and fake-port tests are not ignored and always run. Note the repository has a second convention: `domain_posts` handler tests call `setup_test_space()` unignored and therefore fail without Docker, which is a likely cause of the relayed "domain_posts failures" [A, validate at baseline]. `domain_backup` must not follow that second convention.
- Location: inline `#[cfg(test)]` modules in `apps/api/domain_backup/src/{domain,infra,lifecycle,handlers,migrations}/**` (TS-08a is in `domain/config.rs`); composed-router tests in `apps/api/gateway/src/main.rs` using the `compose_routers_*` stub style and `ScopedEnvVar` [O].
- Command: from `apps/api`: `cargo test -p domain_backup` (logic tests), `cargo test -p domain_backup -- --ignored` (DB-backed, needs Docker), and `cargo test -p gateway`.
- Proves: durable idempotency, concurrency limit 1, audit-before-effect, terminal immutability, all ten crash windows, bounds, startup order, fresh and upgrade migrations of the backup Migrator; the "beside posts and seo" case is conditional on RK-15 (see WU-02).
- Entry: Docker reachable (see Environments); the crate-local `backup_test_db()` fixture exists (WU-02, RK-04). Exit: all TS integration cases of the WU green (DB-backed ones run with `--ignored`).

### End to end
- Applies: yes (evidence only, not an automated gate)
- Why: the UI part is BLOCKED pending an environment decision [Q]. It confirms the admin flow across API and DB: create job, ends `incomplete`, direct API download returns 403 `DOWNLOAD_DENIED`, and the UI exposes no download control (TS-41).
- Blocking issue [Q, E2E-ENV-01]: locally the UI runs on `localhost:3002` and calls the API cross-origin at `localhost:8989` [O `apps/web/rsbuild.config.ts` server.port 3002 without proxy; `apps/web/public/config.js` `restApiUrl`]. The gateway composes no CORS layer [O `apps/api/gateway/src/main.rs`; `cors_layer` exists only at `domain_posts/src/domain/layers.rs:14` and is not applied], and TS-07 forbids adding a CORS exception. Browser requests carrying `Authorization`, JSON `Content-Type` or `Idempotency-Key` are preflighted and would be blocked. In the docker-swarm stack the allowed headers are `Authorization,Content-Type,apikey` (no `Idempotency-Key`) [O `deployments/docker-swarm/apps/docker-compose.yaml:145-149`] and `HOST=0.0.0.0` [O `deployments/docker-swarm/apps/.env.example:39`] would trip the loopback guard and disable backup. Decision needed from the conductor or owner among: (a) same-origin dev proxy (rsbuild `server.proxy` for `/api`), (b) a loopback-only CORS allowance limited to backup routes, (c) API-level E2E only with the UI covered by component tests (TS-12a, TS-18, TS-35, TS-37, TS-46, TS-50). Until decided, only the API-level part of TS-41 is specified; the UI part is blocked and not runnable.
- Framework: scripted HTTP client flow against a locally run API; no CI claim.
- Location: procedure and captured evidence under the WU-15 evidence record; any script lives outside application source until the conductor approves its location [Q].
- Command: API `cargo run -p gateway --bin my-cms-api` (from `apps/api`), then the scripted HTTP flow of TS-41 (script location is an open item, owner: conductor, decided before WU-15 [Q]).
- Entry criteria: all WU-01..WU-14 exit criteria met; full gateway environment present: `DATABASE_URL`, `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_SERVICE_ROLE_KEY`, media configuration [O `gateway/src/main.rs:128-142, 347-353`], plus `BACKUP_ENABLED=true`, the CFG keys and a loopback `HOST`; a Supabase instance (or equivalent HS256 test token signed with the configured secret [A]) to issue an administrator token; Docker Postgres. Exit: evidence recorded for TS-41; evidence, not an automated gate.

### Performance
- Applies: yes
- Why: measurement only; P-01..P-05 have no approved targets [Q]. Bounds (bytes, concurrency, deadline) are functional requirements and are tested functionally (TS-30, TS-40); latency is measured, not judged.
- Framework: a small Rust or script-driven load harness [A]; location decided at WU-15 [Q].
- Command: open item, owner: conductor, to be fixed at WU-15 start [Q]. Proposed shape [A]: a scripted concurrent HTTP client (for example a small Rust bin or PowerShell/Node script outside application source) run against `cargo run -p gateway --bin my-cms-api` from `apps/api`. No command is claimed as verified.
- Proves: p50/p95/p99 of job creation recorded (TS-39); temporary bytes and concurrency stay bounded under load (TS-40).
- Entry: functional suites green. Exit: numbers recorded with environment description; no pass/fail claim against P-xx.

### Security
- Applies: yes (mandatory, T-01..T-14)
- Why: owner accepted the threat model as construction requirements; release-candidate gate performs a final security evidence check.
- Framework: composed-router tests (tower `oneshot` against `compose_routers` output), side-effect spies, filesystem tests, canary scanning (including a web bundle scan), web escaping tests.
- Location: `apps/api/gateway/src/main.rs` tests (TS-03, TS-07, TS-08b, TS-55), `apps/api/domain_backup/src/handlers/**` and `infra/**` tests (TS-04..TS-06, TS-09, TS-10, TS-17, TS-19, TS-31, TS-44a, TS-44b, TS-56, TS-57, TS-59..TS-66, TS-69), `apps/web/src/app/admin/backup/**` tests (TS-12a, TS-18).
- Command: same `cargo test` and `pnpm test` commands, plus `cargo fmt -- --check`, `cargo clippy` (from `apps/api`) and `pnpm lint`, `pnpm exec tsc --noEmit` [A tsc], `pnpm build` (from `apps/web`).
- Entry: threat-to-test table complete. Exit: every T-01..T-14 test green, canary run clean (TS-17 full at WU-15), `succeeded` and `recovery-required` unreachable for U1 jobs across state machine, DB, serializers and API (the UI accepts and renders every value, download always denied).

## Commands

- Build (from `apps/api`): `cargo check`, `cargo check -p domain_backup` (confirms dependency versions in WU-01) [O README, A for per-crate form].
- Build (from `apps/web`): `pnpm build` [O].
- Test (Rust, from `apps/api`): `cargo test` (whole workspace), `cargo test -p domain_backup`, `cargo test -p domain_backup -- --ignored` (Docker), `cargo test -p gateway`.
- Test (web, from `apps/web`): `pnpm test`.
- Static: `cargo fmt -- --check`, `cargo clippy` (from `apps/api`); `pnpm lint`, `pnpm exec tsc --noEmit` (from `apps/web`; there is no `typecheck` script [O], so this is [A]).
- Coverage: `apps/api/take-coverage.sh` exists [O]. WARNING: it runs `rustup override set stable` and `cargo clean` [O `apps/api/take-coverage.sh`], which change local toolchain override and build state; it is NOT part of the baseline and must not be run casually. The 80% line target stays a measurement, not a gate (spine D-07).
- CI (RK-09): `.github/workflows/ci-my-cms.yml` is marked temporarily disabled by a comment only; its triggers (`on: pull_request` to `main`, `workflow_dispatch`) remain present [O `.github/workflows/ci-my-cms.yml:3-7`]. Actual enablement is unverified (it may be disabled in GitHub settings, which the file cannot show). A PR from this branch may run it on a nightly toolchain with coverage flags. Do not claim a green CI.

## Baseline procedure and regression comparison [D]

Owner: the red-tests stage (Q-12, RK-01). Run before the first source change, on a clean tree, and record exact output in the stage evidence.

1. Record environment: `cargo --version`, `rustc --version`, `node --version`, `git rev-parse HEAD`, `git status --short`, Docker availability (`docker version`, `docker ps`) (RK-02). Record `pnpm --version` run from inside `apps/web` as the effective version: `apps/web/package.json` pins `packageManager: pnpm@10.33.0` [O] while the conductor observed 10.30.3 on PATH (2026-11-22; cargo 1.100.0, rustc 1.100.0, node v24.18.0). pnpm 10 may switch to or download the pinned version, which is a network action; a failed switch is a possible blocker to record, not a test failure.
2. Cargo workspace baseline, from `apps/api`: `cargo test --no-fail-fast 2>&1` captured to a file in the evidence folder; also `cargo fmt -- --check` and `cargo clippy`. `--no-fail-fast` ensures every crate's result is listed. Also run `cargo test --no-fail-fast -- --ignored` only when Docker is available, to baseline the ignored tests separately.
3. Web baseline, from `apps/web`: `pnpm install --frozen-lockfile` if dependencies are missing (network may be required), then `pnpm test`, `pnpm lint`, `pnpm exec tsc --noEmit`, `pnpm build`.
4. Produce a baseline table: suite, command, total, passed, failed, ignored, names of failures. Historical expectation, unverified [O relayed]: 8 `domain_posts` failures. Ignored tests: three `#[ignore]` attributes are visible in source today (one in `domain_seo/src/integration_tests.rs:16`, two in `domain_posts/src/api/post/graphql/tests.rs:415,427`) [O, grep 2026-11-22], superseding the relayed figure of 2; the baseline run fixes the real count.
5. Pre-existing failures are recorded by test name and left untouched; they are not claimed as caused or fixed by U1.
6. Regression comparison: after each WU, rerun the same commands and diff by test name against the baseline. Rule: no test that passed at baseline may fail; the set of failing names must be a subset of the baseline failing set; the set of `#[ignore]`d tests outside `domain_backup` must not grow. U1's own DB-backed tests are ignored by design (see Integration) and must all use the exact reason string `requires Docker for PostgreSQL testcontainer`; any other new `#[ignore]` requires a recorded reason.
7. If Docker is unavailable, DB-backed TS cases (ignored) are reported "not run (Docker unavailable)" and never as passed; pure-logic ports still run (see Environments).

## Existing tests likely affected [O relayed, unverified]

- `apps/api/gateway/src/main.rs`: `manifest_with_five_services_returns_five_entries` becomes 6; `manifest(...)` signature gains a backup state parameter, affecting every call.
- `compose_routers_*` tests in `apps/api/gateway/src/main.rs` [O, read from the file]: `compose_routers_supplies_cookie_extractor_to_domain_routes`, `compose_routers_traces_health_requests_without_changing_response`, `compose_routers_traces_rejected_protected_requests_without_calling_handler`. WU-10 changes `compose_routers` (F6: a separate administrator router for the backup registration, see TS-55 and FD F-38), so these three must stay green and are rerun after WU-10; none registers a backup service, so they are expected unaffected. `Mount` gains one variant (`Mount::AdministratorIngress`,) in `apps/api/domain_interface/src/lib.rs`; the only exhaustive `match reg.mount` found is in `compose_routers` [O `main.rs:251-255`], other uses construct a variant and are unaffected (not compiled here, unverified).
- `apps/api/gateway/src/backup_cli.rs`: six existing tests stay unchanged; `reports_missing_credentials_without_echoing_secrets` is a tautology, noted and not touched.
- `apps/api/gateway/src/migrate_cli.rs`: six existing tests [O `migrate_cli.rs:121-175`]. `all_migration_identities_include_seo_after_posts` asserts `ids.last() == Some("m20260822_000001_seo_head_assets")` [O `:170-174`] and WILL break if backup identities are chained after seo (`all_migration_identities` chains posts then seo [O `:77-82`]). WU-10 changes this test; one decision to record at the plan gate [D, Q]: either (a) keep the chain order posts, seo, backup and update the assertion to the new last backup identity (proposed: simplest, keeps ordering by creation), or (b) chain backup before seo and leave the assertion. Reason must be recorded in WU-10 evidence; work-units WU-10 "Done when" and FD 2.4 should mirror it.
- `apps/api/test_helpers` and its consumers: not changed by U1 (backup tests use a crate-local fixture); must still be rerun at baseline comparison (RK-04).
- `apps/web/src/app/admin/components/left-menu.test.tsx` (new menu item) and any `App.tsx` route tests.

## Red-first expectations [D]

- Rule: RED-GREEN-REFACTOR per work unit (spine D-07), resolving the FEAT-01 section 13 `test-after` versus `agent-team.md` tension. The red-tests stage precedes build.
- Skeleton ownership [D]: the red-tests stage may create only the minimum crate skeleton needed to get past the compile error (workspace member entry, `domain_backup/Cargo.toml`, `lib.rs` and empty or `todo!()`/`unimplemented!()` stubs for the symbols the tests name). Stubs contain no logic. The build stage replaces stubs. The first red is the compile error; the recorded red is the assertion/panic failure after stubs exist.
- A red that fails for an environmental reason (no Docker, missing crate version) does not count as red; record it as blocked.
- Mutation-style sanity check per security test: temporarily weaken the guard (for example allow `succeeded` in the CHECK, or poll the body in a disabled handler) and confirm the test fails, then restore. Recorded as evidence in the red or verify stage; not claimed done here.
- Green: minimal code; refactor only with all tests green.
- Stub-ordering note: F-04 / `policy.rs` is built in WU-08 but the worker (WU-05) re-checks at claim (work-units discrepancy 1). In WU-05 the re-check goes through a narrow port with a test double (TS-05a); the real policy is exercised in WU-08 (TS-05b). The same split applies wherever a test needs code from a later unit (see the split rows in the test case table).

## Fault-injection approach (crash, DB outage, deadline) [D]

- Ports from spine section 4.5 (`Clock`, `FixtureStorage`, `AllocationStore`, `AuditSink`, `JobStore`) get fault-injecting wrappers behind `#[cfg(test)]` or a `test-util` feature. They are not reachable in production composition (T-05).
- Crash windows 1-10 (TS-20..TS-29): inject a one-shot failpoint at a named step (for example `after_ledger_commit`, `after_file_create`, `after_unlink`), then drop the runtime without graceful shutdown and construct a new `BackupRuntime::start` over the same DB and temp root. Assert on the post-restart ledger rows, inventory, job statuses, audit events and counters. This simulates a crash by abandoning in-memory state; it does not kill a real process [A: sufficient fidelity; validate by one two-process test in TS-32].
- Window 10 and DB outage (TS-29): a wrapper store returns `StoreError` on demand; assert `db_healthy=false`, admissions closed, no delete calls reached the `FixtureStorage` spy, a safe health event emitted, and reconcile runs before admissions reopen.
- Commit failure (TS-16): fail the transaction commit after the job insert; assert no accepted response and that an idempotent retry resolves to one job.
- Audit unavailable (TS-15): `AuditSink` fault returns error; assert no job row, no allocation row, no "accepted" body.
- Deadline (TS-30, TS-28, WU-05 test `worker::tests::deadline_exceeded_fails_job`) [D]: one time source per layer. `deadline_at = clock.now() + CFG-10` comes from the `Clock` port; the worker's in-flight timeout (`tokio::time::timeout`, F-20) is computed as `deadline_at - clock.now()`. The test runs under `#[tokio::test(start_paused = true)]` and uses a helper `advance_both(d)` that calls `FakeClock::advance(d)` and `tokio::time::advance(d)` together, so the tokio timer and the Clock port move in step. Steps: blocked capture adapter, `advance_both(600 s)`, assert CAS `running` to `failed DEADLINE_EXCEEDED`; then race a late worker against the sweeper CAS (which reads only `Clock`) and assert the terminal result is not overwritten. FakeClock alone does not fire a tokio timer, which is why both are advanced [A: requires tokio `test-util`; validate in WU-01 `cargo check -p domain_backup`].
- Time-based bounds never use real sleeps; use `FakeClock` and, for tokio timers, `tokio::time::pause` with `advance_both` (above).
- Concurrency races (TS-13, TS-14a, TS-14b): `tokio::join!` of N creates with the same or different keys against Docker Postgres; assert exactly one job per key and at most one queued/running job per instance.

## Leakage canary approach [D]

- A unique canary string (for example `CANARY-` plus a random UUID per test run) is injected into every attacker- or secret-shaped input that backup code itself handles: request body unknown fields, the `Idempotency-Key` header, fixture ids, env values (a fake DSN and storage key) in the process environment, and the storage root path.
- Token headers are excluded from canary injection [D]. The existing auth layer in `domain_auth`, which U1 does not change, returns its own body `{"error":"..."}` built with `format!(r#"{{"error":"{}"}}"#, e)` [O `apps/api/domain_auth/src/lib.rs:148`], and one of its error texts embeds the token's `kid` header (`Key not found for kid: {}`, line 238 [O, per reviewer]), unescaped. A canary in a token header would therefore fail on code outside U1. This reflection is recorded as a pre-existing `domain_auth` finding, out of U1 scope [Q: owner/conductor to decide whether to raise it separately]. TS-17 does not assert on 401/403 bodies produced by the auth layer.
- After exercising success, error, denial, replay and outage paths, scan: HTTP response bodies and headers of backup-handled responses, captured `tracing` output (custom capture layer as in existing gateway tests [O]), every column of `backup_*` tables (including `backup_audit`), error projections and any status/metrics output (labels included, see TS-57).
- Bundle scan: build `apps/web` with canary values present in the build environment and grep the `dist` output for the canary (TS-17 full, WU-15). Maps to T-09 (threat model: scans of "responses/bundles/logs/audit/metrics"; quality requirement PR-02).
- Pass condition: the canary appears nowhere it should be absent; the raw idempotency key is absent from all tables (only the digest exists). The scan helper must itself be proven by a positive control that deliberately logs the canary and is detected.
- Web: a canary-bearing server string renders escaped text and is never interpreted as HTML (TS-18).
- Full run is a WU-15 deliverable (TS-17 full); the API-output part lands in WU-08.

## Criterion to level (acceptance scenarios in U1 scope)

Scenario names are those in `acceptance-criteria.md` (re-read in revision 2). S1/S2/S3 are the first, second and third scenario of each block in file order. The tests column is derived from the TS rows below: every TS-to-AC pair appears in both tables.

| Criterion scenario | Level | Tests | Status |
|---|---|---|---|
| AC-001 S1 Authorized administrator has an eligible instance | unit, integration | TS-45 (IF-01 response), TS-46 (web selector renders label and environment), TS-38 (empty list disables submit), TS-06 (create rejects non-listed instance) | in scope |
| AC-001 S2 Unauthorized instance listing | security | TS-03 (extended to `GET /backup/instances` with writer, non-admin, missing, expired and forged tokens; the list is absent from the response) | in scope; TS-35 and F-42 are UX only and are not evidence |
| AC-002 S1 Job status is observable | unit, integration | TS-13, TS-14a, TS-14b, TS-36, TS-37, TS-49 (reference, status, progress null unless measured, safe error text), TS-52, TS-41 (API part), TS-61 (IF-03 limit/cursor 400), TS-63 (malformed JSON, non-UUID path), TS-67 (claim CAS) | in scope |
| AC-002 S2 Required capture fails | integration, unit | TS-22, TS-28, TS-29, TS-33, TS-47, TS-48, TS-68 (deadline), TS-01, TS-02, TS-30, TS-40, TS-42 (bounds, config and cleanup behaviour; also traced to Q-05 and T-13) | in scope |
| AC-002 S3 U1 database-only result | unit, integration, security, e2e | TS-09 (download denied), TS-10, TS-11, TS-12a, TS-12b, TS-41, TS-47 | in scope |
| AC-003 S1 Invalid or incompatible package | none | none | Deferred: real preflight validation NOT authorized [UP]; U1 only proves rejection (TS-09) |
| AC-003 S2 Unsafe archive | none | none | Deferred: real upload/archive validation NOT authorized [UP]; U1 only proves rejection (TS-09) |
| AC-003 S3 U1 real-data staging is not enabled | security, integration | TS-01, TS-02, TS-09, TS-10, TS-19..TS-21, TS-23..TS-27, TS-30, TS-31, TS-32, TS-42, TS-53 | in scope (synthetic) |
| AC-004 S1 Confirmation describes impact | integration, unit | TS-43, TS-51 (web: names the fixture package, target instance label and data in scope), TS-64 (API: IF-09/IF-10 identification fields) | in scope (synthetic preview only) |
| AC-004 S2 Restore is cancelled | integration | TS-43 | in scope (preview only) |
| AC-004 S3 Restore failure | none | none | Explicitly deferred (C12): restore execution is NOT authorized in U1 [UP], so there is no restore that can fail; only direct restore denial is tested (TS-09, traced to AC-003 S3 and AC-005 S1, not to AC-004 S3) |
| AC-005 S1 Unauthorized operation or resource access | security, integration | TS-03, TS-04, TS-05a, TS-05b, TS-07, TS-59, TS-60, TS-62 | in scope |
| AC-005 S2 Audit records a sensitive operation | security, integration, unit | TS-13, TS-17, TS-18, TS-44a and TS-44b (required metadata columns populated per event, exclusions asserted), TS-66 (revert keeps audit rows), TS-69 (audit sink records in the caller transaction) | in scope |
| AC-005 S3 Audit store unavailable | integration | TS-15, TS-16, TS-29, TS-65 (audit unavailable refuses deletion claim, deletion commit and purge) | in scope |
| AC-006 S1 Production restore readiness | none | none | Deferred: production not authorized; Q-04, Q-06, Q-07, Q-10, Q-11 open |
| AC-006 S2 Unresolved U1 design blockers | review/gate check | TS-58 (open items listed as open, synthetic-only labels present; the only direct AC-006 S2 test; config, bounds and cleanup behaviour carries the behavioural AC-002 S2 and AC-003 S3 traces above) | in scope |
| Bounds, retention, escalation (not an AC scenario; owner decision Q-05, ADR-004, T-13) | unit, integration, performance | TS-01, TS-02, TS-27a, TS-27b, TS-30, TS-40, TS-42, TS-50, TS-55..TS-57 | in scope; traced to owner decision and T-13, and behaviourally to AC-002 S2 and AC-003 S3 (see those rows) |
| a11y AC-01..AC-04 | unit, manual | TS-37 | automated part in scope; manual assistive-technology review recorded separately, not claimed here |
| Any scenario requiring a `succeeded` outcome, download success, Auth/Storage recovery, RPO/RTO, encryption | none | none | Deferred: out of U1 scope, open Q-04/Q-06/Q-10/Q-11 |

There is no "AC-006 S3".

## Test case table (TS -> AC -> T -> work unit)

L: U unit, I integration, S security, E e2e, P performance, G gate/review check. All entries are planned, none executed. Split ids (a/b) exist so each part is red-then-green inside its own unit.

| TS | Test | L | AC | T | WU | Location (primary) |
|---|---|---|---|---|---|---|
| TS-01 | Config defaults equal owner values exactly (CFG-05..CFG-14 bounds; CFG-15 escalation owner) | U | Q-05 owner decision; AC-003 S3 | T-13 | WU-01 | domain_backup/src/domain/config.rs |
| TS-02 | Invalid or missing required config makes backup disabled with `disabled_reason` `INVALID_CONFIG` (closed set `NOT_CONFIGURED`, `NON_LOOPBACK_BIND`, `INVALID_CONFIG`, null): per-key default / below / above cases for each CFG-05..CFG-14 (only the owner-approved default is valid; DB CHECKs are safety ceilings only, FD F-01); CFG-16 `BACKUP_ALLOWED_ORIGINS` missing, malformed or wildcard when enabled; CFG-17 and CFG-18 (ingress limits, no approved default) absent when enabled; storage root absolute, private, not under forbidden paths (see TS-02 definition below) | U | AC-003 S3; AC-002 S2 (bounds behaviour); Q-05 owner decision | T-13, T-14 | WU-01 | domain/config.rs |
| TS-03 | Composed router: missing, expired, forged (HS256 with a wrong secret, so no JWKS fetch), writer and non-admin tokens denied before handler or config access, on `GET /backup/instances` and on job endpoints; instance list absent from the response | S | AC-001 S2; AC-005 S1 | T-01 | WU-10 | gateway/src/main.rs |
| TS-04 | Foreign and unknown job ids give non-disclosing 404 on read, list, replay, download | I | AC-005 S1 | T-02 | WU-08 | handlers/get_job, list_jobs |
| TS-05a | Worker claim re-check goes through the policy port; revoked or unknown instance fails the job (test double) | I | AC-005 S1 | T-02 | WU-05 | lifecycle/worker.rs |
| TS-05b | Real `policy.rs` (F-04) re-check against the instance registry at claim | I | AC-005 S1 | T-02 | WU-08 | domain/policy.rs, handlers/* |
| TS-06 | Unknown instance, extra fields, URL, DSN, key fields rejected on create (strict JSON parse F-05) | U/I | AC-001 S1 (only listed instance is accepted); AC-005 S1 | T-03 | WU-08 | handlers/create_job/request.rs |
| TS-07 | Composed router: content type required (415), no cookie-only mutation auth, no CORS layer on backup routes, no custom header required (bearer-only auth plus JSON content type, IFC 2.1). Host/Origin tests are TS-59; per-handler 415 mapping is TS-62 | S | AC-005 S1 | T-04 | WU-10 | gateway/src/main.rs |
| TS-08a | F-39 loopback guard, unit: `is_loopback_bind` parses `HOST` as an IP; non-loopback IP, `0.0.0.0`, `::` and non-IP hostnames are non-loopback and force backup disabled with `NON_LOOPBACK_BIND`; loopback IPs pass (TS-08 is retired, split into TS-08a and TS-08b) | U | AC-005 S1 | T-04 | WU-01 | apps/api/domain_backup `domain/config.rs` |
| TS-08b | F-39 gateway wiring: the effective bind value the gateway passes to the backup config yields a disabled backup (`NON_LOOPBACK_BIND`) on a non-loopback bind and an enabled one on loopback | S | AC-005 S1 | T-04 | WU-10 | apps/api/gateway/src/main.rs |
| TS-09 | Upload, preflight, restore, download denied; body stream not polled; no ledger rows or files (spies); download is 403 `DOWNLOAD_DENIED` for every status | S | AC-002 S3; AC-003 S3; AC-003 S1, S2 (rejection only); AC-003 S3 and AC-005 S1 (direct restore denied; AC-004 S3 is Restore failure and is deferred, not cited) | T-05 | WU-09 | handlers/disabled_capability, download_denied |
| TS-10 | Client flags cannot select real adapter; runtime composition holds only fixture adapter | S | AC-002 S3; AC-003 S3 | T-05 | WU-09 | lifecycle/runtime.rs, handlers/create_job |
| TS-11 | Status enum keeps every documented value (including `succeeded` and `recovery-required`) and adds `incomplete` (ADR-002, D-10). Exhaustive enumeration (no property-test crate): no transition, write or projection can produce `succeeded` or `recovery-required` for a U1 job: the state-machine guard allows no pair into them; a DB constraint/trigger on backup jobs rejects those values for U1 job kinds and rejects terminal updates; the serializer/API never emits them for U1 jobs (serializer part is TS-12b) | U/I | AC-002 S3 | T-06 | WU-01, WU-02 | domain/state_machine.rs, migrations |
| TS-12a | Web: zod schema and UI accept and render every status value (exhaustive handling, including `succeeded` and `recovery-required`); download always denied (no download control for any status); unknown status safe fallback | U | AC-002 S3 | T-06 | WU-14 | web components |
| TS-12b | Rust serializers (`handlers/*/response.rs`): `download_eligible` false, neither `succeeded` nor `recovery-required` emitted for U1 jobs | U | AC-002 S3 | T-06 | WU-08 | handlers/*/response.rs |
| TS-13 | Idempotency replay, changed-request conflict, concurrent replays; raw key absent from all tables | I | AC-002 S1; AC-005 S2 | T-07 | WU-07 | handlers/create_job |
| TS-14a | Store level: partial unique index violation when a second queued/running job exists for the instance | I | AC-002 S1 | T-07, T-13 | WU-03 | infra/db_job_store.rs |
| TS-14b | API level: second create returns 409 `CONCURRENCY_LIMIT`; durable under races | I | AC-002 S1 | T-07, T-13 | WU-07 | handlers/create_job |
| TS-15 | Audit unavailable: no job, no allocation, no accepted response | I | AC-005 S3 | T-08 | WU-07 | handlers/create_job |
| TS-16 | Commit failure injection: no accepted response; idempotent retry resolves | I | AC-005 S3 | T-08 | WU-07 | handlers/create_job |
| TS-17 | Secret canary absent from backup-handled responses, logs, audit rows, errors and metrics (API part WU-08); full run adds the web bundle scan (WU-15). Token header fields excluded, see canary section | S | AC-005 S2 | T-09 | WU-08, WU-15 | handlers/*, gateway, `apps/web/dist` |
| TS-18 | Malicious strings in labels/errors render escaped in UI | U | AC-005 S2 | T-10 | WU-14 | web components |
| TS-19 | Path safety: traversal, absolute, symlink, junction/reparse (Windows), forged orphan names, ambiguous ownership never deleted | I/S | AC-003 S3 | T-11 | WU-04 | infra/fs_fixture_storage.rs |
| TS-20 | Crash window 1: ledger commit before file creation | I | AC-003 S3 | T-12 | WU-06 | lifecycle/* |
| TS-21 | Window 2: file creation before ledger update | I | AC-003 S3 | T-12 | WU-06 | lifecycle/* |
| TS-22 | Window 3: worker interruption | I | AC-002 S2 | T-12 | WU-06 | lifecycle/worker.rs, reconciler.rs |
| TS-23 | Window 4: orphan namespace entry | I | AC-003 S3 | T-12 | WU-06 | lifecycle/reconciler.rs |
| TS-24 | Window 5: missing file | I | AC-003 S3 | T-12 | WU-06 | lifecycle/reconciler.rs |
| TS-25 | Window 6: cleaner claim before unlink | I | AC-003 S3 | T-12 | WU-06 | lifecycle/cleaner.rs |
| TS-26 | Window 7: unlink before final commit | I | AC-003 S3 | T-12 | WU-06 | lifecycle/cleaner.rs |
| TS-27a | Window 8, ledger level: retry exhaustion (3 attempts, 1-minute backoff) yields `cleanup_failed` and the exhausted counter | I | AC-003 S3 | T-12, T-13 | WU-06 | lifecycle/cleaner.rs |
| TS-27b | IF-11 status endpoint reports escalation (`escalation.visible`, `cleanup.exhausted`) | I | AC-003 S3 | T-12, T-13 | WU-08 | handlers/status |
| TS-28 | Window 9: active worker vs sweeper race; expiry alone does not delete active work | I | AC-002 S2 | T-12 | WU-06 | lifecycle/sweeper.rs |
| TS-29 | Window 10: metadata DB outage; no deletion, safe signal, reconcile before reopening | I | AC-002 S2; AC-005 S3 | T-12 | WU-06 | lifecycle/runtime.rs |
| TS-30 | Bounds with fake clock: 256 MiB per allocation, 1 GiB total, 10 min deadline, 3 cleanup attempts in total (CFG-08) with 1-minute backoff between attempts and no 4th unlink attempt, 24 h expiry, 1 h grace, 15 min sweep | I | Q-05 owner decision; AC-003 S3 | T-13 | WU-04, WU-06 | lifecycle/*, infra/db_allocation_store.rs |
| TS-31 | Private permissions (Unix: modes 0700/0600; Windows: PARTIAL evidence only, see TS-02 definition and T-14 note), no static mapping, no public URL | S | AC-003 S3 | T-14 | WU-04 | infra/fs_fixture_storage.rs |
| TS-32 | Startup order; second process refused ownership; admissions closed until reconcile finishes | I | AC-003 S3 | T-11, T-12 | WU-04, WU-06 | infra/fs_namespace_lock.rs, lifecycle/runtime.rs |
| TS-33 | `backup_cli::handle_args`: create, list, restore, unknown input fail closed; no DB or S3 contact | U | AC-002 S2 | T-05 | WU-11 | gateway/src/backup_cli.rs |
| TS-34 | Migrations: fresh, upgrade beside posts and seo, separate migration table, trigger presence (structural, not an authorization test) | I | none (supports AC-002 S3 structurally) | T-17 | WU-02 | migrations/mod.rs |
| TS-35 | Web route guard, administrator-only menu item, `left-menu.test.tsx` extended. UX only; not evidence for any server-side authorization scenario | U | none (UX) | T-01 (UX only) | WU-13 | web App.tsx, left-menu |
| TS-36 | Web polling with fake timers; stops on terminal; cleans up on unmount | U | AC-002 S1 | T-13 | WU-14 | hooks/use-backup-jobs.test.ts |
| TS-37 | Web accessibility: keyboard, text status, announcements, labels; manual AT review separate | U/E | AC-002 S1 | a11y AC-01..AC-04 | WU-14 | web components |
| TS-38 | Empty or unavailable instance list disables submission | U | AC-001 S1 | T-03 | WU-13 | instance-selector.test.tsx |
| TS-39 | Job creation latency measured; no target claimed | P | AC-002 S1 | T-13 | WU-15 | evidence record |
| TS-40 | Temporary bytes and concurrency bounded under load | P | Q-05 owner decision | T-13 | WU-15 | evidence record |
| TS-41 | API-level: admin creates job, ends `incomplete`; direct `GET .../download` returns 403 `DOWNLOAD_DENIED`. UI-level: the UI exposes no download control for an `incomplete` job (UI part BLOCKED, see E2E-ENV-01) | E | AC-002 S1, S3 | T-05, T-06 | WU-15 | evidence record |
| TS-42 | Retention purge at 30 days; audit rows kept | I | Q-05 owner decision | T-13 | WU-06 | lifecycle/sweeper.rs |
| TS-43 | Preview confirm and cancel enqueue nothing, call no mutation port | I | AC-004 S1, S2 | T-05, T-15 (preview) | WU-09 | handlers/preview |
| TS-44a | `job.accepted` audit event: actor, instance, job, time and outcome populated; no excluded fields (credentials, tokens, content, raw keys) | I | AC-005 S2 | T-09 | WU-07 | infra/db_audit_sink.rs, handlers/create_job |
| TS-44b | `job.running`, `job.incomplete`, `job.failed` and allocation audit events: actor, instance, job, time and outcome populated; no excluded fields | I | AC-005 S2 | T-09 | WU-05 (job events), WU-06 (allocation events) | lifecycle/worker.rs, lifecycle/allocator, infra/db_audit_sink.rs |
| TS-45 | `GET /backup/instances` (IF-01): exactly the configured instance, safe label, environment `local`, capabilities upload/preflight/restore/download all false | I | AC-001 S1 | T-03 | WU-08 | handlers/list_instances |
| TS-46 | Web instance selector renders the instance label and environment label | U | AC-001 S1 | T-03 | WU-13 | instance-selector.test.tsx |
| TS-47 | Worker happy path: job ends `incomplete` with `REQUIRED_COMPONENTS_UNAVAILABLE` and the exact components array (database `DB_SYNTHETIC_ONLY`, auth `AUTH_NOT_CAPTURED`, storage `STORAGE_NOT_CAPTURED`); F-17, F-18; never `succeeded` | I | AC-002 S2, S3 | T-05, T-06 | WU-05 | lifecycle/worker.rs |
| TS-48 | Worker failure outcomes: `ADAPTER_ERROR`, `PERSISTENCE_ERROR` (`mark_written` CAS failure), `ALLOCATION_LIMIT` (budget exceeded), `DISPATCH_FAILED` (queued job cannot be dispatched); F-16..F-20 | I | AC-002 S2 | T-12, T-13 | WU-05 | lifecycle/worker.rs |
| TS-49 | Job projection F-10: reference and status shown, progress null unless counters measured, safe error text only | U | AC-002 S1 | T-06, T-09 | WU-08 | handlers/*/response.rs |
| TS-50 | Web `BackupStatusBanner` (F-49): render IF-11 payloads (`escalation.visible` true/false, state `degraded`/`not_ready`, `cleanup.exhausted` > 0, `disabled_reason` each of `NOT_CONFIGURED`, `NON_LOOPBACK_BIND`, `INVALID_CONFIG`, null; the zod schema rejects other values safely); visible text names the project owner for escalation | U | Q-05 owner decision (escalation via visible admin-UI status) | T-12, T-13 | WU-14 | components/backup-status-banner.test.tsx |
| TS-51 | Web `SyntheticPreviewPanel` (F-47): labelled synthetic; confirmation names the fixture package, target instance label and data in scope; upload shown unavailable | U | AC-004 S1, S2; AC-003 S3 | T-05, T-15 (preview) | WU-14 | components/synthetic-preview-panel.test.tsx |
| TS-52 | Web `JobCreatePanel` (F-44): fresh `crypto.randomUUID()` key per submit; double-submit guard; REST client (`domains/backup.ts`) strict parsing and unknown-status tolerance (F-41) | U | AC-002 S1 | T-06, T-07 | WU-12 (client), WU-13 (panel) | domains/backup.test.ts, job-create-panel.test.tsx |
| TS-53 | `preview_preflight` (F-13): canned result only for a trusted fixture id; uploaded files and paths never accepted | I | AC-003 S3; AC-004 S1 | T-05 | WU-09 | handlers/preview |
| TS-55 | Ingress limits as gateway layers on the backup router (design C10): request timeout (CFG-17) and header-size limit (CFG-18) return a safe error and never reach the handler; with CFG-17/CFG-18 absent, backup is disabled with `INVALID_CONFIG` (TS-02). No approved default exists; owner values are needed before WU-10 [Q]. Ordering (F6, option a): `compose_routers` builds a separate administrator router for the backup registration (`Mount::AdministratorIngress`) wrapped, outermost to innermost, timeout layer (CFG-17), header-size layer (CFG-18), administrator auth layer, backup routes, then merged. Asserts: (a) an unauthenticated request with headers over CFG-18 gets 431, not 401, and never reaches auth or the handler; (b) the timeout layer is outermost: with a deliberately slow inner service (test stub behind the auth layer, valid admin token) exceeding CFG-17 the response is 408 with the crate error body; (c) an unauthenticated request within limits still gets 401 immediately (auth decides from the Authorization header alone, so a 408-before-401 ordering is not observable and is not asserted; slow header delivery is handled by hyper before the router and is a residual for T-13); and (d) non-backup routes (public, protected, existing administrator) are unaffected. The 16 KiB body cap is tested by TS-60 | S | AC-002 S1; Q-05 owner decision | T-13 | WU-10 | gateway/src/main.rs |
| TS-56 | Bounded inventory (design C10): `FixtureStorage::inventory` takes a max-entries bound and returns a truncation flag; over-bound yields truncation true; reconciliation fails closed on truncation (no deletes or fences, safe degraded signal) | I | AC-003 S3; Q-05 owner decision | T-13 | WU-04 (store), WU-06 (reconcile fail-closed) | infra/fs_fixture_storage.rs |
| TS-57 | Metric labels are the closed set `job_kind`, `outcome`, `error_code` only (design C10); no id, path or actor values; canary-bearing inputs never appear as label values | U | AC-005 S2 | T-13, T-09 | WU-08 | handlers/status, domain metrics |
| TS-58 | Gate/review check: Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention and the non-owner guards are listed as open in the implementation plan and `docs/features/backup-restore.md`; UI and API responses carry the synthetic-only label | G | AC-006 S2 | none | WU-15 | evidence record (checklist) |
| TS-59 | Host/Origin policy (IFC 2.1, T-04): on every backup route a `Host` outside the loopback forms (`127.0.0.1`, `localhost`, `[::1]`) with the configured port gives 403 `HOST_NOT_ALLOWED`; on POST routes an `Origin` not exactly matching a CFG-16 entry gives 403 `ORIGIN_NOT_ALLOWED`; absent `Origin` is accepted; allowed Host and matching Origin pass; precedence 401, `ROLE_REQUIRED`, then Host/Origin, as in IFC 2.1 | S | AC-005 S1 | T-04 | WU-08 | handlers/* (host/origin check), gateway/src/main.rs |
| TS-60 | Body cap: IF-02, IF-09 and IF-10 reject a body over 16 KiB (by `Content-Length` and by streamed length) with 413 `PAYLOAD_TOO_LARGE` in the crate error body; IF-06..IF-08 never read a body (covered by TS-09) [E: 16 KiB is a non-owner guard] | S | AC-005 S1 | T-13 | WU-08 | handlers/create_job/request.rs, handlers/preview |
| TS-61 | IF-03 `limit` above 100 or malformed `cursor` gives 400 with `reason_code` `MALFORMED_REQUEST` (coarse `error_code` `ValidationError`) in the crate error body (IFC 2.2, 2.6, 10.1; FD IF-03); default page size applies when absent | I | AC-002 S1 | T-13 | WU-08 | handlers/list_jobs |
| TS-62 | Wrong or missing JSON `Content-Type` on IF-02, IF-09, IF-10 gives 415 `UNSUPPORTED_MEDIA_TYPE` in the crate error body (handler level; TS-07 covers the composed router) | I | AC-005 S1 | T-04 | WU-08 | handlers/create_job/request.rs, handlers/preview |
| TS-63 | Malformed JSON, unknown field and non-UUID path parameter give 4xx in the crate error body with no echo of input | I | AC-002 S1 | T-03 | WU-08 | handlers/*/request.rs |
| TS-64 | IF-09 and IF-10 responses carry the IFC 2.9 identification fields `fixture_label`, `target_instance_id`, `target_instance_label`, `target_environment` (`local`) and `data_scope`, plus `synthetic: true`; the target fields are read from the configured instance (CFG-02, CFG-03), never from the request (a request-supplied target value is rejected or ignored); IF-10 additionally asserts `executed: false`; no `fixture_id` or `scope_summary` field is asserted on responses (`fixture_id` is a request field only) (AC-004 S1; web part is TS-51) | I | AC-004 S1 | T-05, T-15 (preview) | WU-09 | handlers/preview |
| TS-65 | Audit unavailable refuses the deletion claim, the deletion commit and the retention purge: no unlink, no row change, no purge | I | AC-005 S3 | T-08 | WU-06 | lifecycle/cleaner.rs, sweeper.rs |
| TS-66 | Migration revert keeps `backup_audit` rows and its append-only trigger; the down migration refuses while audit rows exist | I | AC-005 S2 | T-08, T-09 | WU-02 | migrations/mod.rs |
| TS-67 | Worker claim: queued to running CAS has one winner under duplicate dispatch; a late or duplicate worker cannot overwrite a terminal job; the claim re-check goes through the policy port (TS-05a) | I | AC-002 S1, S2 | T-07, T-12 | WU-05 | lifecycle/worker.rs |
| TS-68 | Deadline: `worker::tests::deadline_exceeded_fails_job` (blocked capture adapter, `advance_both(600 s)`, CAS running to failed `DEADLINE_EXCEEDED`; late worker vs sweeper does not overwrite) | I | AC-002 S2 | T-12, T-13 | WU-05 | lifecycle/worker.rs |
| TS-69 | Audit sink records in the caller transaction: `AuditSink::record` (F-34) writes the audit row on the caller's transaction; rolling that transaction back leaves no audit row | I | AC-005 S2 | T-08 | WU-03 | ports/audit.rs, infra/db_audit_sink.rs |

TS-54 is intentionally unused (reserved so TS-55..TS-58 keep a stable numbering); do not assign it. TS-59 and TS-60 were added in revision 3; TS-61..TS-68 in revision 4 (TS-69 and the splits TS-08a/TS-08b and TS-44a/TS-44b were added after the final conductor decisions F2, F3 and F5; TS-08 and TS-44 are retired and must not be reused) (C1: TS-61..TS-66 are the renumbered extra tests of interfaces-and-contracts.md, whose body-cap duplicate is merged into TS-60; TS-67 and TS-68 give WU-05 explicit ids).

TS-02 definition [D]: the forbidden-path set is an explicit input, not ambient knowledge: `validate_storage_root(root: &Path, forbidden: &[PathBuf]) -> Result<..>` (name proposed; FD/IFC do not name it [Q]). The gateway supplies `forbidden` = the process current directory (`std::env::current_dir()`) plus its `apps/web/public` and `apps/web/dist` children when they exist, plus any directory the deployment passes as static assets; the validator itself reads no repo state. It rejects: a relative path, a path equal to or beneath any `forbidden` entry, and a reparse point. Test inputs (tempdir `T`): `forbidden=[T/repo]`; reject `T/repo`, `T/repo/data`, `T/repo/apps/web/public/x`, `relative/dir`, and a symlink/junction to `T/repo`; accept `T/private/root`. "Private" is checked per OS: on Unix the root directory must be mode 0700 and files 0600; on Windows no Unix mode bits apply, so U1 verifies only that the root is absolute, not a reparse point, and not under the forbidden set. Owner-only ACL protection is NOT verified on Windows in U1: T-14 is PARTIALLY evidenced on this host. Hard stop on WU-04 (C10): owner acceptance of the Windows ACL limitation, or an added ACL inspection test, before WU-04 is done; the same applies at the release-candidate security check. TS-31 uses the same definition. CFG-04 (fixture-storage location and protection) is an open owner approval and a blocker for WU-04..WU-06; the validator tests use tempdirs only and do not decide it.

Note: for inline Rust tests the "location" is the module owning the code. Concrete test function names are given for WU-01 below; the red-tests stage names the rest following the same `module::tests::behaviour` convention.

## WU-01 concrete red tests [D]

Crate-local `ENV_LOCK` [D]: `domain_backup` has no env lock and the existing one lives in `domain_auth/src/test_lock.rs` [O]; WU-01 adds `domain_backup/src/test_support.rs` (cfg(test)) with a `static ENV_LOCK: Mutex<()>` and a scoped env guard, used by every test that mutates process env (TS-01, TS-02).

| Test (proposed name) | TS | Expected red |
|---|---|---|
| `config::tests::defaults_match_owner_values` | TS-01 | stub style for WU-01 [D]: `Default`/constructors return zero or empty values (no `todo!()` in WU-01), so every red is an assertion failure. Fails on staging expiry 24h, grace 1h, sweep 15 min, cleanup 3 attempts in total (CFG-08) with 1-minute backoff (and a 4th unlink attempt never occurs), deadline 10 min, 256 MiB, 1 GiB, concurrency 1, retention 30 days, escalation owner `project_owner` (CFG-15) |
| `config::tests::enabled_without_storage_root_refuses` | TS-02 | stub accepts; assertion fails |
| `config::tests::storage_root_rejects_forbidden_and_relative_paths` | TS-02 | inputs per the TS-02 definition (`validate_storage_root(root, forbidden)`); stub accepts; assertion fails |
| `config::tests::invalid_numeric_env_refuses_capability` | TS-02 | stub accepts; assertion fails |
| `config::tests::bounds_other_than_default_disable_with_invalid_config` | TS-02 | per key CFG-05..CFG-14: default valid, below and above give disabled with `INVALID_CONFIG` (FD F-01); stub accepts; assertion fails |
| `config::tests::ingress_limits_required` | TS-02 | CFG-17 or CFG-18 absent or malformed when enabled gives `INVALID_CONFIG`; stub accepts; assertion fails |
| `config::tests::allowed_origins_required_and_well_formed` | TS-02 | CFG-16 missing, malformed or wildcard refuses when enabled; stub accepts; assertion fails |
| `state_machine::tests::no_transition_reaches_succeeded_or_recovery_required` | TS-11 | enumerates every (from, to) pair of `BackupJobStatus` (all documented values plus `incomplete`) against `state_machine::is_allowed(from, to)` [D, Q: signature to be confirmed in FD]; fails if any pair into `succeeded` or `recovery-required` is allowed for a U1 job |
| `state_machine::tests::terminal_states_have_no_outgoing_transitions` | TS-11 | stub allows; assertion fails |
| `status::tests::no_write_or_projection_yields_succeeded_or_recovery_required` | TS-11 | no transition, write constructor or projection can produce `succeeded` or `recovery-required` for a U1 job (the enum itself keeps them, D-10); stub allows; assertion fails |

Property testing: TS-11 uses exhaustive enumeration over the finite status pairs, so no property-testing crate is added [A: set is small and finite; impact: if events become unbounded the decision is revisited at the plan gate].

## Threat to test traceability (T-01..T-14, mandatory)

| T | Primary test(s) | Additional |
|---|---|---|
| T-01 | TS-03 | TS-35 (UX only) |
| T-02 | TS-04 | TS-05a, TS-05b |
| T-03 | TS-06 | TS-38, TS-45, TS-46, TS-63 |
| T-04 | TS-08a, TS-08b (loopback guard F-39), TS-59 (Host/Origin, CFG-16), TS-07 (content type, no cookie-only auth, no CORS) | TS-02 (CFG-16 validation), TS-62; residual: reverse proxy, see note |
| T-05 | TS-09 | TS-10, TS-33, TS-41, TS-43, TS-47, TS-53, TS-64 |
| T-06 | TS-11 | TS-12a, TS-12b, TS-41, TS-47, TS-49 |
| T-07 | TS-13 | TS-14a, TS-14b, TS-52, TS-67 |
| T-08 | TS-15 | TS-16, TS-65, TS-66, TS-69 |
| T-09 | TS-17 (including bundle scan) | TS-44a, TS-44b, TS-49, TS-57, TS-66 |
| T-10 | TS-18 | TS-12a |
| T-11 | TS-19 | TS-32 |
| T-12 | TS-20..TS-29 (with TS-27a, TS-27b) | TS-32, TS-47, TS-48, TS-50, TS-67, TS-68 |
| T-13 | TS-30 | TS-01, TS-02, TS-14a, TS-14b, TS-27a, TS-27b, TS-36, TS-40, TS-42, TS-48, TS-50, TS-55, TS-56, TS-57, TS-60, TS-61, TS-68 |
| T-14 | TS-31 (PARTIAL on Windows, hard stop on WU-04, see note) | TS-02 |

T-04 note: the threat model requires restricting allowed origins/hosts through trusted configuration and states that Host/Origin headers or string matching alone cannot prove locality; proxy exposure must be reviewed. IFC designs the policy (Host allow-list of loopback forms plus configured port, 403 `HOST_NOT_ALLOWED`; Origin allow-list for POST from CFG-16 `BACKUP_ALLOWED_ORIGINS`, 403 `ORIGIN_NOT_ALLOWED`, absent Origin accepted; precedence 401, `ROLE_REQUIRED`, then Host/Origin; no custom header). Per decision C7 the control is in scope: CFG-16 and the two 403 checks are designed in FD 4.2, F-01 and F-03 and the IFC error lists, and WU-01 covers CFG-01..CFG-16. Tests: TS-59 (WU-08), TS-02 and `allowed_origins_required_and_well_formed` (CFG-16 validation, WU-01), TS-07 (no custom header, no CORS), TS-08a (F-39 parse, WU-01), TS-08b (F-39 gateway wiring, WU-10), TS-62. Trace is AC-005 S1. F-39 expectation (TS-08a, TS-08b): parse `HOST` as an IP; hostnames, `0.0.0.0` and `::` count as non-loopback; a reverse proxy in front of a loopback bind remains an undetected residual risk [Q].

T-13 note (C10): the mandatory mitigations are designed, not deferred. (1) Ingress request timeout and header-size limit as gateway layers on the backup router, configured by CFG-17 and CFG-18; they have no approved default (to avoid inventing approved numbers), backup is disabled with `INVALID_CONFIG` when they are absent, and owner values are an open decision needed before WU-10, which builds the layers (F-38) and owns TS-55 [Q]; ordering per F6 (timeout, header-size, administrator auth, routes; 431 precedes 401, 408 on slow inner processing; slow header delivery is a hyper-level residual); traced to TS-55 and TS-02. (2) Bounded inventory: `FixtureStorage::inventory` takes a max-entries bound and returns a truncation flag; reconciliation fails closed on truncation; traced to TS-56. (3) Closed metric label set `job_kind`, `outcome`, `error_code` only; traced to TS-57. The 16 KiB body cap (IFC 2.2) is tested by TS-60 (a non-owner guard). The full T-13 test set is TS-30 (primary), TS-01, TS-02, TS-14a, TS-14b, TS-27a, TS-27b, TS-36, TS-40, TS-42, TS-48, TS-50, TS-55, TS-56, TS-57, TS-60, TS-61, TS-68; plan section 6 and FD 9.1 list it in full.

T-14 note: on the Windows 11 host U1 verifies no owner-only access control, so TS-31 is partial evidence for T-14 (see TS-02 definition). Plan section 6 and work-units WU-04 mark T-14 partially evidenced on Windows with a hard stop on WU-04: owner acceptance of the ACL limitation, or an added ACL inspection test, before WU-04 is done, and again at the release-candidate security check.

T-15 is tagged only through TS-43 and TS-51 (preview, no real restore). T-17 only through TS-34 (migration presence). T-16 is not covered in U1 (deferred with real packages).

## Work unit to test mapping and exit

| WU | Tests that must be written red first | Exit criterion |
|---|---|---|
| WU-01 | TS-01, TS-02, TS-08a, TS-11 (pure), names above (CFG-01..CFG-16 plus CFG-17/18 validation) | green; crate compiles; dependency versions confirmed (`cargo check -p domain_backup`) |
| WU-02 | TS-34, TS-11 (DB part), TS-66 | Hard stop: Blocked by [Q]: owner decision on migration gating / release effect (RK-16); no merge to main until decided. Required: green on Docker Postgres (run with `--ignored`) for fresh and upgrade migration of the backup Migrator with `seaql_migrations_backup`. Conditional: "beside posts and seo" is blocked by RK-15 (posts and seo share `seaql_migrations`, a pre-existing defect pending an owner decision); report it as "not run / blocked by RK-15", not as a U1 failure |
| WU-03 | TS-14a, TS-69, CAS race tests | green |
| WU-04 | TS-19, TS-30 (budget), TS-31, TS-32 (lock), TS-56 (store part) | green including Windows path and link tests; blocked until CFG-04 storage location and protection is approved [Q]; T-14 on Windows is partial: hard stop until the owner accepts the ACL limitation or an ACL test is added |
| WU-05 | TS-05a, TS-44b (job events), TS-47, TS-48, TS-67, TS-68 (`worker::tests::deadline_exceeded_fails_job`) | never `succeeded` or `recovery-required`; deadline green; blocked until CFG-04 approved [Q] |
| WU-06 | TS-20..TS-26, TS-27a, TS-28, TS-29, TS-30, TS-32, TS-42, TS-44b (allocation events), TS-56 (reconcile fail-closed part), TS-65 | green; blocked until CFG-04 approved [Q] |
| WU-07 | TS-13, TS-14b, TS-15, TS-16, TS-44a | green (F-34 is consumed here, owned by WU-03) |
| WU-08 | TS-04, TS-05b, TS-06, TS-12b, TS-17 (API outputs), TS-27b, TS-45, TS-49, TS-57, TS-59, TS-60, TS-61, TS-62, TS-63 | green |
| WU-09 | TS-09, TS-10, TS-43, TS-53, TS-64 | green with body-not-read spies |
| WU-10 | TS-03, TS-07, TS-08b, TS-55, manifest test updated to 6, `all_migration_identities_include_seo_after_posts` updated per the recorded decision | green; env docs updated; Done when: Blocked by [Q]: owner decision on migration gating / release effect (RK-16); no merge to main until decided; RK-15 runtime-start condition resolved by owner or recorded as descoped residual; owner values for CFG-17/CFG-18 recorded; existing `compose_routers_*` tests still green |
| WU-11 | TS-33 | green; no behaviour change |
| WU-12 | `domains/backup.test.ts` incl. unknown status tolerance and exhaustive status and `disabled_reason` parsing (TS-52 client part) | green |
| WU-13 | TS-35, TS-38, TS-46, TS-52 (panel part) | green |
| WU-14 | TS-12a, TS-18, TS-36, TS-37, TS-50, TS-51 | green |
| WU-15 | TS-17 full (incl. bundle scan), TS-39..TS-41, TS-58 | recorded; final security evidence check at release-candidate gate |

Every one of T-01..T-14 maps to at least one test above (see the traceability table).

## Environments and data

- Host: Windows 11; shell PowerShell or Git Bash. Windows-specific path tests (junction, reparse points, drive-letter and UNC absolute paths, NamespaceLock behaviour) are required in WU-04 (RK-06).
- Docker: required by DB-backed tests via testcontainers [O]. Availability on this host is unknown [Q]; the baseline reports it (RK-02). Logic tests are designed to run without Docker.
- Data: synthetic fixtures only, fixed small fixture size (code constant, [E], not a Q-05 value, owner may confirm). No real content, no real credentials, no real Supabase Auth or Storage. Canary values are random per run.
- Test tokens: forged-token tests are built as HS256 signed with a wrong secret so the auth layer never performs its live JWKS HTTP fetch (non-HS256 algorithms make it call `reqwest::get` on the JWKS URL [O `domain_auth/src/lib.rs:226-232`]); no test makes network calls.
- Auth-layer response shape: 401 and 403 produced by the `domain_auth` layer use the existing body `{"error": "<string>"}` (for example `Missing Authorization header`, `Insufficient permissions` [O `domain_auth/src/lib.rs:134-139, 166-171`]), not the backup error format. TS-03 asserts status codes and absence of the instance list, not the backup `error_code` shape, for those cases; backup-originated 4xx/5xx use the backup error taxonomy.
- Time: `FakeClock`; no real sleeps for bounds.
- Env vars: tests that set process environment use the crate-local `ENV_LOCK` and scoped guard (pattern in `domain_auth/src/test_lock.rs` and gateway `ScopedEnvVar` [O]).
- Storage root: per-test `tempfile` directory, never a repository path.
- Test DB migrations: `domain_backup` DB tests use the crate-local `backup_test_db()` fixture (own testcontainers Postgres, backup Migrator only, table `seaql_migrations_backup`); `test_helpers` (posts migrations only [O]) is not used or changed (RK-04).

## Cross-artifact follow-ups (for the conductor; this file does not edit them)

Decisions C1..C13 are expected to be reflected in the other four artifacts (verify, do not assume): TS numbering and WU-05 ids (C1), D-10..D-13 ids (C2), status enum not narrowed (C3), bounds and `INVALID_CONFIG` (C4, C5), web client at `domains/backup.ts` (C6), CFG-16 in FD, plan and WU-01 (C7), audit port in WU-03 (C8), hard stops RK-15, RK-16 and CFG-04 (C9), full T-04, T-13 and T-14 sets and the CFG-17/CFG-18 designs (C10), US ids in work-units (C11). Still to check or decide:

- interfaces-and-contracts.md: 401/403 from the auth layer use the existing `{"error": string}` body, not `error_code` `UnAuthorized`/`ForBidden`; the web client must tolerate that shape too.
- interfaces-and-contracts.md section 12: IF-01 row references TS-45 and TS-46; IF-05 download denial references TS-09; BackupStatusBanner row references TS-50; Host/Origin row traces AC-005 S1 with TS-59; IF-09/IF-10 row references TS-64; audit rows reference TS-65 and TS-66; request-limit rows reference TS-60..TS-63.
- implementation-plan.md and work-units.md: record the `all_migration_identities_include_seo_after_posts` decision in WU-10 and FD 2.4; list TS-67 and TS-68 in WU-05.
- functional-design.md: define a pure transition guard (for example `is_allowed(from, to)`); define F-39 parsing; F-42 is UX only.
- AC-006 S2 disposition is the same in all artifacts: bounds trace to the Q-05 owner decision, ADR-004 and T-13 and behaviourally to AC-002 S2 and AC-003 S3; AC-006 S2 itself is covered only by TS-58.

## Unverified

- No baseline has been run (Q-12); the historical 8 `domain_posts` failures are unconfirmed; the ignored count seen in source today is three (see Baseline step 4) and the baseline run fixes it.
- Docker availability on this host and in CI is unknown (RK-02); CI is marked temporarily disabled by comment only and its actual enablement is unverified (RK-09); no CI result is claimed.
- `pnpm exec tsc --noEmit` cleanliness is unknown [A]; there is no typecheck script. Effective pnpm version inside `apps/web` is unverified.
- Per-crate cargo commands and dependency versions (`fd-lock` or `fs4`, `tokio-util`, `sea-orm-migration` features, tokio `test-util`) are unverified until WU-01 (`cargo check -p domain_backup`).
- Raw-SQL triggers inside SeaORM migrations (D-08) and the NamespaceLock crate on Windows (RK-06) are unproven.
- Crash simulation by abandoning in-memory state is assumed adequate; one two-process test (TS-32) is the cross-check.
- Performance targets P-01..P-05, audit retention, and the 16 KiB body cap, page size 20/100 and fixture size remain owner-unconfirmed.
- Open Q-04, Q-06, Q-07, Q-10, Q-11 mean no claim of production, consistency, encryption or recovery readiness; passing synthetic tests does not enable real staging.
- E2E environment (E2E-ENV-01) is undecided [Q]; TS-41 (UI part) depends on it. TS-55 (WU-10) depends on owner values for CFG-17 and CFG-18 (no approved default); the F6 gateway layer ordering is a design proposal, unbuilt and untested [D]. TS-56, TS-57, TS-59 and TS-64 test designs that are proposals pending the plan gate [D].
- Windows owner-only ACL verification is not specified for U1: T-14 is partially evidenced on this host until the owner accepts the limitation or an ACL test is added (TS-02 definition); this is a WU-04 hard stop.
- WU-02 "beside posts and seo" is conditional on RK-15 and unverified; the deadline test's `advance_both` helper and tokio `test-util` are unverified until WU-01/WU-05.
- The pre-existing `domain_auth` reflection of the token `kid` into its 401 body is reported from review evidence and not independently re-verified here [Q].
- Manual assistive-technology review (TS-37) and the e2e/performance evidence are not yet planned beyond the procedure here.

## Uncertainty and limits

This is a plan, not evidence. All commands are proposals pending verification at baseline. Provenance labels for relayed research [O] mean the conductor relayed them; files should be opened before heavy reliance. Approvals recorded are only the [UP] items (attributed to the project owner in solo scope, C13); mechanism designs (audit-in-transaction, NamespaceLock), the RED-GREEN-REFACTOR rule, the Docker `#[ignore]` convention and the TS additions in this revision are [D] proposals for the plan gate.

## Spine discrepancies

- Revision 2 re-read `acceptance-criteria.md`. The earlier AC labels were wrong in several places and are corrected: AC-001 S2 is unauthorized instance listing; AC-006 S2 is unresolved U1 design blockers; AC-006 has only S1 and S2 (no S3); AC-003 S1/S2 and AC-004 S3 are explicitly deferred.
- The spine traces config, bounds, cleanup and sweep tests to "AC-006 S2". This strategy traces them to the Q-05 owner decision, ADR-004 and T-13 and also to the behavioural AC-002 S2 and AC-003 S3 (C12); TS-58 covers AC-006 S2.
- Spine TS ids that sit in a unit before the code they exercise (TS-05, TS-12, TS-14, TS-27) are split by layer with a/b ids; spine IDs are not renamed.
- Resolved in revision 4 and no longer discrepancies: the WU-05 TS ids (TS-05a, TS-47, TS-48, TS-67, TS-68), the TS-59..TS-65 collision with interfaces-and-contracts.md (renumbered, C1), the status enum narrowing (not narrowed, C3), the CFG-16 gap (in scope, C7), and the T-13 designs (C10).
