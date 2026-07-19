import { describe, it, expect, beforeEach, vi } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import MediaGridItem from './media-grid-item';
import { authenticatedFetch } from '@/config/api.config';
import type { MediaMetadata } from '@/models/MediaModels';

vi.mock('@/config/api.config', () => ({
  authenticatedFetch: vi.fn(),
}));

const fetchMock = vi.mocked(authenticatedFetch);
const createObjectUrlMock = vi.fn((blob: Blob) => `blob:${blob.size}`);

const imageMedia: MediaMetadata = {
  path: 'subfolder/foo.png',
  url: 'http://localhost:8989/media/images/subfolder/foo.png',
  contentType: 'image/png',
  size: 1024,
  lastModified: '2026-01-01T00:00:00Z',
};

const nonImageMedia: MediaMetadata = {
  path: 'document.pdf',
  url: 'http://localhost:8989/media/document.pdf',
  contentType: 'application/pdf',
  size: 1024,
  lastModified: '2026-01-01T00:00:00Z',
};

const baseProps = {
  isSelected: false,
  onSelect: () => {},
  onPreview: () => {},
  onCopyUrl: () => {},
  onDelete: () => {},
  token: 'admin-token' as string | null,
};

describe('MediaGridItem', () => {
  beforeEach(() => {
    fetchMock.mockReset();
    createObjectUrlMock.mockClear();
    vi.stubGlobal('URL', {
      createObjectURL: createObjectUrlMock,
      revokeObjectURL: vi.fn(),
    });
  });

  it('uses media.url as the thumbnail src for image media (no resize query)', async () => {
    fetchMock.mockResolvedValue({
      ok: true,
      blob: vi.fn().mockResolvedValue(new Blob(['image'], { type: 'image/png' })),
    } as unknown as Response);

    render(<MediaGridItem {...baseProps} media={imageMedia} />);

    await waitFor(() => expect(screen.getByRole('img')).toBeInTheDocument());

    expect(fetchMock).toHaveBeenCalledWith(imageMedia.url, baseProps.token, {
      cache: 'no-store',
    });
    const fetchedUrl = fetchMock.mock.calls[0]?.[0] as string;
    expect(fetchedUrl).not.toContain('?w=');
    expect(fetchedUrl).not.toContain('?h=');
  });

  it('does not construct /media/images/<path>?w=&h= for image media', async () => {
    fetchMock.mockResolvedValue({
      ok: true,
      blob: vi.fn().mockResolvedValue(new Blob(['image'])),
    } as unknown as Response);

    render(<MediaGridItem {...baseProps} media={imageMedia} />);

    await waitFor(() => expect(fetchMock).toHaveBeenCalled());

    const fetchedUrl = fetchMock.mock.calls[0]?.[0] ?? '';
    expect(fetchedUrl).not.toMatch(/\/media\/images\/.*\?w=/);
  });

  it('does not render an img element for non-image media', () => {
    render(<MediaGridItem {...baseProps} media={nonImageMedia} />);

    expect(screen.queryByRole('img')).not.toBeInTheDocument();
    expect(fetchMock).not.toHaveBeenCalled();
  });
});
