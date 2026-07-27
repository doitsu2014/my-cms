import type { BlogPost, LocalizedPost } from '../../types/content';

export function getLocalizedPost(post: BlogPost, lang: string): LocalizedPost {
  const translation = post.postTranslations?.nodes?.find(
    (candidate) => candidate.languageCode === lang,
  );

  return {
    ...post,
    title: translation?.title || post.title,
    previewContent: translation?.previewContent || post.previewContent || '',
    content: translation?.content || post.content || '',
  };
}
