// Runtime Configuration for local development (without Docker).
// When running in Docker, this file is overwritten at startup from config.js.template.
//
// The /api prefix is only used in single-domain deploys (frontend + API
// on the same host, with Traefik/nginx stripping /api). For local dev
// the API is on a separate port (8989), so we use the bare origin.
// The backend's media upload route is POST /media (not /media/upload).
window.__APP_CONFIG__ = {
  supabaseUrl: 'http://localhost:8001',
  supabaseAnonKey: 'your-anon-key-here',
  graphqlApiUrl: 'http://localhost:8989/graphql',
  graphqlCacheApiUrl: '',
  restApiUrl: 'http://localhost:8989',
  mediaUploadApiUrl: 'http://localhost:8989/media',
};
