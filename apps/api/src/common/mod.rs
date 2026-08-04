// `supabase_auth` module has been extracted into the `domain_auth` crate as
// part of the `extract-auth-into-domain-auth` OpenSpec change. The JWT
// validation layer, role-checking middleware, and the
// `construct_supabase_auth_layer` factory now live in
// `domain_auth::legacy_bootstrap`. See `openspec/changes/extract-auth-into-domain-auth/`.
