## MODIFIED Requirements

### Requirement: Single domain-owned API runtime
The system SHALL expose every supported CMS API route through the `my-cms-api` gateway using an owning domain service, and SHALL NOT require the legacy bootstrap runtime after cutover. The legacy bootstrap binary and the obsolete legacy API module tree SHALL be retired in Phase A ahead of full media/user gateway composition: once Phase A completes, the gateway `my-cms-api` binary is the sole full API runtime, and the `legacy_bootstrap` binary plus the `cms::api::*` adapter tree are removed from the workspace even though the media and user domain services are still being wired into `gateway::manifest()` in a separate follow-up change.

#### Scenario: Gateway exposes the complete route inventory
- **WHEN** the gateway is composed with post, media, user, and auth domain services
- **THEN** every route previously served by `legacy_bootstrap` is registered with the same HTTP method and path
- **AND** each route is registered in the same public, protected, or administrator authorization mount

#### Scenario: Legacy runtime is retired safely in Phase A
- **WHEN** Phase A cleanup completes
- **THEN** the `legacy_bootstrap` binary (`apps/api/src/bin/legacy_bootstrap.rs`) is removed from the workspace
- **AND** the `cms::api::*` adapter tree (`apps/api/src/api/**`) is removed from the workspace
- **AND** the `apps/api/Cargo.toml` workspace manifest no longer declares a `cms` package or a `my-cms-api` bin target inside it (the `gateway` crate is the sole producer of `my-cms-api`)
- **AND** the `application_core` and `migration` crates are removed from the workspace
- **AND** any media/user/administrator route still pending gateway registration remains temporarily unreachable until the follow-up cutover change wires those domains into `gateway::manifest()`
