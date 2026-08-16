import { describe, expect, it } from 'vitest';
import { replacePublicHead, serializePublicHead } from './public-head';
import {
  resolvePublicMetadata,
  toPlainDescription,
  type PublicSiteConfig,
} from './public-metadata';

const config: PublicSiteConfig = {
  siteName: 'Test Site',
  siteUrl: 'https://example.test/',
  defaultTitle: 'Writing index',
  defaultDescription: 'Notes about software and systems.',
  mediaBaseUrl: 'https://media.example.test',
};

const post = {
  id: 'post-1',
  title: 'Fallback title',
  slug: 'fallback-title',
  previewContent: 'Fallback preview',
  content: '<p>Article body</p>',
  createdAt: '2025-01-01',
  published: true,
  thumbnailPaths: ['images/post.png'],
  postTranslations: {
    nodes: [
      {
        languageCode: 'vi',
        title: 'Tiêu đề & <b>đậm</b>',
        previewContent: '<p>Mô tả tiếng Việt</p>',
      },
    ],
  },
};

describe('resolvePublicMetadata', () => {
  it('resolves localized static routes and strips query/hash from canonical URLs', () => {
    const profile = resolvePublicMetadata({
      url: '/vi/about?preview=true#top',
      config,
    });
    expect(profile.lang).toBe('vi');
    expect(profile.title).toBe('Về tôi — Test Site');
    expect(profile.canonicalUrl).toBe('https://example.test/vi/about');
    expect(profile.robots).toBe('index, follow');
  });

  it('uses localized article content and an eligible thumbnail', () => {
    const profile = resolvePublicMetadata({
      url: '/vi/posts/fallback-title',
      config,
      post,
    });
    expect(profile.title).toBe('Tiêu đề & <b>đậm</b> — Test Site');
    expect(profile.description).toBe('Mô tả tiếng Việt');
    expect(profile.openGraph.type).toBe('article');
    expect(profile.openGraph.image).toBe(
      'https://media.example.test/images/post.png',
    );
  });

  it('returns a private fallback for missing or unpublished content', () => {
    const profile = resolvePublicMetadata({
      url: '/en/posts/unknown?x=1',
      config,
      post: null,
    });
    expect(profile.title).toBe('Writing index — Test Site');
    expect(profile.robots).toBe('noindex, nofollow');
    expect(profile.canonicalUrl).toBe('https://example.test/en');
    expect(
      resolvePublicMetadata({
        url: '/en/posts/fallback-title',
        config,
        post: { ...post, published: false },
      }).robots,
    ).toBe('noindex, nofollow');
  });

  it('bounds descriptions and never uses raw HTML', () => {
    const description = toPlainDescription(
      '<p>Hello <strong>world</strong></p>' + ' x'.repeat(200),
      'fallback',
    );
    expect(description).toMatch(/^Hello world/);
    expect(description).not.toContain('<strong>');
    expect(description.length).toBeLessThanOrEqual(160);
  });
});

describe('public head adapter', () => {
  it('escapes SSR values and emits one managed tag per field', () => {
    const profile = resolvePublicMetadata({
      url: '/vi/posts/fallback-title',
      config,
      post,
    });
    const html = serializePublicHead(profile);
    expect(html).toContain('Tiêu đề &amp; &lt;b&gt;đậm&lt;/b&gt;');
    expect(html.match(/data-public-metadata/g)?.length).toBeGreaterThan(1);
    expect(html.match(/rel="canonical"/g)).toHaveLength(1);
    expect(html.match(/name="description"/g)).toHaveLength(1);
  });

  it('replaces managed browser tags without duplicates', () => {
    document.head.innerHTML =
      '<title>old</title><meta name="description" content="old">';
    const profile = resolvePublicMetadata({ url: '/en', config });
    replacePublicHead(profile);
    replacePublicHead(profile);
    expect(document.title).toBe('Writing index — Test Site');
    expect(
      document.head.querySelectorAll('[data-public-metadata]').length,
    ).toBe(12);
    expect(document.head.querySelectorAll('link[rel="canonical"]').length).toBe(
      1,
    );
    expect(
      document.head.querySelectorAll('meta[name="description"]').length,
    ).toBe(1);
  });
});
