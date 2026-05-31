# Docker Compose Setup Implementation Plan (Plan 0)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking. This plan must be completed first — Plans 1 and 2 (auth/vector + image/storage) depend on it.

**Goal:** Create a single `docker compose up` command that starts the full Supabase stack + my-cms API + frontend admin panel + Jaeger tracing.

**Architecture:** All services on a shared `supabase_network` bridge. Supabase services use the standard self-hosted docker-compose from `supabase/supabase`. The my-cms API is built from `services/Dockerfile`. The frontend runs an rsbuild dev server. Jaeger provides OTLP tracing on port 4317.

**Tech Stack:** Docker Compose v3.8+, Kong 2.8, PostgreSQL 15 + pgvector, GoTrue, PostgREST, Supabase Studio, Jaeger all-in-one, Rust (cargo build), Node.js (rsbuild dev).

**Reference:** Official Supabase docker-compose at [supabase/supabase](https://github.com/supabase/supabase/blob/master/docker/docker-compose.yml)

---

### Task 0.1: Create Project Root `.env` Template

**Files:**
- Create: `.env.example`
- Copy: `.env.example` → `.env` (user fills secrets manually)

- [ ] **Step 1: Write `.env.example`**

```bash
# ============================================
# Supabase Configuration
# ============================================
# Generate with: openssl rand -hex 32
# Minimum 32 characters
POSTGRES_PASSWORD=change-me-to-a-secure-password
JWT_SECRET=change-me-to-a-secure-jwt-secret-at-least-32-chars

# Derived from JWT_SECRET — generate with supabase CLI or keep as-is for local dev
# Format: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOi... 
ANON_KEY=change-me
SERVICE_ROLE_KEY=change-me

# Dashboard credentials
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=admin

# ============================================
# Supabase External URLs (for compose setup)
# ============================================
SUPABASE_PUBLIC_URL=http://localhost:8000
SITE_URL=http://localhost:3002
API_EXTERNAL_URL=http://localhost:8989

# SMTP / Email (mailpit for local dev)
SMTP_ADMIN_EMAIL=admin@example.com
SMTP_HOST=mailpit
SMTP_PORT=1025
SMTP_USER=
SMTP_PASS=
SMTP_SENDER_NAME=My CMS

# ============================================
# my-cms API Configuration
# ============================================
DATABASE_URL=postgresql://postgres:${POSTGRES_PASSWORD}@db:5432/postgres
DATABASE_SCHEMA=public
HOST=0.0.0.0
PORT=8989
RUST_LOG=info

# Tracing
ENABLED_OTLP_EXPORTER=true
OTEL_SERVICE_NAME=my-cms-headless-api
SERVICE_NAME=my-cms-headless-api
OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://jaeger:4317
OTEL_TRACES_SAMPLER=always_on

# OpenAI (optional — set for AI translation)
OPENAI_API_KEY=

# Media
MEDIA_BASE_URL=http://localhost:8989
MAX_BODY_LENGTH=10485760

# Supabase Storage (for media, replaces S3)
SUPABASE_URL=http://localhost:8000
SUPABASE_ANON_KEY=${ANON_KEY}
SUPABASE_SERVICE_ROLE_KEY=${SERVICE_ROLE_KEY}
SUPABASE_STORAGE_BUCKET=media

# Auth (Keycloak External — keep until Plan 1 is executed)
KEYCLOAK_ISSUER=https://my-ids-admin.ducth.dev
KEYCLOAK_REALM=my-blogs
AUTHORIZATION_AUDIENCE=my-cms-headless-api

# ============================================
# Frontend Configuration (rsbuild build-time)
# ============================================
PUBLIC_KEYCLOAK_URL=https://my-ids-admin.ducth.dev
PUBLIC_KEYCLOAK_REALM=master
PUBLIC_KEYCLOAK_CLIENT_ID=my-blogs-admin-localhost
PUBLIC_KEYCLOAK_SCOPE=my-headless-cms-api-all email openid profile offline_access
PUBLIC_GRAPHQL_API_URL=http://localhost:8989/graphql
PUBLIC_GRAPHQL_CACHE_API_URL=http://localhost:8989/graphql
PUBLIC_REST_API_URL=http://localhost:8989/api
PUBLIC_MEDIA_UPLOAD_API_URL=http://localhost:8989/api/media/upload
```

- [ ] **Step 2: Commit**

```bash
git add .env.example
git commit -m "feat: add .env.example for docker compose setup"
```

---

### Task 0.2: Create `docker-compose.yml`

**Files:**
- Create: `docker-compose.yml`

- [ ] **Step 1: Write the compose file**

```yaml
# docker-compose.yml
# Full Supabase stack + my-cms API + frontend + Jaeger for local development.
#
# Usage:
#   cp .env.example .env   # then edit secrets in .env
#   docker compose up -d
#
# Access:
#   Supabase Studio:   http://localhost:8000      (admin / admin)
#   Supabase API:      http://localhost:8001      (Kong gateway)
#   my-cms API:        http://localhost:8989
#   Frontend Admin:    http://localhost:3002
#   Jaeger UI:         http://localhost:16686
#   Mailpit (email):   http://localhost:8025
#   Clean reset:       ./resetsupabasedb.sh

version: "3.8"

services:
  # ========================================
  # Supabase: PostgreSQL Database
  # ========================================
  db:
    container_name: supabase-db
    image: supabase/postgres:15.6.1.148
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "pg_isready", "-U", "postgres", "-h", "localhost"]
      interval: 5s
      timeout: 5s
      retries: 10
    command:
      - postgres
      - -c
      - config_file=/etc/postgresql/postgresql.conf
      - -c
      - log_min_messages=fatal
    environment:
      POSTGRES_HOST: /var/run/postgresql
      PGDATA: /var/lib/postgresql/data
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    volumes:
      - supabase_db_data:/var/lib/postgresql/data
      - ./volumes/db/init:/docker-entrypoint-initdb.d:ro
    networks:
      - supabase_network

  # ========================================
  # Supabase: Connection Pooler
  # ========================================
  supavisor:
    container_name: supabase-supavisor
    image: supabase/supavisor:2.7.6
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "supavisor", "healthcheck"]
      interval: 5s
      timeout: 5s
      retries: 5
    environment:
      SUPAVISOR_DATABASE_URL: postgresql://postgres:${POSTGRES_PASSWORD}@db:5432/postgres
      SUPAVISOR_JWT_SECRET: ${JWT_SECRET}
      SUPAVISOR_POOL_SIZE: 15
      SUPAVISOR_MAX_CLIENT_CONN: 100
      SUPAVISOR_DEFAULT_POOL_SIZE: 15
      SUPAVISOR_TENANT_EXTERNAL_ID: your-tenant-id
      SUPAVISOR_PROXY_PORT: 5432
      PORT: 5432
    ports:
      - "5432:5432"
    depends_on:
      db:
        condition: service_healthy
    networks:
      - supabase_network

  # ========================================
  # Supabase: GoTrue Authentication
  # ========================================
  auth:
    container_name: supabase-auth
    image: supabase/gotrue:v2.179.6
    restart: unless-stopped
    healthcheck:
      test:
        [
          "CMD",
          "wget",
          "--no-verbose",
          "--tries=1",
          "--spider",
          "http://localhost:9999/health",
        ]
      interval: 5s
      timeout: 5s
      retries: 5
    environment:
      GOTRUE_API_HOST: 0.0.0.0
      GOTRUE_API_PORT: 9999
      API_EXTERNAL_URL: ${API_EXTERNAL_URL}
      GOTRUE_DB_DATABASE_URL: postgresql://supabase_auth_admin:${POSTGRES_PASSWORD}@db:5432/postgres
      GOTRUE_SITE_URL: ${SITE_URL}
      GOTRUE_URI_ALLOW_LIST: ""
      GOTRUE_JWT_ADMIN_ROLES: service_role
      GOTRUE_JWT_AUD: authenticated
      GOTRUE_JWT_DEFAULT_GROUP_NAME: authenticated
      GOTRUE_JWT_EXP: 3600
      GOTRUE_JWT_SECRET: ${JWT_SECRET}
      GOTRUE_EXTERNAL_EMAIL_ENABLED: true
      GOTRUE_EXTERNAL_ANONYMOUS_USERS_ENABLED: false
      GOTRUE_MAILER_AUTOCONFIRM: true
      GOTRUE_MAILER_SUBJECTS_CONFIRMATION: "Confirm your email"
      GOTRUE_SMTP_ADMIN_EMAIL: ${SMTP_ADMIN_EMAIL}
      GOTRUE_SMTP_HOST: ${SMTP_HOST}
      GOTRUE_SMTP_PORT: ${SMTP_PORT}
      GOTRUE_SMTP_USER: ${SMTP_USER}
      GOTRUE_SMTP_PASS: ${SMTP_PASS}
      GOTRUE_SMTP_SENDER_NAME: ${SMTP_SENDER_NAME}
      GOTRUE_LOG_LEVEL: warn
      GOTRUE_DISABLE_SIGNUP: false
      GOTRUE_PASSWORD_MIN_LENGTH: 8
    depends_on:
      db:
        condition: service_healthy
    networks:
      - supabase_network

  # ========================================
  # Supabase: PostgREST (Auto REST API)
  # ========================================
  rest:
    container_name: supabase-rest
    image: postgrest/postgrest:v14.1
    restart: unless-stopped
    environment:
      PGRST_DB_URI: postgresql://authenticator:${POSTGRES_PASSWORD}@db:5432/postgres
      PGRST_DB_SCHEMAS: public,storage,graphql_public
      PGRST_DB_ANON_ROLE: anon
      PGRST_DB_PLAN_ENABLED: false
      PGRST_JWT_SECRET: ${JWT_SECRET}
      PGRST_DB_USE_LEGACY_GUCS: false
      PGRST_APP_SETTINGS_JWT_SECRET: ${JWT_SECRET}
      PGRST_APP_SETTINGS_JWT_EXP: "3600"
    depends_on:
      db:
        condition: service_healthy
    networks:
      - supabase_network

  # ========================================
  # Supabase: Realtime (WebSocket)
  # ========================================
  realtime:
    container_name: supabase-realtime
    image: supabase/realtime:v2.67.1
    restart: unless-stopped
    environment:
      PORT: 4000
      DB_HOST: db
      DB_PORT: 5432
      DB_USER: postgres
      DB_PASSWORD: ${POSTGRES_PASSWORD}
      DB_NAME: postgres
      DB_AFTER_CONNECT_QUERY: "SET search_path TO _realtime"
      DB_ENC_KEY: supabaserealtime
      API_JWT_SECRET: ${JWT_SECRET}
      SECRET_KEY_BASE: ${JWT_SECRET}
      ERL_AFLAGS: -proto_dist inet_tcp
      DNS_NODES: "''"
      RLIMIT_NOFILE: "10000"
      APP_NAME: realtime
      SEED_SELF_HOST: "true"
    depends_on:
      db:
        condition: service_healthy
    networks:
      - supabase_network

  # ========================================
  # Supabase: Storage (S3-Compatible)
  # ========================================
  storage:
    container_name: supabase-storage
    image: supabase/storage-api:v1.33.23
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "wget", "--no-verbose", "--tries=1", "--spider", "http://localhost:5000/status"]
      interval: 5s
      timeout: 5s
      retries: 5
    environment:
      ANON_KEY: ${ANON_KEY}
      SERVICE_KEY: ${SERVICE_ROLE_KEY}
      POSTGREST_URL: http://rest:3000
      PGRST_JWT_SECRET: ${JWT_SECRET}
      DATABASE_URL: postgresql://supabase_storage_admin:${POSTGRES_PASSWORD}@db:5432/postgres
      FILE_SIZE_LIMIT: 52428800
      STORAGE_BACKEND: file
      FILE_STORAGE_BACKEND_PATH: /var/lib/storage
      TENANT_ID: stub
      REGION: local
      GLOBAL_S3_BUCKET: stub
      ENABLE_IMAGE_TRANSFORMATION: "true"
      IMGPROXY_URL: http://imgproxy:5001
      IMGPROXY_REQUEST_TIMEOUT: 15
      DATABASE_SEARCH_PATH: storage
      REQUEST_ALLOWED_CONTENT_TYPES: "*"
      STORAGE_S3_PROTOCOL_ACCESS_KEY_ID: ${AWS_ACCESS_KEY_ID:-}
      STORAGE_S3_PROTOCOL_SECRET_ACCESS_KEY: ${AWS_SECRET_ACCESS_KEY:-}
      STORAGE_S3_REGION: ${AWS_DEFAULT_REGION:-}
    volumes:
      - supabase_storage_data:/var/lib/storage
    depends_on:
      db:
        condition: service_healthy
      rest:
        condition: service_started
      imgproxy:
        condition: service_started
    networks:
      - supabase_network

  # ========================================
  # Supabase: Image Proxy (imgproxy)
  # ========================================
  imgproxy:
    container_name: supabase-imgproxy
    image: darthsim/imgproxy:v3.26.0
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "imgproxy", "health"]
      interval: 5s
      timeout: 5s
      retries: 5
    environment:
      IMGPROXY_BIND: ":5001"
      IMGPROXY_LOCAL_FILESYSTEM_ROOT: /
      IMGPROXY_USE_ETAG: "true"
      IMGPROXY_ENABLE_WEBP_DETECTION: "true"
    volumes:
      - supabase_storage_data:/var/lib/storage:ro
    networks:
      - supabase_network

  # ========================================
  # Supabase: Postgres Meta (for Studio)
  # ========================================
  meta:
    container_name: supabase-meta
    image: supabase/postgres-meta:v0.91.0
    restart: unless-stopped
    environment:
      PG_META_PORT: 8080
      PG_META_DB_HOST: db
      PG_META_DB_PORT: 5432
      PG_META_DB_NAME: postgres
      PG_META_DB_USER: postgres
      PG_META_DB_PASSWORD: ${POSTGRES_PASSWORD}
      PG_META_DB_SSL_MODE: disable
    depends_on:
      db:
        condition: service_healthy
    networks:
      - supabase_network

  # ========================================
  # Supabase: Studio Dashboard
  # ========================================
  studio:
    container_name: supabase-studio
    image: supabase/studio:2026.05.07-sha-2e4841f
    restart: unless-stopped
    environment:
      STUDIO_PG_META_URL: http://meta:8080
      STUDIO_DEFAULT_ORGANIZATION_NAME: "My CMS"
      STUDIO_DEFAULT_PROJECT_NAME: "my-cms-local"
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
      SUPABASE_URL: http://localhost:8000
      SUPABASE_PUBLIC_URL: ${SUPABASE_PUBLIC_URL}
      SUPABASE_ANON_KEY: ${ANON_KEY}
      SUPABASE_SERVICE_KEY: ${SERVICE_ROLE_KEY}
      AUTH_JWT_SECRET: ${JWT_SECRET}
      LOGIN_MESSAGE: "Welcome to My CMS"
    depends_on:
      db:
        condition: service_healthy
      rest:
        condition: service_started
      auth:
        condition: service_started
      storage:
        condition: service_started
      realtime:
        condition: service_started
    ports:
      - "8000:3000"
    networks:
      - supabase_network

  # ========================================
  # Supabase: Kong API Gateway
  # ========================================
  kong:
    container_name: supabase-kong
    image: kong:2.8.1
    restart: unless-stopped
    ports:
      - "8001:8000/tcp"
      - "8443:8443/tcp"
    environment:
      KONG_DATABASE: "off"
      KONG_DECLARATIVE_CONFIG: /var/lib/kong/kong.yml
      KONG_DNS_ORDER: LAST,A,CNAME
      KONG_PLUGINS: request-termination,cors,key-auth,acl,basic-auth
      KONG_NGINX_PROXY_PROXY_BUFFER_SIZE: 160k
      KONG_NGINX_PROXY_PROXY_BUFFERS: 64 160k
      KONG_LOG_LEVEL: warn
      KONG_PLUGINSERVER_NAMES: js
      KONG_PLUGINSERVER_JS_SOCKET: /usr/local/kong/js_pluginserver.sock
      KONG_PLUGINSERVER_JS_START_CMD: /usr/local/bin/kong-js-pluginserver --dump-all-plugins
      KONG_PLUGINSERVER_JS_QUERY_CMD: /usr/local/bin/kong-js-pluginserver --dump-all-plugins
    volumes:
      - ./volumes/api/kong.yml:/var/lib/kong/kong.yml:ro
    depends_on:
      auth:
        condition: service_started
      rest:
        condition: service_started
      realtime:
        condition: service_started
      storage:
        condition: service_started
    networks:
      - supabase_network

  # ========================================
  # Supabase: Analytics (Logflare)
  # ========================================
  analytics:
    container_name: supabase-analytics
    image: supabase/logflare:1.24.0
    restart: unless-stopped
    environment:
      LOGFLARE_NODE_HOST: 127.0.0.1
      DB_USERNAME: supabase_admin
      DB_DATABASE: postgres
      DB_HOSTNAME: db
      DB_PORT: 5432
      DB_PASSWORD: ${POSTGRES_PASSWORD}
      DB_SCHEMA: _analytics
      LOGFLARE_SINGLE_TENANT: "true"
      LOGFLARE_SUPABASE_MODE: "true"
      LOGFLARE_MIN_CLUSTER_SIZE: "1"
      LOGFLARE_API_KEY: ${JWT_SECRET}
      LOGFLARE_LOG_LEVEL: warn
      LOGFLARE_FEATURE_FLAG_OVERRIDE: multibackend=true,ingest=true
    depends_on:
      db:
        condition: service_healthy
    networks:
      - supabase_network

  # ========================================
  # Supabase: Mailpit (Email Testing)
  # ========================================
  mailpit:
    container_name: supabase-mailpit
    image: axllent/mailpit:v1.23
    restart: unless-stopped
    ports:
      - "8025:8025"
    environment:
      MP_MAX_MESSAGES: 5000
      MP_DATABASE: /data/mailpit.db
      MP_SMTP_AUTH_ACCEPT_ANY: "true"
      MP_SMTP_AUTH_ALLOW_INSECURE: "true"
    volumes:
      - mailpit_data:/data
    networks:
      - supabase_network

  # ========================================
  # Project: Jaeger Tracing
  # ========================================
  jaeger:
    container_name: my-cms-jaeger
    image: jaegertracing/all-in-one:1.53
    restart: unless-stopped
    environment:
      COLLECTOR_OTLP_ENABLED: "true"
      LOG_LEVEL: debug
    ports:
      - "16686:16686"   # Jaeger UI
      - "4317:4317"     # OTLP gRPC receiver
      - "4318:4318"     # OTLP HTTP receiver
    networks:
      - supabase_network

  # ========================================
  # Project: my-cms API (Rust)
  # ========================================
  my-cms-api:
    container_name: my-cms-api
    build:
      context: ./services
      dockerfile: Dockerfile
    restart: unless-stopped
    ports:
      - "8989:8989"
    environment:
      DATABASE_URL: ${DATABASE_URL}
      DATABASE_SCHEMA: ${DATABASE_SCHEMA}
      HOST: ${HOST}
      PORT: ${PORT}
      RUST_LOG: ${RUST_LOG}
      ENABLED_OTLP_EXPORTER: ${ENABLED_OTLP_EXPORTER}
      OTEL_SERVICE_NAME: ${OTEL_SERVICE_NAME}
      SERVICE_NAME: ${SERVICE_NAME}
      OTEL_EXPORTER_OTLP_TRACES_ENDPOINT: ${OTEL_EXPORTER_OTLP_TRACES_ENDPOINT}
      OTEL_TRACES_SAMPLER: ${OTEL_TRACES_SAMPLER}
      OPENAI_API_KEY: ${OPENAI_API_KEY}
      MEDIA_BASE_URL: ${MEDIA_BASE_URL}
      MAX_BODY_LENGTH: ${MAX_BODY_LENGTH}
      SUPABASE_URL: ${SUPABASE_URL}
      SUPABASE_ANON_KEY: ${SUPABASE_ANON_KEY}
      SUPABASE_SERVICE_ROLE_KEY: ${SUPABASE_SERVICE_ROLE_KEY}
      SUPABASE_STORAGE_BUCKET: ${SUPABASE_STORAGE_BUCKET}
      KEYCLOAK_ISSUER: ${KEYCLOAK_ISSUER}
      KEYCLOAK_REALM: ${KEYCLOAK_REALM}
      AUTHORIZATION_AUDIENCE: ${AUTHORIZATION_AUDIENCE}
    depends_on:
      db:
        condition: service_healthy
      jaeger:
        condition: service_started
    networks:
      - supabase_network

  # ========================================
  # Project: Frontend Admin Panel (React)
  # ========================================
  my-cms-frontend:
    container_name: my-cms-frontend
    build:
      context: ./frontend
      dockerfile: Dockerfile.dev
    restart: unless-stopped
    ports:
      - "3002:3002"
    environment:
      PUBLIC_KEYCLOAK_URL: ${PUBLIC_KEYCLOAK_URL}
      PUBLIC_KEYCLOAK_REALM: ${PUBLIC_KEYCLOAK_REALM}
      PUBLIC_KEYCLOAK_CLIENT_ID: ${PUBLIC_KEYCLOAK_CLIENT_ID}
      PUBLIC_KEYCLOAK_SCOPE: ${PUBLIC_KEYCLOAK_SCOPE}
      PUBLIC_GRAPHQL_API_URL: ${PUBLIC_GRAPHQL_API_URL}
      PUBLIC_GRAPHQL_CACHE_API_URL: ${PUBLIC_GRAPHQL_CACHE_API_URL}
      PUBLIC_REST_API_URL: ${PUBLIC_REST_API_URL}
      PUBLIC_MEDIA_UPLOAD_API_URL: ${PUBLIC_MEDIA_UPLOAD_API_URL}
    volumes:
      - ./frontend/src:/app/src:ro
    depends_on:
      my-cms-api:
        condition: service_started
    networks:
      - supabase_network

# ========================================
# Networks
# ========================================
networks:
  supabase_network:
    driver: bridge
    name: supabase_network

# ========================================
# Volumes
# ========================================
volumes:
  supabase_db_data:
    name: supabase_db_data
  supabase_storage_data:
    name: supabase_storage_data
  mailpit_data:
    name: mailpit_data
```

- [ ] **Step 2: Commit**

```bash
git add docker-compose.yml
git commit -m "feat: add docker-compose.yml with full Supabase stack + project services"
```

---

### Task 0.3: Create Kong API Gateway Configuration

**Files:**
- Create: `volumes/api/kong.yml`

- [ ] **Step 1: Create directories and kong config**

```bash
mkdir -p volumes/api volumes/db/init
```

```yaml
# volumes/api/kong.yml
# Kong declarative configuration for Supabase self-hosted

_format_version: "2.1"
_transform: true

services:
  - name: auth-v1-open
    url: http://auth:9999/verify
    routes:
      - name: auth-v1-open
        strip_path: true
        paths:
          - /auth/v1/verify
    plugins:
      - name: cors
  - name: auth-v1-open-callback
    url: http://auth:9999/callback
    routes:
      - name: auth-v1-open-callback
        strip_path: true
        paths:
          - /auth/v1/callback
    plugins:
      - name: cors
  - name: auth-v1-open-authorize
    url: http://auth:9999/authorize
    routes:
      - name: auth-v1-open-authorize
        strip_path: true
        paths:
          - /auth/v1/authorize
    plugins:
      - name: cors
  - name: auth-v1-sso-saml
    url: http://auth:9999/sso/saml
    routes:
      - name: auth-v1-sso-saml
        strip_path: true
        paths:
          - /auth/v1/sso/saml
    plugins:
      - name: cors
  - name: auth-v1
    url: http://auth:9999/
    routes:
      - name: auth-v1-all
        strip_path: true
        paths:
          - /auth/v1/
    plugins:
      - name: cors
      - name: key-auth
        config:
          hide_credentials: false
      - name: acl
        config:
          hide_groups_header: true
          allow:
            - admin
  - name: rest
    url: http://rest:3000/
    routes:
      - name: rest
        strip_path: true
        paths:
          - /rest/v1/
    plugins:
      - name: cors
      - name: key-auth
        config:
          hide_credentials: true
      - name: acl
        config:
          hide_groups_header: true
          allow:
            - admin
            - anon
  - name: rest-graphql
    url: http://rest:3000/rpc/graphql
    routes:
      - name: rest-graphql
        strip_path: true
        paths:
          - /graphql/v1
    plugins:
      - name: cors
      - name: key-auth
        config:
          hide_credentials: true
      - name: acl
        config:
          hide_groups_header: true
          allow:
            - admin
            - anon
  - name: realtime
    url: http://realtime:4000/socket/
    routes:
      - name: realtime
        strip_path: true
        paths:
          - /realtime/v1/
    plugins:
      - name: cors
      - name: key-auth
        config:
          hide_credentials: false
      - name: acl
        config:
          hide_groups_header: true
          allow:
            - admin
            - anon
  - name: storage
    url: http://storage:5000/
    routes:
      - name: storage-v1
        strip_path: true
        paths:
          - /storage/v1/
    plugins:
      - name: cors
  - name: meta
    url: http://meta:8080/
    routes:
      - name: meta
        strip_path: true
        paths:
          - /pg/
    plugins:
      - name: key-auth
        config:
          hide_credentials: false
      - name: acl
        config:
          hide_groups_header: true
          allow:
            - admin
  - name: analytics
    url: http://analytics:4000/
    routes:
      - name: analytics-v1
        strip_path: true
        paths:
          - /analytics/v1/
    plugins:
      - name: cors
      - name: key-auth
        config:
          hide_credentials: true
      - name: acl
        config:
          hide_groups_header: true
          allow:
            - admin
```

- [ ] **Step 2: Commit**

```bash
git add volumes/api/kong.yml
git commit -m "feat: add Kong API gateway configuration for Supabase"
```

---

### Task 0.4: Supabase DB Init Scripts

**Files:**
- Create: `volumes/db/init/00-setup-roles.sql`

- [ ] **Step 1: Write init SQL for required roles**

```sql
-- volumes/db/init/00-setup-roles.sql
-- Creates required roles for Supabase services on first startup.

-- Used by GoTrue auth service
CREATE ROLE supabase_auth_admin WITH LOGIN CREATEROLE NOINHERIT PASSWORD '${POSTGRES_PASSWORD}';

-- Used by Storage API
CREATE ROLE supabase_storage_admin WITH LOGIN CREATEROLE NOINHERIT PASSWORD '${POSTGRES_PASSWORD}';

-- Used by PostgREST for anonymous requests
CREATE ROLE anon WITH LOGIN NOINHERIT;
CREATE ROLE authenticated WITH LOGIN NOINHERIT;
CREATE ROLE authenticator WITH LOGIN NOINHERIT PASSWORD '${POSTGRES_PASSWORD}';

-- Used by service_role key
CREATE ROLE service_role WITH LOGIN NOINHERIT;

-- Used by Supavisor connection pooler
CREATE ROLE supabase_admin WITH LOGIN CREATEROLE NOINHERIT PASSWORD '${POSTGRES_PASSWORD}';

-- Grant schema access to authenticator
GRANT anon, authenticated, service_role TO authenticator;

-- Grant necessary permissions to supabase_auth_admin
GRANT CREATE ON DATABASE postgres TO supabase_auth_admin;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO supabase_auth_admin;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO supabase_auth_admin;

-- Grant necessary permissions to supabase_storage_admin
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO supabase_storage_admin;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO supabase_storage_admin;

-- Grant anonymous role access
GRANT USAGE ON SCHEMA public TO anon, authenticated, service_role;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO anon, authenticated, service_role;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO anon, authenticated, service_role;

-- Enable pgvector for AI translation (replaces Qdrant)
CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA public;

-- Enable other useful extensions
CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;
CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;
```

Note: This script uses `${POSTGRES_PASSWORD}` which Docker Compose does **not** substitute in volume-mounted files. You need to either:
1. Hardcode the password in this file (matches `.env`), or
2. Use a `.env`-driven init script approach

The simpler approach: write the password directly since this is local dev.

- [ ] **Step 2: Commit**

```bash
git add volumes/db/init/00-setup-roles.sql
git commit -m "feat: add DB init script for Supabase roles and pgvector"
```

---

### Task 0.5: Create Frontend Dockerfile.dev

**Files:**
- Create: `frontend/Dockerfile.dev`

- [ ] **Step 1: Write dev Dockerfile**

```dockerfile
# frontend/Dockerfile.dev
# Development server for the React admin panel (rsbuild).

FROM node:22-alpine

WORKDIR /app

COPY package.json pnpm-lock.yaml ./

RUN corepack enable && pnpm install --frozen-lockfile

COPY . .

EXPOSE 3002

CMD ["pnpm", "run", "dev", "--port", "3002", "--host", "0.0.0.0"]
```

- [ ] **Step 2: Check if pnpm-lock.yaml exists, commit**

```bash
git add frontend/Dockerfile.dev
git commit -m "feat: add frontend dev Dockerfile for compose"
```

---

### Task 0.6: Create Reset Script

**Files:**
- Create: `resetsupabasedb.sh`

- [ ] **Step 1: Write reset script**

```bash
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
```

- [ ] **Step 2: Make executable and commit**

```bash
chmod +x resetsupabasedb.sh
git add resetsupabasedb.sh
git commit -m "feat: add reset script for clean supabase restart"
```

---

### Task 0.7: Create docker-compose.override.example.yml

**Files:**
- Create: `docker-compose.override.example.yml`

- [ ] **Step 1: Write overrides for hot-reload dev**

```yaml
# docker-compose.override.example.yml
# Copy to docker-compose.override.yml for development overrides.
# Enables: hot-reload for API, local volumes for frontend, debug ports.

services:
  my-cms-api:
    build:
      context: ./services
      dockerfile: Dockerfile.dev
    volumes:
      - ./services/src:/app/src:ro
      - ./services/application_core:/app/application_core:ro

  my-cms-frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile.dev
    volumes:
      - ./frontend/src:/app/src
      - ./frontend/public:/app/public:ro
      - ./frontend/rsbuild.config.ts:/app/rsbuild.config.ts:ro
    environment:
      - NODE_ENV=development
```

- [ ] **Step 2: Commit**

```bash
git add docker-compose.override.example.yml
git commit -m "feat: add docker compose override example for hot-reload dev"
```

---

### Task 0.8: Add .env and volumes to .gitignore

**Files:**
- Modify: `.gitignore`

- [ ] **Step 1: Add entries**

```gitignore
# Add these lines if not present:
.env
docker-compose.override.yml
volumes/
```

- [ ] **Step 2: Commit**

```bash
git add .gitignore
git commit -m "chore: add .env, volumes/, and override to .gitignore"
```

---

### Task 0.9: Verify the Setup

- [ ] **Step 1: Start all services**

```bash
cp .env.example .env
# Edit .env: set POSTGRES_PASSWORD and JWT_SECRET
docker compose up -d
```

- [ ] **Step 2: Health-check Supabase**

```bash
curl -s http://localhost:8000 | head -20
# Expected: HTML of Supabase Studio login page

curl -s http://localhost:8001/auth/v1/health
# Expected: {"healthy":true} (may need Bearer token)
```

- [ ] **Step 3: Health-check my-cms API**

```bash
curl -s http://localhost:8989/health
# Expected: {"status":"ok"}
```

- [ ] **Step 4: Health-check Jaeger**

```bash
curl -s http://localhost:16686 | head -5
# Expected: HTML of Jaeger UI
```

- [ ] **Step 5: Verify Supabase Studio login**

Open `http://localhost:8000` in a browser. Login with `admin` / `admin` (set in `.env` as `DASHBOARD_USERNAME` / `DASHBOARD_PASSWORD`).

---

### Task 0.10: Final Commit

```bash
git add .
git commit -m "chore: finalize docker compose setup for local development"
```

---

## Verification Checklist

- [ ] `docker compose up -d` starts all 12+ services without errors
- [ ] `docker compose ps` shows all services as "healthy" or "running"
- [ ] Supabase Studio accessible at `http://localhost:8000`
- [ ] Kong API gateway accessible at `http://localhost:8001`
- [ ] my-cms API health check returns 200 at `http://localhost:8989/health`
- [ ] Frontend dev server accessible at `http://localhost:3002`
- [ ] Jaeger UI accessible at `http://localhost:16686`
- [ ] Mailpit accessible at `http://localhost:8025`
- [ ] PostgreSQL accessible at `localhost:5432` (via supavisor)
- [ ] `docker compose down` stops and removes all containers
- [ ] `./resetsupabasedb.sh` wipes volumes and restarts cleanly
- [ ] pgvector extension is enabled in PostgreSQL
- [ ] Supabase Storage bucket creation works in Studio UI
