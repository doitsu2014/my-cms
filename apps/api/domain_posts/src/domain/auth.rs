//! `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken` —
//! the JWT validation layer used by every protected post endpoint.
//!
//! The legacy source lives in `apps/api/src/common/supabase_auth.rs`. The
//! contract crate (`domain_interface`) does not depend on it; this module
//! is owned by `domain_posts` and re-exported from the legacy crate during
//! the transition.

pub use cms::common::supabase_auth::{
    SupabaseAuthConfig, SupabaseAuthLayer, SupabaseClaims, SupabaseToken,
};