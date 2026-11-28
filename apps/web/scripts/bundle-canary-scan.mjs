// WU-15 / TS-17 (full): build the admin app with canary values in the build environment and
// scan the output for them (T-09, PR-02).
//
// Usage (from apps/web):  pnpm scan:bundle-canary
//
// Steps:
//   1. `pnpm build` with BACKUP_*, VITE_*, PUBLIC_* and server-secret style canaries set.
//   2. Scan `dist` for every canary value. Any hit fails the run.
//   3. Positive control A: copy `dist`, append a canary to one script, prove the scanner hits.
//   4. Positive control B: build a tiny project whose source deliberately reads a PUBLIC_*
//      build-time variable (the one mechanism by which rsbuild embeds env values) and prove
//      the scanner finds the canary in that build.
// A report is printed as JSON on the last line. The test file beside this script covers the
// scanner on its own (`pnpm test`); this CLI is the slow, whole-build evidence run.

import { spawnSync } from 'node:child_process';
import {
  cpSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

export const BUILD_CANARIES = {
  BACKUP_DATABASE_URL: 'CANARY-backup-dsn-0001',
  BACKUP_STORAGE_KEY: 'CANARY-backup-storage-key-0002',
  VITE_BACKUP_SECRET: 'CANARY-vite-backup-secret-0003',
  PUBLIC_BACKUP_SECRET: 'CANARY-public-backup-secret-0004',
  SUPABASE_SERVICE_ROLE_KEY: 'CANARY-service-role-key-0005',
  DATABASE_URL: 'CANARY-database-url-0006',
};

function* walk(dir) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) yield* walk(path);
    else if (entry.isFile()) yield path;
  }
}

/**
 * Scan every file below `dir` for each canary string.
 * @returns {Promise<{file: string, canary: string}[]>} one hit per file and canary found,
 * with `file` relative to `dir`.
 */
export async function scanDirectory(dir, canaries) {
  const hits = [];
  for (const path of walk(dir)) {
    const content = readFileSync(path);
    for (const canary of canaries) {
      if (content.includes(canary)) hits.push({ file: relative(dir, path), canary });
    }
  }
  return hits;
}

function run(command, args, options) {
  const result = spawnSync(command, args, { encoding: 'utf8', ...options });
  if (result.status !== 0) {
    throw new Error(
      `${command} ${args.join(' ')} failed (${result.status})\n${result.stdout}\n${result.stderr}`,
    );
  }
  return result;
}

async function main() {
  const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
  const canaries = Object.values(BUILD_CANARIES);
  const report = { canaries: canaries.length };

  // 1. Real production build with the canaries in the environment.
  rmSync(join(root, 'dist'), { recursive: true, force: true });
  const build = run('pnpm', ['build'], {
    cwd: root,
    env: { ...process.env, ...BUILD_CANARIES },
  });
  process.stdout.write(`${build.stdout.split('\n').slice(-12).join('\n')}\n`);
  const dist = join(root, 'dist');
  report.distFiles = [...walk(dist)].length;

  // 2. Scan the real build.
  const hits = await scanDirectory(dist, canaries);
  report.buildHits = hits;

  // 3. Positive control A: the scanner detects a canary injected into the real output.
  const copy = mkdtempSync(join(tmpdir(), 'canary-dist-'));
  cpSync(dist, copy, { recursive: true });
  const script = [...walk(copy)].find((file) => file.endsWith('.js'));
  writeFileSync(script, `${readFileSync(script, 'utf8')}\n;var __leak="${canaries[0]}";`);
  const injected = await scanDirectory(copy, canaries);
  report.controlInjectedDetected = injected.some((hit) => hit.canary === canaries[0]);
  rmSync(copy, { recursive: true, force: true });

  // 4. Positive control B: a deliberately leaking build-time value.
  const controlDir = join(root, 'node_modules', '.cache', 'canary-control');
  rmSync(controlDir, { recursive: true, force: true });
  mkdirSync(controlDir, { recursive: true });
  writeFileSync(
    join(controlDir, 'entry.js'),
    'console.log(process.env.PUBLIC_BACKUP_SECRET);\n',
  );
  const { createRsbuild, loadEnv } = await import('@rsbuild/core');
  process.env.PUBLIC_BACKUP_SECRET = BUILD_CANARIES.PUBLIC_BACKUP_SECRET;
  const { publicVars } = loadEnv({ cwd: root });
  const rsbuild = await createRsbuild({
    cwd: controlDir,
    rsbuildConfig: {
      source: { entry: { index: join(controlDir, 'entry.js') }, define: publicVars },
      output: { distPath: { root: join(controlDir, 'dist') } },
    },
  });
  await rsbuild.build();
  const leaked = await scanDirectory(join(controlDir, 'dist'), canaries);
  report.controlLeakDetected = leaked.some(
    (hit) => hit.canary === BUILD_CANARIES.PUBLIC_BACKUP_SECRET,
  );
  rmSync(controlDir, { recursive: true, force: true });

  const ok = hits.length === 0 && report.controlInjectedDetected && report.controlLeakDetected;
  report.result = ok ? 'PASS' : 'FAIL';
  console.log(JSON.stringify(report));
  process.exit(ok ? 0 : 1);
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  main().catch((error) => {
    console.error(error);
    process.exit(2);
  });
}
