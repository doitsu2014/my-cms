## ADDED Requirements

### Requirement: Media commands have a dedicated domain boundary
The application core SHALL expose media and bucket command handlers, media DTOs, storage adapters, cache types, and media configuration through the `commands::media` domain module, with the existing Trait + Struct handler contracts and `Result<T, AppError>` behavior preserved.

#### Scenario: Existing media API operation resolves through the media domain
- **WHEN** an API adapter handles any existing media or bucket route
- **THEN** it invokes the same command handler behavior through `application_core::commands::media`
- **AND** the route, authentication boundary, request/response shape, status mapping, and external storage behavior remain unchanged.

#### Scenario: Media internal tests remain valid after relocation
- **WHEN** media command unit or integration tests execute after the module move
- **THEN** they compile and exercise the same success, validation, storage-failure, cache, and `AppError` paths
- **AND** no generated entity or migration is changed.

### Requirement: User commands have a dedicated domain boundary
The application core SHALL expose user CRUD, password reset, user DTOs, and the Supabase admin client through the `commands::user` domain module, preserving authorization, error, tracing, and external authentication behavior.

#### Scenario: Existing user API operation resolves through the user domain
- **WHEN** an API adapter handles an existing user operation
- **THEN** it invokes the same command handler behavior through `application_core::commands::user`
- **AND** route contracts, request/response shapes, auth checks, status mapping, and Supabase admin calls remain unchanged.

#### Scenario: User failure behavior is preserved
- **WHEN** the Supabase admin service returns a validation, authorization, not-found, conflict, or transport failure
- **THEN** the relocated command returns the same `AppError` category and observable HTTP mapping as before
- **AND** the command remains outside the API handler business-logic boundary.

### Requirement: Media domain owns its own crate

The system SHALL depend on media command handlers via the `domain_media` crate, not `application_core::commands::media`.

#### Scenario: API layer retargeted

- WHEN an API handler under `apps/api/src/api/media/**` constructs a media command handler
- THEN it SHALL `use domain_media::handlers::...`
- AND it SHALL NOT `use application_core::commands::media::...`

#### Scenario: AppState resolves from domain_media

- WHEN `apps/api/src/lib.rs` defines `AppState.media_config`, `AppState.media_cache`, `AppState.bucket_visibility_cache`
- THEN the types SHALL be re-exported from `domain_media`
- AND the field initialisation SHALL remain identical (no behavior change)

#### Scenario: Legacy module is removed

- WHEN the extraction is complete
- THEN `apps/api/application_core/src/commands/media/` SHALL NOT exist
- AND `pub mod media;` SHALL NOT appear in `application_core/src/commands/mod.rs`

#### Scenario: No public contract drift

- WHEN the extraction is complete
- THEN every HTTP route under `apps/api/src/api/media/**` SHALL respond with the same status codes and bodies as before
- AND the GraphQL schema SHALL be unchanged (media publishes no GraphQL today; preserve that)