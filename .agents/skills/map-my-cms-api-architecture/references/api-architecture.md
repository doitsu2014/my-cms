# My-CMS API Architecture Baseline

Source-derived snapshot: 2026-07-23. Revalidate against the current repository
before making decisions.

## 1. Workspace and dependency direction

| Area | Source anchor | Observed responsibility |
|---|---|---|
| Runtime/API | `apps/api/gateway/src/main.rs` and `apps/api/domain_*/src/` | Gateway composition, routers, middleware, domain-owned HTTP handlers, response mapping, and domain state |
| Domain handlers and entities | `apps/api/domain_*/src/handlers/`, `apps/api/domain_posts/src/entities/` | Domain business logic, DTO conversion, SeaORM queries/transactions, and canonical generated entities |
| Migrations | `apps/api/domain_posts/src/migrations/` | Canonical ordered SeaORM migrations; the operator CLI is the `domain_posts` bin's `migrate` subcommand (`apps/api/domain_posts/src/main.rs` → `domain_posts::migrations_cli::handle_args`) |
| Test support | `apps/api/test_helpers/src/lib.rs` | PostgreSQL/pgvector testcontainers and migration refresh; imports `domain_posts::migrations::{Migrator, MigratorTrait}` directly |

> **Phase A and migration-crate cleanup status:** `application_core`, the legacy `apps/api/src/**` runtime, and `apps/api/migration/` were all removed by [`purge-legacy-cms-and-application-core`](../../../../openspec/changes/purge-legacy-cms-and-application-core/). Canonical migrations and the operator CLI both live under `domain_posts`; `test_helpers` imports `domain_posts::migrations` directly. References to `legacy_bootstrap` below are historical: the binary is gone; the `domain_auth::legacy_bootstrap` module name is a separately labelled historical reference on the auth-layer factory.

## 2. Runtime composition

`apps/api/gateway/src/main.rs` composes routes contributed by registered domain services into public, protected, and administrator router groups.

- public: health, public media delivery, immutable GraphQL;
- protected: category/post/tag/media management, translation, AI models, mutable
  GraphQL; writer or administrator role;
- administrator: migrations, users, and bucket management; administrator role.

Each domain service owns its configuration and state; the gateway supplies shared `DomainContext` dependencies during composition.


## 3. Request and business flow

The dominant REST flow is:

```text
Gateway route composition
  -> domain API adapter in apps/api/domain_*/src/api/<capability>/
  -> construct domain handler with Arc dependencies
  -> call *HandlerTrait method
  -> SeaORM transaction/query or external client
  -> Result<T, domain AppError>
  -> domain response mapping
```

Representative anchors:

- API adapter: `apps/api/domain_posts/src/api/`
- transactional command: `apps/api/domain_posts/src/handlers/category/create/`
- request and response models: `apps/api/domain_posts/src/domain/`
- gateway composition: `apps/api/gateway/src/main.rs`

Handlers generally expose traits returning futures. Business logic belongs in the owning domain crate; API adapters extract state/auth/path/body and translate responses.

## 4. Authentication and authorization

`apps/api/domain_auth/src/` implements the Tower authentication layer that:

- validates Supabase JWTs with an audience;
- supports shared-secret and JWKS-based verification paths;
- applies OR semantics to configured required roles;
- inserts `SupabaseToken` into request extensions;
- returns 401 for invalid authentication and 403 for missing roles.

Writer routes allow `my-headless-cms-writer` or
`my-headless-cms-administrator`; administrative routes require
`my-headless-cms-administrator`. External admin operations use a service-role
key through `SupabaseAdminClient`; its `Debug` implementation redacts the key.

## 5. Domain capabilities and persistence

### Categories, posts, tags, and translations

- SeaORM entities are generated under `apps/api/domain_posts/src/entities/`; do not edit them manually.
- Categories are self-referencing and have posts, tags through
  `category_tags`, and translations.
- Posts belong to categories and have tags through `post_tags` and
  translations.
