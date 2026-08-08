# domain-post-service Specification

## Purpose
TBD - created by archiving change refactor-api-into-pluggable-domain-libraries. Update Purpose after archive.
## Requirements
### Requirement: Blog Post Service owns the post vertical slice including categories, AI model registry, and translation
`domain-posts` SHALL be a self-contained Cargo crate (lib + bin) that exposes post REST adapters, post application-layer command handlers, post-aggregate entities (`posts`, `post_tags`, `post_translations`, `translation_jobs`, `tags`), the post-related category entities (`categories`, `category_tags`, `category_translations`, `CategoryType` enum), the post-related AI model registry (`ModelsHandler`, `OpenAIModelInfo`, `ModelsListResponse`, `OpenAIClient` factory), the post translation pipeline (`PostTranslateHandler`, `VectorStore`, translation-job lifecycle, similarity-reuse threshold logic), the tag helper (`TagCreateHandler`, `TagReadHandler`), GraphQL contribution for the post-aggregate + category subgraphs, and post migrations through one package boundary. It SHALL NOT depend on `application_core`, `cms`, or any sibling domain. It SHALL depend only on `domain_interface` (plus its own infrastructure dependencies — SeaORM, Axum, OpenAI, pgvector, jsonwebtoken, tower-http, tower-cookies, axum-tracing-opentelemetry, init-tracing-opentelemetry, moka, async-openai, html5ever, markup5ever_rcdom, slugify, dotenv, reqwest, async-std, tokio). It SHALL preserve the existing post, category, AI, and translation HTTP and GraphQL contracts.

#### Scenario: Existing post route remains compatible after consolidation
- **WHEN** an authenticated client calls the existing post create, read, modify, delete, or translate route
- **THEN** the same path, method, authorization role, response envelope, and error mapping are returned
- **AND** command business logic remains outside the Axum gateway
- **AND** no `application_core` import appears in `domain-posts::src/**`

#### Scenario: Existing category route is served by the post domain
- **WHEN** an authenticated client calls `GET /categories`, `GET /categories/{category_id}`, `POST /categories`, `PUT /categories`, or `DELETE /categories`
- **THEN** the same path, method, authorization role, response envelope, and error mapping are returned
- **AND** the handler reads from `domain_posts::entities::{categories, category_tags, category_translations}`
- **AND** the handler reaches the tag helper through `crate::handlers::tag_helper::TagCreateHandler` (in-crate path import)

#### Scenario: Existing AI model route is served by the post domain
- **WHEN** an authenticated client calls `GET /ai/models`
- **THEN** the same path, method, authorization role, response envelope, and model catalogue are returned
- **AND** the handler reads from `domain_posts::handlers::ai::models::ModelsHandler`
- **AND** the OpenAI client factory is `domain_posts::handlers::ai::openai_client_from_env`

#### Scenario: Existing translation route is served by the post domain
- **WHEN** an authenticated client calls `POST /posts/{post_id}/translate`, `POST /posts/{post_id}/translate/background`, `GET /posts/{post_id}/translate/jobs`, or `GET /posts/{post_id}/translate/jobs/{job_id}`
- **THEN** the same path, method, authorization role, response envelope, and error mapping are returned
- **AND** the handler reads from `domain_posts::handlers::post::translate::PostTranslateHandler`
- **AND** the `VectorStore` reads from `domain_posts::handlers::vector_store::VectorStore`
- **AND** the OpenAI client factory is reached through the post-domain AI subsystem

#### Scenario: Post GraphQL contribution includes the category subgraph
- **WHEN** the gateway registers `domain-posts` and builds both schemas
- **THEN** post query fields remain available on immutable GraphQL
- **AND** category query fields are contributed by the post domain's `domain_posts::domain::graphql::contribute_post_schema` (because categories are part of the post aggregate)
- **AND** the existing mutable authorization boundary remains enforced

#### Scenario: Post service is deployable as a standalone microservice
`domain-posts` SHALL be runnable as a standalone binary (`cargo run -p domain-posts`) that boots its own Axum server, opens its own database connection, applies its own auth/CORS/cookie/body-limit/OpenTelemetry layers, runs its own migrations through `cargo run -p domain-posts -- migrate`, and exposes `/health`, `/healthz`, `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**`, `/categories/**`, `/categories/{category_id}`, and `/ai/models` from its own Axum router. The same `domain-posts` crate SHALL be composable as a `Box<dyn DomainService>` inside the gateway without duplicating routes, handlers, or migrations.

#### Scenario: Standalone post serves the full route set
- **WHEN** `cargo run -p domain-posts` is run with the same env-var surface as the current `my-cms-api`
- **THEN** the post, category, AI, and translation routes return the same status codes, envelopes, and authorization decisions as the composed gateway
- **AND** the domain's own `/health` returns 200 with the post service's health descriptor

### Requirement: application_core becomes a re-export shim
After this change, `application_core::commands::*` SHALL be empty (no command modules). `application_core::entities::mod.rs` SHALL be `pub use domain_posts::entities::*;`. `application_core::common::*` SHALL continue to expose `app_error::AppError`, `datetime_generator::generate_vietnam_now`, and `StringExtension` because `cms::api::{media,user,administrator}::*` and `test_helpers` still depend on them. The `migration` crate SHALL continue to re-export from `domain_posts::migrations::*`.

#### Scenario: Legacy cms::api handlers keep compiling through the shim
- **WHEN** `cargo check -p cms` runs
- **THEN** the legacy `cms::api::{media,user,administrator}::*` modules continue to compile
- **AND** they reference `application_core::entities::*` which forwards to `domain_posts::entities::*`
- **AND** no entity file lives in `application_core::entities` after the change

### Requirement: Post GraphQL contribution is registered by the domain
The post service SHALL contribute its Seaography schema/query registration to the gateway's immutable and mutable GraphQL mounts without the gateway importing post entities directly. Existing `/graphql/immutable` and `/graphql/mutable` behavior SHALL remain available.

#### Scenario: GraphQL schema includes post operations
- **WHEN** the gateway registers domain-posts and builds both schemas
- **THEN** post query fields remain available on immutable GraphQL
- **AND** the existing mutable authorization boundary remains enforced

### Requirement: Post migration ownership preserves schema
The post service SHALL own or explicitly reference the migrations for posts, post relations, translations, and translation jobs, while preserving the current ordered database history and schema-first entity regeneration. The migration identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`) SHALL be preserved exactly so the database `up` history is unchanged.

#### Scenario: Existing database upgrades without destructive reset
- **WHEN** the refactored migration runner is applied to a database at the current migration level
- **THEN** no existing post data is dropped
- **AND** already-applied migration identities are not re-executed

### Requirement: Post service is deployable as a standalone microservice
`domain-posts` SHALL be runnable as a standalone binary (its own `bin` target) that boots its own Axum server, opens its own database connection, applies its own auth/CORS/cookie/body-limit/OpenTelemetry layers, runs its own migrations through `cargo run -p domain-posts -- migrate`, and exposes `/health`, `/healthz`, `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**` from its own Axum router. The same `domain-posts` crate SHALL be composable as a `Box<dyn DomainService>` inside the gateway without duplicating routes, handlers, or migrations.

#### Scenario: Standalone domain serves the same route set
- **WHEN** `cargo run -p domain-posts` is run with the same env-var surface as the current `my-cms-api`
- **THEN** the post-related routes return the same status codes, envelopes, and authorization decisions as the composed gateway
- **AND** the domain's own `/health` returns 200 with the post service's health descriptor

