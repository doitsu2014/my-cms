# extract-auth-into-domain-auth

Extract SupabaseAuthLayer, SupabaseAuthConfig, SupabaseClaims, and SupabaseToken out of `domain_posts::domain::auth` and `cms::src/common::supabase_auth` into a self-contained `domain_auth` crate. The auth layer is cross-cutting infrastructure used by every protected and administrator route across all domains (posts, categories, ai, translate, media, users, administrator). Pulling it out as its own crate lets every domain consume auth through `domain_auth` without any one domain owning it, and lets `domain_auth` be deployed, versioned, and tested independently.

## Template for future domain extractions

This change is the **reference migration** for extracting any future business domain (`domain-media`, `domain-users`, `domain-administrator`) out of the legacy `cms` tree or out of `domain_posts`. Future authors should:

1. Copy the artifact structure (`proposal.md`, `design.md`, `tasks.md`, `specs/<capability>/spec.md`) and follow the five-phase migration pattern documented in `tasks.md`.
2. Read the **Template value for future domains** section in `design.md` for the architecture decisions and contract patterns.
3. Implement `DomainService` against `domain_interface`, ensuring the contract-compliance tests pass (see task group 5 of this change).
4. Update every consumer's `use` statement and `Extension<...>` extractor mechanically (see task group 6 of this change for the exhaustive file list pattern).
5. Update `docs/adding-a-domain.md` with a domain-specific checklist derived from the steps that worked here.

The contract changes introduced by this change are reusable:

- `domain_interface::AuthenticatedActor` is the canonical actor type for every future domain's HTTP adapters. Future domains import `AuthenticatedActor` from `domain_interface`, never `SupabaseToken` from `domain_auth`.
- `DomainService::startup_health` has a default `Ok(())` impl. Infrastructure-only domains use the default; DB-backed domains override with a `SELECT 1` probe.
