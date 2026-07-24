// @vitest-environment node
import { describe, it, expect, afterEach } from 'vitest';
import { getRuntimeConfig, setRuntimeConfigForServer } from './get-runtime-config';
import type { RuntimeConfig } from './runtime-config';

const SAMPLE_CONFIG: RuntimeConfig = {
  siteName: 'Test',
  siteUrl: 'https://test.test',
  avatarUrl: 'https://test.test/a.png',
  defaultTitle: 'Test',
  defaultDescription: 'Test',
  defaultLocale: 'en',
  graphqlApiUrl: 'https://api.test/graphql',
  graphqlCacheApiUrl: 'https://api.test/graphql',
  mediaBaseUrl: 'https://api.test/media',
  port: '3001',
};

describe('getRuntimeConfig — server (no document)', () => {
  afterEach(() => {
    deleteRuntimeConfigForServer();
  });

  it('returns the injected server runtime config when present', () => {
    setRuntimeConfigForServer(SAMPLE_CONFIG);
    expect(getRuntimeConfig()).toEqual(SAMPLE_CONFIG);
  });

  it('throws a clear error when called on the server without an injected config', () => {
    expect(() => getRuntimeConfig()).toThrow(/runtime config/i);
  });

  it('does not touch `document` while resolving the server config', () => {
    setRuntimeConfigForServer(SAMPLE_CONFIG);
    expect(typeof (globalThis as { document?: unknown }).document).toBe('undefined');
    expect(() => getRuntimeConfig()).not.toThrow();
  });
});

function deleteRuntimeConfigForServer(): void {
  delete (globalThis as { __WEBSITE_RUNTIME_CONFIG__?: RuntimeConfig }).__WEBSITE_RUNTIME_CONFIG__;
}
