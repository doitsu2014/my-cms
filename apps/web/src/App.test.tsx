import { render, screen } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import App from './App';
import { UserRoleEnum } from '@/domains/user';

const auth = {
  authenticated: true,
  loading: false,
  token: 'tok',
  userInfo: { name: 'User', email: 'user@example.com' },
  user: { app_metadata: { roles: [] as string[] } },
};

vi.mock('@/auth/AuthContext', () => ({
  AuthProvider: ({ children }: { children: React.ReactNode }) => <>{children}</>,
  useAuth: () => auth,
}));
vi.mock('./app/admin/layout', () => ({
  default: ({ children }: { children: React.ReactNode }) => <div data-testid="admin-layout">{children}</div>,
}));
vi.mock('./app/admin/backup/page', () => ({ default: () => <div>backup page content</div> }));
vi.mock('./app/admin/login/page', () => ({ default: () => <div>login page content</div> }));

describe('App backup route', () => {
  beforeEach(() => {
    auth.authenticated = true;
    auth.user.app_metadata.roles = [];
    window.history.pushState({}, '', '/admin/backup');
  });

  // TS-35 (UX only; not evidence for server-side authorization)
  it('ts35_backup_route_renders_for_administrators_inside_the_admin_layout', () => {
    auth.user.app_metadata.roles = [UserRoleEnum.Administrator];
    render(<App />);
    expect(screen.getByText('backup page content')).toBeInTheDocument();
    expect(screen.getByTestId('admin-layout')).toContainElement(screen.getByText('backup page content'));
  });

  it('ts35_backup_route_shows_access_denied_to_non_administrators', () => {
    auth.user.app_metadata.roles = [UserRoleEnum.Writer];
    render(<App />);
    expect(screen.queryByText('backup page content')).not.toBeInTheDocument();
    expect(screen.getByText(/do not have permission/i)).toBeInTheDocument();
  });

  it('ts35_backup_route_redirects_unauthenticated_visitors_to_login', () => {
    auth.authenticated = false;
    render(<App />);
    expect(screen.queryByText('backup page content')).not.toBeInTheDocument();
    expect(screen.getByText('login page content')).toBeInTheDocument();
  });
});
