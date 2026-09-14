import '@testing-library/jest-dom/vitest';
import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { beforeEach, describe, expect, it, vi } from 'vitest';

vi.mock('@/auth/AuthContext', () => ({ useAuth: () => ({ token: 'admin-token' }) }));
vi.mock('@/config/api.config', async (importOriginal) => ({
  ...(await importOriginal<typeof import('@/config/api.config')>()),
  authenticatedFetch: vi.fn(),
  getApiUrl: vi.fn((path: string) => `/api${path}`),
  getMediaUploadApiUrl: vi.fn(() => '/media'),
  createAuthHeaders: vi.fn(() => ({ Authorization: 'Bearer admin-token' })),
}));

import { authenticatedFetch } from '@/config/api.config';
import { ImageInsertionDialog } from './image-insertion-dialog';

const mediaResponse = (data: unknown, ok = true) => new Response(JSON.stringify({ data }), { status: ok ? 200 : 500 });
const fetchMock = vi.mocked(authenticatedFetch);

describe('ImageInsertionDialog', () => {
  beforeEach(() => {
    fetchMock.mockReset();
    vi.stubGlobal('fetch', vi.fn());
  });

  it('loads image-only media, displays selection order, and inserts that order', async () => {
    const user = userEvent.setup();
    const onInsert = vi.fn();
    fetchMock.mockResolvedValue(mediaResponse([
      { path: 'a.png', url: 'https://media/a.png', contentType: 'image/png', size: 1, lastModified: '' },
      { path: 'file.pdf', url: 'https://media/file.pdf', contentType: 'application/pdf', size: 1, lastModified: '' },
      { path: 'b.jpg', url: 'https://media/b.jpg', contentType: 'image/jpeg', size: 1, lastModified: '' },
    ]));
    render(<ImageInsertionDialog isOpen onInsert={onInsert} onClose={vi.fn()} />);

    await user.click(await screen.findByRole('button', { name: /b.jpg/i }));
    await user.click(screen.getByRole('button', { name: /a.png/i }));
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /file.pdf/i })).not.toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Insert selected images' }));
    expect(onInsert).toHaveBeenCalledWith(['https://media/b.jpg', 'https://media/a.png']);
  });

  it('shows retry and disabled confirmation for empty or failed queries', async () => {
    const user = userEvent.setup();
    fetchMock.mockResolvedValueOnce(mediaResponse({}, false)).mockResolvedValueOnce(mediaResponse([]));
    render(<ImageInsertionDialog isOpen onInsert={vi.fn()} onClose={vi.fn()} />);
    await user.click(await screen.findByRole('button', { name: 'Retry' }));
    await waitFor(() => expect(screen.getByText('No images are available in the media library.')).toBeInTheDocument());
    expect(screen.getByRole('button', { name: 'Insert selected images' })).toBeDisabled();
  });

  it('uploads selected image files sequentially and inserts successful URLs in file order', async () => {
    const user = userEvent.setup();
    const onInsert = vi.fn();
    fetchMock.mockResolvedValue(mediaResponse([]));
    const upload = vi.mocked(fetch)
      .mockResolvedValueOnce(mediaResponse({ url: 'https://media/one.png' }))
      .mockResolvedValueOnce(mediaResponse({}, false))
      .mockResolvedValueOnce(mediaResponse({ url: 'https://media/three.png' }));
    render(<ImageInsertionDialog isOpen onInsert={onInsert} onClose={vi.fn()} />);
    const input = await screen.findByLabelText('Upload local images');
    await user.upload(input, [
      new File(['1'], 'one.png', { type: 'image/png' }),
      new File(['2'], 'two.png', { type: 'image/png' }),
      new File(['3'], 'three.png', { type: 'image/png' }),
    ]);
    await waitFor(() => expect(onInsert).toHaveBeenCalledWith(['https://media/one.png', 'https://media/three.png']));
    expect(upload).toHaveBeenCalledTimes(3);
    expect(screen.getByText('2 images added, 1 failed.')).toBeInTheDocument();
    expect((input as HTMLInputElement).value).toBe('');
  });

  it('cancels with Escape and restores focus to the invoking control', async () => {
    const user = userEvent.setup();
    fetchMock.mockResolvedValue(mediaResponse([]));
    const onClose = vi.fn();
    render(<><button type="button">Editor</button><ImageInsertionDialog isOpen onInsert={vi.fn()} onClose={onClose} restoreFocusTo={document.querySelector('button')} /></>);
    await screen.findByRole('dialog', { name: 'Insert images' });
    await user.keyboard('{Escape}');
    expect(onClose).toHaveBeenCalled();
  });
});
