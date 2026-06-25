// Runtime Configuration for local development (without Docker).
// When running in Docker, this file is overwritten at startup from config.js.template.
//
// The /api prefix is only used in single-domain deploys (frontend + API
// on the same host, with Traefik/nginx stripping /api). For local dev
// the API is on a separate port (8989), so we use the bare origin.
// The backend's media upload route is POST /media (not /media/upload).
window.__APP_CONFIG__ = {
  supabaseUrl: 'http://localhost:8000',
  supabaseAnonKey: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlzcyI6InN1cGFiYXNlIiwiaWF0IjoxNzgxNTk4OTQ0LCJleHAiOjE4MTMxMzQ5NDQsInN1YiI6IjAwMDAwMDAwLTAwMDAtMDAwMC0wMDAwLTAwMDAwMDAwMDAwMCIsImF1ZCI6ImF1dGhlbnRpY2F0ZWQiLCJlbWFpbCI6IiIsInBob25lIjoiIiwiYXBwX21ldGFkYXRhIjp7InByb3ZpZGVyIjoiYW5vbiIsInByb3ZpZGVycyI6WyJhbm9uIl19LCJ1c2VyX21ldGFkYXRhIjp7fSwiaXNfYW5vbnltb3VzIjp0cnVlfQ.PKzvDajcmVqOY_xoRftRUSrLgcw_0RQYAuIcn9nrE8E',
  graphqlApiUrl: 'http://localhost:8989/graphql',
  graphqlCacheApiUrl: '',
  restApiUrl: 'http://localhost:8989',
  mediaUploadApiUrl: 'http://localhost:8989/media',
};
