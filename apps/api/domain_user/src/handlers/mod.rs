//! User-domain handlers — owns user CRUD, password reset, the
//! `SupabaseAdminClient` adapter, and the user DTOs.

pub mod create;
pub mod delete;
pub mod modify;
pub mod read_list;
pub mod read_one;
pub mod reset_password;
pub mod supabase_admin_client;
