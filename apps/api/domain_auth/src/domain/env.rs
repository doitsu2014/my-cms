//! Required environment variables and validation for the auth domain.

/// Required env vars for the auth-domain service.
pub const AUTH_REQUIRED_ENV: &[&str] = &[
    "SUPABASE_URL",
    "SUPABASE_JWT_SECRET",
    "AUTHORIZATION_AUDIENCE",
];

/// Validate that every required env var is present. Returns a comma-separated
/// error message listing every missing variable, or `Ok(())` when all are set.
pub fn validate() -> Result<(), String> {
    let missing: Vec<&str> = AUTH_REQUIRED_ENV
        .iter()
        .copied()
        .filter(|var| std::env::var(var).is_err())
        .collect();

    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "Missing required auth env vars: {}",
            missing.join(", ")
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_lock::{with_env_var, ENV_LOCK};

    #[test]
    fn validate_succeeds_when_all_required_vars_are_set() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        with_env_var("SUPABASE_URL", Some("http://localhost"), || {
            with_env_var("SUPABASE_JWT_SECRET", Some("secret"), || {
                with_env_var("AUTHORIZATION_AUDIENCE", Some("authenticated"), || {
                    assert!(validate().is_ok());
                })
            })
        });
    }

    #[test]
    fn validate_lists_missing_vars_in_error_message() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Ensure all three are set, then unset one and verify the error mentions it.
        with_env_var("SUPABASE_URL", Some("http://localhost"), || {
            with_env_var("SUPABASE_JWT_SECRET", Some("secret"), || {
                with_env_var("AUTHORIZATION_AUDIENCE", None, || match validate() {
                    Err(msg) => {
                        assert!(msg.contains("AUTHORIZATION_AUDIENCE"));
                    }
                    Ok(()) => panic!("expected error when AUTHORIZATION_AUDIENCE is unset"),
                })
            })
        });
    }

    #[test]
    fn auth_required_env_lists_exactly_three_vars() {
        assert_eq!(AUTH_REQUIRED_ENV.len(), 3);
        assert!(AUTH_REQUIRED_ENV.contains(&"SUPABASE_URL"));
        assert!(AUTH_REQUIRED_ENV.contains(&"SUPABASE_JWT_SECRET"));
        assert!(AUTH_REQUIRED_ENV.contains(&"AUTHORIZATION_AUDIENCE"));
    }
}
