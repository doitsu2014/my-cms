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
# The reset scripts (supabase/reset.sh, apps/reset.sh) already ensure the
# network exists before `up`, so you only need to run bootstrap.sh
# explicitly if you intend to bring the stack up by hand (e.g.
# `docker compose -f supabase/docker-compose.yaml up -d` without using a
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
echo "  cp supabase/.env.example supabase/.env"
echo "  cp apps/.env.example     apps/.env"
echo "  cp traefik/.env.example  traefik/.env    # optional — defaults work for local dev"
echo "  # edit secrets in supabase/.env and apps/.env"
echo "  rm -rf supabase/volumes/db/data   # only on first boot — let init SQL run fresh"
echo "  ./supabase/reset.sh    # starts Supabase + Traefik + seeds admin"
echo "  ./apps/reset.sh        # starts my-cms API + frontend + Jaeger"
