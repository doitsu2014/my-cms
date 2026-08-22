import { render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import HeadAssetForm from './head-asset-form';

vi.mock('@/auth/AuthContext', () => ({ useAuth: () => ({ token: 'token' }) }));
vi.mock('@/domains/seo-head-asset', async () => {
  const actual = await vi.importActual<typeof import('@/domains/seo-head-asset')>('@/domains/seo-head-asset');
  return { ...actual, getSeoHeadAsset: vi.fn().mockResolvedValue({ id: 'asset', label: 'gtag', html: '<script>window.__shouldNotRun = true;</script>', enabled: true, sortOrder: 1, rowVersion: 1, createdAt: '', updatedAt: '', createdBy: '', updatedBy: '' }) };
});

describe('SEO head asset editor', () => {
  beforeEach(() => { delete (window as Window & { __shouldNotRun?: boolean }).__shouldNotRun; });
  it('keeps submitted source in an inert textarea value', async () => {
    render(<MemoryRouter><HeadAssetForm id="asset" /></MemoryRouter>);
    const textarea = await screen.findByLabelText('Head source');
    expect(textarea).toHaveValue('<script>window.__shouldNotRun = true;</script>');
    expect(document.querySelector('script:not([src])')).toBeNull();
    expect((window as Window & { __shouldNotRun?: boolean }).__shouldNotRun).toBeUndefined();
  });
});
