import '@testing-library/jest-dom/vitest';
import { render, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MemoryRouter, Link } from 'react-router-dom';
import { describe, expect, it } from 'vitest';
import AdminMetadataBoundary from './AdminMetadataBoundary';

describe('AdminMetadataBoundary', () => {
  it('updates private route titles during navigation', async () => {
    const { findByRole } = render(
      <MemoryRouter initialEntries={['/admin/login']}>
        <AdminMetadataBoundary />
        <Link to="/admin/blogs/create">Create</Link>
      </MemoryRouter>,
    );
    await findByRole('link', { name: 'Create' });
    expect(document.title).toBe('Sign in — My-CMS Admin');
    await userEvent.click(document.querySelector('a') as HTMLAnchorElement);
    await waitFor(() =>
      expect(document.title).toBe('Create blog — My-CMS Admin'),
    );
    expect(document.head.querySelectorAll('meta[name="robots"]')).toHaveLength(
      1,
    );
  });
});
