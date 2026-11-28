# KL-11 Known limitations: WU-11

- **ID:** KL-11 (WU-11)
- **Stage:** Develop / build-and-verify, unit WU-11.

1. **No red run.** The unit adds characterization tests for behavior that already exists, so none failed first. The mutation checks in the green report show the tests can fail; they are not a substitute for a red run on new behavior.
2. **Exit codes only.** `handle_args` reports through `eprintln!`, so the tests assert the exit code and not the message text. Asserting the text would need an output seam, which is a behavior-adjacent change outside this unit.
3. **Listener check is local.** The no-contact test proves no connection reaches the address the CLI could read from the environment. It cannot rule out a hard-coded address; the source has none today.
4. **Environment is global.** The tests serialize on a static mutex and restore variables on drop, but another test in the same binary that reads `AWS_*`, `BACKUP_*` or `DATABASE_URL` without that lock could interleave. None exists today.
5. **Pre-existing warnings.** Dead-code warnings for the manifest helpers (`Manifest`, `ObjectEntry`, `safe_relative_path`, `digest`, `now_seconds`, `FORMAT_VERSION`) are not new and were left alone because the unit forbids behavior or structure changes.
6. **Doc reconciliation scope.** Only comments in `backup_cli.rs` were changed, as the unit's file list states. `docs/features/backup-restore.md` was not edited.
7. **Platform.** Verified on Windows only.
