use std::sync::Arc;

use crate::{
    commands::user::{
        reset_password::{ResetPasswordRequest, ResetPasswordResponse},
        supabase_admin_client::SupabaseAdminClient,
    },
    common::app_error::AppError,
};
use tracing::{info, instrument};
use uuid::Uuid;

pub trait ResetPasswordHandlerTrait {
    fn handle_reset_password(
        &self,
        id: Uuid,
        req: ResetPasswordRequest,
        actor_user_id: &str,
    ) -> impl std::future::Future<Output = Result<ResetPasswordResponse, AppError>> + Send;
}

#[derive(Debug)]
pub struct ResetPasswordHandler {
    pub supabase: Arc<SupabaseAdminClient>,
}

impl ResetPasswordHandlerTrait for ResetPasswordHandler {
    #[instrument(skip(self))]
    async fn handle_reset_password(
        &self,
        id: Uuid,
        req: ResetPasswordRequest,
        actor_user_id: &str,
    ) -> Result<ResetPasswordResponse, AppError> {
        if req.password.len() < 8 {
            return Err(AppError::Validation(
                "password".to_string(),
                "Password must be at least 8 characters".to_string(),
            ));
        }

        self.supabase.reset_password(id, &req.password).await?;

        info!(
            action = "reset_password",
            actor_user_id = actor_user_id,
            target_user_id = %id,
            "admin user action"
        );

        Ok(ResetPasswordResponse {
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

    async fn setup_handler_with_mock() -> (ResetPasswordHandler, MockServer) {
        let server = MockServer::start().await;
        let client = Arc::new(SupabaseAdminClient::new(
            server.uri(),
            "service-role-test-key",
        ));
        (ResetPasswordHandler { supabase: client }, server)
    }

    #[async_std::test]
    async fn handle_reset_password_rejects_short_password() {
        let (handler, _server) = setup_handler_with_mock().await;
        let id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();
        let req = ResetPasswordRequest {
            password: "short".to_string(),
        };
        let err = handler
            .handle_reset_password(id, req, "actor-1")
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Validation(_, _)));
    }

    #[async_std::test]
    async fn handle_reset_password_returns_response_on_success() {
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
                "user_metadata": { "full_name": "Alice Example" },
                "phone": "+1 555-0100",
                "created_at": "2026-01-01T00:00:00Z",
                "updated_at": "2026-01-02T00:00:00Z",
                "last_sign_in_at": null,
                "banned_until": null
            })))
            .mount(&server)
            .await;

        let req = ResetPasswordRequest {
            password: "newsecret123".to_string(),
        };
        let result = handler
            .handle_reset_password(id, req, "actor-1")
            .await
            .expect("reset ok");
        assert_eq!(result.temporary_password, "newsecret123");
    }

    #[async_std::test]
    async fn handle_reset_password_returns_not_found_on_404() {
        let (handler, server) = setup_handler_with_mock().await;
        let id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();

        Mock::given(method("PUT"))
            .and(path(
                "/auth/v1/admin/users/33333333-3333-3333-3333-333333333333",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&server)
            .await;

        let req = ResetPasswordRequest {
            password: "newsecret123".to_string(),
        };
        let err = handler
            .handle_reset_password(id, req, "actor-1")
            .await
            .expect_err("expected 404");
        assert!(matches!(err, AppError::NotFound));
    }
}
