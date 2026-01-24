/**
 * Runtime Configuration
 *
 * This module provides runtime configuration that can be injected at container startup.
 * The config is loaded from window.__APP_CONFIG__ which is populated by /config.js
 *
 * For development: Uses import.meta.env values (build-time)
 * For production: Uses injected JSON config (runtime)
 */

export interface AppConfig {
  // Keycloak Configuration
  keycloakUrl: string;
  keycloakRealm: string;
  keycloakClientId: string;
  keycloakScope: string;

  // Backend API Configuration
  graphqlApiUrl: string;
  graphqlCacheApiUrl?: string;
  restApiUrl: string;
  mediaUploadApiUrl?: string;
}

declare global {
  interface Window {
    __APP_CONFIG__?: AppConfig;
  }
}

/**
 * Get runtime configuration
 * Priority: window.__APP_CONFIG__ > import.meta.env > defaults
 */
export const getConfig = (): AppConfig => {
  // If runtime config is injected (production), use it
  if (window.__APP_CONFIG__) {
    return window.__APP_CONFIG__;
  }

  // Fall back to build-time env vars (development)
  return {
    keycloakUrl: import.meta.env.PUBLIC_KEYCLOAK_URL || 'https://my-ids-admin.ducth.dev',
    keycloakRealm: import.meta.env.PUBLIC_KEYCLOAK_REALM || 'master',
    keycloakClientId: import.meta.env.PUBLIC_KEYCLOAK_CLIENT_ID || 'admin-side-client',
    keycloakScope: import.meta.env.PUBLIC_KEYCLOAK_SCOPE || 'my-headless-cms-api-all email openid profile',
    graphqlApiUrl: import.meta.env.PUBLIC_GRAPHQL_API_URL || 'http://localhost:4000/graphql',
    graphqlCacheApiUrl: import.meta.env.PUBLIC_GRAPHQL_CACHE_API_URL,
    restApiUrl: import.meta.env.PUBLIC_REST_API_URL || '',
    mediaUploadApiUrl: import.meta.env.PUBLIC_MEDIA_UPLOAD_API_URL,
  };
};

// Export a singleton config instance
let configInstance: AppConfig | null = null;

export const config = (): AppConfig => {
  if (!configInstance) {
    configInstance = getConfig();
  }
  return configInstance;
};
