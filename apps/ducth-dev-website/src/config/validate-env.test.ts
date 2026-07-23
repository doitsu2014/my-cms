import { describe, it, expect } from 'vitest';
import {
  parseRequiredEnv,
  parseRequiredString,
  resolveRuntimeConfig,
} from './validate-env';

describe('parseRequiredEnv', () => {
  it('(a) throws with the variable name when the value is missing', () => {
    expect(() => parseRequiredEnv('WEBSITE_PUBLIC_GRAPHQL_API_URL', undefined)).toThrow(
      /WEBSITE_PUBLIC_GRAPHQL_API_URL/
    );
  });

  it('(a) throws with the variable name when the value is an empty string', () => {
    expect(() => parseRequiredEnv('WEBSITE_PUBLIC_GRAPHQL_API_URL', '')).toThrow(
      /WEBSITE_PUBLIC_GRAPHQL_API_URL/
    );
  });

  it('(b) throws with the variable name and the first 64 chars of the value when the URL is malformed', () => {
    const malformed = 'not-a-url';
    expect(() => parseRequiredEnv('WEBSITE_PUBLIC_GRAPHQL_API_URL', malformed)).toThrow(
      /WEBSITE_PUBLIC_GRAPHQL_API_URL/
    );
    expect(() => parseRequiredEnv('WEBSITE_PUBLIC_GRAPHQL_API_URL', malformed)).toThrow(
      /not-a-url/
    );
  });

  it('(b) truncates the offending value to the first 64 characters in the error message', () => {
    // Use a malformed URL so the URL parse fails
    const longValue = 'not-a-url-' + 'x'.repeat(200);
    let message = '';
    try {
      parseRequiredEnv('WEBSITE_PUBLIC_MEDIA_BASE_URL', longValue);
    } catch (e) {
      message = (e as Error).message;
    }
    expect(message).toContain('WEBSITE_PUBLIC_MEDIA_BASE_URL');
    // The first 64 chars of the value should appear, but the 65th should not
    expect(message).toContain(longValue.slice(0, 64));
    expect(message).not.toContain(longValue.slice(0, 65));
  });

  it('(c) returns the trimmed value when the URL is valid', () => {
    expect(parseRequiredEnv('WEBSITE_PUBLIC_GRAPHQL_API_URL', '  https://example.test/graphql  '))
      .toBe('https://example.test/graphql');
  });
});

describe('parseRequiredString', () => {
  it('throws with the variable name when the value is missing', () => {
    expect(() => parseRequiredString('WEBSITE_SITE_NAME', undefined)).toThrow(
      /WEBSITE_SITE_NAME/,
    );
  });

  it('throws with the variable name when the value is empty', () => {
    expect(() => parseRequiredString('WEBSITE_SITE_NAME', '')).toThrow(/WEBSITE_SITE_NAME/);
  });

  it('returns the trimmed value when the string is non-empty', () => {
    expect(parseRequiredString('WEBSITE_SITE_NAME', '  My Site  ')).toBe('My Site');
  });
});

