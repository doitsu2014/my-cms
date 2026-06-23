use std::sync::Arc;

use crate::{
    commands::user::{
        create::create_request::CreateUserRequest,
        dto::{
            is_recognised_role, sanitise_email, AppUserModel, CreateUserResponse, RECOGNISED_ROLES,
        },
        supabase_admin_client::SupabaseAdminClient,
    },
    common::app_error::AppError,
};
use tracing::{info, instrument};

pub trait CreateUserHandlerTrait {
    fn handle_create_user(
        &self,
        req: CreateUserRequest,
        actor_user_id: &str,
    ) -> impl std::future::Future<Output = Result<CreateUserResponse, AppError>> + Send;
}

#[derive(Debug)]
pub struct CreateUserHandler {
    pub supabase: Arc<SupabaseAdminClient>,
}

impl CreateUserHandlerTrait for CreateUserHandler {
    #[instrument(skip(self))]
    async fn handle_create_user(
        &self,
        req: CreateUserRequest,
        actor_user_id: &str,
    ) -> Result<CreateUserResponse, AppError> {
        let email = sanitise_email(&req.email);
        if email.is_empty() {
            return Err(AppError::Validation(
                "email".to_string(),
                "Email must not be empty".to_string(),
            ));
        }
        if req.password.len() < 8 {
            return Err(AppError::Validation(
                "password".to_string(),
                "Password must be at least 8 characters".to_string(),
            ));
        }
        if !is_recognised_role(&req.role) {
            return Err(AppError::Validation(
                "role".to_string(),
                format!("Role must be one of: {}", RECOGNISED_ROLES.join(", ")),
            ));
        }

        let normalised = CreateUserRequest {
            email,
            password: req.password.clone(),
            role: req.role.clone(),
        };
        let user: AppUserModel = self.supabase.create_user(&normalised).await?;

        info!(
            action = "create",
            actor_user_id = actor_user_id,
            target_user_id = %user.id,
            "admin user action"
        );

        Ok(CreateUserResponse {
            user,
            temporary_password: req.password,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::user::supabase_admin_client::SupabaseAdminClient;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    async fn setup_handler_with_mock() -> (CreateUserHandler, MockServer) {
        let server = MockServer::start().await;
        let client = Arc::new(SupabaseAdminClient::new(
            server.uri(),
            "service-role-test-key",
        ));
        (CreateUserHandler { supabase: client }, server)
    }

    #[async_std::test]
    async fn handle_create_user_returns_response_on_success() {
        let (handler, server) = setup_handler_with_mock().await;

        Mock::given(method("POST"))
            .and(path("/auth/v1/admin/users"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({
                "id": "33333333-3333-3333-3333-333333333333",
                "email": "alice@example.com",
                "app_metadata": { "roles": ["my-headless-cms-writer"] },
                "created_at": "2026-01-01T00:00:00Z",
                "updated_at": "2026-01-01T00:00:00Z",
                "last_sign_in_at": null,
                "banned_until": null
            })))
            .mount(&server)
            .await;

        let req = CreateUserRequest {
            email: "ALICE@Example.com".to_string(),
            password: "supersecret".to_string(),
            role: "my-headless-cms-writer".to_string(),
        };
        let result = handler
            .handle_create_user(req, "actor-1")
            .await
            .expect("create ok");

        assert_eq!(result.user.email, "alice@example.com");
        assert_eq!(result.user.role.as_deref(), Some("my-headless-cms-writer"));
        assert_eq!(result.temporary_password, "supersecret");
    }

    #[async_std::test]
    async fn handle_create_user_rejects_short_password() {
        let (handler, _server) = setup_handler_with_mock().await;
        let req = CreateUserRequest {
            email: "alice@example.com".to_string(),
            password: "short".to_string(),
            role: "my-headless-cms-writer".to_string(),
        };
        let err = handler
            .handle_create_user(req, "actor-1")
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Validation(_, _)));
    }

    #[async_std::test]
    async fn handle_create_user_rejects_unknown_role() {
        let (handler, _server) = setup_handler_with_mock().await;
        let req = CreateUserRequest {
            email: "alice@example.com".to_string(),
            password: "supersecret".to_string(),
            role: "made-up-role".to_string(),
        };
        let err = handler
            .handle_create_user(req, "actor-1")
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Validation(_, _)));
    }

    #[async_std::test]
    async fn handle_create_user_returns_conflict_on_duplicate_email() {
        let (handler, server) = setup_handler_with_mock().await;

        Mock::given(method("POST"))
            .and(path("/auth/v1/admin/users"))
            .respond_with(ResponseTemplate::new(409).set_body_string("email exists"))
            .mount(&server)
            .await;

        let req = CreateUserRequest {
            email: "alice@example.com".to_string(),
            password: "supersecret".to_string(),
            role: "my-headless-cms-writer".to_string(),
        };
        let err = handler
            .handle_create_user(req, "actor-1")
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Conflict(_)));
    }
}
