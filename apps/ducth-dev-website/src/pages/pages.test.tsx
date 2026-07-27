import '@testing-library/jest-dom/vitest';
import { MockedProvider, type MockedResponse } from '@apollo/client/testing';
import { render, screen } from '@testing-library/react';
import { MemoryRouter, Route, Routes } from 'react-router-dom';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { GET_BLOG_POST_BY_SLUG, GET_BLOG_POSTS, GET_CATEGORIES, GET_POSTS_BY_CATEGORY } from '../infrastructure/graphql/queries';
import CategoriesPage from './CategoriesPage';
import CategoryDetailPage from './CategoryDetailPage';
import HomePage from './HomePage';
import PostDetailPage from './PostDetailPage';

const runtimeConfig = {
  siteName: 'Test site',
  siteUrl: 'https://example.test',
  defaultTitle: 'Test',
  defaultDescription: 'Test description',
  defaultLocale: 'en',
  graphqlApiUrl: 'https://example.test/graphql/immutable',
  graphqlCacheApiUrl: 'https://example.test/graphql/immutable',
  mediaBaseUrl: 'https://example.test/media',
  port: '3001',
};

const post = {
  id: 'post-1',
  title: 'A real article',
  slug: 'real-article',
  previewContent: 'A real preview.',
  content: '<p>Article content</p><pre><code>const value = 1;</code></pre>',
  createdBy: 'Author',
  createdAt: '2025-03-14T00:00:00.000Z',
  lastModifiedBy: null,
  lastModifiedAt: null,
  categoryId: 'category-1',
  rowVersion: '1',
  published: true,
  thumbnailPaths: [],
  categories: { id: 'category-1', displayName: 'Systems', slug: 'systems', __typename: 'Category' },
  postTags: { nodes: [], __typename: 'PostTagsConnection' },
  postTranslations: { nodes: [], __typename: 'PostTranslationsConnection' },
  __typename: 'Post',
};

const category = {
  id: 'category-1',
  displayName: 'Systems',
  slug: 'systems',
  categoryType: 'Blog',
  createdAt: '2025-01-01T00:00:00.000Z',
  categoryTags: { nodes: [], __typename: 'CategoryTagsConnection' },
  categoryTranslations: { nodes: [], __typename: 'CategoryTranslationsConnection' },
  __typename: 'Category',
};

function withProviders(ui: React.ReactElement, path: string, mocks: MockedResponse[]) {
  return render(
    <MockedProvider mocks={mocks}>
      <MemoryRouter initialEntries={[path]}>{ui}</MemoryRouter>
    </MockedProvider>,
  );
}

beforeEach(() => {
  document.body.innerHTML = `<script id="app-config" type="application/json">${JSON.stringify(runtimeConfig)}</script>`;
});

