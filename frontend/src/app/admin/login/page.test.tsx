import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter, Routes, Route } from 'react-router-dom';
import { toast } from 'sonner';

const signInWithPassword = vi.fn();

vi.mock('@/auth/supabase', () => ({
  getSupabaseClient: () => ({
    auth: {
      signInWithPassword,
    },
  }),
}));

vi.mock('sonner', () => ({
  toast: {
    error: vi.fn(),
    success: vi.fn(),
  },
}));

import AdminLoginPage from './page';

function renderAt(initialEntry: string) {
  return render(
    <MemoryRouter initialEntries={[initialEntry]}>
      <Routes>
        <Route path="/admin/login" element={<AdminLoginPage />} />
        <Route
          path="/admin"
          element={<div data-testid="admin-landing">Admin Landing</div>}
        />
        <Route
          path="/admin/posts"
          element={<div data-testid="admin-posts">Admin Posts</div>}
        />
      </Routes>
    </MemoryRouter>,
  );
}

describe('AdminLoginPage', () => {
  beforeEach(() => {
    signInWithPassword.mockReset();
    vi.mocked(toast.error).mockReset();
  });

  it('calls signInWithPassword with typed values on valid submit', async () => {
    signInWithPassword.mockResolvedValue({ data: {}, error: null });
    const user = userEvent.setup();

    renderAt('/admin/login?from=/admin/posts');

    await user.type(screen.getByLabelText(/email/i), 'admin@example.com');
    await user.type(screen.getByLabelText(/password/i), 'password123');
    await user.click(screen.getByRole('button', { name: /sign in/i }));

    await waitFor(() => {
      expect(signInWithPassword).toHaveBeenCalledTimes(1);
    });
    expect(signInWithPassword).toHaveBeenCalledWith({
      email: 'admin@example.com',
      password: 'password123',
    });
  });

  it('does not call signInWithPassword and shows inline error on invalid email', async () => {
    const user = userEvent.setup();

    renderAt('/admin/login');

    await user.type(screen.getByLabelText(/email/i), 'not-an-email');
    await user.type(screen.getByLabelText(/password/i), 'password123');
    await user.click(screen.getByRole('button', { name: /sign in/i }));

    await waitFor(() => {
      expect(signInWithPassword).not.toHaveBeenCalled();
    });
    expect(await screen.findByText(/invalid email/i)).toBeInTheDocument();
  });

  it('does not call signInWithPassword and shows inline error on short password', async () => {
    const user = userEvent.setup();

    renderAt('/admin/login');

    await user.type(screen.getByLabelText(/email/i), 'admin@example.com');
    await user.type(screen.getByLabelText(/password/i), 'short');
    await user.click(screen.getByRole('button', { name: /sign in/i }));

    expect(signInWithPassword).not.toHaveBeenCalled();
    expect(
      await screen.findByText(/at least 8 characters/i),
    ).toBeInTheDocument();
  });

  it('shows a toast and does not navigate when signInWithPassword returns an error', async () => {
    signInWithPassword.mockResolvedValue({
      data: {},
      error: { message: 'Invalid login credentials' },
    });
    const user = userEvent.setup();

    renderAt('/admin/login?from=/admin/posts');

    await user.type(screen.getByLabelText(/email/i), 'admin@example.com');
    await user.type(screen.getByLabelText(/password/i), 'password123');
    await user.click(screen.getByRole('button', { name: /sign in/i }));

    await waitFor(() => {
      expect(toast.error).toHaveBeenCalledWith('Invalid login credentials');
    });
    expect(screen.queryByTestId('admin-posts')).not.toBeInTheDocument();
    expect(screen.queryByTestId('admin-landing')).not.toBeInTheDocument();
  });

  it('navigates to the `from` query param on successful login', async () => {
    signInWithPassword.mockResolvedValue({ data: {}, error: null });
    const user = userEvent.setup();

    renderAt('/admin/login?from=/admin/posts');

    await user.type(screen.getByLabelText(/email/i), 'admin@example.com');
    await user.type(screen.getByLabelText(/password/i), 'password123');
    await user.click(screen.getByRole('button', { name: /sign in/i }));

    expect(await screen.findByTestId('admin-posts')).toBeInTheDocument();
  });

  it('navigates to /admin by default on successful login', async () => {
    signInWithPassword.mockResolvedValue({ data: {}, error: null });
    const user = userEvent.setup();

    renderAt('/admin/login');

    await user.type(screen.getByLabelText(/email/i), 'admin@example.com');
    await user.type(screen.getByLabelText(/password/i), 'password123');
    await user.click(screen.getByRole('button', { name: /sign in/i }));

    expect(await screen.findByTestId('admin-landing')).toBeInTheDocument();
  });
});
