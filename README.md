[![CI Workflow](https://github.com/doitsu2014/my-cms/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/doitsu2014/my-cms/actions/workflows/ci.yml)
<br/>
[![codecov](https://codecov.io/gh/doitsu2014/my-cms/branch/main/graph/badge.svg?token=7V6BYO0TJO)](https://codecov.io/gh/doitsu2014/my-cms)
<br/>
[![Coverage Status](https://coveralls.io/repos/github/doitsu2014/my-cms/badge.svg?branch=main)](https://coveralls.io/github/doitsu2014/my-cms?branch=main)

# Overview

There is my-cms project, it is an api system to handle biz services of a headless CMS for my website. I choose Rust as the main programming language to build the system because of its performance and safety.
Let's see how far I can go with this project.

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

    %% Level 5: Deployment & External/Config/Testing
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
    API_Module -->|"includes"| API_Media
    API_Module -->|"includes"| API_Tag
    API_Module -->|"includes"| API_Public
    API_Module -->|"includes"| API_GraphQL
    API_Module -->|"includes"| API_Admin

    %% Internal Application Core connections
    Core_Module -->|"executes"| CMD_Category
    Core_Module -->|"executes"| CMD_Post
    Core_Module -->|"executes"| CMD_Media
    Core_Module -->|"executes"| CMD_Tag
    Core_Module -->|"utilizes"| Common_Domain

    %% Deployment & Config Influence
    Docker ---|"deploys"| API_Module
    Helm ---|"orchestrates"| API_Module
    CICD ---|"integrates"| API_Module
    CICD ---|"integrates"| Core_Module

    Config ---|"configures"| API_Module
    Config ---|"configures"| Core_Module
    Config ---|"configures"| DB

    %% External Services Influence
    API_Module ---|"traced by"| Jaeger
    API_Module ---|"media stored in"| S3

    %% Testing Influence
    Testing ---|"validates"| Core_Module

    %% Click Events for API Layer
    click API_Module "https://github.com/doitsu2014/my-cms/tree/main/src/api"
    click API_Category "https://github.com/doitsu2014/my-cms/tree/main/src/api/category"
    click API_Post "https://github.com/doitsu2014/my-cms/tree/main/src/api/post"
    click API_Media "https://github.com/doitsu2014/my-cms/tree/main/src/api/media"
    click API_Tag "https://github.com/doitsu2014/my-cms/tree/main/src/api/tag"
    click API_Public "https://github.com/doitsu2014/my-cms/tree/main/src/api/public"
    click API_GraphQL "https://github.com/doitsu2014/my-cms/tree/main/src/api/graphql"
    click API_Admin "https://github.com/doitsu2014/my-cms/tree/main/src/api/administrator"

    %% Click Events for Application Core
    click Core_Module "https://github.com/doitsu2014/my-cms/tree/main/application_core"
    click CMD_Category "https://github.com/doitsu2014/my-cms/tree/main/application_core/src/commands/category"
    click CMD_Post "https://github.com/doitsu2014/my-cms/tree/main/application_core/src/commands/post"
    click CMD_Media "https://github.com/doitsu2014/my-cms/tree/main/application_core/src/commands/media"
    click CMD_Tag "https://github.com/doitsu2014/my-cms/tree/main/application_core/src/commands/tag"
    click Common_Domain "https://github.com/doitsu2014/my-cms/tree/main/application_core/src/common"

    %% Click Events for Database & ORM Layer
    click Entities "https://github.com/doitsu2014/my-cms/tree/main/application_core/src/entities"
    click Migrations "https://github.com/doitsu2014/my-cms/tree/main/migration"

    %% Click Events for Deployment & Infrastructure
    click Docker "https://github.com/doitsu2014/my-cms/tree/main/Dockerfile"
    click Helm "https://github.com/doitsu2014/my-cms/tree/main/deployments/charts/my-cms-api"
    click CICD "https://github.com/doitsu2014/my-cms/tree/main/.github/workflows"

    %% Click Event for Configuration
    click Config "https://github.com/doitsu2014/my-cms/blob/main/.env"

    %% Click Event for Testing
    click Testing "https://github.com/doitsu2014/my-cms/tree/main/test_helpers"

    %% Styles
    classDef api fill:#a6cee3,stroke:#1f78b4,stroke-width:2px;
    classDef app fill:#b2df8a,stroke:#33a02c,stroke-width:2px;
    classDef db fill:#fcae91,stroke:#fb9a99,stroke-width:2px;
    classDef deploy fill:#ffe599,stroke:#b08d57,stroke-width:2px;
    classDef config fill:#d9d9d9,stroke:#7f7f7f,stroke-width:2px;
    classDef test fill:#f4cccc,stroke:#e06666,stroke-width:2px;
    classDef ext fill:#c5b0d5,stroke:#6a3d9a,stroke-width:2px;
```

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

Use .env file to configure the system.

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
# S3 Configuration
S3_REGION=ap-southeast-1
S3_BUCKET_NAME=
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=

MEDIA_IMG_PROXY_SERVER=https://imgproxy.doitsu.tech
```

## Development Guidelines

### 1. ORM

The project using SeaORM to interact with the database. SeaORM is a modern and easy-to-use ORM for Rust.
We use Schema First approach to design the database schema and generate the code from the schema. It helps us to keep the schema and the code in sync.

The `entities` will be generated from the schema, and we can use them to interact with the database.

#### Commands

Please replace `connection_string` for each commands below:

- Command migrate up (up latest version of migration to database)

```sh
sea-orm-cli migrate --database-url connection_string up
```

- Command to generate entities (scaffold from latest on database to source code)

```sh
sea-orm-cli generate entity --database-url postgres://postgres:1234567890@localhost:5432/my-cms -o application_core/src/entities --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --seaography
```

### 2. Unit Tests and Integration Tests

For unit tests, we use built-in feature mock of SeaORM to test the database interaction. For integration tests, we use the test database to test the whole system.
For integration tests, we use testcontainers to setup whole infrastructure to make sure the system is working as expected.

### 3. CI/CD

I use Docker to build the image and Github Actions to run the CI/CD pipeline.

## Play around

You can play around the project using Postman Collection in folder `postman_collection`.

Construct your Environment Variable before you start playing.
