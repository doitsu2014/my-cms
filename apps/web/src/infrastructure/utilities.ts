import { env } from 'process';

export function getHomePageCacheEnabled() {
  return (
    env.HOME_PAGE_CACHE_ENABLED?.toLowerCase() === 'true' ||
    env.HOME_PAGE_CACHE_ENABLED?.toLowerCase() === '1' ||
    env.HOME_PAGE_CACHE_ENABLED?.toLowerCase() === 'yes'
  );
}
