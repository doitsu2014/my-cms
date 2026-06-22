// Example config.js for Kubernetes ConfigMap
// Mount this as: /usr/share/nginx/html/config.js
//
// Two deploy modes are supported (see apps/web/.env.example for full detail):
//
//   Mode A (subdomain) — set restApiUrl to the bare API origin:
//     restApiUrl: "https://cms-api.ducth.dev"
//
//   Mode B (single-domain) — leave restApiUrl empty; the helper adds /api:
//     restApiUrl: ""
//
// mediaUploadApiUrl is the URL the webapp POSTs multipart uploads to.
// On the backend this is `POST /media` (see apps/api/src/bin/my-cms-api.rs).
// Leaving it empty is also fine — the helper will derive it from restApiUrl
// as `<restApiUrl>/media` (Mode A) or `/media` (Mode B).
window.__APP_CONFIG__ = {
  supabaseUrl: "https://your-project.supabase.co",
  supabaseAnonKey: "your-anon-key",
  graphqlApiUrl: "https://your-api/graphql",
  graphqlCacheApiUrl: "",
  restApiUrl: "https://your-api",
  mediaUploadApiUrl: ""
};
