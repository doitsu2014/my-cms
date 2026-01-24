// Runtime Configuration for Development
// This file is loaded before the app to provide runtime configuration.
// In production, this file is generated from config.json by the entrypoint script.
window.__APP_CONFIG__ = {
  keycloakUrl: 'https://my-ids-admin.ducth.dev',
  keycloakRealm: 'my-blogs',
  keycloakClientId: 'my-blogs-website-oidc',
  keycloakScope: 'my-headless-cms-api-all email openid profile',
  graphqlApiUrl: 'http://localhost:8989/graphql',
  graphqlCacheApiUrl: '',
  restApiUrl: 'http://localhost:8989',
  mediaUploadApiUrl: 'http://localhost:8989',
  // graphqlApiUrl: 'https://my-cms-api.ducth.dev/graphql',
  // graphqlCacheApiUrl: '',
  // restApiUrl: 'https://my-cms-api.ducth.dev',
  // mediaUploadApiUrl: 'https://my-cms-api.ducth.dev/media',
};
