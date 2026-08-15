import type { BlogPost, LocalizedPost } from '../../types/content';

const PARAGRAPH_TAG = /<p\b[^>]*>/i;

const hasParagraphs = (html: string | null | undefined): boolean => {
  if (!html) return false;
  return PARAGRAPH_TAG.test(html);
};

export function getLocalizedPost(post: BlogPost, lang: string): LocalizedPost {
  const translation = post.postTranslations?.nodes?.find(
    (candidate) => candidate.languageCode === lang,
  );

  const translationContent = hasParagraphs(translation?.content) ? translation!.content! : null;

  return {
    ...post,
    title: translation?.title || post.title,
    previewContent: translation?.previewContent || post.previewContent || '',
    content: translationContent || post.content || '',
  };
}
