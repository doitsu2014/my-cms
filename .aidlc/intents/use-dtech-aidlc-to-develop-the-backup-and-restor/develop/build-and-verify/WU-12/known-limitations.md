# WU-12 known limitations

- `pnpm lint` fails in apps/web at ESLint config load ("Flat config requires plugins to be an object", react-hooks recommended-latest), exit 2, identically on the base commit before any change. Not caused by this unit. The touched files were linted with the repo config minus the react-hooks entry (temporary config, removed) and report no issues.
- `pnpm exec tsc --noEmit` reports 23 errors on the base commit and 23 after, with the same set. None are in files touched by this unit.
- No browser or e2e verification. E2E-ENV-01 (CORS or proxy decision) is open and Idempotency-Key preflight is unresolved. The client is verified only against mocked fetch and the documented contract.
- Out of scope, left to later units: idempotency-key generation and double-submit guard (WU-13), polling and backoff constants (WU-14), status rendering and the banner (WU-14), IF-05..IF-10 (download denied, previews; WU-14).
- IF-01 `environment` is typed as a free string, not an enum, because the contract only shows "local" and a later environment must not break parsing.
- `getBackupApiUrl` has no dedicated test; it is exercised through the URL assertions in backup.test.ts.- The client was checked against the committed WU-08 serializers; later serializer changes (for example WU-10) must re-run `backup.test.ts` contract cases.
