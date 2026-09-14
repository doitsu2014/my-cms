import type { BlogPost, LocalizedPost } from '../../types/content';

const PARAGRAPH_TAG = /<p\b[^>]*>/i;
const MERMAID_CODE_BLOCK = /<pre\b[^>]*>\s*<code\b([^>]*)>/gi;

const hasLocalizedContent = (html: string | null | undefined): boolean => {
  if (!html) return false;
  if (PARAGRAPH_TAG.test(html)) return true;
  return Array.from(html.matchAll(MERMAID_CODE_BLOCK)).some((match) => {
    const classNames = match[1].match(/class=(['"])(.*?)\1/i)?.[2]?.split(/\s+/) ?? [];
    return classNames.includes('language-mermaid');
  });
};

export function getLocalizedPost(post: BlogPost, lang: string): LocalizedPost {
  const translation = post.postTranslations?.nodes?.find(
    (candidate) => candidate.languageCode === lang,
  );

  const translationContent = hasLocalizedContent(translation?.content) ? translation!.content! : null;

  return {
    ...post,
    title: translation?.title || post.title,
    previewContent: translation?.previewContent || post.previewContent || '',
    content: translationContent || post.content || '',
  };
}
