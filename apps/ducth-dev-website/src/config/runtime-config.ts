export interface RuntimeConfig {
  siteName: string;
  siteUrl: string;
  avatarUrl?: string;
  defaultTitle: string;
  defaultDescription: string;
  defaultLocale: string;
  graphqlApiUrl: string;
  graphqlCacheApiUrl: string;
  mediaBaseUrl: string;
  port: string;
}

declare global {
  interface Window {
    __APP_CONFIG__?: RuntimeConfig;
  }
}
