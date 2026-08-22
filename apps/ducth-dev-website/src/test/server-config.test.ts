import { describe, expect, it } from 'vitest';
import { requiredServerUrl } from '../../server-config.mjs';

describe('server-only SEO endpoint configuration', () => {
  it('requires a valid endpoint and does not echo its value in errors', () => {
    expect(() => requiredServerUrl('WEBSITE_SEO_HEAD_ASSETS_API_URL', undefined)).toThrow('WEBSITE_SEO_HEAD_ASSETS_API_URL is required');
    expect(() => requiredServerUrl('WEBSITE_SEO_HEAD_ASSETS_API_URL', 'not a url')).toThrow('WEBSITE_SEO_HEAD_ASSETS_API_URL is invalid');
    expect(() => requiredServerUrl('WEBSITE_SEO_HEAD_ASSETS_API_URL', 'not a url')).not.toThrow(/not a url/);
    expect(requiredServerUrl('WEBSITE_SEO_HEAD_ASSETS_API_URL', ' https://api.test/assets ')).toBe('https://api.test/assets');
  });
});
