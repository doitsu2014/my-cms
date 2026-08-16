import { describe, expect, it } from 'vitest';
import { applyAdminMetadata, resolveAdminMetadata } from './admin-metadata';

describe('admin metadata', () => {
  it.each([
    ['/admin/login', 'Sign in — My-CMS Admin'],
    ['/admin', 'Dashboard — My-CMS Admin'],
    ['/admin/blogs', 'Blogs — My-CMS Admin'],
    ['/admin/blogs/create', 'Create blog — My-CMS Admin'],
    ['/admin/blogs/edit/1', 'Edit blog — My-CMS Admin'],
  ])('names %s', (path, title) => {
    expect(resolveAdminMetadata(path)).toEqual({
      title,
      robots: 'noindex, nofollow',
    });
  });

  it('keeps one private robots tag and removes public tags', () => {
    document.head.innerHTML =
      '<meta name="robots" content="index"><meta name="robots" content="index"><link rel="canonical" href="https://public.test"><meta property="og:title" content="public"><meta name="twitter:title" content="public">';
    applyAdminMetadata('/admin/categories');
    expect(document.title).toBe('Categories — My-CMS Admin');
    expect(document.head.querySelectorAll('meta[name="robots"]')).toHaveLength(
      1,
    );
    expect(document.head.querySelector('meta[name="robots"]')).toHaveAttribute(
      'content',
      'noindex, nofollow',
    );
    expect(document.head.querySelector('link[rel="canonical"]')).toBeNull();
    expect(document.head.querySelector('meta[property^="og:"]')).toBeNull();
    expect(document.head.querySelector('meta[name^="twitter:"]')).toBeNull();
  });
});
