//! Required environment variables and validation for the post domain.

/// Required env vars for the post service's translation pipeline.
pub const POST_REQUIRED_ENV: &[&str] = &[
    "DATABASE_URL",
    "SUPABASE_URL",
    "SUPABASE_JWT_SECRET",
    "OPENAI_API_KEY",
];

/// Validate that every required env var is present.
pub fn validate_env() -> Result<(), String> {
    for var in POST_REQUIRED_ENV {
        if std::env::var(var).is_err() {
            return Err(format!("Missing required env var: {}", var));
        }
    }
    Ok(())
}