# extract-auth-into-domain-auth

Extract SupabaseAuthLayer, SupabaseAuthConfig, SupabaseClaims, and SupabaseToken out of domain_posts::domain::auth and cms::src/common::supabase_auth into a self-contained domain_auth crate. The auth layer is cross-cutting infrastructure used by every protected and administrator route across all domains (posts, categories, ai, translate, media, users, administrator). Pulling it out as its own crate lets every domain consume auth through domain_auth without any one domain owning it, and lets domain_auth be deployed, versioned, and tested independently.
