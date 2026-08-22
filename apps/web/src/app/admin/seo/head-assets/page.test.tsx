import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { describe, expect, it, vi } from 'vitest';
import AdminSeoHeadAssetsPage from './page';
import { deleteSeoHeadAsset, listSeoHeadAssets } from '@/domains/seo-head-asset';

vi.mock('@/auth/AuthContext', () => ({ useAuth: () => ({ token: 'token', userInfo: { name: 'Admin', email: 'admin@example.com' } }) }));
vi.mock('@/domains/seo-head-asset', async () => {
  const actual = await vi.importActual<typeof import('@/domains/seo-head-asset')>('@/domains/seo-head-asset');
  return { ...actual, listSeoHeadAssets: vi.fn().mockResolvedValue([{ id: 'asset', label: 'gtag', html: '<script></script>', enabled: true, sortOrder: 1, rowVersion: 1, createdAt: '', updatedAt: '2026-01-01T00:00:00Z', createdBy: '', updatedBy: '' }]), deleteSeoHeadAsset: vi.fn().mockResolvedValue(undefined) };
});

describe('SEO head asset list', () => {
  it('does not send deletion before confirmation', async () => {
    vi.spyOn(window, 'confirm').mockReturnValue(false);
    render(<MemoryRouter><AdminSeoHeadAssetsPage /></MemoryRouter>);
    await screen.findByText('gtag');
    fireEvent.click(screen.getByRole('button', { name: 'Delete gtag' }));
    await waitFor(() => expect(deleteSeoHeadAsset).not.toHaveBeenCalled());
    expect(listSeoHeadAssets).toHaveBeenCalledWith('token');
  });
});
