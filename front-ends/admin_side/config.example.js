// Example config.js for Kubernetes ConfigMap
// Mount this as: /usr/share/nginx/html/config.js
window.__APP_CONFIG__ = {
  keycloakUrl: "https://your-keycloak-url",
  keycloakRealm: "your-realm",
  keycloakClientId: "your-client-id",
  keycloakScope: "my-headless-cms-api-all email openid profile",
  graphqlApiUrl: "https://your-api/graphql",
  graphqlCacheApiUrl: "",
  restApiUrl: "https://your-api",
  mediaUploadApiUrl: ""
};
