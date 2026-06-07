#!/usr/bin/env bash
# reset-apps.sh
# Stops the my-cms apps stack and starts fresh.
# Does NOT touch the Supabase stack.

set -euo pipefail

COMPOSE_FILE="docker-compose.my-cms.yaml"
ENV_FILE=".env.my-cms"

# Both compose files declare supabase_network as external. The supabase
# stack is responsible for declaring it, but this script must still work
# when invoked before the supabase stack has been started (e.g. CI). Create
# the network idempotently so no manual pre-step is required.
echo "Ensuring external supabase_network exists..."
docker network create supabase_network >/dev/null 2>&1 || true

echo "Stopping apps stack..."
docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" down -v --remove-orphans

echo "Starting apps stack fresh..."
docker compose -f "$COMPOSE_FILE" --env-file "$ENV_FILE" up -d

echo ""
echo "Apps stack starting. Check status with:"
echo "  docker compose -f $COMPOSE_FILE ps"
echo "my-cms API:       http://localhost:8989"
echo "Frontend admin:   http://localhost:3002"
echo "Jaeger UI:        http://localhost:16686"