- Create/modify flows use SeaORM transactions for aggregate updates.
- Category and post updates use `row_version` predicates for optimistic
  concurrency.
- Source currently maps a zero-row version update to `AppError::Logical`,
  although `AppError::ConcurrencyOptimistic` also exists. Treat this as a
  contract decision to review, not an automatic precedent.

### User administration

User records are managed through Supabase GoTrue Admin HTTP endpoints rather
than local SeaORM entities. `SupabaseAdminClient` maps upstream status codes
into `AppError`, and command handlers apply domain validation, recognized-role
rules, actor logging, and self-protection behavior. Wiremock tests cover the
upstream contract and secret-redaction expectations.

### Media and buckets

Media uses a Supabase Storage-compatible client in `apps/api/domain_media/src/handlers/supabase_storage.rs`.

- public media is proxied through `/media/...`;
- image rendering supports width/height variants;
- bytes are cached in-process by bucket, path, width, and height;
- bucket visibility is cached separately;
- non-admin access to private buckets is obscured as not found;
- administrator routes manage bucket lifecycle.

Review cache invalidation, private/public behavior, MIME/size constraints,
reserved bucket rules, and service-role handling for every media change.

### AI translation

`PostTranslateHandler` implements:

1. reuse an existing post translation;
2. search pgvector for a sufficiently similar translation;
3. call OpenAI and store the result/embedding.

Synchronous and `tokio::spawn` background paths exist. Background status is
stored in `translation_jobs`; this is durable status around an in-process task,
not a durable queue. The pgvector migration conditionally creates a 1536-
dimension embeddings table and IVFFlat cosine index when the extension exists.
The API treats vector-store initialization as optional and falls back to direct
translation.

### GraphQL

Seaography dynamically registers category/post/tag/translation entities.
`/graphql/immutable` disables mutations and is public; `/graphql/mutable` is
protected by writer/admin auth. The runtime currently supplies no explicit
depth or complexity limits to `schema(...)`. Review exposure, mutation scope,
authorization granularity, and query-cost controls for GraphQL changes.

## 6. Error and response model

`AppError` includes database, transaction, storage, validation, logical,
conflict, optimistic-concurrency, not-found, unknown, and OpenAI variants.
`ApiResponseError` maps these to HTTP status and numeric/string error codes.
Successful `ApiResponseWith<T>` responses currently use HTTP 200.

When designing changes, specify the exact `AppError` and HTTP behavior. Do not
leak service keys, upstream bodies containing secrets, or internal database
details.

## 7. Observability and tests

- Important API and command functions commonly use `#[instrument]`.
- OpenTelemetry middleware wraps all router groups.
- Module-local tests use `#[test]`, `#[async_std::test]`, and
  `#[tokio::test]`.
- Database behavior is commonly tested with PostgreSQL testcontainers, not only
  SeaORM mocks.
- Supabase Storage/GoTrue behavior is commonly tested with wiremock.
- pgvector behavior uses a pgvector PostgreSQL container.
- Some live OpenAI tests are ignored; design for dependency injection when
  deterministic tests require replacing the OpenAI client.

Repository verification:

```text
cargo check
cargo test
cargo fmt -- --check
cargo clippy
pnpm --dir apps/web build
```

## 8. Recheck these architecture risks

These are observed review points, not blanket bug claims:

- three independently constructed `AppState` values before router merge;
- in-process caches and in-process background translation execution;
- permissive CORS (`Any`) in the runtime;
- public immutable GraphQL and broad generated mutable GraphQL;
- conditional pgvector schema availability;
- `unwrap`/`expect` still present in runtime/bootstrap/response paths despite
  stricter repository policy;
- optimistic-lock mismatch semantics are not consistently represented by the
  dedicated error variant;
- aggregate update handlers may perform read operations through their
  connection-backed read handler while a transaction is active.

Revalidate the relevant point in current source and record whether a proposed
change preserves, fixes, or deliberately accepts it.
