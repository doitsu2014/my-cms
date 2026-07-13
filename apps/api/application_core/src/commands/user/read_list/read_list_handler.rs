use std::sync::Arc;

use crate::{
    commands::user::{
        dto::{is_recognised_role, AppUserModel, RECOGNISED_ROLES},
        supabase_admin_client::SupabaseAdminClient,
    },
    common::app_error::AppError,
};
use tracing::instrument;

pub trait ReadListUserHandlerTrait {
    fn handle_list_users(
        &self,
        page: u32,
        per_page: u32,
        role: Option<String>,
        email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Vec<AppUserModel>, AppError>> + Send;
}

#[derive(Debug)]
pub struct ReadListUserHandler {
    pub supabase: Arc<SupabaseAdminClient>,
}

impl ReadListUserHandlerTrait for ReadListUserHandler {
    #[instrument(skip(self))]
    async fn handle_list_users(
        &self,
        page: u32,
        per_page: u32,
        role: Option<String>,
        email: Option<String>,
    ) -> Result<Vec<AppUserModel>, AppError> {
        if page < 1 {
            return Err(AppError::Validation(
                "page".to_string(),
                "Page must be >= 1".to_string(),
            ));
        }
        if !(1..=200).contains(&per_page) {
            return Err(AppError::Validation(
                "perPage".to_string(),
                "perPage must be between 1 and 200".to_string(),
            ));
        }
        if let Some(ref r) = role {
            if !is_recognised_role(r) {
                return Err(AppError::Validation(
                    "role".to_string(),
                    format!(
                        "Role filter must be one of: {}",
                        RECOGNISED_ROLES.join(", ")
                    ),
                ));
            }
        }

        let users = self.supabase.list_users(page, per_page).await?;

        let email_lower = email.as_ref().map(|e| e.to_lowercase());
        let filtered: Vec<AppUserModel> = users
            .into_iter()
            .filter(|u| {
                if let Some(ref r) = role {
                    if u.role.as_deref() != Some(r.as_str()) {
                        return false;
                    }
                }
                if let Some(ref needle) = email_lower {
                    if !u.email.to_lowercase().contains(needle) {
                        return false;
                    }
                }
                true
            })
            .collect();

        Ok(filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::user::supabase_admin_client::SupabaseAdminClient;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

    async fn setup_handler_with_mock() -> (ReadListUserHandler, MockServer) {
        let server = MockServer::start().await;
        let client = Arc::new(SupabaseAdminClient::new(
            server.uri(),
            "service-role-test-key",
        ));
        (ReadListUserHandler { supabase: client }, server)
    }

    #[async_std::test]
    async fn handle_list_users_rejects_page_zero() {
        let (handler, _server) = setup_handler_with_mock().await;
        let err = handler
            .handle_list_users(0, 50, None, None)
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Validation(_, _)));
    }

    #[async_std::test]
    async fn handle_list_users_rejects_per_page_zero() {
        let (handler, _server) = setup_handler_with_mock().await;
        let err = handler
            .handle_list_users(1, 0, None, None)
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Validation(_, _)));
    }

    #[async_std::test]
    async fn handle_list_users_rejects_per_page_over_200() {
        let (handler, _server) = setup_handler_with_mock().await;
        let err = handler
            .handle_list_users(1, 500, None, None)
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Validation(_, _)));
    }

    #[async_std::test]
    async fn handle_list_users_rejects_unknown_role_filter() {
        let (handler, _server) = setup_handler_with_mock().await;
        let err = handler
            .handle_list_users(1, 50, Some("nope".to_string()), None)
            .await
            .expect_err("should fail");
        assert!(matches!(err, AppError::Validation(_, _)));
    }

    #[async_std::test]
    async fn handle_list_users_returns_users_and_forwards_query_params() {
        let (handler, server) = setup_handler_with_mock().await;

        Mock::given(method("GET"))
            .and(path("/auth/v1/admin/users"))
            .and(query_param("page", "2"))
            .and(query_param("per_page", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "users": [
                    {
                        "id": "33333333-3333-3333-3333-333333333333",
                        "email": "alice@example.com",
                        "app_metadata": { "roles": ["my-headless-cms-writer"] },
                        "created_at": "2026-01-01T00:00:00Z",
                        "updated_at": "2026-01-01T00:00:00Z",
                        "last_sign_in_at": null,
                        "banned_until": null
                    }
                ]
            })))
            .mount(&server)
            .await;

        let users = handler
            .handle_list_users(2, 10, None, None)
            .await
            .expect("list ok");
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].email, "alice@example.com");
    }

    #[async_std::test]
    async fn handle_list_users_filters_by_email_substring_case_insensitively() {
        let (handler, server) = setup_handler_with_mock().await;

        Mock::given(method("GET"))
            .and(path("/auth/v1/admin/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "users": [
                    {
                        "id": "33333333-3333-3333-3333-333333333333",
                        "email": "alice@example.com",
                        "app_metadata": { "roles": ["my-headless-cms-writer"] },
                        "created_at": "2026-01-01T00:00:00Z",
                        "updated_at": "2026-01-01T00:00:00Z",
                        "last_sign_in_at": null,
                        "banned_until": null
                    },
                    {
                        "id": "44444444-4444-4444-4444-444444444444",
                        "email": "bob@example.com",
                        "app_metadata": { "roles": ["my-headless-cms-writer"] },
                        "created_at": "2026-01-02T00:00:00Z",
                        "updated_at": "2026-01-02T00:00:00Z",
                        "last_sign_in_at": null,
                        "banned_until": null
                    }
                ]
            })))
            .mount(&server)
            .await;

        let users = handler
            .handle_list_users(1, 50, None, Some("ALICE".to_string()))
            .await
            .expect("list ok");
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].email, "alice@example.com");
    }
}
