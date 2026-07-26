export interface PostTranslation {
  languageCode: string;
  title?: string | null;
  previewContent?: string | null;
  content?: string | null;
}

export interface CategoryTranslation {
  languageCode: string;
  displayName?: string | null;
  slug?: string | null;
}

export interface Category {
  id?: string;
  displayName: string;
  slug: string;
  categoryType?: string | null;
  categoryTranslations?: { nodes?: CategoryTranslation[] | null } | null;
}

export interface BlogPost {
  id: string;
  title: string;
  slug: string;
  previewContent?: string | null;
  content?: string | null;
  createdAt: string;
  createdBy?: string | null;
  thumbnailPaths?: string[] | null;
  published?: boolean | null;
  categoryId?: string | null;
  categories?: Category | null;
  postTranslations?: { nodes?: PostTranslation[] | null } | null;
}

export interface LocalizedPost extends BlogPost {
  title: string;
  previewContent: string;
  content: string;
  categories?: Category | null;
}

export interface LocalizedCategory extends Category {
  displayName: string;
  slug: string;
}
