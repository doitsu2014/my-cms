#!/bin/sh
# Generate runtime config from environment variables (no rebuild required).
# Uses envsubst to substitute ${VAR} placeholders in config.js.template.
set -e

envsubst < /app/public/config.js.template > /app/public/config.js

exec pnpm run dev --port 3002 --host 0.0.0.0
