# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A headless CMS system with a Rust API backend (Axum + SeaORM) and React admin frontend. Provides REST and GraphQL APIs for managing categories, posts, tags, and media, with AI-powered translation capabilities.

## Build and Run Commands

### Backend (Rust API)

```bash
# Build the project
cargo build

# Build release version
cargo build --release

# Run the API server (requires .env or .env.local configuration)
cargo run --bin my-cms-api

# Run all tests
cargo test --all --all-features

# Run a specific test
cargo test <test_name>

# Run tests for a specific crate
cargo test -p application_core
cargo test -p migration
cargo test -p test_helpers
```

### Frontend (Admin Panel)

```bash
# From repository root
cd front-ends

# Install dependencies
pnpm install

# Start development server (port 3002)
pnpm dev

# Build for production
pnpm build

# Lint code
pnpm lint

# Format code
pnpm format
```

## Database Commands (SeaORM)

```bash
# Run migrations
sea-orm-cli migrate --database-url <connection_string> up

# Rollback migrations
sea-orm-cli migrate --database-url <connection_string> down

# Generate entities from database schema (Schema First approach)
sea-orm-cli generate entity --database-url <connection_string> -o application_core/src/entities --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --seaography
```

## Architecture

### Repository Structure

```
my-cms/
├── services/                    # Backend Rust workspace
│   ├── src/                     # Main API application (cms crate)
│   │   ├── bin/my-cms-api.rs    # Application entry point
│   │   └── api/                 # REST/GraphQL handlers
│   ├── application_core/        # Domain logic, commands, entities
│   ├── migration/               # SeaORM migrations
│   └── test_helpers/            # Integration test utilities
├── front-ends/                  # Frontend applications
│   └── admin_side/              # React admin panel (Rsbuild + DaisyUI)
└── deployments/                 # Helm charts for Kubernetes
```

### Backend Layered Architecture

```
API Layer (src/api/)          → REST endpoints and GraphQL handlers
    ↓
Application Core              → Business logic and command handlers
    ↓
Entities (SeaORM)             → Database models and ORM operations
    ↓
PostgreSQL Database
```

### Key Backend Modules

**src/bin/my-cms-api.rs**: Application entry point with three router groups:
- `public_router`: Health checks, immutable GraphQL, media delivery
- `protected_router`: CRUD operations (requires `my-headless-cms-writer` role)
- `protected_administrator_router`: Migration endpoint (requires `my-headless-cms-administrator` role)

**application_core/src/commands/**: Command handlers for each domain:
- `category/` - Category CRUD with hierarchical support
- `post/` - Post CRUD with translations
- `media/` - S3 upload, delivery with auto-resize, caching
- `tag/` - Tag operations
- `ai/translate/` - AI-powered translation (3-tier lookup)

**application_core/src/entities/**: SeaORM entities generated from database schema

**application_core/src/graphql/**: Dynamic GraphQL schema using async-graphql and seaography

### Frontend Architecture (admin_side)

- **Build Tool**: Rsbuild with Rspack
- **Framework**: React 19 with TypeScript
- **UI**: DaisyUI + Tailwind CSS
- **Rich Text**: TipTap editor
- **Auth**: Keycloak (Authorization Code Flow + PKCE)
- **API Client**: Apollo Client (GraphQL) + fetch (REST)

Key directories:
- `src/app/admin/` - Admin pages (blogs, categories, media)
- `src/auth/` - Keycloak authentication
- `src/infrastructure/` - GraphQL client and utilities
- `src/domains/` - Domain models

### Authentication

Uses Keycloak via `axum-keycloak-auth` with role-based access control:
- `my-headless-cms-writer` - Standard content management access
- `my-headless-cms-administrator` - Database migration access

### AI Translation Service

3-tier lookup strategy to minimize OpenAI API costs:
1. **Database cache** - Check for existing translations
2. **Qdrant similarity search** - Find similar translations (≥95% match)
3. **OpenAI GPT-4o-mini** - Generate new translation if no match found

Features: HTML-aware processing, chunked processing for large content, background execution support.

### Media API

- **Upload**: `POST /media` - Multipart form data, supports images and documents
- **List**: `GET /media` - List files with optional `?prefix=` filter
- **Delivery**: `GET /media/images/{path}` - Images with optional resize (`?w=800&h=600`)
- **Delivery**: `GET /media/{path}` - Documents and other files
- **Delete**: `DELETE /media/delete/{path}` or batch `DELETE /media`
- Includes in-memory caching (Moka, 1 hour TTL, max 500 files)

## Environment Configuration

### Backend (.env)
```
DATABASE_URL=postgresql://user:pass@localhost:5432/my-cms
HOST=127.0.0.1
PORT=8989

# S3-Compatible Storage
S3_ENDPOINT=https://sin1.contabostorage.com
S3_BUCKET_NAME=<bucket>
AWS_ACCESS_KEY_ID=<key>
AWS_SECRET_ACCESS_KEY=<secret>
MEDIA_BASE_URL=http://127.0.0.1:8989

# Keycloak Authentication
KEYCLOAK_ISSUER=<issuer>
KEYCLOAK_REALM=<realm>
AUTHORIZATION_AUDIENCE=<audience>

# AI Translation (optional)
OPENAI_API_KEY=<key>
QDRANT_URL=<url>

# Tracing (optional)
ENABLED_OTLP_EXPORTER=false
```

### Frontend (front-ends/.env.local)
```
PUBLIC_KEYCLOAK_URL=https://your-keycloak-url
PUBLIC_KEYCLOAK_REALM=master
PUBLIC_KEYCLOAK_CLIENT_ID=your-client-id
PUBLIC_KEYCLOAK_SCOPE=my-headless-cms-api-all email openid profile
PUBLIC_GRAPHQL_API_URL=http://localhost:8989/graphql
PUBLIC_REST_API_URL=http://localhost:8989/api
```

## Testing

- **Unit tests**: SeaORM's built-in mock feature
- **Integration tests**: testcontainers for PostgreSQL setup
- **Coverage**: grcov with nightly toolchain (`llvm-tools-preview`)
