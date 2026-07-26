import type { BlogPost } from '../../types/content';

export function getRelatedPosts(currentPost: BlogPost, posts: BlogPost[]): BlogPost[] {
  return posts
    .filter(
      (post) =>
        post.published !== false &&
        post.id !== currentPost.id &&
        Boolean(currentPost.categories?.slug) &&
        post.categories?.slug === currentPost.categories?.slug,
    )
    .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
    .slice(0, 3);
}
