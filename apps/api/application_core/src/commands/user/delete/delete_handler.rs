use std::sync::Arc;

use crate::{
    commands::user::supabase_admin_client::SupabaseAdminClient,
    common::app_error::AppError,
};
use tracing::{info, instrument};
use uuid::Uuid;

pub trait DeleteUserHandlerTrait {
    fn handle_delete_user(
        &self,
        id: Uuid,
        actor_user_id: &str,
    ) -> impl std::future::Future<Output = Result<(), AppError>> + Send;
}

#[derive(Debug)]
pub struct DeleteUserHandler {
    pub supabase: Arc<SupabaseAdminClient>,
}

impl DeleteUserHandlerTrait for DeleteUserHandler {
    #[instrument(skip(self))]
    async fn handle_delete_user(
        &self,
        id: Uuid,
        actor_user_id: &str,
    ) -> Result<(), AppError> {
        if id.to_string() == actor_user_id {
            return Err(AppError::Logical(
                "Cannot delete your own account".to_string(),
            ));
        }

        self.supabase.delete_user(id).await?;

        info!(
            action = "delete",
            actor_user_id = actor_user_id,
            target_user_id = %id,
            "admin user action"
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::user::supabase_admin_client::SupabaseAdminClient;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    async fn setup_handler_with_mock() -> (DeleteUserHandler, MockServer) {
        let server = MockServer::start().await;
        let client = Arc::new(SupabaseAdminClient::new(
            server.uri(),
            "service-role-test-key",
        ));
        (DeleteUserHandler { supabase: client }, server)
    }

    #[async_std::test]
    async fn handle_delete_user_blocks_self_delete() {
        let (handler, _server) = setup_handler_with_mock().await;
        let id = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();
        let actor = id.to_string();
        let err = handler
            .handle_delete_user(id, &actor)
            .await
            .expect_err("self-delete blocked");
        assert!(matches!(err, AppError::Logical(_)));
    }

    #[async_std::test]
    async fn handle_delete_user_returns_ok_on_success() {
        let (handler, server) = setup_handler_with_mock().await;
        let target = Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap();
        let actor = Uuid::parse_str("99999999-9999-9999-9999-999999999999").unwrap();

        Mock::given(method("DELETE"))
            .and(path(
                "/auth/v1/admin/users/33333333-3333-3333-3333-333333333333",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&server)
            .await;

        let result = handler
            .handle_delete_user(target, &actor.to_string())
            .await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }
}
