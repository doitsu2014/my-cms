import { getArtworkVariant } from '../../lib/media/getArtworkVariant';
import EditorialImage from '../editorial/EditorialImage';

interface PostArtworkProps {
  src?: string;
  slug: string;
  title: string;
  aspect?: string;
  fit?: 'cover' | 'natural';
}

const PostArtwork = ({ src, slug, title, aspect = '4 / 3', fit = 'cover' }: PostArtworkProps) => {
  if (src) {
    return (
      <EditorialImage
        src={src}
        alt={title}
        aspect={fit === 'natural' ? undefined : aspect}
        fit={fit}
        className="post-artwork"
      />
    );
  }
  return (
    <div
      className={`post-artwork post-artwork--fallback post-artwork--variant-${getArtworkVariant(slug)}`}
      style={{ aspectRatio: aspect }}
      role="img"
      aria-label={title}
      data-artwork-variant={getArtworkVariant(slug)}
    >
      <span aria-hidden="true" />
    </div>
  );
};

export default PostArtwork;
