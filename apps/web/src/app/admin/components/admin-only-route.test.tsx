import { render, screen } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { AdminOnlyRoute } from './admin-only-route';
import { UserRoleEnum } from '@/domains/user';

const auth = { user: { app_metadata: { roles: [] as string[] } } };
vi.mock('@/auth/AuthContext', () => ({ useAuth: () => auth }));

describe('AdminOnlyRoute', () => {
  beforeEach(() => { auth.user.app_metadata.roles = []; });
  it('shows the existing access-denied state to writers', () => { render(<MemoryRouter><AdminOnlyRoute><div>authoring UI</div></AdminOnlyRoute></MemoryRouter>); expect(screen.queryByText('authoring UI')).not.toBeInTheDocument(); expect(screen.getByText(/do not have permission/i)).toBeInTheDocument(); });
  it('renders protected content for administrators', () => { auth.user.app_metadata.roles = [UserRoleEnum.Administrator]; render(<MemoryRouter><AdminOnlyRoute><div>authoring UI</div></AdminOnlyRoute></MemoryRouter>); expect(screen.getByText('authoring UI')).toBeInTheDocument(); });
});
