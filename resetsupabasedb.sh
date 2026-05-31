#!/usr/bin/env bash
# resetsupabasedb.sh
# Stops all services, removes volumes, and starts fresh.
# WARNING: This deletes ALL data (database, storage, mailpit).

set -euo pipefail

echo "Stopping all services..."
docker compose down -v --remove-orphans

echo "Removing named volumes..."
docker volume rm -f supabase_db_data supabase_storage_data mailpit_data 2>/dev/null || true

echo "Starting fresh..."
docker compose up -d

echo ""
echo "Services starting. Check status with: docker compose ps"
echo "Supabase Studio: http://localhost:8000"
echo "my-cms API:      http://localhost:8989"
echo "Frontend:        http://localhost:3002"
echo "Jaeger UI:       http://localhost:16686"
echo "Mailpit:         http://localhost:8025"
