#!/usr/bin/env bash
# reset-supabase.sh
# Stops the Supabase stack, wipes its named volumes, and starts fresh.
# Does NOT touch my-cms apps stack. Does NOT remove the supabase_network.
#
# Usage:
#   ./reset-supabase.sh                # Full reset: stop, wipe volumes, start, seed
#   ./reset-supabase.sh --restart      # Restart only: stop + start, keep volumes
#   ./reset-supabase.sh -h | --help    # Show this help

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="docker-compose.supabase.yaml"
ENV_FILE=".env.supabase"

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

# Source SUPABASE_PUBLIC_URL from .env.supabase so the health-poll loop can
# reach GoTrue via Kong (the same URL the seeder uses).
set -a
# shellcheck disable=SC1090
. "$REPO_ROOT/$ENV_FILE"
set +a
: "${SUPABASE_PUBLIC_URL:?SUPABASE_PUBLIC_URL must be set in .env.supabase}"

# Both compose files declare supabase_network as external, so it must exist
# before `up` runs. Create it idempotently here so a fresh checkout (or a
# machine where the network was removed) can run this script with no pre-step.
echo "Ensuring external supabase_network exists..."
docker network create supabase_network >/dev/null 2>&1 || true

if [ "$RESTART_ONLY" -eq 1 ]; then
  echo "Restarting Supabase stack (volumes preserved, no re-seed)..."
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" down --remove-orphans
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d
else
  echo "Stopping Supabase stack..."
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" down -v --remove-orphans

  echo "Removing Supabase named volumes..."
  docker volume rm -f mailpit_data 2>/dev/null || true
  # supabase_storage_data was already removed by `docker compose down -v` above
  # (intentional — media bucket is reset). The db-config named volume (pgsodium
  # decryption key) is also wiped above; it must regenerate alongside the data dir.
  # Note: ./volumes/db/data is a BIND MOUNT (not a named volume) and is NOT wiped
  # by this script. Wipe it manually with: rm -rf volumes/db/data

  echo "Starting Supabase stack fresh..."
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d

  echo "Waiting for GoTrue to become healthy..."
  # Poll GoTrue through Kong: a 401 from /auth/v1/admin/users means Kong
  # forwarded and GoTrue is up (the endpoint requires an apikey, which we
  # don't send). Anything else (5xx, connection refused) means the stack is
  # still starting.
  for i in $(seq 1 60); do
    PROBE_CODE="$(curl -sS -o /dev/null -w '%{http_code}' --max-time 2 "$SUPABASE_PUBLIC_URL/auth/v1/admin/users" 2>/dev/null || echo 000)"
    if [ "$PROBE_CODE" = "401" ]; then
      break
    fi
    if [ "$i" -eq 60 ]; then
      echo "ERROR: GoTrue did not become reachable through Kong within 120s (last code: $PROBE_CODE)." >&2
      echo "       Check 'docker compose -f $COMPOSE_FILE ps' and the auth logs." >&2
      exit 1
    fi
    sleep 2
  done

  echo "Seeding administrator user..."
  bash "$REPO_ROOT/scripts/seed-admin.sh"
fi

echo ""
echo "Supabase stack starting. Check status with:"
echo "  docker compose -f $COMPOSE_FILE ps"
echo "Supabase Studio:  http://localhost:8000"
echo "Kong gateway:     http://localhost:8001"
echo "Mailpit UI:       http://localhost:8025"
