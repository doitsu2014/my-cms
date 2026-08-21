const SUPPORTED_LOCALES = ['en', 'vi'];
const DEFAULT_CACHE_TTL_MS = 5 * 60 * 1000;
const DEFAULT_TIMEOUT_MS = 2_000;

export const SITEMAP_QUERY = `
  query WebsiteSitemap {
    categories(filters: { categoryType: { eq: Blog }, parentId: { is_null: "true" } }) {
      nodes {
        slug
        categoryType
        categoryTranslations {
          nodes {
            languageCode
            slug
          }
        }
      }
    }
    posts {
      nodes {
        slug
        published
        lastModifiedAt
      }
    }
  }
`;

function normalizeSiteUrl(siteUrl) {
  const url = new URL(siteUrl);
  url.search = '';
  url.hash = '';
  return url.toString().replace(/\/+$/, '');
}

function xmlEscape(value) {
  return String(value)
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&apos;');
}

function encodedSegment(value) {
  const segment = String(value || '').trim();
  return segment ? encodeURIComponent(segment) : null;
}

function publicUrl(siteUrl, path) {
  return `${normalizeSiteUrl(siteUrl)}/${path.replace(/^\/+/, '')}`;
}

function staticEntries(siteUrl) {
  return SUPPORTED_LOCALES.flatMap((locale) =>
    ['', 'categories', 'about'].map((path) => ({
      loc: publicUrl(siteUrl, [locale, path].filter(Boolean).join('/')),
    })),
  );
}

function validLastModifiedAt(value) {
  if (!value) return undefined;
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? undefined : date.toISOString();
}

function categoryEntries(siteUrl, categories) {
  return categories.flatMap((category) => {
    if (!category || category.categoryType !== 'Blog') {
      return [];
    }
    const baseSlug = encodedSegment(category.slug);
    if (!baseSlug) return [];
    return SUPPORTED_LOCALES.map((locale) => {
      const translation = Array.isArray(category.categoryTranslations?.nodes)
        ? category.categoryTranslations.nodes.find(
            (candidate) => candidate?.languageCode === locale && candidate.slug,
          )
        : undefined;
      const slug = encodedSegment(translation?.slug) || baseSlug;
      return { loc: publicUrl(siteUrl, `${locale}/categories/${slug}`) };
    });
  });
}

function postEntries(siteUrl, posts) {
  return posts.flatMap((post) => {
    if (!post || post.published !== true) return [];
    const slug = encodedSegment(post.slug);
    if (!slug) return [];
    const lastmod = validLastModifiedAt(post.lastModifiedAt);
    return SUPPORTED_LOCALES.map((locale) => ({
      loc: publicUrl(siteUrl, `${locale}/posts/${slug}`),
      ...(lastmod ? { lastmod } : {}),
    }));
  });
}

export function buildSitemapEntries(siteUrl, data = {}) {
  const categories = Array.isArray(data.categories?.nodes)
    ? data.categories.nodes
    : [];
  const posts = Array.isArray(data.posts?.nodes) ? data.posts.nodes : [];
  const seen = new Set();
  return [
    ...staticEntries(siteUrl),
    ...categoryEntries(siteUrl, categories),
    ...postEntries(siteUrl, posts),
  ].filter((entry) => {
    if (seen.has(entry.loc)) return false;
    seen.add(entry.loc);
    return true;
  });
}

export function serializeSitemap(entries) {
  const urls = entries
    .map(
      ({ loc, lastmod }) =>
        `  <url><loc>${xmlEscape(loc)}</loc>${lastmod ? `<lastmod>${xmlEscape(lastmod)}</lastmod>` : ''}</url>`,
    )
    .join('\n');
  return `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n${urls}\n</urlset>\n`;
}

async function fetchContent({ fetchImpl, graphqlApiUrl, timeoutMs }) {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), timeoutMs);
  try {
    const response = await fetchImpl(graphqlApiUrl, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({ query: SITEMAP_QUERY }),
      signal: controller.signal,
    });
    if (!response.ok)
      throw new Error(`Sitemap upstream returned ${response.status}`);
    const payload = await response.json();
    if (payload?.errors || !payload?.data || typeof payload.data !== 'object') {
      throw new Error('Sitemap upstream returned invalid data');
    }
    return payload.data;
  } finally {
    clearTimeout(timeout);
  }
}

export function createSitemapService({
  siteUrl,
  graphqlApiUrl,
  fetchImpl = globalThis.fetch,
  now = () => Date.now(),
  ttlMs = DEFAULT_CACHE_TTL_MS,
  timeoutMs = DEFAULT_TIMEOUT_MS,
}) {
  let cached;
  return {
    async getSitemapXml() {
      if (cached && now() - cached.createdAt < ttlMs) return cached.xml;

      let data;
      try {
        data = await fetchContent({ fetchImpl, graphqlApiUrl, timeoutMs });
      } catch {
        data = undefined;
      }
      const xml = serializeSitemap(buildSitemapEntries(siteUrl, data));
      cached = { createdAt: now(), xml };
      return xml;
    },
  };
}
