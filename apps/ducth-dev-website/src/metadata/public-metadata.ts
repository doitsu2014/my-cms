import type { BlogPost, Category } from '../types/content';

export type SupportedLocale = 'en' | 'vi';
export type PublicRouteKind =
  'home' | 'categories' | 'category' | 'post' | 'about' | 'unknown';
export type PublicMetadataState = 'ready' | 'loading' | 'error' | 'not-found';

export interface PublicSiteConfig {
  siteName: string;
  siteUrl: string;
  defaultTitle: string;
  defaultDescription: string;
  avatarUrl?: string;
  mediaBaseUrl?: string;
}

export interface PublicMetadataInput {
  url: string;
  config: PublicSiteConfig;
  state?: PublicMetadataState;
  post?: BlogPost | null;
  category?: Category | null;
}

export interface OpenGraphMetadata {
  title: string;
  description: string;
  url: string;
  type: 'website' | 'article';
  locale: SupportedLocale;
  image?: string;
}

export interface TwitterMetadata {
  card: 'summary' | 'summary_large_image';
  title: string;
  description: string;
  image?: string;
}

export interface PublicMetadataProfile {
  title: string;
  description: string;
  canonicalUrl: string;
  lang: SupportedLocale;
  robots: 'index, follow' | 'noindex, nofollow';
  openGraph: OpenGraphMetadata;
  twitter: TwitterMetadata;
}

const DESCRIPTION_LIMIT = 160;
const LOCALES = new Set<SupportedLocale>(['en', 'vi']);

export function normalizeLocale(
  value: string | undefined,
  fallback: SupportedLocale = 'en',
): SupportedLocale {
  const normalized = value?.toLowerCase().split('-')[0];
  return normalized && LOCALES.has(normalized as SupportedLocale)
    ? (normalized as SupportedLocale)
    : fallback;
}

