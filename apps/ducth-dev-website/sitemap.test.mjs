import { describe, expect, it } from 'vitest';
import {
  buildSitemapEntries,
  createSitemapService,
  serializeSitemap,
} from './sitemap.mjs';

const siteUrl = 'https://example.test';

describe('website sitemap', () => {
  it('includes localized static, translated category, and published post routes', () => {
    const entries = buildSitemapEntries(siteUrl, {
      categories: {
        nodes: [
          {
            slug: 'engineering',
            categoryType: 'Blog',
            categoryTranslations: {
              nodes: [{ languageCode: 'vi', slug: 'ky-thuat' }],
            },
          },
          { slug: 'private', categoryType: 'Private' },
          { slug: 'missing-type' },
        ],
      },
      posts: {
        nodes: [
          {
            slug: 'published-post',
            published: true,
            lastModifiedAt: '2026-08-20T00:00:00Z',
          },
          { slug: 'draft-post', published: false },
          { slug: 'unknown-post', published: null },
        ],
      },
    });

    const locations = entries.map(({ loc }) => loc);
    expect(locations).toEqual([
      'https://example.test/en',
      'https://example.test/en/categories',
      'https://example.test/en/about',
      'https://example.test/vi',
      'https://example.test/vi/categories',
      'https://example.test/vi/about',
      'https://example.test/en/categories/engineering',
      'https://example.test/vi/categories/ky-thuat',
      'https://example.test/en/posts/published-post',
      'https://example.test/vi/posts/published-post',
    ]);
  });

  it('escapes XML values and strips site URL query/hash values', () => {
    const entries = buildSitemapEntries(
      'https://example.test/?tracking=1#top',
      {
        posts: { nodes: [{ slug: 'a&b', published: true }] },
      },
    );
    const xml = serializeSitemap(entries);

    expect(xml).toContain('<loc>https://example.test/en/posts/a%26b</loc>');
    expect(xml).not.toContain('tracking=1');
    expect(
      serializeSitemap([
        {
          loc: 'https://example.test/a&b?<x>',
          lastmod: '2026-08-20T00:00:00.000Z',
        },
      ]),
    ).toContain('https://example.test/a&amp;b?&lt;x&gt;');
    expect(xml).toMatch(/^<\?xml version="1\.0" encoding="UTF-8"\?>/);
  });

  it('falls back to localized static routes when the content API fails', async () => {
    const service = createSitemapService({
      siteUrl,
      graphqlApiUrl: 'https://api.example.test/graphql',
      fetchImpl: async () => {
        throw new Error('upstream unavailable');
      },
    });

    const xml = await service.getSitemapXml();

    expect(xml).toContain('<loc>https://example.test/en</loc>');
    expect(xml).toContain('<loc>https://example.test/vi/about</loc>');
    expect(xml).not.toContain('upstream unavailable');
  });

  it('ignores malformed content collections without exposing an upstream error', async () => {
    const service = createSitemapService({
      siteUrl,
      graphqlApiUrl: 'https://api.example.test/graphql',
      fetchImpl: async () =>
        new Response(
          JSON.stringify({ data: { categories: 'invalid', posts: null } }),
          { status: 200 },
        ),
    });

    const xml = await service.getSitemapXml();

    expect(xml).toContain('<loc>https://example.test/en/categories</loc>');
    expect(xml).not.toContain('/posts/');
  });

  it('adds valid upstream content and caches the generated XML', async () => {
    let calls = 0;
    const service = createSitemapService({
      siteUrl,
      graphqlApiUrl: 'https://api.example.test/graphql',
      fetchImpl: async () => {
        calls += 1;
        return new Response(
          JSON.stringify({
            data: {
              categories: { nodes: [] },
              posts: { nodes: [{ slug: 'cached-post', published: true }] },
            },
          }),
          { status: 200, headers: { 'content-type': 'application/json' } },
        );
      },
    });

    const first = await service.getSitemapXml();
    const second = await service.getSitemapXml();

    expect(first).toBe(second);
    expect(first).toContain('/en/posts/cached-post');
    expect(calls).toBe(1);
  });
});
