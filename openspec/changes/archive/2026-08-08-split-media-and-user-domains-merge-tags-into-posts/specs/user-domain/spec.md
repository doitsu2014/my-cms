## ADDED Requirements

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

### Requirement: User domain owns its own crate

The system SHALL depend on user command handlers via the `domain_user` crate, not `application_core::commands::user`.

#### Scenario: API layer retargeted

- WHEN an API handler under `apps/api/src/api/user/**` constructs a user command handler
- THEN it SHALL `use domain_user::handlers::...`
- AND it SHALL NOT `use application_core::commands::user::...`

#### Scenario: SupabaseAdminClient lives in domain_user

- WHEN `apps/api/src/lib.rs` initialises `AppState.supabase_admin_client`
- THEN the type SHALL be `domain_user::handlers::supabase_admin_client::SupabaseAdminClient`
- AND the constructor signature SHALL remain unchanged

#### Scenario: Legacy module is removed

- WHEN the extraction is complete
- THEN `apps/api/application_core/src/commands/user/` SHALL NOT exist
- AND `pub mod user;` SHALL NOT appear in `application_core/src/commands/mod.rs`

#### Scenario: No public contract drift

- WHEN the extraction is complete
- THEN every HTTP route under `apps/api/src/api/user/**` SHALL respond with the same status codes and bodies as before
- AND authentication behavior SHALL be unchanged (auth concerns stay in `domain_auth`, admin GoTrue endpoints stay in `domain_user`)