import { getRelatedPosts } from '../../lib/posts/getRelatedPosts';
import type { BlogPost } from '../../types/content';
import PostCard from './PostCard';

interface RelatedPostsProps {
  currentPost: BlogPost;
  posts: BlogPost[];
  lang: string;
  mediaBaseUrl: string;
}

const RelatedPosts = ({ currentPost, posts, lang, mediaBaseUrl }: RelatedPostsProps) => {
  const relatedPosts = getRelatedPosts(currentPost, posts);
  if (relatedPosts.length === 0) return null;
  return (
    <section className="related-posts" aria-labelledby="related-heading">
      <div className="site-container">
        <h2 id="related-heading" className="display-h2">{lang === 'vi' ? 'Đọc tiếp' : 'Read next'}</h2>
        <div className="related-posts__grid">
          {relatedPosts.map((post) => (
            <PostCard key={post.id} post={post} lang={lang} variant="related" mediaBaseUrl={mediaBaseUrl} />
          ))}
        </div>
      </div>
    </section>
  );
};

export default RelatedPosts;
