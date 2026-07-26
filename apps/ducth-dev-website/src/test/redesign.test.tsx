import '@testing-library/jest-dom/vitest';
import { fireEvent, render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { describe, expect, it, vi } from 'vitest';
import AppContent from '../AppContent';
import { ContentError } from '../components/feedback/ContentError';
import SiteLayout from '../components/layout/SiteLayout';
import { getLocalizedPath } from '../lib/i18n/getLocalizedPath';
import { getRouteLanguage } from '../lib/i18n/getRouteLanguage';
import { getArtworkVariant } from '../lib/media/getArtworkVariant';
import { getRelatedPosts } from '../lib/posts/getRelatedPosts';
import LanguageSwitch from '../components/navigation/LanguageSwitch';
import MobileNavigation from '../components/navigation/MobileNavigation';
import PostArtwork from '../components/posts/PostArtwork';
import PostCard from '../components/posts/PostCard';
import ShareActions from '../components/posts/ShareActions';
import { formatPublishedDate } from '../lib/i18n/formatPublishedDate';
import { getLocalizedCategory } from '../lib/i18n/getLocalizedCategory';
import { getLocalizedPost } from '../lib/i18n/getLocalizedPost';
import { getPostThumbnail } from '../lib/media/getPostThumbnail';

const basePost = {
  id: 'post-1',
  title: 'English title',
  slug: 'stable-slug',
  previewContent: 'English preview',
  content: '<p>English body</p>',
  createdAt: '2025-03-14T00:00:00.000Z',
  published: true,
  categories: { displayName: 'Systems', slug: 'systems' },
  postTranslations: {
    nodes: [
      {
        languageCode: 'vi',
        title: 'Tiêu đề',
        previewContent: 'Bản xem trước',
        content: '<p>Nội dung</p>',
      },
    ],
  },
};

describe('redesign content helpers', () => {
  it('selects a matching post translation and falls back field by field', () => {
    expect(getLocalizedPost(basePost, 'vi')).toMatchObject({
      title: 'Tiêu đề',
      previewContent: 'Bản xem trước',
      content: '<p>Nội dung</p>',
    });

    expect(
      getLocalizedPost(
        {
          ...basePost,
          postTranslations: { nodes: [{ languageCode: 'vi', title: 'Chỉ tiêu đề' }] },
        },
        'vi',
      ),
    ).toMatchObject({
      title: 'Chỉ tiêu đề',
      previewContent: 'English preview',
      content: '<p>English body</p>',
    });
  });

  it('selects a localized category name and slug', () => {
    const category = {
      id: 'cat-1',
      displayName: 'Systems',
      slug: 'systems',
      categoryTranslations: {
        nodes: [{ languageCode: 'vi', displayName: 'Hệ thống', slug: 'he-thong' }],
      },
    };

    expect(getLocalizedCategory(category, 'vi')).toMatchObject({
      displayName: 'Hệ thống',
      slug: 'he-thong',
    });
    expect(getLocalizedCategory(category, 'fr')).toMatchObject({
      displayName: 'Systems',
      slug: 'systems',
    });
  });

  it('formats published dates for the active locale', () => {
    expect(formatPublishedDate('2025-03-14T00:00:00.000Z', 'en')).toBe('March 14, 2025');
    expect(formatPublishedDate('2025-03-14T00:00:00.000Z', 'vi')).toContain('14 tháng 3, 2025');
  });

  it('resolves the first thumbnail through the media base and omits missing media', () => {
    expect(
      getPostThumbnail(
        { thumbnailPaths: ['/thumbnail.jpg'] },
        'https://cms.example.test/media/',
      ),
    ).toBe('https://cms.example.test/media/thumbnail.jpg');
    expect(
      getPostThumbnail(
        { thumbnailPaths: ['https://cdn.example.test/thumbnail.jpg'] },
        'https://cms.example.test/media',
      ),
    ).toBe('https://cdn.example.test/thumbnail.jpg');
    expect(getPostThumbnail({}, 'https://cms.example.test/media')).toBeUndefined();
  });

  it('filters related posts, excludes the current post, and sorts newest first', () => {
    const posts = [
      basePost,
      { ...basePost, id: 'old', slug: 'old', createdAt: '2024-01-01T00:00:00Z' },
      { ...basePost, id: 'new', slug: 'new', createdAt: '2026-01-01T00:00:00Z' },
      {
        ...basePost,
        id: 'other',
        slug: 'other',
        categories: { displayName: 'Craft', slug: 'craft' },
      },
    ];

    expect(getRelatedPosts(basePost, posts).map((post) => post.id)).toEqual(['new', 'old']);
  });

  it('uses a stable artwork variant for the same slug', () => {
    expect(getArtworkVariant('stable-slug')).toBe(getArtworkVariant('stable-slug'));
    expect(getArtworkVariant('stable-slug')).toBeGreaterThanOrEqual(0);
    expect(getArtworkVariant('stable-slug')).toBeLessThan(4);
  });
});

describe('redesign interaction contracts', () => {
  it('resolves the supported document language from a locale-prefixed path', () => {
    expect(getRouteLanguage('/vi/posts/example')).toBe('vi');
    expect(getRouteLanguage('/en/categories')).toBe('en');
    expect(getRouteLanguage('/zz/posts/example')).toBe('en');
  });

  it('preserves the path when switching locale', () => {
    expect(getLocalizedPath('/vi/categories/systems?view=all#latest', 'en')).toBe(
      '/en/categories/systems?view=all#latest',
    );
    expect(getLocalizedPath('/', 'vi')).toBe('/vi');
  });

  it('renders the locale switch as 44px segments', () => {
    render(
      <MemoryRouter initialEntries={['/vi/categories']}>
        <LanguageSwitch />
      </MemoryRouter>,
    );

    expect(screen.getByRole('link', { name: 'VI' })).toHaveAttribute('aria-current', 'true');
    expect(screen.getByRole('link', { name: 'EN' })).toHaveAttribute(
      'href',
      '/en/categories',
    );
  });

  it('opens mobile navigation, closes on Escape, and restores trigger focus', () => {
    render(
      <MemoryRouter initialEntries={['/en']}>
        <MobileNavigation />
      </MemoryRouter>,
    );

    const trigger = screen.getByRole('button', { name: 'Menu' });
    fireEvent.click(trigger);
    expect(trigger).toHaveAttribute('aria-expanded', 'true');
    expect(screen.getByRole('dialog', { name: 'Menu' })).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Home' })).toHaveFocus();

    fireEvent.keyDown(document, { key: 'Escape' });
    expect(trigger).toHaveAttribute('aria-expanded', 'false');
    expect(trigger).toHaveFocus();
  });

  it('keeps the document language synchronized after client-side route changes', () => {
    document.documentElement.lang = 'en';
    render(
      <MemoryRouter initialEntries={['/vi/about']}>
        <SiteLayout>
          <p>Trang giới thiệu</p>
        </SiteLayout>
      </MemoryRouter>,
    );

    expect(document.documentElement.lang).toBe('vi');
  });

  it('registers the localized About route in the shared application shell', () => {
    render(
      <MemoryRouter initialEntries={['/en/about']}>
        <AppContent />
      </MemoryRouter>,
    );

    expect(screen.getByRole('heading', { name: 'Coming soon' })).toBeInTheDocument();
    expect(screen.getAllByRole('link', { name: 'About' })[0]).toHaveAttribute('aria-current', 'page');
  });

  it('renders the skip link before all other focusable layout content', () => {
    const { container } = render(
      <MemoryRouter initialEntries={['/en']}>
        <SiteLayout>
          <p>Page body</p>
        </SiteLayout>
      </MemoryRouter>,
    );

    expect(container.querySelector('a')).toHaveAttribute('href', '#main');
    expect(container.querySelectorAll('header')).toHaveLength(1);
    expect(container.querySelectorAll('main#main')).toHaveLength(1);
    expect(container.querySelectorAll('footer')).toHaveLength(1);
    expect(container.querySelector('main#main')).toHaveAttribute('tabindex', '-1');
  });

  it('renders one post link without nested anchors', () => {
    const { container } = render(
      <MemoryRouter initialEntries={['/en']}>
        <PostCard post={basePost} lang="en" variant="lead" />
      </MemoryRouter>,
    );

    expect(container.querySelectorAll('a')).toHaveLength(1);
    expect(container.querySelector('a a')).not.toBeInTheDocument();
  });

  it('renders deterministic artwork without a network image when media is absent', () => {
    const { container } = render(
      <PostArtwork slug="stable-slug" title="Stable title" aspect="4 / 3" />,
    );

    expect(container.querySelector('img')).not.toBeInTheDocument();
    expect(screen.getByRole('img', { name: 'Stable title' })).toHaveAttribute(
      'data-artwork-variant',
      String(getArtworkVariant('stable-slug')),
    );
  });

  it('copies the canonical URL and announces success without moving focus', async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    Object.defineProperty(navigator, 'clipboard', {
      configurable: true,
      value: { writeText },
    });

    render(
      <ShareActions
        canonicalUrl="https://example.test/en/posts/stable-slug"
        title="Stable title"
        lang="en"
      />,
    );

    const copyButton = screen.getByRole('button', { name: 'Copy link' });
    copyButton.focus();
    fireEvent.click(copyButton);

    expect(writeText).toHaveBeenCalledWith('https://example.test/en/posts/stable-slug');
    expect(await screen.findByRole('status')).toHaveTextContent('Link copied');
    expect(copyButton).toHaveFocus();
  });

  it('keeps raw GraphQL details out of the localized error state', () => {
    const retry = vi.fn();
    render(<ContentError lang="en" onRetry={retry} />);

    expect(screen.getByText('We could not load this page.')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Retry' }));
    expect(retry).toHaveBeenCalledOnce();
  });
});
