# WU-09 known limitations

- **AC-004 S3 (restore failure)** is deferred by design: restore execution is not authorized in U1 (test strategy). IF-08 always returns 403 `CAPABILITY_DISABLED`.
- **Previews are canned:** IF-09/IF-10 return compiled-in results for three synthetic fixtures; nothing is uploaded, read or restored. Labels state "Synthetic preview only".
- **Denial audit when disabled:** with backup disabled there is no configured instance, so no `capability.denied` / `download.denied` row is written (audit schema requires `instance_id`). Denials are still returned and counted in metrics.
- **Audit attempt cap:** a denial waits at most 2 s for the audit write; a slower or failed write is dropped silently by design (attempt-only, IFC 6.3).
- **Gateway composition:** routes are verified at the crate router; composed-gateway ordering (431/401/408 layers, CFG-17/CFG-18) is WU-10.
- **Docker test reliability (pre-existing harness issue):** at default parallelism (~213 concurrent Postgres testcontainers) one random test intermittently fails at container start (`PortNotExposed`, or "postgres never became ready" with an unexpected SSL response). Two such runs are kept as evidence; the same tree passes 213/213 with `--test-threads=4`. Not caused by WU-09; candidate follow-up: retry or limit concurrency in `test_db.rs`.
- **Rollback:** source and tests only.
