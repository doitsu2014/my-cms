import { readBrowserConfig } from './read-browser-config';

export const getGraphqlApiUrl = (): string => readBrowserConfig().graphqlApiUrl;
export const getMediaBaseUrl = (): string => readBrowserConfig().mediaBaseUrl;
export const API_CONFIG = {
  get graphqlApiUrl() {
    return getGraphqlApiUrl();
  },
  get mediaBaseUrl() {
    return getMediaBaseUrl();
  },
};
export { getMediaUrl } from './get-media-url';
