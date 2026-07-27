export type SupportedLanguage = 'en' | 'vi';

export function getLocalizedPath(pathname: string, lang: SupportedLanguage, search = '', hash = ''): string {
  const match = pathname.match(/^\/(en|vi)(?=\/|$)/);
  const remainder = match ? pathname.slice(match[0].length) : pathname === '/' ? '' : pathname;
  return `/${lang}${remainder || ''}${search}${hash}`;
}
