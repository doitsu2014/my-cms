export interface AdminMetadataProfile {
  title: string;
  robots: 'noindex, nofollow';
}

const ADMIN_NAME = 'My-CMS Admin';

const routeTitles: Array<[RegExp, string]> = [
  [/^\/admin\/login(?:\/|$)/, 'Sign in'],
  [/^\/admin\/categories\/create(?:\/|$)/, 'Create category'],
  [/^\/admin\/categories\/edit(?:\/|$)/, 'Edit category'],
  [/^\/admin\/categories(?:\/|$)/, 'Categories'],
  [/^\/admin\/blogs\/create(?:\/|$)/, 'Create blog'],
  [/^\/admin\/blogs\/edit(?:\/|$)/, 'Edit blog'],
  [/^\/admin\/blogs(?:\/|$)/, 'Blogs'],
  [/^\/admin\/media\/buckets(?:\/|$)/, 'Media buckets'],
  [/^\/admin\/media(?:\/|$)/, 'Media'],
  [/^\/admin\/users\/create(?:\/|$)/, 'Create user'],
  [/^\/admin\/users\/edit(?:\/|$)/, 'Edit user'],
  [/^\/admin\/users(?:\/|$)/, 'Users'],
  [/^\/admin(?:\/|$)/, 'Dashboard'],
];

export function resolveAdminMetadata(pathname: string): AdminMetadataProfile {
  const path = pathname.split(/[?#]/, 1)[0] || '/admin';
  const label =
    routeTitles.find(([pattern]) => pattern.test(path))?.[1] ||
    'Administration';
  return { title: `${label} — ${ADMIN_NAME}`, robots: 'noindex, nofollow' };
}

export function applyAdminMetadata(
  pathname: string,
  documentRef: Document = document,
): void {
  const profile = resolveAdminMetadata(pathname);
  documentRef.title = profile.title;
  const robots = documentRef.head.querySelectorAll('meta[name="robots"]');
  const existing = robots[0] || documentRef.createElement('meta');
  existing.setAttribute('name', 'robots');
  existing.setAttribute('content', profile.robots);
  documentRef.head.append(existing);
  robots.forEach((tag, index) => {
    if (index > 0) tag.remove();
  });
  documentRef.head
    .querySelectorAll(
      'link[rel="canonical"], meta[property^="og:"], meta[name^="twitter:"]',
    )
    .forEach((tag) => tag.remove());
}
