use std::sync::Arc;

use crate::{
    commands::user::{dto::AppUserModel, supabase_admin_client::SupabaseAdminClient},
    common::app_error::AppError,
};
use tracing::instrument;
use uuid::Uuid;

pub trait ReadOneUserHandlerTrait {
    fn handle_get_user(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<AppUserModel, AppError>> + Send;
}

#[derive(Debug)]
pub struct ReadOneUserHandler {
    pub supabase: Arc<SupabaseAdminClient>,
}

impl ReadOneUserHandlerTrait for ReadOneUserHandler {
    #[instrument(skip(self))]
    async fn handle_get_user(&self, id: Uuid) -> Result<AppUserModel, AppError> {
        self.supabase.get_user(id).await
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

    async fn setup_handler_with_mock() -> (ReadOneUserHandler, MockServer) {
        let server = MockServer::start().await;
        let client = Arc::new(SupabaseAdminClient::new(
            server.uri(),
            "service-role-test-key",
        ));
        (ReadOneUserHandler { supabase: client }, server)
    }

    #[async_std::test]
    async fn handle_get_user_returns_user_on_200() {
        let (handler, server) = setup_handler_with_mock().await;

        Mock::given(method("GET"))
            .and(path(
                "/auth/v1/admin/users/33333333-3333-3333-3333-333333333333",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "id": "33333333-3333-3333-3333-333333333333",
                "email": "alice@example.com",
                "app_metadata": { "roles": ["my-headless-cms-administrator"] },
                "created_at": "2026-01-01T00:00:00Z",
                "updated_at": "2026-01-01T00:00:00Z",
                "last_sign_in_at": null,
                "banned_until": null
            })))
            .mount(&server)
            .await;

        let id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();
        let user = handler.handle_get_user(id).await.expect("get ok");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.role.as_deref(), Some("my-headless-cms-administrator"));
    }

    #[async_std::test]
    async fn handle_get_user_returns_not_found_on_404() {
        let (handler, server) = setup_handler_with_mock().await;

        Mock::given(method("GET"))
            .and(path(
                "/auth/v1/admin/users/33333333-3333-3333-3333-333333333333",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&server)
            .await;

        let id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();
        let err = handler.handle_get_user(id).await.expect_err("should fail");
        assert!(matches!(err, AppError::NotFound));
    }
}
