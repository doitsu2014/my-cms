export type RouteLanguage = 'en' | 'vi';

export function getRouteLanguage(pathname: string): RouteLanguage {
  const firstSegment = pathname.split('/').filter(Boolean)[0];
  return firstSegment === 'vi' ? 'vi' : 'en';
}
