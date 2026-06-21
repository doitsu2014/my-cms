#!/usr/bin/env bash
# reset.sh (Traefik proxy stack)
# Ensures Traefik is running. The proxy has no state, so "reset" is a
# no-op — this script just brings the container up (idempotent).
#
# Lives under deployments/docker-swarm/traefik/ so the proxy deployment
# surface stays isolated from the apps and Supabase stacks. Paths in this
# script are relative to the script's own directory.
#
# Usage:
#   ./reset.sh            # ensure Traefik is up
#   ./reset.sh --restart  # stop and start Traefik (preserves nothing — it has no state)
#   ./reset.sh -h | --help

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

COMPOSE_FILE="docker-compose.yaml"
ENV_FILE=".env"

usage() {
  cat <<'EOF'
reset.sh — Bring up (or restart) the Traefik reverse proxy

Usage:
  ./reset.sh            Ensure Traefik is up. Idempotent — safe to re-run.
  ./reset.sh --restart  Stop and start Traefik. The proxy has no state, so
                        this is equivalent to ./reset.sh except containers
                        are recreated.
  ./reset.sh -h | --help  Show this help.

By default this script does a `docker compose up -d` (no-op if already up).
The Traefik proxy keeps no application state; no volumes to wipe.

Env file: ./.env (required by docker-compose.yaml via env_file: ./.env).
The Traefik dynamic config (./dynamic/my-cms.yml) consumes
CMS_HOST, CMS_ADMIN_ORIGIN, CMS_SUPABASE_ORIGIN, and STUDIO_BASIC_AUTH_USERS.
EOF
}

RESTART_ONLY=0
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

# Both the apps and Supabase compose files declare supabase_network as
# external, and the Traefik proxy joins it too. Create the network
# idempotently here so this script can run with no pre-step.
echo "Ensuring external supabase_network exists..."
docker network create supabase_network >/dev/null 2>&1 || true

if [ ! -f "$ENV_FILE" ]; then
  echo "ERROR: $ENV_FILE not found. Copy .env.example to .env and try again." >&2
  exit 1
fi

if [ "$RESTART_ONLY" -eq 1 ]; then
  echo "Restarting Traefik..."
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" down --remove-orphans
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d
else
  echo "Ensuring Traefik is up..."
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d
fi

echo ""
echo "Traefik running. Dashboard: http://localhost:8080"
echo "Routes:"
echo "  http://\${CMS_HOST:-localhost}/          → admin frontend (when Host header matches)"
echo "  http://\${CMS_HOST:-localhost}/api/...   → my-cms API"
echo "  http://\${CMS_HOST:-localhost}/auth/...  → Kong / Supabase API"
echo "  http://\${CMS_HOST:-localhost}/ (Studio) → Supabase Studio (Basic Auth)"