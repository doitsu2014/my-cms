import { readBrowserConfig } from './read-browser-config';

export const SITE_CONFIG = {
  get siteName() {
    return readBrowserConfig().siteName;
  },
  get siteUrl() {
    return readBrowserConfig().siteUrl;
  },
  get avatarUrl() {
    return readBrowserConfig().avatarUrl;
  },
  socialLinks: {
    github: 'https://github.com',
    twitter: 'https://twitter.com',
    linkedin: 'https://linkedin.com',
  },
  seo: {
    get defaultTitle() {
      return readBrowserConfig().defaultTitle;
    },
    get defaultDescription() {
      return readBrowserConfig().defaultDescription;
    },
  },
};
