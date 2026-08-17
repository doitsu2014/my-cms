import '@testing-library/jest-dom/vitest';
import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import PostArtwork from './PostArtwork';
import { getArtworkVariant } from '../../lib/media/getArtworkVariant';

describe('PostArtwork', () => {
  it('preserves the source ratio for natural detail artwork', () => {
    const { container } = render(
      <PostArtwork
        src="https://example.test/thumbnail.png"
        slug="natural-slug"
        title="Natural title"
        aspect="21 / 9"
        fit="natural"
      />,
    );

    const figure = container.querySelector('figure');
    expect(figure).toHaveClass('editorial-image--natural');
    expect(figure).not.toHaveAttribute('style');
    expect(screen.getByRole('img', { name: 'Natural title' })).toHaveAttribute(
      'src',
      'https://example.test/thumbnail.png',
    );
  });

  it('keeps fixed fallback sizing and cover mode as defaults', () => {
    const { container: fallbackContainer } = render(
      <PostArtwork slug="stable-slug" title="Stable title" aspect="4 / 3" />,
    );

    expect(fallbackContainer.querySelector('img')).not.toBeInTheDocument();
    expect(screen.getByRole('img', { name: 'Stable title' })).toHaveAttribute(
      'data-artwork-variant',
      String(getArtworkVariant('stable-slug')),
    );
    expect(fallbackContainer.firstElementChild).toHaveStyle({ aspectRatio: '4 / 3' });

    const { container: coverContainer } = render(
      <PostArtwork
        src="https://example.test/card.png"
        slug="cover-slug"
        title="Cover title"
        aspect="4 / 3"
      />,
    );

    const coverFigure = coverContainer.querySelector('figure');
    expect(coverFigure).not.toHaveClass('editorial-image--natural');
    expect(coverFigure).toHaveStyle({ aspectRatio: '4 / 3' });
  });
});
