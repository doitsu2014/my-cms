#!/usr/bin/env bash
# reset-apps.sh
# Stops the my-cms apps stack and starts fresh.
# Does NOT touch the Supabase stack.
#
# Lives under deployments/docker-swarm/ so the deployment surface stays
# isolated from the application source tree. Paths in this script are
# relative to the script's own directory; the script can be invoked from
# anywhere.
#
# Usage:
#   ./reset-apps.sh                       # Full reset: stop, wipe volumes, start
#   ./reset-apps.sh --restart             # Restart only: stop + start, keep volumes
#   ./reset-apps.sh --rebuild [SVC ...]   # Rebuild target services, keep volumes
#   ./reset-apps.sh -h | --help           # Show this help

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Always run docker compose from the script's own directory so the relative
# volume mounts and build contexts in docker-compose.my-cms.yaml resolve.
cd "$SCRIPT_DIR"

COMPOSE_FILE="docker-compose.my-cms.yaml"
ENV_FILE=".env.my-cms"

# Mode flags. At most one of these is honored; the order of precedence is
# --rebuild > --restart > default (full reset).
RESTART_ONLY=0
REBUILD=0
REBUILD_SERVICES=()

usage() {
  cat <<'EOF'
reset-apps.sh — Reset, restart, or rebuild the my-cms apps Docker stack

Usage:
  ./reset-apps.sh                       Full reset: stop, wipe named volumes, start.
  ./reset-apps.sh --restart             Restart only: stop + start. Named volumes
                                        and bind mounts are preserved.
  ./reset-apps.sh --rebuild [SVC ...]   Rebuild the image(s) for the given
                                        services (or all services if none are
                                        listed) and (re)start the stack. Named
                                        volumes and bind mounts are preserved.
  ./reset-apps.sh -h | --help           Show this help.

Examples:
  ./reset-apps.sh --restart
  ./reset-apps.sh --rebuild                       # rebuild every service image
  ./reset-apps.sh --rebuild api frontend          # rebuild only api and frontend
  ./reset-apps.sh --rebuild api --restart         # rebuild api, then restart stack

By default this script wipes the apps stack's named volumes. Pass --restart to
recycle containers without losing data, or --rebuild to rebuild one or more
service images and start them (data is preserved). If both --rebuild and
--restart are passed, --rebuild takes precedence.
EOF
}

while [ $# -gt 0 ]; do
  case "$1" in
    --restart|-r)
      RESTART_ONLY=1
      shift
      ;;
    --rebuild|-b)
      REBUILD=1
      shift
      # Collect all following non-flag args as service names. Anything starting
      # with '-' (e.g. another flag) terminates the service list.
      while [ $# -gt 0 ] && [[ "$1" != -* ]]; do
        REBUILD_SERVICES+=("$1")
        shift
      done
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

# Both compose files declare supabase_network as external. The supabase
# stack is responsible for declaring it, but this script must still work
# when invoked before the supabase stack has been started (e.g. CI). Create
# the network idempotently so no manual pre-step is required.
echo "Ensuring external supabase_network exists..."
docker network create supabase_network >/dev/null 2>&1 || true

if [ "$REBUILD" -eq 1 ]; then
  if [ "${#REBUILD_SERVICES[@]}" -gt 0 ]; then
    echo "Rebuilding services: ${REBUILD_SERVICES[*]} (volumes preserved)..."
    # `up -d --build <services>` builds the listed service images, recreates
    # only those containers, and leaves the rest of the stack untouched.
    # If the stack isn't running yet, this also starts everything.
    docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d --build "${REBUILD_SERVICES[@]}"
  else
    echo "Rebuilding all services (volumes preserved)..."
    docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d --build
  fi
elif [ "$RESTART_ONLY" -eq 1 ]; then
  echo "Restarting apps stack (volumes preserved)..."
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" down --remove-orphans
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d
else
  echo "Stopping apps stack..."
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" down -v --remove-orphans

  echo "Starting apps stack fresh..."
  docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d
fi

echo ""
echo "Apps stack starting. Check status with:"
echo "  docker compose -f $COMPOSE_FILE ps"
echo "my-cms API:       http://localhost:8989"
echo "Frontend admin:   http://localhost:3002"
echo "Jaeger UI:        http://localhost:16686"
