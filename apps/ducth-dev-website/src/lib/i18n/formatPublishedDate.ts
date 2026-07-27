export function formatPublishedDate(date: string, lang: string): string {
  return new Date(date).toLocaleDateString(lang === 'vi' ? 'vi-VN' : 'en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  });
}
