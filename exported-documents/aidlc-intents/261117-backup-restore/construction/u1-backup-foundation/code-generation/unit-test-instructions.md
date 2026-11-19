# U1 Unit Test Instructions — u1-backup-foundation

## Scope and framework

- Backend package is Cargo package `gateway` (binary `my-cms-api`), under workspace manifest `apps/api/Cargo.toml`.
- Backend tests use Rust's built-in `#[cfg(test)]`/`cargo test` conventions. Existing tests are inline modules; PostgreSQL integration tests may be explicitly ignored when Docker is required.
- Admin UI is React/TypeScript under `apps/web`, using Vitest + Testing Library; `apps/web/package.json` defines `test: "vitest run"`.
- Do not run tests against real Supabase data, AWS credentials, or production endpoints. Use synthetic records and mocks/fakes; database integration must use a disposable local/test database and only when approved/prerequisites are present.

## Exact scoped commands

Run from the workspace root:

```bash
cargo test --manifest-path apps/api/Cargo.toml -p gateway
pnpm --dir apps/web exec vitest run src/app/admin/backup-restore
```

The Rust command is package-scoped to the existing `gateway` Cargo package; this is not a strict test-name filter, since no U1 backup tests exist yet. Keep new Rust tests named/grouped under the backup module and report that this command runs the gateway crate suite. The Vitest path filter must target U1's actual test files after they exist; if path filtering is unsupported, list exact test paths instead. Confirm both commands after creating planned test files and before use; if runner behavior differs, revise this instructions file and the plan before approval. Do not describe either command as testing code that has not yet been written.

## Test volume and cases

The Testing Contract is Standard: target 5–8 meaningful tests for each implemented component, plus integration tests for key boundaries. Apply this to the actual implementation components, not merely a list of conceptual boundaries; when a concern is an interface within one component, distribute the test cases under that component rather than claiming a separately deployed component. Expected components include the gateway backup API/module, persistence, worker/orchestrator, staging/archive adapter and UI where implemented. Tests should not count assertion variations as separate tests. Every public behavior has a happy path and at least two negative/edge cases. Include key API→durable job→worker→incomplete outcome integration coverage.

### API and authorization boundary

- Admin can list configured instance IDs/labels; credentials/endpoints are absent.
- Unauthenticated and non-admin calls are denied before data access.
- Unknown request field / client-injected URL or secret is rejected.
- Job status lookup checks resource authorization; guessed foreign ID returns safe denial.
- Persistence failure does not return accepted job.

### Configuration resolver

- Known enabled configured ID resolves server-side source handle.
- Unknown/disabled ID is rejected without opening a DB connection.
- Client-provided endpoint/config override is ignored or rejected.
- Missing TLS/security policy disables real-data mode.
- Missing/invalid finite byte/time/temp/expiry values refuse before first staging write.

### Persistence and idempotency

- Durable job stores actor/source/request digest and returns ID after commit.
- Same actor/operation/key/digest returns same job.
- Same key with different request digest returns conflict.
- Composite uniqueness is scoped by actor + operation + key digest; collisions across actors/operations do not cross-authorize or reject unrelated work.
- Stale state version cannot overwrite newer status; DB error maps to safe code.

### Worker and lifecycle

- Valid job transitions queued→running→incomplete when required Auth/Storage is absent.
- Missing staging policy fails before writing bytes.
- Timeout/unknown side effect remains non-success and is never blindly retried.
- Duplicate worker claim/state transition is rejected.
- Restart reconciliation preserves unknown state; it never infers success from file presence.
- Cleanup/reconciliation identifies orphaned/expired synthetic artifacts; delete failure is bounded, observable and blocks real-data writes.

### Database/staging/archive boundary

- Synthetic database fixture stages only under enabled private/TLS/at-rest/bounds policy.
- Digest and byte count match the staged bytes; digest does not imply completeness.
- Path traversal, absolute path, symlink/conflict and size/entry/depth violations are rejected before writes/extraction.
- Artifact state remains incomplete and no API route returns a download URL.
- Failure between object write and metadata commit is discoverable/reconciled.

### Audit and error projection

- Request/started/incomplete/failed transitions record actor/source/job/time/outcome and digest where available.
- Audit failure before side effect denies execution.
- Safe errors contain stable code/correlation ID but no raw upstream payload.
- Secret canary, raw row and staged content do not appear in response/log/audit/metric labels.

### Admin UI

- Configured instance list shows safe labels; no-instance state disables create action.
- Loading and error states are distinct and actionable.
- Create action sends configured ID and idempotency token; duplicate click does not create duplicate UI action.
- Queued/running job renders status; progress is not fabricated when unknown.
- Incomplete/failed state clearly warns no complete backup exists and never offers download.
- Keyboard interaction and accessible status text are verified; statuses are not color-only.

## Integration coverage

At least one focused API→durable job→worker→database staging→digest→incomplete outcome test must prove the end-to-end U1 skeleton. Include tests proving missing Auth/Storage keeps job non-success and non-downloadable, plus failure injection for persistence, timeout and cleanup. If Docker is unavailable, mark the Docker-backed PostgreSQL test not run with its prerequisite; retain runnable unit/handler tests and do not claim integration or coverage evidence that was not run.

## Coverage target

The project/team target is 80% line coverage, and Build and Test evaluates it on the full assembled suite. The existing `.github/workflows/ci-my-cms.yml` declares grcov/coverage but is marked temporarily disabled. Do not claim CI-green or waive the 80% target. U1-scoped test command may be used for the checkpoint; record coverage evidence separately if the local coverage runner is verified, and report the target as unverified/blocked if no active local/CI mechanism can measure it. Never alter the existing threshold to make U1 tests pass.
