import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import ThumbnailsInput from './thumbnail-input';
import { useAuth } from '@/auth/AuthContext';

vi.mock('@/auth/AuthContext', () => ({
  useAuth: vi.fn(),
}));

const fetchMock = vi.fn();
vi.stubGlobal('fetch', fetchMock);

function makeFile(name = 'photo.png', type = 'image/png'): File {
  return new File([new Uint8Array([1, 2, 3])], name, { type });
}

describe('ThumbnailsInput', () => {
  beforeEach(() => {
    fetchMock.mockReset();
    vi.mocked(useAuth).mockReturnValue({ token: 'fake-token' } as any);
  });

  it('uses uploadResponse.data.url as the stored thumbnail src (not constructed from path)', async () => {
    const backendUrl = 'http://localhost:8989/media/images/abc12345-photo.png';

    fetchMock.mockResolvedValueOnce({
      ok: true,
      json: async () => ({
        data: {
          path: 'abc12345-photo.png',
          url: backendUrl,
        },
      }),
    });

    const onUploadSuccess = vi.fn();
    render(<ThumbnailsInput onUploadSuccess={onUploadSuccess} value={[]} />);

    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    expect(input).toBeTruthy();
    const file = makeFile();
    await userEvent.upload(input, file);

    await waitFor(() => {
      expect(onUploadSuccess).toHaveBeenCalled();
    });

    const urls = onUploadSuccess.mock.calls[0][0] as string[];
    expect(urls).toEqual([backendUrl]);
    expect(urls[0]).not.toMatch(/\/media\/images\/abc12345-photo\.png\?w=/);
    expect(urls[0]).not.toMatch(/getMediaImageUrl/);

    const img = document.querySelector('.avatar img') as HTMLImageElement;
    expect(img).toBeTruthy();
    expect(img.getAttribute('src')).toBe(backendUrl);
  });
});
