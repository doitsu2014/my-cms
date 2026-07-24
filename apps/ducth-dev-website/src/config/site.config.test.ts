// @vitest-environment node
import { describe, it, expect, afterEach } from 'vitest';
import { SITE_CONFIG } from './site.config';
import type { RuntimeConfig } from './runtime-config';
import { setRuntimeConfigForServer } from './get-runtime-config';

const SAMPLE_CONFIG: RuntimeConfig = {
  siteName: 'Test Site',
  siteUrl: 'https://test.test',
  avatarUrl: 'https://test.test/a.png',
  defaultTitle: 'Test Title',
  defaultDescription: 'Test Description',
  defaultLocale: 'en',
  graphqlApiUrl: 'https://api.test/graphql',
  graphqlCacheApiUrl: 'https://api.test/graphql',
  mediaBaseUrl: 'https://api.test/media',
  port: '3001',
};

describe('SITE_CONFIG — server (no document)', () => {
  afterEach(() => {
    deleteServerConfig();
  });

  it('avatarUrl does not throw and returns the server-injected value', () => {
    setRuntimeConfigForServer(SAMPLE_CONFIG);
    expect(() => SITE_CONFIG.avatarUrl).not.toThrow();
    expect(SITE_CONFIG.avatarUrl).toBe(SAMPLE_CONFIG.avatarUrl);
  });

  it('siteName, siteUrl, defaultTitle, defaultDescription return server-injected values', () => {
    setRuntimeConfigForServer(SAMPLE_CONFIG);
    expect(SITE_CONFIG.siteName).toBe(SAMPLE_CONFIG.siteName);
    expect(SITE_CONFIG.siteUrl).toBe(SAMPLE_CONFIG.siteUrl);
    expect(SITE_CONFIG.seo.defaultTitle).toBe(SAMPLE_CONFIG.defaultTitle);
    expect(SITE_CONFIG.seo.defaultDescription).toBe(SAMPLE_CONFIG.defaultDescription);
  });

  it('socialLinks is a static object (no document access)', () => {
    expect(SITE_CONFIG.socialLinks).toEqual({
      github: 'https://github.com',
      twitter: 'https://twitter.com',
      linkedin: 'https://linkedin.com',
    });
  });
});

function deleteServerConfig(): void {
  delete (globalThis as { __WEBSITE_RUNTIME_CONFIG__?: RuntimeConfig }).__WEBSITE_RUNTIME_CONFIG__;
}
