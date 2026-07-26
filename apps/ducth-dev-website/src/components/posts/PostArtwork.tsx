import { getArtworkVariant } from '../../lib/media/getArtworkVariant';
import EditorialImage from '../editorial/EditorialImage';

interface PostArtworkProps {
  src?: string;
  slug: string;
  title: string;
  aspect?: string;
}

const PostArtwork = ({ src, slug, title, aspect = '4 / 3' }: PostArtworkProps) => {
  if (src) {
    return <EditorialImage src={src} alt={title} aspect={aspect} className="post-artwork" />;
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