function routeFromUrl(url: string): {
  locale: SupportedLocale;
  kind: PublicRouteKind;
  slug?: string;
} {
  let pathname = url;
  try {
    pathname = new URL(url, 'https://metadata.invalid').pathname;
  } catch {
    pathname = url.split(/[?#]/, 1)[0] || '/';
  }
  const segments = pathname.split('/').filter(Boolean);
  const locale = normalizeLocale(segments[0]);
  if (!segments[0] || !LOCALES.has(segments[0] as SupportedLocale))
    return { locale, kind: 'unknown' };
  if (segments.length === 1) return { locale, kind: 'home' };
  if (segments[1] === 'categories' && segments.length === 2)
    return { locale, kind: 'categories' };
  if (segments[1] === 'categories' && segments[2])
    return { locale, kind: 'category', slug: segments[2] };
  if (segments[1] === 'posts' && segments[2])
    return { locale, kind: 'post', slug: segments[2] };
  if (segments[1] === 'about' && segments.length === 2)
    return { locale, kind: 'about' };
  return { locale, kind: 'unknown' };
}

function stripMarkup(value: string): string {
  return value
    .replace(/<[^>]*>/g, ' ')
    .replace(
      /&(?:amp|lt|gt|quot|#39|nbsp);/gi,
      (entity) =>
        ({
          '&amp;': '&',
          '&lt;': '<',
          '&gt;': '>',
          '&quot;': '"',
          '&#39;': "'",
          '&nbsp;': ' ',
        })[entity.toLowerCase()] ?? ' ',
    )
    .replace(/\s+/g, ' ')
    .trim();
}

export function toPlainDescription(
  value: string | null | undefined,
  fallback: string,
): string {
  const text = stripMarkup(value?.trim() || '') || stripMarkup(fallback);
  if (text.length <= DESCRIPTION_LIMIT) return text;
  return `${text.slice(0, DESCRIPTION_LIMIT - 1).trimEnd()}…`;
}

function titleWithSite(title: string, siteName: string): string {
  const cleanTitle = title.trim() || siteName.trim();
  return cleanTitle.toLowerCase().includes(siteName.trim().toLowerCase())
    ? cleanTitle
    : `${cleanTitle} — ${siteName.trim()}`;
}

function canonicalFor(
  config: PublicSiteConfig,
  locale: SupportedLocale,
  path = '',
): string {
  const base = config.siteUrl.replace(/\/$/, '');
  const normalizedPath = path.replace(/^\//, '').replace(/\/$/, '');
  return `${base}/${locale}${normalizedPath ? `/${normalizedPath}` : ''}`;
}

function thumbnailFor(
  post: BlogPost,
  config: PublicSiteConfig,
): string | undefined {
  const raw = post.thumbnailPaths?.find(Boolean);
  if (!raw) return undefined;
  if (/^https?:\/\//i.test(raw)) return raw;
  if (!config.mediaBaseUrl) return undefined;
  return `${config.mediaBaseUrl.replace(/\/$/, '')}/${raw.replace(/^\//, '')}`;
}

function profile(
  config: PublicSiteConfig,
  locale: SupportedLocale,
  values: {
    title?: string;
    description?: string;
    canonicalPath?: string;
    type?: 'website' | 'article';
    image?: string;
    state?: PublicMetadataState;
  },
): PublicMetadataProfile {
  const title = titleWithSite(
    values.title || config.defaultTitle,
    config.siteName,
  );
  const description = toPlainDescription(
    values.description,
    config.defaultDescription,
  );
  const robots =
    values.state === 'not-found' ||
    values.state === 'error' ||
    values.state === 'loading'
      ? 'noindex, nofollow'
      : 'index, follow';
  const canonicalUrl = canonicalFor(config, locale, values.canonicalPath);
  return {
    title,
    description,
    canonicalUrl,
    lang: locale,
    robots,
    openGraph: {
      title,
      description,
      url: canonicalUrl,
      type: values.type || 'website',
      locale,
      ...(values.image ? { image: values.image } : {}),
    },
    twitter: {
      card: values.image ? 'summary_large_image' : 'summary',
      title,
      description,
      ...(values.image ? { image: values.image } : {}),
    },
  };
}

export function resolvePublicMetadata(
  input: PublicMetadataInput,
): PublicMetadataProfile {
  const route = routeFromUrl(input.url);
  const state = input.state || 'ready';
  const localizedPost = input.post
    ? input.post.postTranslations?.nodes?.find(
        (item) => item.languageCode === route.locale,
      )
    : undefined;
  const localizedCategory = input.category?.categoryTranslations?.nodes?.find(
    (item) => item.languageCode === route.locale,
  );

  if (route.kind === 'post' && input.post?.published !== false && input.post) {
    const title = localizedPost?.title || input.post.title;
    const description =
      localizedPost?.previewContent ||
      input.post.previewContent ||
      input.post.content ||
      undefined;
    return profile(input.config, route.locale, {
      title,
      description,
      canonicalPath: `posts/${encodeURIComponent(input.post.slug)}`,
      type: 'article',
      image: thumbnailFor(input.post, input.config),
      state,
    });
  }

  if (route.kind === 'category' && input.category) {
    const name = localizedCategory?.displayName || input.category.displayName;
    return profile(input.config, route.locale, {
      title: name,
      description: `${name} — ${input.config.defaultDescription}`,
      canonicalPath: `categories/${encodeURIComponent(localizedCategory?.slug || input.category.slug)}`,
      state,
    });
  }

  if (
    route.kind === 'post' ||
    route.kind === 'category' ||
    route.kind === 'unknown'
  ) {
    return profile(input.config, route.locale, {
      state:
        route.kind === 'unknown' || state === 'ready' ? 'not-found' : state,
    });
  }

  const staticTitles: Record<
    Exclude<PublicRouteKind, 'post' | 'category' | 'unknown'>,
    Record<SupportedLocale, string>
  > = {
    home: { en: input.config.defaultTitle, vi: input.config.defaultTitle },
    categories: { en: 'Categories', vi: 'Danh mục' },
    about: { en: 'About', vi: 'Về tôi' },
  };
  const canonicalPath =
    route.kind === 'about'
      ? 'about'
      : route.kind === 'categories'
        ? 'categories'
        : undefined;
  return profile(input.config, route.locale, {
    title: staticTitles[route.kind][route.locale],
    canonicalPath,
    state,
  });
}

export function metadataInputFromApolloState(
  url: string,
  config: PublicSiteConfig,
  state: Record<string, unknown>,
): PublicMetadataInput {
  const route = routeFromUrl(url);
  const entities = Object.values(state).filter(
    (value): value is Record<string, unknown> =>
      Boolean(value && typeof value === 'object'),
  );
  const post =
    route.kind === 'post'
      ? (entities.find(
          (value) => value.__typename === 'Post' && value.slug === route.slug,
        ) as BlogPost | undefined)
      : undefined;
  const category =
    route.kind === 'category'
      ? (entities.find(
          (value) =>
            value.__typename === 'Category' &&
            (value.slug === route.slug ||
              value.slug === decodeURIComponent(route.slug || '')),
        ) as Category | undefined)
      : undefined;
  return {
    url,
    config,
    post,
    category,
    state:
      post ||
      category ||
      route.kind === 'home' ||
      route.kind === 'categories' ||
      route.kind === 'about'
        ? 'ready'
        : 'not-found',
  };
}
