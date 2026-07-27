import type { Category, LocalizedCategory } from '../../types/content';

export function getLocalizedCategory(category: Category, lang: string): LocalizedCategory {
  const translation = category.categoryTranslations?.nodes?.find(
    (candidate) => candidate.languageCode === lang,
  );

  return {
    ...category,
    displayName: translation?.displayName || category.displayName,
    slug: translation?.slug || category.slug,
  };
}
