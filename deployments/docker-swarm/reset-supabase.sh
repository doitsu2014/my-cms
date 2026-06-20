#!/usr/bin/env bash
# reset-supabase.sh
# Stops the Supabase stack, wipes its named volumes, and starts fresh.
# Does NOT touch my-cms apps stack. Does NOT remove the supabase_network.
#
# Lives under deployments/docker-swarm/ so the deployment surface stays
# isolated from the application source tree. Paths in this script are
# relative to the script's own directory; the project root is resolved
# dynamically so the script can be invoked from anywhere.
#
# Usage:
#   ./reset-supabase.sh                # Full reset: stop, wipe volumes, start, seed
#   ./reset-supabase.sh --restart      # Restart only: stop + start, keep volumes
#   ./reset-supabase.sh -h | --help    # Show this help

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Project root is the parent of deployments/, two levels up from this script.
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
# Always run docker compose from the script's own directory so the relative
# volume mounts and build contexts in docker-compose.supabase.yaml resolve.
cd "$SCRIPT_DIR"

COMPOSE_FILE="docker-compose.supabase.yaml"
EXPOSE_FILE="docker-compose.supabase.expose.yaml"
ENV_FILE=".env.supabase"

# Source .env.supabase BEFORE the override-detection block below, so the
# EXPOSE_*_PORT variables are populated in the shell when this script
# decides whether to include docker-compose.supabase.expose.yaml. Sourcing
# here also makes shared secrets (JWT_SECRET, ANON_KEY, etc.) available to
# `docker compose --env-file` resolution and downstream scripts (seed-admin.sh).
set -a
# shellcheck disable=SC1090
. "$ENV_FILE"
set +a

# Build the optional override-file flag. When any EXPOSE_*_PORT is set in
# .env.supabase, include docker-compose.supabase.expose.yaml so that those
# services are reachable directly on the Docker host (in addition to the
# Traefik routes). When all are empty (default), no override is included.
COMPOSE_FILES=("-f" "$COMPOSE_FILE")
if [ -n "${EXPOSE_STUDIO_PORT:-}${EXPOSE_DB_PORT:-}${EXPOSE_KONG_PORT:-}${EXPOSE_AUTH_PORT:-}" ]; then
  COMPOSE_FILES+=("-f" "$EXPOSE_FILE")
  echo "Direct port exposure enabled (EXPOSE_*_PORT set). Including override file."
fi

# Default mode: full reset (wipe named volumes + seed administrator).
RESTART_ONLY=0

usage() {
  cat <<'EOF'
reset-supabase.sh — Reset or restart the Supabase Docker stack

Usage:
  ./reset-supabase.sh                Full reset: stop, wipe named volumes, start,
                                     wait for GoTrue, seed administrator user.
  ./reset-supabase.sh --restart      Restart only: stop + start. Named volumes and
                                     bind mounts are preserved, and the
                                     administrator user is NOT re-seeded.
  ./reset-supabase.sh -h | --help    Show this help.

By default this script wipes the Supabase named volumes (e.g. mailpit_data) and
re-seeds the administrator user. Pass --restart to skip both: data and
credentials are preserved and only the containers are recycled.

Note: ./volumes/db/data is a BIND MOUNT, not a named volume. It is NEVER wiped
by this script. Wipe it manually with: rm -rf volumes/db/data
EOF
}

while [ $# -gt 0 ]; do
  case "$1" in
    --restart|-r)
      RESTART_ONLY=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

# Both compose files declare supabase_network as external, so it must exist
# before `up` runs. Create it idempotently here so a fresh checkout (or a
# machine where the network was removed) can run this script with no pre-step.
echo "Ensuring external supabase_network exists..."
docker network create supabase_network >/dev/null 2>&1 || true

if [ "$RESTART_ONLY" -eq 1 ]; then
  echo "Restarting Supabase stack (volumes preserved, no re-seed)..."
  docker compose "${COMPOSE_FILES[@]}" --env-file "$ENV_FILE" down --remove-orphans
  docker compose "${COMPOSE_FILES[@]}" --env-file "$ENV_FILE" up -d
else
  echo "Stopping Supabase stack..."
  docker compose "${COMPOSE_FILES[@]}" --env-file "$ENV_FILE" down -v --remove-orphans

  echo "Removing Supabase named volumes..."
  docker volume rm -f mailpit_data 2>/dev/null || true
  # supabase_storage_data was already removed by `docker compose down -v` above
  # (intentional — media bucket is reset). The db-config named volume (pgsodium
  # decryption key) is also wiped above; it must regenerate alongside the data dir.
  # Note: ./volumes/db/data is a BIND MOUNT (not a named volume) and is NOT wiped
  # by this script. Wipe it manually with: rm -rf volumes/db/data

  echo "Starting Supabase stack fresh..."
  docker compose "${COMPOSE_FILES[@]}" --env-file "$ENV_FILE" up -d

  # Traefik must be up BEFORE the GoTrue probe loop, because the legacy
  # probe path goes through Traefik → Kong (port 80, Host: $SUPABASE_API_HOST).
  # If Traefik is started only at the end of the script, the probe never
  # succeeds even when every Supabase container is healthy.
  echo "Ensuring Traefik is running..."
  docker compose -f docker-compose.traefik.yaml up -d

  echo "Waiting for GoTrue to become healthy..."
    # Poll GoTrue through Kong. A 401 from /auth/v1/admin/users means Kong
    # forwarded and GoTrue is up (the endpoint requires an apikey, which we
    # don't send). Anything else (5xx, connection refused) means the stack is
    # still starting.
    #
    # When EXPOSE_KONG_PORT is set, probe Kong directly on the host — bypasses
    # Traefik so the check is faster and independent of Traefik state.
    # Otherwise fall back to the Traefik-routed path.
    if [ -n "${EXPOSE_KONG_PORT:-}" ]; then
      PROBE_URL="http://localhost:${EXPOSE_KONG_PORT}/auth/v1/admin/users"
      PROBE_LABEL="Kong (localhost:${EXPOSE_KONG_PORT})"
    else
      PROBE_URL="http://localhost/auth/v1/admin/users"
      PROBE_LABEL="Traefik → Kong (Host: ${SUPABASE_API_HOST})"
    fi
    for i in $(seq 1 60); do
      PROBE_CODE="$(curl -sS -o /dev/null -w '%{http_code}' --max-time 2 -H "Host: ${SUPABASE_API_HOST}" "$PROBE_URL" 2>/dev/null || echo 000)"
    if [ "$PROBE_CODE" = "401" ]; then
      echo "GoTrue healthy via ${PROBE_LABEL}."
      break
    fi
    if [ "$i" -eq 60 ]; then
      echo "ERROR: GoTrue did not become reachable through ${PROBE_LABEL} within 120s (last code: $PROBE_CODE)." >&2
      echo "       Check 'docker compose -f $COMPOSE_FILE ps' and the auth logs." >&2
      exit 1
    fi
    sleep 2
  done

  echo "Seeding administrator user..."
  bash "$REPO_ROOT/scripts/seed-admin.sh"
fi

echo "Ensuring Traefik is running (final)..."
docker compose -f docker-compose.traefik.yaml up -d

echo ""
echo "Supabase stack starting. Check status with:"
echo "  docker compose -f $COMPOSE_FILE ps"
echo "Supabase Studio:  https://supabase.ducth.dev       (Basic Auth + Studio login)"
echo "Kong gateway:     https://supabase-api.ducth.dev    (JWT via GoTrue)"
echo "Mailpit UI:       internal Docker network only"
