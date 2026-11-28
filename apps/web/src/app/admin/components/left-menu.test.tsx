import { render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { describe, expect, it, vi } from 'vitest';
import LeftMenu from './left-menu';
import { UserRoleEnum } from '@/domains/user';

const auth = { userInfo: { name: 'User', email: 'user@example.com' }, user: { app_metadata: { roles: [] as string[] } } };
vi.mock('@/auth/AuthContext', () => ({ useAuth: () => auth }));

describe('admin navigation', () => {
  it('only shows SEO navigation to administrators', () => {
    const view = render(<MemoryRouter><LeftMenu /></MemoryRouter>);
    expect(screen.queryByText('SEO head assets')).not.toBeInTheDocument();
    auth.user.app_metadata.roles = [UserRoleEnum.Administrator];
    view.rerender(<MemoryRouter><LeftMenu /></MemoryRouter>);
    expect(screen.getByText('SEO head assets')).toBeInTheDocument();
  });

  // TS-35 (UX only; not evidence for server-side authorization)
  it('ts35_only_shows_backup_navigation_to_administrators', () => {
    auth.user.app_metadata.roles = [];
    const view = render(<MemoryRouter><LeftMenu /></MemoryRouter>);
    expect(screen.queryByRole('link', { name: 'Backup' })).not.toBeInTheDocument();
    auth.user.app_metadata.roles = [UserRoleEnum.Administrator];
    view.rerender(<MemoryRouter><LeftMenu /></MemoryRouter>);
    expect(screen.getByRole('link', { name: 'Backup' })).toHaveAttribute('href', '/admin/backup');
  });
});
