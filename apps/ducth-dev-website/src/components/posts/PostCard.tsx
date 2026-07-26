import { Link } from 'react-router-dom';
import { getLocalizedPost } from '../../lib/i18n/getLocalizedPost';
import { getPostThumbnail } from '../../lib/media/getPostThumbnail';
import type { BlogPost } from '../../types/content';
import CategoryLabel from '../editorial/CategoryLabel';
import PostMeta from '../editorial/PostMeta';
import PostArtwork from './PostArtwork';

export type PostCardVariant = 'lead' | 'compact' | 'standard' | 'wide' | 'related';

interface PostCardProps {
  post: BlogPost;
  lang: string;
  variant?: PostCardVariant;
  mediaBaseUrl?: string;
}

const PostCard = ({ post, lang, variant = 'standard', mediaBaseUrl = '' }: PostCardProps) => {
  const localizedPost = getLocalizedPost(post, lang);
  const thumbnail = mediaBaseUrl ? getPostThumbnail(post, mediaBaseUrl) : undefined;
  const href = `/${lang}/posts/${post.slug}`;
  return (
    <article className={`post-card post-card--${variant}`}>
      <Link to={href} className="post-card__link">
        <PostArtwork
          src={thumbnail}
          slug={post.slug}
          title={localizedPost.title}
          aspect={variant === 'wide' ? '16 / 10' : variant === 'lead' ? '5 / 3' : '4 / 3'}
        />
        <div className="post-card__body">
          {localizedPost.categories?.displayName && (
            <CategoryLabel>{localizedPost.categories.displayName}</CategoryLabel>
          )}
          <PostMeta date={post.createdAt} lang={lang} />
          <h3 className="post-card__title">{localizedPost.title}</h3>
          {variant !== 'compact' && localizedPost.previewContent && (
            <p className="post-card__excerpt">{localizedPost.previewContent}</p>
          )}
          <span className="text-link post-card__read">{lang === 'vi' ? 'Đọc tiếp' : 'Read article'} <span aria-hidden="true">→</span></span>
        </div>
      </Link>
    </article>
  );
};

export default PostCard;
