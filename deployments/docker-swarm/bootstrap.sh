#!/usr/bin/env bash
# bootstrap.sh
# One-time setup: creates the external supabase_network that both Compose
# files join. Idempotent — safe to run multiple times.
#
# Lives under deployments/docker-swarm/ so the deployment surface stays
# isolated from the application source tree.
#
# Usage:
#   ./bootstrap.sh
#
# The reset scripts (reset-supabase.sh, reset-apps.sh) already ensure the
# network exists before `up`, so you only need to run bootstrap.sh
# explicitly if you intend to bring the stack up by hand (e.g.
# `docker compose -f docker-compose.supabase.yaml up -d` without using a
# reset script).

set -euo pipefail

NETWORK="supabase_network"

if docker network inspect "$NETWORK" >/dev/null 2>&1; then
  echo "Network '$NETWORK' already exists — nothing to do."
  exit 0
fi

echo "Creating external network '$NETWORK'..."
docker network create "$NETWORK"

echo ""
echo "Done. Next steps:"
echo "  cp .env.supabase.example .env.supabase"
echo "  cp .env.my-cms.example   .env.my-cms"
echo "  # edit secrets in both .env files"
echo "  rm -rf volumes/db/data   # only on first boot — let init SQL run fresh"
echo "  docker compose -f docker-compose.supabase.yaml --env-file .env.supabase up -d"
echo "  docker compose -f docker-compose.my-cms.yaml   --env-file .env.my-cms   up -d"
echo "  docker compose -f docker-compose.traefik.yaml  up -d"
