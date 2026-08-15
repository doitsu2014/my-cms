# rename-legacy-bootstrap-auth-factory

Rename domain_auth::legacy_bootstrap module + construct_supabase_auth_layer function and modernize the factory to return Result<T, DomainConfigError> instead of panicking on missing env vars.
