//! `UserApiState` — per-process state for the user domain's Axum router.
//!
//! Holds the `Arc<SupabaseAdminClient>` shared with every per-request handler.
//! Constructed once by `DomainUserService` at gateway startup, then cloned
//! (`Arc`-wrapped) into every per-request handler. The `Debug` impl defers to
//! `SupabaseAdminClient`'s own redaction so the service-role key never appears
//! in diagnostics.

use std::sync::Arc;

use crate::handlers::supabase_admin_client::SupabaseAdminClient;

/// Wrapper struct so the router's `State<UserApiState>` is a single
/// `Clone`-able value. The inner field is `Arc`-shared.
#[derive(Clone)]
pub struct UserApiState {
    pub supabase_admin_client: Arc<SupabaseAdminClient>,
}

impl std::fmt::Debug for UserApiState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserApiState")
            .field("supabase_admin_client", &self.supabase_admin_client)
            .finish()
    }
}

impl UserApiState {
    /// Build a `UserApiState` from a `SupabaseAdminClient`.
    pub fn new(client: SupabaseAdminClient) -> Self {
        Self {
            supabase_admin_client: Arc::new(client),
        }
    }
}