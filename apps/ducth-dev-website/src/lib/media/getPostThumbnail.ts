import { getMediaUrl } from '../../config/get-media-url';
import type { BlogPost } from '../../types/content';

export function getPostThumbnail(
  post: Pick<BlogPost, 'thumbnailPaths'>,
  mediaBaseUrl: string,
): string | undefined {
  const path = post.thumbnailPaths?.find(Boolean);
  return path ? getMediaUrl(path, mediaBaseUrl) : undefined;
}
