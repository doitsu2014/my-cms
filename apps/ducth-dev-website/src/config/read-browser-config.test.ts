import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { readBrowserConfig } from './read-browser-config';
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

describe('readBrowserConfig', () => {
  let originalGetElementById: typeof document.getElementById;

  beforeEach(() => {
    // Reset the document between tests
    document.head.innerHTML = '';
    document.body.innerHTML = '';
    originalGetElementById = document.getElementById.bind(document);
  });

  afterEach(() => {
    document.head.innerHTML = '';
    document.body.innerHTML = '';
    document.getElementById = originalGetElementById;
  });

  it('(a) returns the parsed object when the script tag is present', () => {
    const script = document.createElement('script');
    script.id = 'app-config';
    script.type = 'application/json';
    script.textContent = JSON.stringify(SAMPLE_CONFIG);
    document.head.appendChild(script);

    expect(readBrowserConfig()).toEqual(SAMPLE_CONFIG);
  });

  it('(b) throws a clear error containing "app-config" when the script tag is absent', () => {
    expect(() => readBrowserConfig()).toThrow(/app-config/);
  });

  it('(c) throws the underlying SyntaxError when the script tag contains malformed JSON', () => {
    const script = document.createElement('script');
    script.id = 'app-config';
    script.type = 'application/json';
    script.textContent = '{ this is not valid JSON';
    document.head.appendChild(script);

    expect(() => readBrowserConfig()).toThrow(SyntaxError);
  });
});
