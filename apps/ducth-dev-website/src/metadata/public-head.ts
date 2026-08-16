import type { PublicMetadataProfile } from './public-metadata';

const MANAGED_ATTRIBUTE = 'data-public-metadata';

function escapeHtml(value: string): string {
  return value.replace(
    /[&<>"']/g,
    (character) =>
      ({
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#39;',
      })[character] || character,
  );
}

function meta(name: string, content: string, property = false): string {
  const attribute = property ? 'property' : 'name';
  return `<meta ${MANAGED_ATTRIBUTE} ${attribute}="${escapeHtml(name)}" content="${escapeHtml(content)}">`;
}

export function serializePublicHead(profile: PublicMetadataProfile): string {
  const tags = [
    `<title ${MANAGED_ATTRIBUTE}>${escapeHtml(profile.title)}</title>`,
    meta('description', profile.description),
    `<link ${MANAGED_ATTRIBUTE} rel="canonical" href="${escapeHtml(profile.canonicalUrl)}">`,
    meta('robots', profile.robots),
    meta('og:title', profile.openGraph.title, true),
    meta('og:description', profile.openGraph.description, true),
    meta('og:url', profile.openGraph.url, true),
    meta('og:type', profile.openGraph.type, true),
    meta('og:locale', profile.openGraph.locale, true),
    ...(profile.openGraph.image
      ? [meta('og:image', profile.openGraph.image, true)]
      : []),
    meta('twitter:card', profile.twitter.card),
    meta('twitter:title', profile.twitter.title),
    meta('twitter:description', profile.twitter.description),
    ...(profile.twitter.image
      ? [meta('twitter:image', profile.twitter.image)]
      : []),
  ];
  return tags.join('');
}

export function replacePublicHead(
  profile: PublicMetadataProfile,
  documentRef: Document = document,
): void {
  documentRef
    .querySelectorAll(`[${MANAGED_ATTRIBUTE}]`)
    .forEach((element) => element.remove());
  documentRef.head
    .querySelectorAll(
      'title, link[rel="canonical"], meta[name="description"], meta[name="robots"], meta[property^="og:"], meta[name^="twitter:"]',
    )
    .forEach((element) => element.remove());
  const title = documentRef.createElement('title');
  title.setAttribute(MANAGED_ATTRIBUTE, '');
  title.textContent = profile.title;
  documentRef.head.append(title);

  const addMeta = (name: string, content: string, property = false) => {
    const element = documentRef.createElement('meta');
    element.setAttribute(MANAGED_ATTRIBUTE, '');
    element.setAttribute(property ? 'property' : 'name', name);
    element.setAttribute('content', content);
    documentRef.head.append(element);
  };
  const canonical = documentRef.createElement('link');
  canonical.setAttribute(MANAGED_ATTRIBUTE, '');
  canonical.setAttribute('rel', 'canonical');
  canonical.setAttribute('href', profile.canonicalUrl);
  documentRef.head.append(canonical);
  addMeta('description', profile.description);
  addMeta('robots', profile.robots);
  addMeta('og:title', profile.openGraph.title, true);
  addMeta('og:description', profile.openGraph.description, true);
  addMeta('og:url', profile.openGraph.url, true);
  addMeta('og:type', profile.openGraph.type, true);
  addMeta('og:locale', profile.openGraph.locale, true);
  if (profile.openGraph.image)
    addMeta('og:image', profile.openGraph.image, true);
  addMeta('twitter:card', profile.twitter.card);
  addMeta('twitter:title', profile.twitter.title);
  addMeta('twitter:description', profile.twitter.description);
  if (profile.twitter.image) addMeta('twitter:image', profile.twitter.image);
  documentRef.documentElement.lang = profile.lang;
}
