#!/usr/bin/env bash
# scripts/seed-admin.sh
# One-shot seeder: creates the single administrator user in GoTrue on a fresh
# Supabase reset. Idempotent — if the user already exists, exits 0.
#
# Required env (sourced from .env.supabase):
#   SERVICE_ROLE_KEY    admin API key (GoTrue admin endpoints)
#   SUPABASE_API_HOST   Traefik Host header for Kong (e.g. supabase-api.ducth.dev)
#
# Optional env:
#   SEED_ADMIN_EMAIL   admin email (default: admin@my-cms.local)
#
# Side effects:
#   - Writes email + generated password to volumes/secrets/admin-password.txt
#   - Prints email + password to stdout on first creation

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Load env vars from .env.supabase (gitignored). It must exist by this point —
# reset-supabase.sh is the entry point and the operator is expected to have
# copied .env.supabase.example to .env.supabase.
ENV_FILE="$REPO_ROOT/deployments/docker-swarm/.env.supabase"
if [ ! -f "$ENV_FILE" ]; then
  echo "ERROR: $ENV_FILE not found. Copy .env.supabase.example to .env.supabase and try again." >&2
  exit 1
fi

set -a
# shellcheck disable=SC1090
. "$ENV_FILE"
set +a

: "${SERVICE_ROLE_KEY:?SERVICE_ROLE_KEY must be set in .env.supabase}"
: "${SUPABASE_API_HOST:?SUPABASE_API_HOST must be set in .env.supabase}"

# All API calls route through Traefik on localhost so no direct Kong port is needed.
# The Host header tells Traefik which backend to forward to.
API_BASE="http://localhost"
HOST_HEADER="Host: ${SUPABASE_API_HOST}"

SEED_ADMIN_EMAIL="${SEED_ADMIN_EMAIL:-admin@my-cms.local}"
SECRETS_DIR="$REPO_ROOT/deployments/docker-swarm/volumes/secrets"
PASSWORD_FILE="$SECRETS_DIR/admin-password.txt"

mkdir -p "$SECRETS_DIR"

# Sanity check: is GoTrue reachable at all? Avoid hanging on a half-up stack.
# Use the admin user list endpoint with no apikey: if GoTrue is up, it returns
# 401 from Kong (no API key found). If GoTrue is not up, the request times out
# or Kong returns 502/503. Either way we can distinguish "stack is up" from
# "stack is still starting" without requiring an open /health route.
AUTH_PROBE_URL="$API_BASE/auth/v1/admin/users"
if ! curl -fsS -o /dev/null --max-time 5 -H "$HOST_HEADER" "$AUTH_PROBE_URL" 2>/dev/null; then
  PROBE_CODE="$(curl -sS -o /dev/null -w '%{http_code}' --max-time 5 -H "$HOST_HEADER" "$AUTH_PROBE_URL" 2>/dev/null || echo 000)"
  # 401 is the expected "no API key" response — it means Kong forwarded to
  # GoTrue and GoTrue is up. 5xx / 000 means the stack is still starting.
  if [ "$PROBE_CODE" != "401" ]; then
    echo "ERROR: GoTrue is not reachable through Kong (got HTTP $PROBE_CODE from $AUTH_PROBE_URL)." >&2
    echo "       Wait for the Supabase stack to finish starting, then re-run." >&2
    exit 1
  fi
fi

# Check if the user already exists.
LIST_URL="$API_BASE/auth/v1/admin/users?email=$SEED_ADMIN_EMAIL"
EXISTING_RESPONSE="$(curl -fsS -G \
  -H "$HOST_HEADER" \
  -H "apikey: $SERVICE_ROLE_KEY" \
  -H "Authorization: Bearer $SERVICE_ROLE_KEY" \
  "$LIST_URL" 2>/dev/null || true)"

# GoTrue admin user list returns either { "users": [...] } or an array directly
# depending on version. Handle both.
if echo "$EXISTING_RESPONSE" | grep -q "\"email\":\"$SEED_ADMIN_EMAIL\""; then
  echo "Admin user '$SEED_ADMIN_EMAIL' already exists in GoTrue. Skipping."
  if [ -f "$PASSWORD_FILE" ]; then
    echo "Existing password file: $PASSWORD_FILE"
  else
    echo "WARNING: user exists but $PASSWORD_FILE is missing." >&2
    echo "         The password from the first reset cannot be recovered; if" >&2
    echo "         you need to log in, either:" >&2
    echo "           (a) Reset the password via the GoTrue admin API, then update $PASSWORD_FILE manually, or" >&2
    echo "           (b) Delete the user in GoTrue Studio and re-run this script." >&2
  fi
  exit 0
fi

# Generate a 24-character alphanumeric password.
PASSWORD="$(LC_ALL=C tr -dc 'A-Za-z0-9' </dev/urandom | head -c 24 || true)"

CREATE_URL="$API_BASE/auth/v1/admin/users"
CREATE_BODY="$(cat <<EOF
{
  "email": "$SEED_ADMIN_EMAIL",
  "password": "$PASSWORD",
  "email_confirm": true,
  "app_metadata": { "roles": ["my-headless-cms-administrator"] }
}
EOF
)"

CREATE_RESPONSE="$(curl -fsS -X POST \
  -H "$HOST_HEADER" \
  -H "apikey: $SERVICE_ROLE_KEY" \
  -H "Authorization: Bearer $SERVICE_ROLE_KEY" \
  -H "Content-Type: application/json" \
  -d "$CREATE_BODY" \
  "$CREATE_URL")"

# Verify the response contains a UUID id (a minimal sanity check that GoTrue
# actually accepted the create call).
if ! echo "$CREATE_RESPONSE" | grep -Eq '"id":\s*"[0-9a-f-]{36}"'; then
  echo "ERROR: GoTrue did not return a UUID id. Response:" >&2
  echo "$CREATE_RESPONSE" >&2
  exit 1
fi

# Persist credentials. Restrictive perms on the file.
cat > "$PASSWORD_FILE" <<EOF
email=$SEED_ADMIN_EMAIL
password=$PASSWORD
EOF
chmod 600 "$PASSWORD_FILE"

echo ""
echo "==============================================="
echo "  Admin user seeded in GoTrue"
echo "==============================================="
echo "  Email:    $SEED_ADMIN_EMAIL"
echo "  Password: $PASSWORD"
echo "  File:     $PASSWORD_FILE"
echo "==============================================="
echo ""
