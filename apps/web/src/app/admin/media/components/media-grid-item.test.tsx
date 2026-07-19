import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import MediaGridItem from './media-grid-item';
import type { MediaMetadata } from '@/models/MediaModels';

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

describe('MediaGridItem', () => {
  it('uses media.url as the thumbnail src for image media (no resize query)', () => {
    render(
      <MediaGridItem
        media={imageMedia}
        isSelected={false}
        onSelect={() => {}}
        onPreview={() => {}}
        onCopyUrl={() => {}}
        onDelete={() => {}}
      />
    );

    const img = screen.getByRole('img');
    expect(img).toHaveAttribute('src', imageMedia.url);
    expect(img.getAttribute('src')).not.toContain('?w=');
    expect(img.getAttribute('src')).not.toContain('?h=');
  });

  it('does not construct /media/images/<path>?w=&h= for image media', () => {
    render(
      <MediaGridItem
        media={imageMedia}
        isSelected={false}
        onSelect={() => {}}
        onPreview={() => {}}
        onCopyUrl={() => {}}
        onDelete={() => {}}
      />
    );

    const img = screen.getByRole('img');
    const src = img.getAttribute('src') ?? '';
    expect(src).not.toMatch(/\/media\/images\/.*\?w=/);
  });

  it('does not render an img element for non-image media', () => {
    render(
      <MediaGridItem
        media={nonImageMedia}
        isSelected={false}
        onSelect={() => {}}
        onPreview={() => {}}
        onCopyUrl={() => {}}
        onDelete={() => {}}
      />
    );

    expect(screen.queryByRole('img')).not.toBeInTheDocument();
  });
});
