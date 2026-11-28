# WU-14 known limitations

- Browser verification is blocked by E2E-ENV-01 (cross-origin UI to API with no CORS); only jsdom component tests exist. Manual assistive-technology review (TS-37) is not done and not claimed; the automated part covers text status, hidden icons, labels, keyboard operation and polite live regions.
- The react-hooks ESLint rules were not run (pre-existing `pnpm lint` config failure); lint was run with a temporary config without them. The hooks set state only from async callbacks, not synchronously in effects.
- Jobs list shows the first page only (IF-03 default limit); `next_cursor` is not followed. Not in the unit's goal.
- The status banner loads on mount and after job creation; it does not poll (IFC 3.2 defines no status polling).
- A request error retries indefinitely with backoff (ceiling 30 s); the contract does not cap it.
- Date strings are shown as the raw RFC 3339 values (no locale formatting).
- Confirm in the preview panel is offered only for a compatible preflight result; the API itself does not enforce this for IF-10 (stateless).
- Rust serializer match for IF-09/IF-10 checked by reading apps/api/domain_backup/src/handlers/preview/mod.rs; no cross-language contract test exists.
- tsc: 23 pre-existing errors in untouched files remain.
