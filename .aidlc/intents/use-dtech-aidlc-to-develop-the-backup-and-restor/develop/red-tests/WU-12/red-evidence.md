# WU-12 red evidence

Command: `cd apps/web && pnpm exec vitest run src/domains/backup.test.ts`

State: behavioural red against a placeholder stub of apps/web/src/domains/backup.ts (planned exports and signatures; schemas are z.any(), client functions return {} without calling fetch, isTerminalBackupJobStatus returns false). Before this, apps/web baseline: 19 files, 49 tests passed.

Result: Test Files 1 failed, Tests 21 failed | 12 passed (33). The 12 that passed are acceptance-only checks that a permissive stub also satisfies; they turn into real checks once the stricter cases are implemented.

Verbatim excerpt:

```
 FAIL  src/domains/backup.test.ts > TS-52 client: status schema > maps an unknown status string to unknown instead of failing
AssertionError: expected 'archived' to be 'unknown' // Object.is equality
 FAIL  ... > status schema > rejects a status that is not a string
AssertionError: expected true to be false // Object.is equality
 FAIL  ... > status schema > classifies terminal statuses exhaustively and treats unknown as non-terminal
AssertionError: expected false to be true // Object.is equality
 FAIL  ... > job schema > fails closed when a job claims to be downloadable or not synthetic
AssertionError: expected true to be false // Object.is equality
 FAIL  ... > status object > rejects a disabled_reason outside the closed set and a missing one
AssertionError: expected true to be false // Object.is equality
 FAIL  ... > REST calls > lists instances with an authorised GET against the backup path
AssertionError: expected "vi.fn()" to be called 1 times, but got 0 times
 FAIL  ... > REST calls > reports an idempotent replay from the Idempotent-Replay header on a 200
AssertionError: expected undefined to be true // Object.is equality
 FAIL  ... > error handling > parses the crate error body into a typed BackupApiError
AssertionError: expected {} to be an instance of BackupApiError
 FAIL  ... > error handling > keeps BACKUP_DISABLED and BACKUP_NOT_READY 503 reasons
AssertionError: promise resolved "{}" instead of rejecting
 Test Files  1 failed (1)
      Tests  21 failed | 12 passed (33)
```

Why behavioural: the module compiles and imports; every failure is an assertion about missing behaviour (strict rejection, unknown-status mapping, URL, headers, body, replay flag, error projection). An earlier run showed a TypeError from the test helper destructuring a missing fetch call; the helper now asserts the call count first, so those cases fail on an assertion. Full raw output: `evidence/wu12-red.log`.
Environment: macOS Darwin 25.6.0, node v24.14.1, pnpm (apps/web), worker worktree based on `36361ae`.
