## Context

The My-CMS backend communicates with Supabase Storage through Kong API Gateway. Kong's `request-transformer` plugin (defined in `kong.yml`) rewrites the `Authorization` header using the Lua expression `Bearer $(headers.apikey)`. This means:

1. The client must send an `apikey` header containing the auth key
2. Kong **replaces** `Authorization` with `Bearer <value-of-apikey-header>`
3. The original `Authorization` header is discarded

The Rust backend's `SupabaseStorage` client (in `supabase_storage.rs`) calls `.bearer_auth(self.auth_key())` which sets the `Authorization` header directly, but never sends an `apikey` header. Kong's Lua expression evaluates to `Bearer ` (empty string), and Supabase Storage rejects the empty token as "Invalid Compact JWS".

Additionally, `deployments/docker-swarm/apps/.env.example` has `SERVICE_ROLE_KEY=devkey`, which is not a valid JWT. Even if Kong forwarded it, Storage would reject it. The Supabase stack (`supabase/.env.example`) uses a proper HS256 JWT.

## Goals / Non-Goals

**Goals:**
- Fix all 6 storage HTTP methods to send `apikey` header alongside `Authorization`, matching the pattern Kong expects
- Sync `SERVICE_ROLE_KEY` in `apps/.env.example` to match the valid JWT in `supabase/.env.example`
- Zero impact on API contract (same request/response shapes)

**Non-Goals:**
- Changing how Kong's storage route is configured
- Forwarding the user's GoTrue JWT for RLS (defer to a future change)
- Altering the media upload flow or bucket structure

## Decisions

### Decision 1: Add `apikey` header alongside `bearer_auth`

**Chosen:** Add `.header("apikey", self.auth_key())` to every HTTP request builder in `supabase_storage.rs`.

**Alternative considered:** Remove Kong's `request-transformer` for the Storage route and use `key-auth` instead. Rejected because:
- Kong's `key-auth` for the Storage route would require API consumers (including the web frontend) to present a Kong API key, which is not how Supabase clients normally work
- The `request-transformer` approach is standard in the Supabase self-hosted Kong configuration
- The Supabase JS SDK sends both `apikey` and `Authorization` headers — we should match that pattern

**Alternative considered:** Modify Kong's Lua expression to read `Authorization` directly instead of `apikey`. Rejected because:
- This would require changes to the deployment infra (Kong config) that are harder to test/verify
- Keeping the `apikey` approach aligns with the official Supabase SDK pattern
- Backend-only fix is simpler, safer, and easier to rollback

### Decision 2: Where to add headers

**Chosen:** Inline in each of the 6 methods (`upload`, `download`, `get_info`, `list_objects`, `delete`, `delete_batch`) rather than a shared helper.

**Rationale:** Each method has a unique request builder chain (different HTTP methods, endpoints, body types). A shared helper would need to be generic over the builder type, adding complexity for 6 one-line additions. Inline is clearer and matches the existing code style.

### Decision 3: `SERVICE_ROLE_KEY` fix

**Chosen:** Update `apps/.env.example` to have `SERVICE_ROLE_KEY=eyJhbGci...U3FDbs` (matching `supabase/.env.example`).

**Alternative considered:** Generate a random key. Rejected because the Supabase stack already has this baked into its JWT secret; changing it would break RLS/service_role operations across the stack.

## Risks / Trade-offs

- **Risk:** Fixing the `SERVICE_ROLE_KEY` in `.env.example` doesn't update active `.env` files on developer machines → **Mitigation:** Document the change clearly in the task; developers must manually update their local `.env`
- **Risk:** The `apikey` header is sent to Supabase Storage directly (not just through Kong), which may cause confusion in non-Docker setups → **Mitigation:** Supabase Storage ignores unknown headers; it's a no-op outside Kong environments
