#!/usr/bin/env bash
# scripts/generate-jwt.sh
# Generates valid HS256 ANON_KEY and SERVICE_ROLE_KEY JWTs signed with the
# project's JWT_SECRET. GoTrue and the rest of the Supabase stack validate
# these as JWTs on every API call, so they MUST be real tokens (not the
# literal "devkey" placeholder).
#
# Usage: bash scripts/generate-jwt.sh
# Writes ANON_KEY and SERVICE_ROLE_KEY to stdout, one per line, in the form
# `KEY=value`. Pipe into `deployments/docker-swarm/supabase/.env` and
# `deployments/docker-swarm/apps/.env` to update both.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Load JWT_SECRET from supabase/.env (or apps/.env — they share the value).
for f in deployments/docker-swarm/supabase/.env deployments/docker-swarm/apps/.env; do
  if [ -f "$REPO_ROOT/$f" ]; then
    JWT_SECRET="$(grep -E '^JWT_SECRET=' "$REPO_ROOT/$f" | head -1 | cut -d= -f2-)"
    if [ -n "$JWT_SECRET" ]; then break; fi
  fi
done

if [ -z "${JWT_SECRET:-}" ]; then
  echo "ERROR: JWT_SECRET not found in deployments/docker-swarm/supabase/.env or deployments/docker-swarm/apps/.env" >&2
  exit 1
fi

# Generate the two JWTs with node. Falls back to openssl + base64 if node is
# not available.
if command -v node >/dev/null 2>&1; then
  node -e "
    const crypto = require('crypto');
    const secret = process.env.JWT_SECRET;
    const b64url = (buf) =>
      buf.toString('base64').replace(/\+/g, '-').replace(/\//g, '_').replace(/=+\$/, '');
    const makeJwt = (payload) => {
      const header = { alg: 'HS256', typ: 'JWT' };
      const h = b64url(Buffer.from(JSON.stringify(header)));
      const p = b64url(Buffer.from(JSON.stringify(payload)));
      const sig = b64url(crypto.createHmac('sha256', secret).update(h + '.' + p).digest());
      return h + '.' + p + '.' + sig;
    };
    const now = Math.floor(Date.now() / 1000);
    const exp = now + 31536000; // 1 year
    const anonPayload = {
      role: 'anon', iss: 'supabase', iat: now, exp,
      sub: '00000000-0000-0000-0000-000000000000', aud: 'authenticated',
      email: '', phone: '',
      app_metadata: { provider: 'anon', providers: ['anon'] },
      user_metadata: {}, is_anonymous: true
    };
    const servicePayload = {
      role: 'service_role', iss: 'supabase', iat: now, exp,
      sub: '00000000-0000-0000-0000-000000000000', aud: 'authenticated',
      email: '', phone: '',
      app_metadata: { provider: 'service_role', providers: ['service_role'] },
      user_metadata: {}, is_anonymous: false
    };
    process.stdout.write('ANON_KEY=' + makeJwt(anonPayload) + '\n');
    process.stdout.write('SERVICE_ROLE_KEY=' + makeJwt(servicePayload) + '\n');
  " JWT_SECRET="$JWT_SECRET"
else
  echo "ERROR: node is required (no fallback implemented). Install node >= 16." >&2
  exit 1
fi
