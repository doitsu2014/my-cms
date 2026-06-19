#!/bin/sh
set -e

# Defaults (matches .env.my-cms)
PUBLIC_SUPABASE_URL="${PUBLIC_SUPABASE_URL:-http://localhost:8001}"
PUBLIC_SUPABASE_ANON_KEY="${PUBLIC_SUPABASE_ANON_KEY:-}"
PUBLIC_GRAPHQL_API_URL="${PUBLIC_GRAPHQL_API_URL:-http://localhost:8989/graphql}"
PUBLIC_GRAPHQL_CACHE_API_URL="${PUBLIC_GRAPHQL_CACHE_API_URL:-}"
PUBLIC_REST_API_URL="${PUBLIC_REST_API_URL:-http://localhost:8989/api}"
PUBLIC_MEDIA_UPLOAD_API_URL="${PUBLIC_MEDIA_UPLOAD_API_URL:-}"

sed \
  -e "s|\${PUBLIC_SUPABASE_URL}|${PUBLIC_SUPABASE_URL}|g" \
  -e "s|\${PUBLIC_SUPABASE_ANON_KEY}|${PUBLIC_SUPABASE_ANON_KEY}|g" \
  -e "s|\${PUBLIC_GRAPHQL_API_URL}|${PUBLIC_GRAPHQL_API_URL}|g" \
  -e "s|\${PUBLIC_GRAPHQL_CACHE_API_URL}|${PUBLIC_GRAPHQL_CACHE_API_URL}|g" \
  -e "s|\${PUBLIC_REST_API_URL}|${PUBLIC_REST_API_URL}|g" \
  -e "s|\${PUBLIC_MEDIA_UPLOAD_API_URL}|${PUBLIC_MEDIA_UPLOAD_API_URL}|g" \
  /usr/share/nginx/html/config.js.template \
  > /usr/share/nginx/html/config.js

# Cache-bust: inject a fresh version query on config.js in every HTML file
# so browsers and CDNs always fetch the latest config after a deploy
CONFIG_VERSION=$(date +%s)
find /usr/share/nginx/html -name "*.html" -exec sed -i "s|src=\"/config.js\"|src=\"/config.js?v=${CONFIG_VERSION}\"|g" {} \;

exec nginx -g "daemon off;"
