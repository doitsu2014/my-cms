import { getRuntimeConfig } from './get-runtime-config';

export const getGraphqlApiUrl = (): string => getRuntimeConfig().graphqlApiUrl;
export const getMediaBaseUrl = (): string => getRuntimeConfig().mediaBaseUrl;
export const API_CONFIG = {
  get graphqlApiUrl() {
    return getGraphqlApiUrl();
  },
  get mediaBaseUrl() {
    return getMediaBaseUrl();
  },
};
export { getMediaUrl } from './get-media-url';
