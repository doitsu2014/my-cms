#!/usr/bin/env bash
# reset-supabase.sh
# Stops the Supabase stack, wipes its named volumes, and starts fresh.
# Does NOT touch my-cms apps stack. Does NOT remove the supabase_network.

set -euo pipefail

COMPOSE_FILE="docker-compose.supabase.yaml"
ENV_FILE=".env.supabase"

# Both compose files declare supabase_network as external, so it must exist
# before `up` runs. Create it idempotently here so a fresh checkout (or a
# machine where the network was removed) can run this script with no pre-step.
echo "Ensuring external supabase_network exists..."
docker network create supabase_network >/dev/null 2>&1 || true

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

echo ""
echo "Supabase stack starting. Check status with:"
echo "  docker compose -f $COMPOSE_FILE ps"
echo "Supabase Studio:  http://localhost:8000"
echo "Kong gateway:     http://localhost:8001"
echo "Mailpit UI:       http://localhost:8025"
