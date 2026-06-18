#!/bin/sh
# volumes/api/kong-entrypoint.sh
# Custom entrypoint for Kong 2.8.1 that builds template expressions for the
# request-transformer plugin and performs environment variable substitution
# in the declarative config before exec-ing the stock kong:2.8.1 entrypoint
# (/docker-entrypoint.sh).
#
# Derived from upstream supabase/supabase
# (docker/volumes/api/kong-entrypoint.sh) with my-cms-specific adjustments:
#   - Template syntax is the Kong 2.8.x single-paren form `$(...)`, NOT the
#     Kong 3.x double-paren form `$((...))` (see https://docs.konghq.com/hub/
#     kong-inc/request-transformer/2.8.x/). The 2.x sandbox supports only
#     string methods on headers/query params, so the Lua expression is
#     limited to basic concatenation and string ops.
#   - Final `exec` targets /docker-entrypoint.sh (kong:2.8.1), not
#     /entrypoint.sh (kong:3.x used by upstream).
#   - The anonymous "if [ -n "$SUPABASE_SECRET_KEY" ] ..." branch is
#     collapsed: my-cms only uses the legacy HS256 path (no
#     $SUPABASE_SECRET_KEY / $SUPABASE_PUBLISHABLE_KEY are set), so we
#     always pass apikey through as a Bearer token. This keeps the Lua
#     expression trivial.

# Write the template expression to a file, then read it into the env var.
# This avoids the shell trying to interpret the Lua body. Heredoc with
# 'LUA_EOF' (quoted) disables all shell expansion inside the heredoc body.
LUA_FILE="$(mktemp)"
trap 'rm -f "$LUA_FILE"' EXIT

# Template expression evaluated by Kong's request-transformer plugin.
# Kong 2.8.x syntax: $(<expression>). The expression is Lua but limited to
# string methods (headers/query_params tables + string ops). We forward the
# apikey as a Bearer credential:
#   - If Authorization is set, use it as-is (e.g. a real user session JWT).
#   - Otherwise, return "Bearer <apikey>" for the legacy HS256 flow.
cat > "$LUA_FILE" <<'LUA_EOF'
Bearer $(headers.apikey)
LUA_EOF
export LUA_AUTH_EXPR="$(cat "$LUA_FILE")"

# Realtime WebSocket: the JS client sends apikey via query string; forward
# to x-api-key as-is (Realtime checks x-api-key first).
cat > "$LUA_FILE" <<'LUA_EOF'
$(query_params.apikey or headers.apikey or '')
LUA_EOF
export LUA_RT_WS_EXPR="$(cat "$LUA_FILE")"

# Substitute environment variables in the Kong declarative config.
# Uses awk instead of eval/echo to preserve YAML quoting (eval strips double
# quotes, breaking "Header: value" patterns that YAML parses as mappings).
awk '{
  result = ""
  rest = $0
  while (match(rest, /\$[A-Za-z_][A-Za-z_0-9]*/)) {
    varname = substr(rest, RSTART + 1, RLENGTH - 1)
    if (varname in ENVIRON) {
      result = result substr(rest, 1, RSTART - 1) ENVIRON[varname]
    } else {
      result = result substr(rest, 1, RSTART + RLENGTH - 1)
    }
    rest = substr(rest, RSTART + RLENGTH)
  }
  print result rest
}' /home/kong/temp.yml > "$KONG_DECLARATIVE_CONFIG"

# Remove empty key-auth credentials (unconfigured opaque keys)
sed -i '/^[[:space:]]*- key:[[:space:]]*$/d' "$KONG_DECLARATIVE_CONFIG"

# kong:2.8.1's stock entrypoint is /docker-entrypoint.sh (NOT /entrypoint.sh,
# which is kong:3.x). Upstream supabase uses kong:3.9.1; we pin 2.8.1 for
# Kong 2.x plugin server compatibility with the local OpenResty/nginx config.
exec /docker-entrypoint.sh kong docker-start
