import { getLocalizedPost } from '../../lib/i18n/getLocalizedPost';
import { getPostThumbnail } from '../../lib/media/getPostThumbnail';
import type { BlogPost } from '../../types/content';
import PostMeta from '../editorial/PostMeta';
import PostArtwork from './PostArtwork';

interface PostListRowProps {
  post: BlogPost;
  lang: string;
  mediaBaseUrl: string;
}

const PostListRow = ({ post, lang, mediaBaseUrl }: PostListRowProps) => {
  const thumbnail = getPostThumbnail(post, mediaBaseUrl);
  const localizedPost = getLocalizedPost(post, lang);
  return (
    <article className="post-list-row">
      <PostArtwork src={thumbnail} slug={post.slug} title={localizedPost.title} />
      <div>
        <PostMeta category={post.categories?.displayName} date={post.createdAt} lang={lang} />
        <a className="post-list-row__title" href={`/${lang}/posts/${post.slug}`}>{localizedPost.title}</a>
        {localizedPost.previewContent && <p className="post-list-row__excerpt">{localizedPost.previewContent}</p>}
        <a className="text-link" href={`/${lang}/posts/${post.slug}`}>
          {lang === 'vi' ? 'Đọc tiếp' : 'Read article'} <span aria-hidden="true">→</span>
        </a>
      </div>
    </article>
  );
};

export default PostListRow;
