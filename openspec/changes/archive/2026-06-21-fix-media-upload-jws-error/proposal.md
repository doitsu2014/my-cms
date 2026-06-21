## Why

Media uploads fail with `403 "Invalid Compact JWS"` because the Rust backend's Supabase Storage client omits the `apikey` header that Kong's request-transformer requires to reconstruct the `Authorization` header. Additionally, the configured `SERVICE_ROLE_KEY` (`devkey`) is not a valid JWT, so even if the header issue were fixed, authentication would still fail.

## What Changes

- Add `.header("apikey", self.auth_key())` to all 6 Supabase Storage HTTP methods (upload, download, get_info, list_objects, delete, delete_batch) so Kong can forward the correct Bearer token
- Fix `SERVICE_ROLE_KEY` in `deployments/docker-swarm/apps/.env.example` to match the valid JWT from `supabase/.env.example`
- Add a note in `deployments/docker-swarm/apps/.env.example` about the relationship between `apikey`/`Authorization` headers and Kong's `request-transformer`

## Capabilities

### New Capabilities

None — this is a bug fix that corrects the implementation to match the existing `supabase-storage` spec.

### Modified Capabilities

None — the `supabase-storage` spec's requirements (Bearer auth, upload/download/delete operations) are already correct; only the implementation deviates.

## Impact

- **Affected code**: `apps/api/application_core/src/commands/media/supabase_storage.rs` (all 6 HTTP methods)
- **Affected config**: `deployments/docker-swarm/apps/.env.example` (`SERVICE_ROLE_KEY`)
- **No API contract changes**: request/response shapes are unchanged
- **No database changes**
