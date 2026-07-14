[![Automation Test my-cms-api](https://github.com/doitsu2014/my-cms/actions/workflows/ci-my-cms.yml/badge.svg)](https://github.com/doitsu2014/my-cms/actions/workflows/ci-my-cms.yml)
<br/>
[![codecov](https://codecov.io/gh/doitsu2014/my-cms/branch/main/graph/badge.svg?token=7V6BYO0TJO)](https://codecov.io/gh/doitsu2014/my-cms)
<br/>
[![Coverage Status](https://coveralls.io/repos/github/doitsu2014/my-cms/badge.svg?branch=main)](https://coveralls.io/github/doitsu2014/my-cms?branch=main)

# my-cms

A self-hosted, AI-augmented headless CMS — Rust backend, React admin, Supabase platform.

## Project Overview

my-cms is a self-hosted, AI-augmented headless CMS built for performance, safety, and developer ergonomics. It combines a Rust backend API, a React admin web app, and a Supabase platform (PostgreSQL + pgvector + Storage + GoTrue) into one cohesive content stack.

The Rust API uses Axum 0.8 and SeaORM 1.1 with a strict layered architecture (API → Application Core → Database) and schema-first migrations. The React 19 admin (DaisyUI 5 + Tailwind 4 + TipTap) provides a polished editor for categories, posts, tags, and media. Authentication is handled by Supabase GoTrue JWTs through a custom Axum middleware; media lives in Supabase Storage (S3-compatible); AI translation jobs run through OpenAI with vector similarity reuse via pgvector.

Development follows an OpenSpec-driven SDLC: every change ships with a proposal, testable capability spec, design, and tasks document. Implementation work is executed with Superpowers skills (TDD, subagent-driven development, code review, verification before completion).

## Architecture

The diagram below maps the runtime layers and the AI translation flow. Click any node to jump to the corresponding source tree.

```mermaid
flowchart TD
    %% Level 1: Clients
    Clients["Clients (REST & GraphQL)"]:::ext

    %% Level 2: API Layer
    subgraph "API Layer"
        direction TB
        API_Module["API Modules"]:::api
        API_Category["Category API"]:::api
        API_Post["Post API"]:::api
        API_Translate["Translation API<br/>(AI-powered)"]:::api_ai
        API_Media["Media API"]:::api
        API_Tag["Tag API"]:::api
        API_Public["Public API"]:::api
        API_GraphQL["GraphQL API"]:::api
        API_Admin["Administrator API"]:::api
    end

    %% Level 3: Application Core
    subgraph "Application Core"
        direction TB
        Core_Module["Core Module"]:::app
        CMD_Category["Category Command Handler"]:::app
        CMD_Post["Post Command Handler"]:::app
        CMD_AI["AI Translation Handler<br/>(3-Tier Lookup)"]:::app_ai
        CMD_Media["Media Command Handler"]:::app
        CMD_Tag["Tag Command Handler"]:::app
        Common_Domain["Common Domain Logic"]:::app
    end

    %% Level 4: Database & ORM Layer
    subgraph "Database & ORM Layer"
        direction TB
        Entities["SeaORM Entities"]:::db
        Migrations["Migrations"]:::db
        DB["((PostgreSQL Database))"]:::db
    end

    %% Level 5: External AI Services
    subgraph "AI Services & Vector DB"
        direction TB
        OpenAI["OpenAI API<br/>(GPT-4o-mini)"]:::ai
        Qdrant["Qdrant Vector DB<br/>(Similarity Search)"]:::ai
    end

    %% Level 6: Deployment & External/Config/Testing
    subgraph "Infrastructure & Cross-Cutting Concerns"
        direction TB
        Config[".env Configuration"]:::config
        Jaeger["Jaeger Tracing"]:::ext
        S3["S3 Media Storage"]:::ext
        Docker["Docker Deployment"]:::deploy
        Helm["Helm Charts"]:::deploy
        CICD["CI/CD Workflows"]:::deploy
        Testing["Unit Test Helpers"]:::test
    end

    %% Connections in Main Flow
    Clients -->|"requests"| API_Module
    API_Module -->|"dispatch"| Core_Module
    Core_Module -->|"DB operations"| Entities
    Entities -->|"maps to"| DB
    Migrations -->|"schema update"| DB

    %% Internal API Layer connections
    API_Module -->|"includes"| API_Category
    API_Module -->|"includes"| API_Post
    API_Module -->|"includes"| API_Translate
    API_Module -->|"includes"| API_Media
    API_Module -->|"includes"| API_Tag
    API_Module -->|"includes"| API_Public
    API_Module -->|"includes"| API_GraphQL
    API_Module -->|"includes"| API_Admin

    %% Internal Application Core connections
    Core_Module -->|"executes"| CMD_Category
    Core_Module -->|"executes"| CMD_Post
    Core_Module -->|"executes"| CMD_AI
    Core_Module -->|"executes"| CMD_Media
    Core_Module -->|"executes"| CMD_Tag
    Core_Module -->|"utilizes"| Common_Domain

    %% AI Translation Flow (3-Tier Lookup)
    CMD_AI -->|"1. Check cache"| DB
    CMD_AI -->|"2. Similarity search"| Qdrant
    CMD_AI -->|"3. Translate"| OpenAI
    CMD_AI -->|"Store embeddings"| Qdrant

    %% Deployment & Config Influence
    Docker ---|"deploys"| API_Module
    Helm ---|"orchestrates"| API_Module
    CICD ---|"integrates"| API_Module
    CICD ---|"integrates"| Core_Module

    Config ---|"configures"| API_Module
    Config ---|"configures"| Core_Module
    Config ---|"configures"| DB
    Config ---|"API keys"| OpenAI
    Config ---|"connection"| Qdrant

    %% External Services Influence
    API_Module ---|"traced by"| Jaeger
    API_Module ---|"media stored in"| S3

    %% Testing Influence
    Testing ---|"validates"| Core_Module
    Testing ---|"validates"| CMD_AI

    %% Click Events for API Layer
    click API_Module "apps/api/src/api"
    click API_Category "apps/api/src/api/category"
    click API_Post "apps/api/src/api/post"
    click API_Translate "apps/api/src/api/post/translate"
    click API_Media "apps/api/src/api/media"
    click API_Tag "apps/api/src/api/tag"
    click API_Public "apps/api/src/api/public"
    click API_GraphQL "apps/api/src/api/graphql"
    click API_Admin "apps/api/src/api/administrator"

    %% Click Events for Application Core
    click Core_Module "apps/api/application_core"
    click CMD_Category "apps/api/application_core/src/commands/category"
    click CMD_Post "apps/api/application_core/src/commands/post"
    click CMD_AI "apps/api/application_core/src/commands/ai"
    click CMD_Media "apps/api/application_core/src/commands/media"
    click CMD_Tag "apps/api/application_core/src/commands/tag"
    click Common_Domain "apps/api/application_core/src/common"

    %% Click Events for Database & ORM Layer
    click Entities "apps/api/application_core/src/entities"
    click Migrations "apps/api/migration"

    %% Click Events for Deployment & Infrastructure
    click Docker "deployments/docker-swarm"
    click Helm "deployments/k8s/charts"
    click CICD ".github/workflows"

    %% Click Event for Configuration
    click Config "apps/api/.env.example"

    %% Click Event for Testing
    click Testing "apps/api/test_helpers"

    %% Styles
    classDef api fill:#a6cee3,stroke:#1f78b4,stroke-width:2px;
    classDef api_ai fill:#ff9999,stroke:#cc0000,stroke-width:3px;
    classDef app fill:#b2df8a,stroke:#33a02c,stroke-width:2px;
    classDef app_ai fill:#ffcc99,stroke:#ff6600,stroke-width:3px;
    classDef ai fill:#ffccff,stroke:#cc00cc,stroke-width:3px;
    classDef db fill:#fcae91,stroke:#fb9a99,stroke-width:2px;
    classDef deploy fill:#ffe599,stroke:#b08d57,stroke-width:2px;
    classDef config fill:#d9d9d9,stroke:#7f7f7f,stroke-width:2px;
    classDef test fill:#f4cccc,stroke:#e06666,stroke-width:2px;
    classDef ext fill:#c5b0d5,stroke:#6a3d9a,stroke-width:2px;
```

## Features

- **REST + GraphQL API** — Axum 0.8 + SeaORM 1.1, schema-first migrations, GraphQL auto-generated with Seaography.
- **React admin UI** — React 19 + DaisyUI 5 + Tailwind CSS 4 + TipTap rich-text editor.
- **Content model** — Categories, Posts, Tags, and Media with end-to-end CRUD via REST and GraphQL.
- **[AI Translation Platform](docs/ai-platform.md)** — 3-tier lookup (DB → vector similarity → OpenAI), background job tracking, HTML-aware processing, and smart translation reuse.
- **Authentication** — Supabase GoTrue JWT verified by a custom Axum middleware, with role-based access.
- **Observability** — OpenTelemetry traces exported to Jaeger for distributed tracing across the API.
- **Deployment** — Docker Compose local stack (`deployments/docker-swarm`) and Helm charts for production (`deployments/k8s`).

## Quickstart

The fastest way to run my-cms locally is the Docker Compose stack — it brings up Supabase, Traefik, the API, the admin web app, and Jaeger together.

```bash
# See deployments/docker-swarm/README.md for prerequisites and one-time bootstrap
deployments/docker-swarm/bootstrap.sh
deployments/docker-swarm/apps/reset.sh
```

Once the stack is up, exercise the API using the Postman collection under [`docs/postman_collection/`](docs/postman_collection/). Construct your environment variables in Postman before sending requests.

### Prerequisites (Linux)

- `libssl-dev` (for OpenSSL)
- `pkg-config` (for building some dependencies)
- `build-essential` (for building some dependencies)

## Configuration

### 1. Cross-cutting concerns

- Jaeger

```bash
docker image pull jaegertracing/all-in-one:1.53
docker run --rm -d --name jaeger \
  -e COLLECTOR_OTLP_ENABLED:true \
  -e LOG_LEVEL:debug \
  -p 6831:6831/udp \
  -p 6832:6832/udp \
  -p 5778:5778 \
  -p 16686:16686 \
  -p 4317:4317\
  -p 4318:4318 \
  -p 14250:14250 \
  -p 14268:14268 \
  -p 14269:14269 \
  -p 9411:9411 \
  jaegertracing/all-in-one:1.53
```

### 2. Environment Setup

Use the `.env` file to configure the system. The reference template lives at `apps/api/.env.example`.

```text
DATABASE_SCHEMA=public
DATABASE_URL=postgresql://postgres:1234567890@localhost:5432/my-cms
HOST=127.0.0.1
PORT=8989

# Api Configuration
# Trace
ENABLED_OTLP_EXPORTER=false
OTEL_SERVICE_NAME=my-cms-headless-api
SERVICE_NAME=my-cms-headless-api
OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://localhost:4317
OTEL_TRACES_SAMPLER=always_on

# Request Limit
# Default: 10MB
MAX_BODY_LENGTH=10485760

# Media Config
# S3-Compatible Storage Configuration
S3_ENDPOINT=https://sin1.contabostorage.com
S3_BUCKET_NAME=
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=

MEDIA_IMG_PROXY_SERVER=https://imgproxy.doitsu.tech
```

## Development Guidelines

### 1. ORM

The project uses SeaORM to interact with the database. SeaORM is a modern and easy-to-use ORM for Rust.
We use the Schema First approach to design the database schema and generate the code from the schema. It helps us to keep the schema and the code in sync.

The `entities` are generated from the schema, and we use them to interact with the database.

#### Commands

Replace `connection_string` for each command below:

- Migrate up (apply latest migration to the database):

```sh
sea-orm-cli migrate --database-url connection_string up
```

- Generate entities (scaffold from the latest database state to source code):

```sh
sea-orm-cli generate entity --database-url postgres://postgres:1234567890@localhost:5432/my-cms -o apps/api/application_core/src/entities --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --seaography
```

### 2. Unit Tests and Integration Tests

For unit tests, we use SeaORM's built-in mock feature to test the database interaction. For integration tests, we use testcontainers to set up the whole infrastructure and verify the system works end-to-end.

### 3. CI/CD

I use Docker to build the image and GitHub Actions to run the CI/CD pipeline.

## Project Layout

```
my-cms/
├── apps/
│   ├── api/                  Rust (Axum + SeaORM) backend
│   └── web/                  React (DaisyUI + TipTap) admin
├── deployments/
│   ├── docker-swarm/         Local stack (Compose: Supabase + Traefik + apps)
│   └── k8s/                  Production Helm charts
├── openspec/                 Spec & change management (OpenSpec)
├── docs/                     Extended documentation
└── AGENTS.md                 SDLC conventions and project rules
```

## More Documentation

- [AI Translation Platform](docs/ai-platform.md) — platform overview, 3-tier lookup, background job tracking.
- [AI Translation deep dive](apps/api/application_core/src/commands/ai/README.md) — implementation details, architecture, troubleshooting.
- [API playground](docs/postman_collection/) — Postman collection for the REST + GraphQL API.
- [GoTrue access token guideline](docs/guidelines/gotrue-access-token.md) — issuing and using Supabase JWTs.
- [OpenSpec](openspec/) — capability specs and the proposal → design → tasks → archive workflow.
- [Local Docker stack](deployments/docker-swarm/README.md) — Docker Compose quickstart for the full platform.

## License

Released under the terms of the [LICENSE](LICENSE) file (MIT OR Apache-2.0).