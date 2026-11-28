// WU-15 / TS-17 (full): the bundle scanner finds a canary in a build output directory and
// stays silent on a clean one. The positive controls prove the scanner can fail; the full
// production-build scan is `pnpm scan:bundle-canary` (see scripts/bundle-canary-scan.mjs).
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { BUILD_CANARIES, scanDirectory } from './bundle-canary-scan.mjs';

const dirs = [];
function makeDist(files) {
  const dir = mkdtempSync(join(tmpdir(), 'canary-scan-'));
  dirs.push(dir);
  for (const [name, content] of Object.entries(files)) {
    const target = join(dir, name);
    mkdirSync(dirname(target), { recursive: true });
    writeFileSync(target, content);
  }
  return dir;
}
afterEach(() => {
  while (dirs.length) rmSync(dirs.pop(), { recursive: true, force: true });
});

describe('TS-17 bundle canary scanner', () => {
  it('declares canaries for BACKUP_* and VITE_/PUBLIC_-prefixed build variables', () => {
    const names = Object.keys(BUILD_CANARIES);
    expect(names.some((n) => n.startsWith('BACKUP_'))).toBe(true);
    expect(names.some((n) => n.startsWith('VITE_'))).toBe(true);
    expect(names.some((n) => n.startsWith('PUBLIC_'))).toBe(true);
    for (const value of Object.values(BUILD_CANARIES)) {
      expect(value).toMatch(/^CANARY-/);
    }
  });

  it('positive control: reports a canary placed in a js asset', async () => {
    const canary = Object.values(BUILD_CANARIES)[0];
    const dist = makeDist({
      'index.html': '<html></html>',
      'static/js/main.abc.js': `var leaked="${canary}";`,
    });
    const hits = await scanDirectory(dist, Object.values(BUILD_CANARIES));
    expect(hits).toEqual([
      expect.objectContaining({ canary, file: join('static', 'js', 'main.abc.js') }),
    ]);
  });

  it('positive control: reports a canary in html, css, json and map files', async () => {
    const [a, b, c, d] = Object.values(BUILD_CANARIES);
    const dist = makeDist({
      'index.html': a,
      'x.css': `/* ${b} */`,
      'config.json': `{"k":"${c}"}`,
      'static/js/main.js.map': d,
    });
    const hits = await scanDirectory(dist, Object.values(BUILD_CANARIES));
    expect(hits.map((h) => h.canary).sort()).toEqual([a, b, c, d].sort());
  });

  it('negative control: a clean output yields no hits', async () => {
    const dist = makeDist({
      'index.html': '<html>My-CMS Admin</html>',
      'static/js/main.js': 'console.log("hello")',
    });
    expect(await scanDirectory(dist, Object.values(BUILD_CANARIES))).toEqual([]);
  });
});
