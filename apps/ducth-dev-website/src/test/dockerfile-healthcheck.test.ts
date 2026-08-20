// @vitest-environment node

import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const dockerfilePath = fileURLToPath(
  new URL('../../Dockerfile', import.meta.url),
);

describe('website runner healthcheck', () => {
  it('uses the shallow endpoint without changing probe behavior', async () => {
    const dockerfile = await readFile(dockerfilePath, 'utf8');

    expect(dockerfile).toContain(
      'HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD wget --no-verbose --tries=1 --spider http://localhost:3001/healthz || exit 1',
    );
    expect(dockerfile).not.toContain('http://localhost:3001/en');
  });
});
