import { describe, expect, it, vi } from 'vitest';
import { SeoHeadAssetApiError, listSeoHeadAssets, seoHeadAssetFormSchema } from './seo-head-asset';
import { authenticatedFetch } from '@/config/api.config';

vi.mock('@/config/api.config', () => ({
  getApiUrl: (path: string) => path,
  authenticatedFetch: vi.fn(),
}));

describe('SEO head asset contracts', () => {
  it('validates required fields and positive ordering', () => {
    expect(seoHeadAssetFormSchema.safeParse({ label: '', html: '', enabled: true, sortOrder: 0 }).success).toBe(false);
    expect(seoHeadAssetFormSchema.safeParse({ label: 'gtag', html: '<script></script>', enabled: true, sortOrder: 1 }).success).toBe(true);
  });

  it('maps server validation and conflict responses without replacing server authority', async () => {
    vi.mocked(authenticatedFetch).mockResolvedValue(new Response(JSON.stringify({ errorCode: '409', errors: ['label already exists'] }), { status: 409 }));
    await expect(listSeoHeadAssets('token')).rejects.toEqual(expect.objectContaining<Partial<SeoHeadAssetApiError>>({ status: 409, code: '409' }));
  });
});
