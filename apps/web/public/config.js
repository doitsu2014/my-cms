// Runtime Configuration for local development (without Docker).
// When running in Docker, this file is overwritten at startup from config.js.template.
window.__APP_CONFIG__ = {
  supabaseUrl: 'http://localhost:8001',
  supabaseAnonKey: 'your-anon-key-here',
  graphqlApiUrl: 'http://localhost:8989/graphql',
  graphqlCacheApiUrl: '',
  restApiUrl: 'http://localhost:8989/api',
  mediaUploadApiUrl: 'http://localhost:8989/api/media/upload',
};
