import { describe, expect, it, beforeEach, vi } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import { authenticatedFetch } from '@/config/api.config';
import AuthenticatedImage from './authenticated-image';

vi.mock('@/config/api.config', () => ({
  authenticatedFetch: vi.fn(),
}));

const fetchMock = vi.mocked(authenticatedFetch);
const createObjectUrlMock = vi.fn(() => 'blob:media-preview');
const revokeObjectUrlMock = vi.fn();

const imageProps = {
  src: 'http://localhost:8989/media/images/photo.png',
  token: 'admin-token',
  alt: 'Photo',
};

describe('AuthenticatedImage', () => {
  beforeEach(() => {
    fetchMock.mockReset();
    createObjectUrlMock.mockReset();
    createObjectUrlMock.mockReturnValue('blob:media-preview');
    revokeObjectUrlMock.mockReset();
    vi.stubGlobal('URL', {
      createObjectURL: createObjectUrlMock,
      revokeObjectURL: revokeObjectUrlMock,
    });
  });

  it('fetches the image with authentication and renders its blob URL', async () => {
    const blob = new Blob(['image'], { type: 'image/png' });
    fetchMock.mockResolvedValue({
      ok: true,
      blob: vi.fn().mockResolvedValue(blob),
    } as unknown as Response);

    render(<AuthenticatedImage {...imageProps} className="preview" />);

    await waitFor(() => expect(screen.getByRole('img')).toBeInTheDocument());

    expect(fetchMock).toHaveBeenCalledWith(imageProps.src, imageProps.token, {
      cache: 'no-store',
    });
    expect(createObjectUrlMock).toHaveBeenCalledWith(blob);
    expect(screen.getByRole('img')).toHaveAttribute('src', 'blob:media-preview');
    expect(screen.getByRole('img')).toHaveClass('preview');
  });

  it('revokes the blob URL when the image unmounts', async () => {
    fetchMock.mockResolvedValue({
      ok: true,
      blob: vi.fn().mockResolvedValue(new Blob(['image'])),
    } as unknown as Response);

    const { unmount } = render(<AuthenticatedImage {...imageProps} />);
    await waitFor(() => expect(screen.getByRole('img')).toBeInTheDocument());

    unmount();

    expect(revokeObjectUrlMock).toHaveBeenCalledWith('blob:media-preview');
  });

  it('keeps a busy placeholder when the authenticated fetch fails', async () => {
    fetchMock.mockRejectedValue(new Error('request failed'));

    render(<AuthenticatedImage {...imageProps} />);

    await waitFor(() => expect(fetchMock).toHaveBeenCalled());

    expect(screen.queryByRole('img')).not.toBeInTheDocument();
    expect(screen.getByTestId('authenticated-image-placeholder')).toHaveAttribute(
      'aria-busy',
      'true',
    );
  });
});
