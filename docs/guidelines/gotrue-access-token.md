# GoTrue Access Token Guide

How to obtain an access token directly from GoTrue (Supabase Auth) and use it
to authenticate requests to the GoTrue Admin API or the My-CMS backend.

---

## Architecture Overview

```
┌──────────────┐      POST /auth/v1/token?grant_type=password     ┌──────────────┐
│   Client     │ ────────────────────────────────────────────────▶ │    Kong      │
│  (curl /     │                                                   │  (gateway)   │
│   Postman)   │ ◀──────────────────────────────────────────────── │              │
└──────────────┘         HS256 JWT  signed with JWT_SECRET         └──────┬───────┘
                                                                          │
                                                                   apikey │ header
                                                                   (anon  │ or service_role key)
                                                                          │
                                                                  ┌───────▼───────┐
                                                                  │    GoTrue     │
                                                                  │  (auth v2)    │
                                                                  │               │
                                                                  │ JWT_SECRET ───┤──▶ shared with My-CMS backend
                                                                  │ HS256 sign    │
                                                                  └───────────────┘
```

- **GoTrue** issues HS256 JWTs signed with a shared `JWT_SECRET`.
- **Kong** sits in front of GoTrue and validates `apikey` headers (anon key for
  public endpoints, service-role key for admin endpoints).
- The **My-CMS backend** validates the same JWTs using the same `JWT_SECRET`,
  checking audience (`aud: "authenticated"`) and role claims
  (`app_metadata.roles`).

---

## Prerequisites

Before making any GoTrue API call, ensure your local Supabase stack is running:

```bash
# From repo root
cd deployments/docker-swarm
docker compose -f supabase/docker-compose.yaml up -d
```

Source your environment:

```bash
set -a
source deployments/docker-swarm/supabase/.env
set +a
```

**Key environment variables:**

| Variable              | Purpose                                | Set By               |
|-----------------------|----------------------------------------|----------------------|
| `JWT_SECRET`          | HS256 signing secret (shared)          | `.env` (manual)      |
| `ANON_KEY`            | Pre-signed JWT with `role: "anon"`     | `generate-jwt.sh`    |
| `SERVICE_ROLE_KEY`    | Pre-signed JWT with `role: "service_role"` | `generate-jwt.sh` |
| `SUPABASE_API_HOST`   | Host header if routed through Traefik  | `.env`               |
| `EXPOSE_KONG_PORT`    | Direct Kong port (skip Traefik)        | `.env` (optional)    |

Generate valid keys if you haven't already:

```bash
bash scripts/generate-jwt.sh
# Outputs:
#   ANON_KEY=eyJhbGciOi...
#   SERVICE_ROLE_KEY=eyJhbGciOi...
```

Paste the output into both `deployments/docker-swarm/supabase/.env` and
`deployments/docker-swarm/apps/.env`.

---

## Determining the API Base URL

Depending on your setup, GoTrue is reachable at different URLs:

### Option A: Direct Kong (when `EXPOSE_KONG_PORT` is set)

```bash
# supabase/docker-compose.expose.yaml maps Kong to a host port
API_BASE="http://localhost:${EXPOSE_KONG_PORT}"
# No Host header needed — hitting Kong directly
```

### Option B: Through Traefik (default)

```bash
API_BASE="http://localhost"
# Must pass the Host header so Traefik routes to the supabase-api router
HOST_HEADER="Host: ${SUPABASE_API_HOST}"
```

The examples below use `$API_BASE` as a placeholder. Replace it with your actual
base URL.

---

## Method 1: Password Grant (Get a User Token)

This is the standard OAuth2 flow for an **existing user** to obtain an access
token. Use this to authenticate as a known user (e.g., the admin user seeded by
`scripts/seed-admin.sh`).

### Step 1: Ensure a User Exists

If the admin user has been seeded:

```bash
cat deployments/docker-swarm/supabase/volumes/secrets/admin-password.txt
# email=admin@my-cms.local
# password=XXXXXXXX
```

### Step 2: Exchange Credentials for a Token

```bash
curl -X POST "$API_BASE/auth/v1/token?grant_type=password" \
  -H "apikey: $ANON_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@my-cms.local",
    "password": "YOUR_ADMIN_PASSWORD"
  }'
```

### Response

```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "token_type": "bearer",
  "expires_in": 3600,
  "expires_at": 1719001234,
  "refresh_token": "abc-123...",
  "user": {
    "id": "a1b2c3d4-...",
    "email": "admin@my-cms.local",
    "app_metadata": { "roles": ["my-headless-cms-administrator"] },
    ...
  }
}
```

