# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A headless CMS API system built in Rust using Axum for the web framework and SeaORM for database interactions. The system provides REST and GraphQL APIs for managing categories, posts, tags, and media.

## Build and Run Commands

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

## Database Commands (SeaORM)

```bash
# Run migrations (replace connection_string with your DATABASE_URL)
sea-orm-cli migrate --database-url <connection_string> up

# Rollback migrations
sea-orm-cli migrate --database-url <connection_string> down

# Generate entities from database schema (Schema First approach)
sea-orm-cli generate entity --database-url <connection_string> -o application_core/src/entities --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --seaography
```

## Architecture

### Workspace Structure

- **cms** (root crate): Main API application with Axum routes and middleware
- **application_core**: Domain logic, command handlers, entities, and GraphQL schema
- **migration**: SeaORM database migrations
- **test_helpers**: Shared utilities for integration tests using testcontainers

### Layered Architecture

```
API Layer (src/api/)          → REST endpoints and GraphQL handlers
    ↓
Application Core              → Business logic and command handlers
    ↓
Entities (SeaORM)             → Database models and ORM operations
    ↓
PostgreSQL Database
```

### Key Modules

**src/bin/my-cms-api.rs**: Application entry point with three router groups:
- `public_router`: Health checks and immutable GraphQL endpoint
- `protected_router`: CRUD operations (requires `my-headless-cms-writer` role)
- `protected_administrator_router`: Migration endpoint (requires `my-headless-cms-administrator` role)

**application_core/src/commands/**: Command handlers for each domain:
- `category/` - Category CRUD operations
- `post/` - Post CRUD operations
- `media/` - S3 media upload and delivery with auto-resize
- `tag/` - Tag operations

**application_core/src/entities/**: SeaORM entities generated from database schema

**application_core/src/graphql/**: Dynamic GraphQL schema using async-graphql and seaography

### Authentication

Uses Keycloak via `axum-keycloak-auth` with role-based access control. Configure via environment variables:
- `KEYCLOAK_ISSUER`
- `KEYCLOAK_REALM`
- `AUTHORIZATION_AUDIENCE`

### Media API

**Upload**: `POST /media/images` (protected, requires `my-headless-cms-writer` role)
- Accepts multipart form data with `image` field
- Returns JSON with `path` and `url`

**Delivery**: `GET /media/images/{path}` (public)
- Serves images directly from S3 with optional auto-resize
- Query parameters:
  - `w` - target width (maintains aspect ratio if only width specified)
  - `h` - target height (maintains aspect ratio if only height specified)
- Includes in-memory caching (1 hour TTL, max 500 images)
- Example: `/media/images/example.jpg?w=800`

### External Dependencies

- **PostgreSQL**: Primary database
- **S3-Compatible Storage**: Media storage (Contabo or any S3-compatible service)
- **Jaeger**: Optional OpenTelemetry tracing (enable via `ENABLED_OTLP_EXPORTER=true`)

## Environment Configuration

Key environment variables (see `.env` for full list):
- `DATABASE_URL`: PostgreSQL connection string
- `HOST` / `PORT`: API server binding (default: 127.0.0.1:8989)
- `S3_ENDPOINT`: S3-compatible storage endpoint (e.g., `https://sin1.contabostorage.com`)
- `S3_BUCKET_NAME`: Storage bucket name
- `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`: S3 credentials
- `MEDIA_BASE_URL`: Base URL for media delivery URLs (defaults to `http://{HOST}:{PORT}`)
- `ENABLED_OTLP_EXPORTER`: Enable Jaeger tracing

## Testing

- Unit tests use SeaORM's built-in mock feature
- Integration tests use testcontainers for PostgreSQL setup
- Coverage reports generated with grcov (CI uses nightly toolchain with `llvm-tools-preview`)
