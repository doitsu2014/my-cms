// @vitest-environment node
import { describe, it, expect, afterEach } from 'vitest';
import { getMediaBaseUrl, getGraphqlApiUrl } from './api.config';
import type { RuntimeConfig } from './runtime-config';
import { setRuntimeConfigForServer } from './get-runtime-config';

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

describe('api.config — server (no document)', () => {
  afterEach(() => {
    deleteServerConfig();
  });

  it('getMediaBaseUrl returns the server-injected media base url', () => {
    setRuntimeConfigForServer(SAMPLE_CONFIG);
    expect(() => getMediaBaseUrl()).not.toThrow();
    expect(getMediaBaseUrl()).toBe(SAMPLE_CONFIG.mediaBaseUrl);
  });

  it('getGraphqlApiUrl returns the server-injected GraphQL url', () => {
    setRuntimeConfigForServer(SAMPLE_CONFIG);
    expect(() => getGraphqlApiUrl()).not.toThrow();
    expect(getGraphqlApiUrl()).toBe(SAMPLE_CONFIG.graphqlApiUrl);
  });
});

function deleteServerConfig(): void {
  delete (globalThis as { __WEBSITE_RUNTIME_CONFIG__?: RuntimeConfig }).__WEBSITE_RUNTIME_CONFIG__;
}