describe('redesigned page data states', () => {
  it('renders the six-item recent feed contract without a featured label', async () => {
    const posts = Array.from({ length: 7 }, (_, index) => ({
      ...post,
      id: `post-${index}`,
      slug: `post-${index}`,
      title: `Article ${index}`,
      createdAt: `2025-03-${String(index + 1).padStart(2, '0')}T00:00:00.000Z`,
    }));
    withProviders(<HomePage />, '/en', [
      { request: { query: GET_BLOG_POSTS }, result: { data: { posts: { nodes: posts } } } },
    ]);

    expect(await screen.findByRole('heading', { name: 'Recent articles' })).toBeInTheDocument();
    expect(screen.getAllByRole('article')).toHaveLength(6);
    expect(screen.queryByText(/featured|highlights/i)).not.toBeInTheDocument();
  });

  it('renders localized category rows with three latest previews and no slug prose', async () => {
    const posts = Array.from({ length: 4 }, (_, index) => ({
      ...post,
      id: `post-${index}`,
      slug: `post-${index}`,
      title: `Article ${index}`,
      createdAt: `2025-03-${String(index + 1).padStart(2, '0')}T00:00:00.000Z`,
    }));
    withProviders(<CategoriesPage />, '/en/categories', [
      { request: { query: GET_CATEGORIES }, result: { data: { categories: { nodes: [category] } } } },
      { request: { query: GET_BLOG_POSTS }, result: { data: { posts: { nodes: posts } } } },
    ]);

    expect(await screen.findByRole('link', { name: 'Systems' })).toBeInTheDocument();
    expect(screen.getByText('01', { exact: false })).toBeInTheDocument();
    expect(screen.getAllByRole('article')).toHaveLength(4);
    expect(screen.queryByText('systems', { selector: 'p' })).not.toBeInTheDocument();
  });

  it('keeps an empty category navigable and omits fake pagination', async () => {
    withProviders(
      <Routes><Route path="/:lang/categories/:slug" element={<CategoryDetailPage />} /></Routes>,
      '/en/categories/systems',
      [{
        request: { query: GET_POSTS_BY_CATEGORY, variables: { categorySlug: 'systems' } },
        result: { data: { categories: { nodes: [category] }, posts: { nodes: [] } } },
      }],
    );

    expect(await screen.findByRole('heading', { name: 'Systems' })).toBeInTheDocument();
    expect(screen.getByText('No published posts in this category yet.')).toBeInTheDocument();
    expect(screen.queryByLabelText(/pagination/i)).not.toBeInTheDocument();
    expect(screen.getByRole('link', { name: /Back home/i })).toHaveAttribute('href', '/en/categories');
  });

  it('renders the post reading surface, code, and functional share controls', async () => {
    withProviders(
      <Routes><Route path="/:lang/posts/:slug" element={<PostDetailPage />} /></Routes>,
      '/en/posts/real-article',
      [
        {
          request: { query: GET_BLOG_POST_BY_SLUG, variables: { slug: 'real-article' } },
          result: { data: { posts: { nodes: [post] } } },
        },
        { request: { query: GET_BLOG_POSTS }, result: { data: { posts: { nodes: [post] } } } },
      ],
    );

    expect(await screen.findByRole('heading', { name: 'A real article' })).toBeInTheDocument();
    expect(screen.getByText('Article content')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Share on X' })).toHaveAttribute('rel', 'noopener');
    expect(screen.getByRole('button', { name: 'Copy link' })).toBeInTheDocument();
  });

  it('renders layout-matched busy and sanitized error states', async () => {
    const { unmount } = withProviders(<HomePage />, '/en', [
      {
        request: { query: GET_BLOG_POSTS },
        delay: 10_000,
        result: { data: { posts: { nodes: [], __typename: 'PostsConnection' } } },
      },
    ]);
    expect(screen.getByLabelText('Loading')).toHaveAttribute('aria-busy', 'true');
    unmount();

    const consoleError = vi.spyOn(console, 'error').mockImplementation(() => undefined);
    withProviders(<HomePage />, '/en', [
      { request: { query: GET_BLOG_POSTS }, error: new Error('private GraphQL detail') },
    ]);
    expect(await screen.findByText('We could not load this page.')).toBeInTheDocument();
    expect(screen.queryByText('private GraphQL detail')).not.toBeInTheDocument();
    consoleError.mockRestore();
  });

  it('renders localized not-found states for unknown detail slugs', async () => {
    const { unmount } = withProviders(
      <Routes><Route path="/:lang/posts/:slug" element={<PostDetailPage />} /></Routes>,
      '/en/posts/unknown',
      [
        {
          request: { query: GET_BLOG_POST_BY_SLUG, variables: { slug: 'unknown' } },
          result: { data: { posts: { nodes: [] } } },
        },
        { request: { query: GET_BLOG_POSTS }, result: { data: { posts: { nodes: [] } } } },
      ],
    );
    expect(await screen.findByRole('heading', { name: 'Not found' })).toBeInTheDocument();
    unmount();

    withProviders(
      <Routes><Route path="/:lang/categories/:slug" element={<CategoryDetailPage />} /></Routes>,
      '/en/categories/unknown',
      [{
        request: { query: GET_POSTS_BY_CATEGORY, variables: { categorySlug: 'unknown' } },
        result: { data: { categories: { nodes: [] }, posts: { nodes: [] } } },
      }],
    );
    expect(await screen.findByRole('heading', { name: 'Not found' })).toBeInTheDocument();
  });
});
