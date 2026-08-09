//! `routes` — single entry point that builds the user domain's
//! `Vec<RouteRegistration>` for the gateway composition root.
//!
//! Wires six administrator-only Axum routes on `Mount::Administrator`:
//!
//! - `POST   /users`                       — create user
//! - `GET    /users`                       — list users
//! - `GET    /users/{id}`                  — read one user
//! - `PUT    /users/{id}`                  — modify user
//! - `DELETE /users/{id}`                  — delete user
//! - `POST   /users/{id}/reset-password`   — reset password
//!
//! Each adapter is a thin Axum function that constructs the command-handler
//! struct from `state.supabase_admin_client.clone()` and calls its
//! `*HandlerTrait::handle_*` method, mirroring
//! `apps/api/domain_posts/src/api/post/create/create_handler.rs:13-28`.

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use domain_interface::{AuthenticatedActor, DomainContext, Mount, RouteRegistration};
use uuid::Uuid;

use crate::{
    api::state::UserApiState,
    domain::response::{ApiResponseError, ApiResponseWith, AxumResponse},
    handlers::{
        create::create_handler::{CreateUserHandler, CreateUserHandlerTrait},
        create::create_request::CreateUserRequest,
        delete::delete_handler::{DeleteUserHandler, DeleteUserHandlerTrait},
        modify::modify_handler::{ModifyUserHandler, ModifyUserHandlerTrait},
        modify::modify_request::ModifyUserRequest,
        read_list::read_list_handler::{ReadListUserHandler, ReadListUserHandlerTrait},
        read_one::read_one_handler::{ReadOneUserHandler, ReadOneUserHandlerTrait},
        reset_password::{ResetPasswordHandler, ResetPasswordHandlerTrait, ResetPasswordRequest},
        supabase_admin_client::SupabaseAdminClient,
    },
};

/// `POST /users` — create a new CMS user.
pub async fn api_create_user(
    State(state): State<UserApiState>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let handler = CreateUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler.handle_create_user(body, &actor.user_id).await;

    match result {
        Ok(response) => ApiResponseWith::new(response).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// `GET /users` — list CMS users with optional filters.
pub async fn api_list_users(
    State(state): State<UserApiState>,
    _actor: Extension<AuthenticatedActor>,
) -> impl IntoResponse {
    let handler = ReadListUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler.handle_list_users(1, 50, None, None).await;

    match result {
        Ok(users) => ApiResponseWith::new(users).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// `GET /users/{id}` — fetch a single CMS user.
pub async fn api_get_user(
    State(state): State<UserApiState>,
    Path(id): Path<Uuid>,
    _actor: Extension<AuthenticatedActor>,
) -> impl IntoResponse {
    let handler = ReadOneUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler.handle_get_user(id).await;

    match result {
        Ok(user) => ApiResponseWith::new(user).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// `PUT /users/{id}` — modify an existing CMS user.
pub async fn api_modify_user(
    State(state): State<UserApiState>,
    Path(id): Path<Uuid>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<ModifyUserRequest>,
) -> impl IntoResponse {
    let handler = ModifyUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler.handle_modify_user(id, body, &actor.user_id).await;

    match result {
        Ok(user) => ApiResponseWith::new(user).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// `DELETE /users/{id}` — delete a CMS user. Self-delete is blocked.
pub async fn api_delete_user(
    State(state): State<UserApiState>,
    Path(id): Path<Uuid>,
    Extension(actor): Extension<AuthenticatedActor>,
) -> impl IntoResponse {
    let handler = DeleteUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler.handle_delete_user(id, &actor.user_id).await;

    match result {
        Ok(()) => ApiResponseWith::new(()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// `POST /users/{id}/reset-password` — reset a CMS user's password.
pub async fn api_reset_password(
    State(state): State<UserApiState>,
    Path(id): Path<Uuid>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<ResetPasswordRequest>,
) -> impl IntoResponse {
    let handler = ResetPasswordHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler
        .handle_reset_password(id, body, &actor.user_id)
        .await;

    match result {
        Ok(response) => ApiResponseWith::new(response).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}

/// Build the user domain's administrator router — six user CRUD routes.
fn administrator_router(state: UserApiState) -> Router<DomainContext> {
    Router::new()
        .route("/users", post(api_create_user).get(api_list_users))
        .route(
            "/users/{id}",
            get(api_get_user)
                .put(api_modify_user)
                .delete(api_delete_user),
        )
        .route("/users/{id}/reset-password", post(api_reset_password))
        .with_state(state)
}

/// Build the user domain's `RouteRegistration`s. The router builder takes
/// ownership of a `UserApiState` clone (which clones the underlying `Arc`) so
/// the returned `RouteRegistration`s are `Router<DomainContext>` as the
/// gateway expects.
pub fn routes(state: UserApiState) -> Vec<RouteRegistration> {
    vec![RouteRegistration {
        mount: Mount::Administrator,
        router: administrator_router(state),
        prefix: "users",
    }]
}

#[allow(dead_code)]
fn _ensure_supabase_admin_client_import_is_used(_: SupabaseAdminClient) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::supabase_admin_client::SupabaseAdminClient;

    fn build_test_state() -> UserApiState {
        UserApiState::new(SupabaseAdminClient::new(
            "http://localhost:9999".to_string(),
            "service-role-test-key".to_string(),
        ))
    }

    #[test]
    fn routes_returns_administrator_mount_only() {
        let regs = routes(build_test_state());
        assert_eq!(regs.len(), 1);
        let reg = &regs[0];
        assert_eq!(reg.mount, Mount::Administrator);
        assert_eq!(reg.prefix, "users");
    }
}
