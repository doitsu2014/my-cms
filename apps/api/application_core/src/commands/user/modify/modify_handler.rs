use std::sync::Arc;

use crate::{
    commands::user::{
        dto::{
            is_recognised_role, sanitise_email, validate_full_name, validate_phone, AppUserModel,
            RECOGNISED_ROLES,
        },
        modify::modify_request::ModifyUserRequest,
        supabase_admin_client::SupabaseAdminClient,
    },
    common::app_error::AppError,
};
use tracing::{info, instrument};
use uuid::Uuid;

pub trait ModifyUserHandlerTrait {
    fn handle_modify_user(
        &self,
        id: Uuid,
        req: ModifyUserRequest,
        actor_user_id: &str,
    ) -> impl std::future::Future<Output = Result<AppUserModel, AppError>> + Send;
}

#[derive(Debug)]
pub struct ModifyUserHandler {
    pub supabase: Arc<SupabaseAdminClient>,
}

impl ModifyUserHandlerTrait for ModifyUserHandler {
    #[instrument(skip(self))]
    async fn handle_modify_user(
        &self,
        id: Uuid,
        req: ModifyUserRequest,
        actor_user_id: &str,
    ) -> Result<AppUserModel, AppError> {
        if let Some(ref role) = req.role {
            if !is_recognised_role(role) {
                return Err(AppError::Validation(
                    "role".to_string(),
                    format!("Role must be one of: {}", RECOGNISED_ROLES.join(", ")),
                ));
            }
        }
        if let Some(ref email) = req.email {
            let trimmed = email.trim();
            if trimmed.is_empty() {
                return Err(AppError::Validation(
                    "email".to_string(),
                    "Email must not be empty".to_string(),
                ));
            }
        }
        if let Some(ref full_name) = req.full_name {
            if !full_name.is_empty() {
                validate_full_name(full_name)?;
            }
        }
        if let Some(ref phone) = req.phone {
            if !phone.is_empty() {
                validate_phone(phone)?;
            }
        }

        let normalised = ModifyUserRequest {
            email: req.email.as_ref().map(|e| sanitise_email(e)),
            role: req.role.clone(),
            banned: req.banned,
            full_name: req.full_name.clone(),
            phone: req.phone.clone(),
        };

        let user = self.supabase.update_user(id, &normalised).await?;

        info!(
            action = "update",
            actor_user_id = actor_user_id,
            target_user_id = %user.id,
            "admin user action"
        );

        Ok(user)
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

    async fn setup_handler_with_mock() -> (ModifyUserHandler, MockServer) {
        let server = MockServer::start().await;
        let client = Arc::new(SupabaseAdminClient::new(
            server.uri(),
            "service-role-test-key",
        ));
        (ModifyUserHandler { supabase: client }, server)
    }

    #[async_std::test]
    async fn handle_modify_user_rejects_unknown_role() {
        let (handler, _server) = setup_handler_with_mock().await;
        let id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();
        let req = ModifyUserRequest {
            email: None,
            role: Some("made-up-role".to_string()),
            banned: None,
            full_name: None,
            phone: None,
        };
        let err = handler
            .handle_modify_user(id, req, "actor-1")
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Validation(_, _)));
    }

    #[async_std::test]
    async fn handle_modify_user_returns_not_found_on_404() {
        let (handler, server) = setup_handler_with_mock().await;
        let id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();

        Mock::given(method("PUT"))
            .and(path(
                "/auth/v1/admin/users/33333333-3333-3333-3333-333333333333",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&server)
            .await;

        let req = ModifyUserRequest {
            email: None,
            role: Some("my-headless-cms-writer".to_string()),
            banned: None,
            full_name: None,
            phone: None,
        };
        let err = handler
            .handle_modify_user(id, req, "actor-1")
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::NotFound));
    }

    #[async_std::test]
    async fn handle_modify_user_translates_banned_true_to_update() {
        let (handler, server) = setup_handler_with_mock().await;
        let id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();

        Mock::given(method("PUT"))
            .and(path(
                "/auth/v1/admin/users/33333333-3333-3333-3333-333333333333",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "33333333-3333-3333-3333-333333333333",
                "email": "alice@example.com",
                "app_metadata": { "roles": ["my-headless-cms-writer"] },
                "created_at": "2026-01-01T00:00:00Z",
                "updated_at": "2026-01-02T00:00:00Z",
                "last_sign_in_at": null,
                "banned_until": "2099-01-01T00:00:00Z"
            })))
            .mount(&server)
            .await;

        let req = ModifyUserRequest {
            email: None,
            role: None,
            banned: Some(true),
            full_name: None,
            phone: None,
        };
        let user = handler
            .handle_modify_user(id, req, "actor-1")
            .await
            .expect("update ok");
        assert!(user.banned);
    }
}
