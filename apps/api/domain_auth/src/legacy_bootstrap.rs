//! Legacy-bootstrap factory for the Supabase auth layer.
//!
//! This is the single source of truth for constructing `SupabaseAuthLayer`
//! instances from the env-var surface. Both the legacy `legacy_bootstrap`
//! binary and the gateway composition root call this function.

use std::env;

use crate::{SupabaseAuthConfig, SupabaseAuthLayer};

/// Build a `SupabaseAuthLayer` from the env-var surface.
///
/// Reads:
/// - `SUPABASE_URL` (required) — public Supabase URL used for JWKS fallback.
/// - `SUPABASE_INTERNAL_URL` (optional) — internal URL used as a fallback for
///   `SUPABASE_URL` (the JWT validator uses this for JWKS fetches inside the
///   container network).
/// - `SUPABASE_JWT_SECRET` (required) — HS256 secret for local JWT validation.
///
/// # Arguments
///
/// - `expected_audience` — the `aud` claim value the JWT must carry.
/// - `required_roles` — the role(s) a request must hold to pass through.
///   Empty vector allows any authenticated user.
pub fn construct_supabase_auth_layer(
    expected_audience: String,
    required_roles: Vec<String>,
) -> SupabaseAuthLayer {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let supabase_internal_url =
        env::var("SUPABASE_INTERNAL_URL").unwrap_or_else(|_| supabase_url.clone());
    let jwt_secret = env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");

    SupabaseAuthLayer::new(SupabaseAuthConfig {
        supabase_url: supabase_internal_url,
        jwt_secret,
        expected_audience,
        required_roles,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_lock::ENV_LOCK;

    fn with_env_var<F, R>(var: &str, value: Option<&str>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let previous = env::var(var).ok();
        match value {
            Some(v) => env::set_var(var, v),
            None => env::remove_var(var),
        }
        let result = f();
        match previous {
            Some(v) => env::set_var(var, v),
            None => env::remove_var(var),
        }
        result
    }

    #[test]
    fn construct_factory_returns_auth_layer_when_env_vars_are_set() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_env_var("SUPABASE_URL", Some("http://localhost:8001"), || {
            with_env_var("SUPABASE_INTERNAL_URL", None, || {
                with_env_var("SUPABASE_JWT_SECRET", Some("test-secret"), || {
                    let _layer = construct_supabase_auth_layer(
                        "authenticated".to_string(),
                        vec!["writer".to_string()],
                    );
                })
            })
        });
    }

    #[test]
    fn construct_factory_uses_supabase_internal_url_when_present() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_env_var("SUPABASE_URL", Some("http://public-host:8000"), || {
            with_env_var(
                "SUPABASE_INTERNAL_URL",
                Some("http://internal-host:8001"),
                || {
                    with_env_var("SUPABASE_JWT_SECRET", Some("test-secret"), || {
                        let layer =
                            construct_supabase_auth_layer("authenticated".to_string(), vec![]);
                        // We can only assert construction here; the URL field is private.
                        drop(layer);
                    })
                },
            )
        });
    }

    #[test]
    #[should_panic(expected = "SUPABASE_URL must be set")]
    fn construct_factory_panics_when_supabase_url_is_missing() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_env_var("SUPABASE_URL", None, || {
            with_env_var("SUPABASE_JWT_SECRET", Some("test-secret"), || {
                let _layer = construct_supabase_auth_layer("authenticated".to_string(), vec![]);
            })
        });
    }

    #[test]
    #[should_panic(expected = "SUPABASE_JWT_SECRET must be set")]
    fn construct_factory_panics_when_supabase_jwt_secret_is_missing() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_env_var("SUPABASE_URL", Some("http://localhost:8001"), || {
            with_env_var("SUPABASE_JWT_SECRET", None, || {
                let _layer = construct_supabase_auth_layer("authenticated".to_string(), vec![]);
            })
        });
    }
}
