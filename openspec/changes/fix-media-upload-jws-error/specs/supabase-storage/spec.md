# supabase-storage Delta Spec

## Purpose

This change corrects an implementation defect — the `apikey` header required by Kong's request-transformer is missing from Supabase Storage requests, and the `SERVICE_ROLE_KEY` is not a valid JWT. No requirement-level changes are needed; the existing `supabase-storage` spec already specifies correct behavior (Bearer auth to Supabase Storage). This delta exists only to satisfy the artifact dependency chain.

No ADDED, MODIFIED, REMOVED, or RENAMED requirements.
