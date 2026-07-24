import { getRuntimeConfig } from './get-runtime-config';

export const SITE_CONFIG = {
  get siteName() {
    return getRuntimeConfig().siteName;
  },
  get siteUrl() {
    return getRuntimeConfig().siteUrl;
  },
  get avatarUrl() {
    return getRuntimeConfig().avatarUrl;
  },
  socialLinks: {
    github: 'https://github.com',
    twitter: 'https://twitter.com',
    linkedin: 'https://linkedin.com',
  },
  seo: {
    get defaultTitle() {
      return getRuntimeConfig().defaultTitle;
    },
    get defaultDescription() {
      return getRuntimeConfig().defaultDescription;
    },
  },
};
