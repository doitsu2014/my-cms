# WU-15 known limitations

1. **TS-41 UI part BLOCKED** by E2E-ENV-01 (cross-origin dev setup and CORS decision). Only the API part ran; WU-14 component tests cover "no download control". Gateway-boot runtime part remains the descoped RK-15 residual.
2. **TS-39 numbers are not a benchmark.** 30 samples, debug build, local Docker, tiny fixture; no p99; no target (P-01..P-05 open).
3. **TS-40 does not reach the budget edge.** The fixture reserves 4 KiB per allocation, so observed bytes (20 KiB live) are far below 256 MiB and 1 GiB; budget enforcement at the edge is TS-30 (WU-04). In the stress rounds only one create per round is accepted, because the creates beat the worker.
4. **Docker suite is flaky under load.** Two of three full ignored runs lost 1-2 unrelated tests to container start-up errors (`postgres never became ready`, `PortNotExposed`); the final run passed 219/219 and the failed tests pass alone. Same class as WU-10 item 10.
5. **Bundle scan limits.** It proves the canaries do not reach `dist` for this build configuration (rsbuild exposes only `PUBLIC_*`). It does not scan source maps from other configurations, container images or the nginx runtime config (`config.js` written at start-up).
6. **TS-58 is a text check.** It asserts the open items are listed and the synthetic labels exist; it cannot judge whether the owner agrees with the wording.
7. **Windows and CI.** TS-31 stays partial on Windows (T-14 accepted residual); the macOS run exercised the Unix permission tests. CI state (RK-09) is not observed and not claimed green. TS-34 upgrade beside posts and seo remains open.
8. **No mutation run** was done for the new tests in WU-15; TS-17 (scanner) and TS-58 were shown red first, the Docker measurement tests are characterization.
9. **Process incident (fixed).** First red and first green Docker runs leaked 6 `postgres:15-alpine` containers (`std::mem::forget` on the test DB handle). Removed, helper fixed, no leak afterwards.
10. **Docs claim only what was verified.** Section 14 of the feature doc describes U1 as of this commit; sections 1-8 stay the target specification.