The `access_token` is an HS256 JWT. You can decode its payload at
[jwt.io](https://jwt.io) to inspect claims:

```json
{
  "sub": "a1b2c3d4-...",
  "email": "admin@my-cms.local",
  "aud": "authenticated",
  "role": "authenticated",
  "app_metadata": {
    "roles": ["my-headless-cms-administrator"]
  },
  "iat": 1718997634,
  "exp": 1719001234
}
```

### Step 3: Use the Token to Call My-CMS Backend

```bash
curl -X GET "$API_BASE/api/v1/admin/posts" \
  -H "Authorization: Bearer $ACCESS_TOKEN"
```

The backend's `SupabaseAuthLayer` will:
1. Validate the HS256 signature against `SUPABASE_JWT_SECRET`.
2. Check `aud` == `"authenticated"`.
3. Check `app_metadata.roles` contains the required role.

### Token Refresh

When the access token expires, use the refresh token:

```bash
curl -X POST "$API_BASE/auth/v1/token?grant_type=refresh_token" \
  -H "apikey: $ANON_KEY" \
  -H "Content-Type: application/json" \
  -d '{"refresh_token": "abc-123..."}'
```

Returns a fresh `access_token`.

---

## Method 2: Service Role Key (Admin Operations)

The `SERVICE_ROLE_KEY` is a pre-signed JWT with `role: "service_role"`. It
bypasses GoTrue's user authentication and is used for **admin-level** operations
like creating users, listing users, or resetting passwords.

> **Important:** The service role key grants full GoTrue admin access. Never
> expose it on the client side. Use it only in server-side scripts, seeders, or
> local development tooling.

### Generate the Key

```bash
bash scripts/generate-jwt.sh
```

The `SERVICE_ROLE_KEY` payload looks like:

```json
{
  "role": "service_role",
  "iss": "supabase",
  "sub": "00000000-0000-0000-0000-000000000000",
  "aud": "authenticated",
  "iat": 1718997634,
  "exp": 1719313234
}
```

### Use the Key for GoTrue Admin API Calls

Create a user:

```bash
curl -X POST "$API_BASE/auth/v1/admin/users" \
  -H "apikey: $SERVICE_ROLE_KEY" \
  -H "Authorization: Bearer $SERVICE_ROLE_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "editor@my-cms.local",
    "password": "secure-password-here",
    "email_confirm": true,
    "app_metadata": {
      "roles": ["my-headless-cms-writer"]
    }
  }'
```

List all users:

```bash
curl "$API_BASE/auth/v1/admin/users" \
  -H "apikey: $SERVICE_ROLE_KEY" \
  -H "Authorization: Bearer $SERVICE_ROLE_KEY"
```

Get a user by ID:

```bash
curl "$API_BASE/auth/v1/admin/users/A1B2C3D4-..." \
  -H "apikey: $SERVICE_ROLE_KEY" \
  -H "Authorization: Bearer $SERVICE_ROLE_KEY"
```

Delete a user:

```bash
curl -X DELETE "$API_BASE/auth/v1/admin/users/A1B2C3D4-..." \
  -H "apikey: $SERVICE_ROLE_KEY" \
  -H "Authorization: Bearer $SERVICE_ROLE_KEY"
```

> **Note:** You need **both** `apikey` and `Authorization: Bearer` headers. Kong
> validates `apikey` to route the request; GoTrue validates the `Authorization`
> header to authorize the operation.

---

## Method 3: Generate a JWT Programmatically (Custom Claims)

When you need a token with specific claims (for testing or automation) without
calling GoTrue, generate one directly using the shared `JWT_SECRET`.

### Node.js

```javascript
const crypto = require('crypto');

function b64url(buf) {
  return buf.toString('base64')
    .replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

function signJwt(payload, secret) {
  const header = { alg: 'HS256', typ: 'JWT' };
  const h = b64url(Buffer.from(JSON.stringify(header)));
  const p = b64url(Buffer.from(JSON.stringify(payload)));
  const sig = b64url(crypto.createHmac('sha256', secret).update(`${h}.${p}`).digest());
  return `${h}.${p}.${sig}`;
}

const secret = process.env.JWT_SECRET;

// User token
const userJwt = signJwt({
  sub: "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
  email: "admin@my-cms.local",
  aud: "authenticated",
  role: "authenticated",
  app_metadata: { roles: ["my-headless-cms-administrator"] },
  iat: Math.floor(Date.now() / 1000),
  exp: Math.floor(Date.now() / 1000) + 3600,
}, secret);

console.log("Bearer", userJwt);
```

### Rust

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

type HmacSha256 = Hmac<Sha256>;

fn sign_jwt(payload: &serde_json::Value, secret: &str) -> String {
    let header = serde_json::json!({"alg": "HS256", "typ": "JWT"});
    let h = URL_SAFE_NO_PAD.encode(serde_json::to_string(&header).unwrap());
    let p = URL_SAFE_NO_PAD.encode(serde_json::to_string(payload).unwrap());
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(format!("{}.{}", h, p).as_bytes());
    let sig = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
    format!("{}.{}.{}", h, p, sig)
}
```

### Shell (using the project script)

```bash
bash scripts/generate-jwt.sh
```

This generates both `ANON_KEY` and `SERVICE_ROLE_KEY` for you.

---

## Postman Setup

### 1. Environment Variables

Create a Postman environment with:

| Variable            | Initial Value                                          |
|---------------------|--------------------------------------------------------|
| `base_url`          | `http://localhost:8000` (or `EXPOSE_KONG_PORT` value)   |
| `anon_key`          | Your `ANON_KEY` value                                  |
| `service_role_key`  | Your `SERVICE_ROLE_KEY` value                          |
| `access_token`      | (auto-filled after login)                               |

### 2. Pre-request Script for Authenticated Requests

To auto-refresh the token before each authenticated request:

```javascript
// Only run if we have a refresh token stored
const refreshToken = pm.environment.get("refresh_token");
if (!refreshToken) return;

const tokenUrl = `${pm.environment.get("base_url")}/auth/v1/token?grant_type=refresh_token`;

pm.sendRequest({
    url: tokenUrl,
    method: 'POST',
    header: {
        'apikey': pm.environment.get("anon_key"),
        'Content-Type': 'application/json',
    },
    body: { mode: 'raw', raw: JSON.stringify({ refresh_token: refreshToken }) }
}, (err, res) => {
    if (!err && res.code === 200) {
        const data = res.json();
        pm.environment.set("access_token", data.access_token);
        pm.environment.set("refresh_token", data.refresh_token);
    }
});
```

### 3. Collection Authorization

Set collection auth to **Bearer Token** with value `{{access_token}}`.

---

## GoTrue API Endpoints Quick Reference

### Public Endpoints (require `ANON_KEY` as `apikey`)

| Method | Path                                  | Description              |
|--------|---------------------------------------|--------------------------|
| POST   | `/auth/v1/token?grant_type=password`   | Sign in with password    |
| POST   | `/auth/v1/token?grant_type=refresh_token` | Refresh access token |
| POST   | `/auth/v1/signup`                     | Sign up (disabled by default) |
| GET    | `/auth/v1/user`                       | Get current user         |

### Admin Endpoints (require `SERVICE_ROLE_KEY` as `apikey` + `Authorization`)

| Method | Path                              | Description           |
|--------|-----------------------------------|-----------------------|
| POST   | `/auth/v1/admin/users`            | Create a user         |
| GET    | `/auth/v1/admin/users`            | List all users        |
| GET    | `/auth/v1/admin/users/{id}`       | Get user by ID        |
| PUT    | `/auth/v1/admin/users/{id}`       | Update user           |
| DELETE | `/auth/v1/admin/users/{id}`       | Delete user           |
| POST   | `/auth/v1/admin/generate_link`    | Generate email link   |

Full GoTrue API reference: https://supabase.com/docs/reference/gotrue

---

## Backend Token Validation

The My-CMS backend (`SupabaseAuthLayer` in
`apps/api/src/common/supabase_auth.rs`) validates tokens as follows:

1. **HS256** — Attempts validation using `SUPABASE_JWT_SECRET` as the
   symmetric key. Checks `aud` claim matches the configured audience
   (`"authenticated"`).
2. **RS256 fallback** — If HS256 fails and the token's `alg` header is not
   HS256, fetches the JWKS document from
   `{SUPABASE_URL}/auth/v1/.well-known/jwks.json` and validates with the
   matching `kid`.
3. **Role enforcement** — Checks `app_metadata.roles` (OR semantics: any
   match passes).

If validation succeeds, a `SupabaseToken` extension is inserted into the
request, providing `user_id()`, `email()`, and `role()` accessors to
downstream handlers.

---

## Troubleshooting

### "Invalid login credentials" on password grant

- Ensure the user exists (check with `GET /auth/v1/admin/users` using the
  service role key).
- Ensure `email_confirm` was set to `true` when the user was created.
- Ensure `GOTRUE_DISABLE_SIGNUP` does not interfere (it only blocks
  signups, not logins).

### 401 from GoTrue

- Verify `apikey` header matches `ANON_KEY` (for public endpoints) or
  `SERVICE_ROLE_KEY` (for admin endpoints).
- For admin endpoints, verify both `apikey` and `Authorization: Bearer`
  headers are present.

### 401 from My-CMS Backend

- Verify the `JWT_SECRET` in the backend's `.env` matches the one used by
  GoTrue. They must be identical.
- Check the token's `exp` claim — it may have expired.
- Verify `aud` == `"authenticated"`.

### 403 from My-CMS Backend

- The user's `app_metadata.roles` does not include any of the roles required
  by the endpoint:
  - Writer / Admin endpoints: `["my-headless-cms-writer", "my-headless-cms-administrator"]`
  - Admin-only endpoints: `["my-headless-cms-administrator"]`
- Add the missing role via the GoTrue admin API:
  ```bash
  curl -X PUT "$API_BASE/auth/v1/admin/users/USER_ID" \
    -H "apikey: $SERVICE_ROLE_KEY" \
    -H "Authorization: Bearer $SERVICE_ROLE_KEY" \
    -H "Content-Type: application/json" \
    -d '{"app_metadata": {"roles": ["my-headless-cms-administrator"]}}'
  ```

### "GoTrue is not reachable" during seeding

Wait for the Supabase stack to finish starting, then re-run
`bash deployments/docker-swarm/supabase/reset.sh`.