describe('resolveRuntimeConfig', () => {
  const validEnv: NodeJS.ProcessEnv = {
    WEBSITE_PUBLIC_GRAPHQL_API_URL: 'https://api.example.test/graphql',
    WEBSITE_PUBLIC_MEDIA_BASE_URL: 'https://api.example.test/media',
    WEBSITE_SITE_URL: 'https://example.test',
    WEBSITE_SITE_NAME: 'Example',
    WEBSITE_DEFAULT_TITLE: 'Title',
    WEBSITE_DEFAULT_DESCRIPTION: 'Description',
  };

  it('(d) falls back to WEBSITE_PUBLIC_GRAPHQL_API_URL when WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL is unset', () => {
    const cfg = resolveRuntimeConfig(validEnv);
    expect(cfg.graphqlCacheApiUrl).toBe('https://api.example.test/graphql');
  });

  it('(d) uses the explicit WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL when set', () => {
    const cfg = resolveRuntimeConfig({
      ...validEnv,
      WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL: 'https://cache.example.test/graphql',
    });
    expect(cfg.graphqlCacheApiUrl).toBe('https://cache.example.test/graphql');
  });

  it('(e) returns undefined for avatarUrl when WEBSITE_AVATAR_URL is unset', () => {
    const cfg = resolveRuntimeConfig(validEnv);
    expect(cfg.avatarUrl).toBeUndefined();
  });

  it('(e) returns the avatarUrl when WEBSITE_AVATAR_URL is set', () => {
    const cfg = resolveRuntimeConfig({
      ...validEnv,
      WEBSITE_AVATAR_URL: 'https://cdn.example.test/avatar.png',
    });
    expect(cfg.avatarUrl).toBe('https://cdn.example.test/avatar.png');
  });

  it('(f) falls back to "3001" when WEBSITE_PORT is unset', () => {
    const cfg = resolveRuntimeConfig(validEnv);
    expect(cfg.port).toBe('3001');
  });

  it('(g) falls back to "en" when WEBSITE_DEFAULT_LOCALE is unset', () => {
    const cfg = resolveRuntimeConfig(validEnv);
    expect(cfg.defaultLocale).toBe('en');
  });

  it('resolves all required values into the RuntimeConfig', () => {
    const cfg = resolveRuntimeConfig(validEnv);
    expect(cfg.siteName).toBe('Example');
    expect(cfg.siteUrl).toBe('https://example.test');
    expect(cfg.defaultTitle).toBe('Title');
    expect(cfg.defaultDescription).toBe('Description');
    expect(cfg.graphqlApiUrl).toBe('https://api.example.test/graphql');
    expect(cfg.mediaBaseUrl).toBe('https://api.example.test/media');
  });

  it('throws when a required value is missing', () => {
    expect(() => resolveRuntimeConfig({ ...validEnv, WEBSITE_PUBLIC_GRAPHQL_API_URL: undefined }))
      .toThrow(/WEBSITE_PUBLIC_GRAPHQL_API_URL/);
  });

  it('throws when a required value is malformed', () => {
    expect(() =>
      resolveRuntimeConfig({ ...validEnv, WEBSITE_PUBLIC_GRAPHQL_API_URL: 'not-a-url' })
    ).toThrow(/WEBSITE_PUBLIC_GRAPHQL_API_URL/);
  });
});

describe('resolveRuntimeConfig — exit-message contract', () => {
  const validEnv: NodeJS.ProcessEnv = {
    WEBSITE_PUBLIC_GRAPHQL_API_URL: 'https://api.example.test/graphql',
    WEBSITE_PUBLIC_MEDIA_BASE_URL: 'https://api.example.test/media',
    WEBSITE_SITE_URL: 'https://example.test',
    WEBSITE_SITE_NAME: 'Example',
    WEBSITE_DEFAULT_TITLE: 'Title',
    WEBSITE_DEFAULT_DESCRIPTION: 'Description',
  };

  it('emits the literal "WEBSITE_PUBLIC_GRAPHQL_API_URL is required" when the env is missing', () => {
    expect(() => resolveRuntimeConfig({ ...validEnv, WEBSITE_PUBLIC_GRAPHQL_API_URL: undefined }))
      .toThrow('WEBSITE_PUBLIC_GRAPHQL_API_URL is required');
  });

  it('includes both the variable name and "not-a-url" when the URL is malformed', () => {
    let message = '';
    try {
      resolveRuntimeConfig({ ...validEnv, WEBSITE_PUBLIC_GRAPHQL_API_URL: 'not-a-url' });
    } catch (e) {
      message = (e as Error).message;
    }
    expect(message).toContain('WEBSITE_PUBLIC_GRAPHQL_API_URL');
    expect(message).toContain('not-a-url');
  });
});
