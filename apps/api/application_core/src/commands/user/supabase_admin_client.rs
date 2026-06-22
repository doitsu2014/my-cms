use crate::{
    common::app_error::AppError,
    commands::user::{
        create::create_request::CreateUserRequest,
        dto::AppUserModel,
        modify::modify_request::ModifyUserRequest,
    },
};
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageObject {
    pub name: String,
    pub content_type: String,
    pub size: u64,
    pub last_modified: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct SupabaseAdminClient {
    pub supabase_url: String,
    pub service_role_key: String,
    pub client: Client,
}

impl std::fmt::Debug for SupabaseAdminClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SupabaseAdminClient")
            .field("supabase_url", &self.supabase_url)
            .field("service_role_key", &"<redacted>")
            .finish()
    }
}

impl SupabaseAdminClient {
    pub fn new(supabase_url: impl Into<String>, service_role_key: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("reqwest::Client should build with default config");
        Self {
            supabase_url: supabase_url.into(),
            service_role_key: service_role_key.into(),
            client,
        }
    }

    pub fn auth_key(&self) -> &str {
        &self.service_role_key
    }

    pub fn admin_base_url(&self) -> String {
        format!("{}/auth/v1/admin", self.supabase_url)
    }

    pub async fn list_users(&self, page: u32, per_page: u32) -> Result<Vec<AppUserModel>, AppError> {
        let url = format!("{}/users", self.admin_base_url());
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .query(&[("page", page), ("per_page", per_page)])
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("GoTrue list users request failed: {}", e)))?;

        let status = response.status();
        if !status.is_success() {
            return Err(map_gotrue_error(status, response, "GoTrue list users").await);
        }

        let raw: ListUsersResponse = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse GoTrue list users response: {}", e))
        })?;

        Ok(raw
            .users
            .into_iter()
            .filter_map(|u| parse_gotrue_user(u))
            .collect())
    }

    pub async fn get_user(&self, id: Uuid) -> Result<AppUserModel, AppError> {
        let url = format!("{}/users/{}", self.admin_base_url(), id);
        let response = self
            .client
            .get(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("GoTrue get user request failed: {}", e)))?;

        let status = response.status();
        if status == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound);
        }
        if !status.is_success() {
            return Err(map_gotrue_error(status, response, "GoTrue get user").await);
        }

        let raw: GoTrueUserResponse = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse GoTrue get user response: {}", e))
        })?;

        parse_gotrue_user(raw).ok_or(AppError::NotFound)
    }

    pub async fn create_user(&self, req: &CreateUserRequest) -> Result<AppUserModel, AppError> {
        let url = format!("{}/users", self.admin_base_url());
        let body = serde_json::json!({
            "email": req.email,
            "password": req.password,
            "email_confirm": true,
            "app_metadata": { "roles": [req.role] },
        });
        let response = self
            .client
            .post(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(body.to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("GoTrue create user request failed: {}", e)))?;

        let status = response.status();
        if status == StatusCode::CONFLICT {
            return Err(map_gotrue_error(status, response, "GoTrue create user").await);
        }
        if !status.is_success() {
            return Err(map_gotrue_error(status, response, "GoTrue create user").await);
        }

        let raw: GoTrueUserResponse = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse GoTrue create user response: {}", e))
        })?;

        parse_gotrue_user(raw).ok_or(AppError::NotFound)
    }

    pub async fn update_user(
        &self,
        id: Uuid,
        req: &ModifyUserRequest,
    ) -> Result<AppUserModel, AppError> {
        let url = format!("{}/users/{}", self.admin_base_url(), id);
        let mut body: serde_json::Map<String, Value> = serde_json::Map::new();
        if let Some(email) = &req.email {
            body.insert("email".to_string(), Value::String(email.clone()));
        }
        if let Some(role) = &req.role {
            body.insert(
                "app_metadata".to_string(),
                serde_json::json!({ "roles": [role] }),
            );
        }
        match req.banned {
            Some(true) => {
                body.insert(
                    "ban_duration".to_string(),
                    Value::String(crate::commands::user::dto::BAN_DURATION.to_string()),
                );
            }
            Some(false) => {
                body.insert("ban_duration".to_string(), Value::String("none".to_string()));
            }
            None => {}
        }

        let response = self
            .client
            .put(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(Value::Object(body).to_string())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("GoTrue update user request failed: {}", e)))?;

        let status = response.status();
        if status == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound);
        }
        if !status.is_success() {
            return Err(map_gotrue_error(status, response, "GoTrue update user").await);
        }

        let raw: GoTrueUserResponse = response.json().await.map_err(|e| {
            AppError::StorageError(format!("Failed to parse GoTrue update user response: {}", e))
        })?;

        parse_gotrue_user(raw).ok_or(AppError::NotFound)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), AppError> {
        let url = format!("{}/users/{}", self.admin_base_url(), id);
        let response = self
            .client
            .delete(&url)
            .bearer_auth(self.auth_key())
            .header("apikey", self.auth_key())
            .send()
            .await
            .map_err(|e| AppError::StorageError(format!("GoTrue delete user request failed: {}", e)))?;

        let status = response.status();
        if status == StatusCode::NOT_FOUND {
            return Err(AppError::NotFound);
        }
        if !status.is_success() {
            return Err(map_gotrue_error(status, response, "GoTrue delete user").await);
        }

        Ok(())
    }
}

async fn map_gotrue_error(
    status: StatusCode,
    response: reqwest::Response,
    context: &str,
) -> AppError {
    let body = response
        .text()
        .await
        .unwrap_or_else(|_| "<no body>".to_string());
    let body_summary = sanitise_body(&body);
    match status {
        StatusCode::NOT_FOUND => AppError::NotFound,
        StatusCode::CONFLICT => AppError::Conflict(format!(
            "{} conflict ({}): {}",
            context, status, body_summary
        )),
        StatusCode::UNPROCESSABLE_ENTITY | StatusCode::BAD_REQUEST => AppError::Validation(
            "gotrue".to_string(),
            format!("{} validation error ({}): {}", context, status, body_summary),
        ),
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => AppError::Logical(format!(
            "{} authorisation error ({}): {}",
            context, status, body_summary
        )),
        _ => AppError::StorageError(format!(
            "{} failed ({}): {}",
            context, status, body_summary
        )),
    }
}

fn sanitise_body(body: &str) -> String {
    const MAX: usize = 256;
    if body.len() > MAX {
        format!("{}...", &body[..MAX])
    } else {
        body.to_string()
    }
}

#[derive(Debug, Deserialize)]
struct ListUsersResponse {
    users: Vec<GoTrueUserResponse>,
}

#[derive(Debug, Deserialize)]
struct GoTrueUserResponse {
    id: Uuid,
    email: Option<String>,
    #[serde(default)]
    app_metadata: Value,
    #[serde(default)]
    banned_until: Option<String>,
    created_at: String,
    updated_at: String,
    #[serde(default)]
    last_sign_in_at: Option<String>,
}

fn parse_gotrue_user(raw: GoTrueUserResponse) -> Option<AppUserModel> {
    let email = raw.email?;
    let role = raw
        .app_metadata
        .get("roles")
        .and_then(|r| r.as_array())
        .and_then(|arr| arr.first())
        .and_then(|v| v.as_str())
        .filter(|r| crate::commands::user::dto::is_recognised_role(r))
        .map(|s| s.to_string());

    let banned = raw
        .banned_until
        .as_deref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc) > Utc::now())
        .unwrap_or(false);

    let created_at = DateTime::parse_from_rfc3339(&raw.created_at)
        .ok()?
        .with_timezone(&Utc);
    let updated_at = DateTime::parse_from_rfc3339(&raw.updated_at)
        .ok()?
        .with_timezone(&Utc);
    let last_sign_in_at = raw
        .last_sign_in_at
        .as_deref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    Some(AppUserModel {
        id: raw.id,
        email,
        role,
        banned,
        created_at,
        updated_at,
        last_sign_in_at,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        matchers::{header, method, path, query_param},
        Mock, MockServer, ResponseTemplate,
    };

    fn make_client(base_url: &str, service_role_key: &str) -> SupabaseAdminClient {
        SupabaseAdminClient::new(base_url.to_string(), service_role_key.to_string())
    }

    #[test]
    fn auth_key_returns_service_role_key() {
        let c = make_client("http://example.com", "service-role-test-key");
        assert_eq!(c.auth_key(), "service-role-test-key");
    }

    #[test]
    fn admin_base_url_builds_expected_path() {
        let c = make_client("http://localhost:8000", "k");
        assert_eq!(
            c.admin_base_url(),
            "http://localhost:8000/auth/v1/admin"
        );
    }

    #[test]
    fn debug_impl_redacts_service_role_key() {
        let c = make_client("http://localhost:8000", "super-secret-key");
        let debug_str = format!("{:?}", c);
        assert!(debug_str.contains("<redacted>"));
        assert!(!debug_str.contains("super-secret-key"));
    }

    #[async_std::test]
    async fn list_users_issues_bearer_and_apikey_headers() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri(), "service-role-test-key");

        let body = json!({
            "users": [
                {
                    "id": "11111111-1111-1111-1111-111111111111",
                    "email": "alice@example.com",
                    "app_metadata": { "roles": ["my-headless-cms-administrator"] },
                    "created_at": "2026-01-01T00:00:00Z",
                    "updated_at": "2026-01-01T00:00:00Z",
                    "last_sign_in_at": null,
                    "banned_until": null
                }
            ],
            "aud": "authenticated"
        });

        Mock::given(method("GET"))
            .and(path("/auth/v1/admin/users"))
            .and(query_param("page", "1"))
            .and(query_param("per_page", "50"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(1)
            .mount(&server)
            .await;

        let users = client.list_users(1, 50).await.expect("list ok");
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].email, "alice@example.com");
        assert_eq!(
            users[0].role.as_deref(),
            Some("my-headless-cms-administrator")
        );
        assert!(!users[0].banned);
    }

    #[async_std::test]
    async fn get_user_returns_404_on_not_found() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri(), "service-role-test-key");

        Mock::given(method("GET"))
            .and(path("/auth/v1/admin/users/11111111-1111-1111-1111-111111111111"))
            .respond_with(ResponseTemplate::new(404).set_body_string("not found"))
            .mount(&server)
            .await;

        let result = client
            .get_user(Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap())
            .await;
        assert!(matches!(result, Err(AppError::NotFound)));
    }

    #[async_std::test]
    async fn create_user_posts_email_confirm_and_app_metadata() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri(), "service-role-test-key");

        let body = json!({
            "id": "22222222-2222-2222-2222-222222222222",
            "email": "bob@example.com",
            "app_metadata": { "roles": ["my-headless-cms-writer"] },
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-01T00:00:00Z",
            "last_sign_in_at": null,
            "banned_until": null
        });

        Mock::given(method("POST"))
            .and(path("/auth/v1/admin/users"))
            .and(header("content-type", "application/json"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(201).set_body_json(body))
            .expect(1)
            .mount(&server)
            .await;

        let req = CreateUserRequest {
            email: "bob@example.com".to_string(),
            password: "supersecret".to_string(),
            role: "my-headless-cms-writer".to_string(),
        };
        let user = client.create_user(&req).await.expect("create ok");
        assert_eq!(user.email, "bob@example.com");
        assert_eq!(user.role.as_deref(), Some("my-headless-cms-writer"));
    }

    #[async_std::test]
    async fn update_user_translates_banned_true_to_ban_duration() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri(), "service-role-test-key");

        let body = json!({
            "id": "22222222-2222-2222-2222-222222222222",
            "email": "bob@example.com",
            "app_metadata": { "roles": ["my-headless-cms-writer"] },
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-02T00:00:00Z",
            "last_sign_in_at": null,
            "banned_until": "2099-01-01T00:00:00Z"
        });

        Mock::given(method("PUT"))
            .and(path("/auth/v1/admin/users/22222222-2222-2222-2222-222222222222"))
            .and(header("content-type", "application/json"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .expect(1)
            .mount(&server)
            .await;

        let req = ModifyUserRequest {
            banned: Some(true),
            ..Default::default()
        };
        let user = client
            .update_user(
                Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
                &req,
            )
            .await
            .expect("update ok");
        assert!(user.banned);
    }

    #[async_std::test]
    async fn delete_user_issues_delete_to_admin_path() {
        let server = MockServer::start().await;
        let client = make_client(&server.uri(), "service-role-test-key");

        Mock::given(method("DELETE"))
            .and(path("/auth/v1/admin/users/22222222-2222-2222-2222-222222222222"))
            .and(header("authorization", "Bearer service-role-test-key"))
            .and(header("apikey", "service-role-test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .expect(1)
            .mount(&server)
            .await;

        let result = client
            .delete_user(Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap())
            .await;
        assert!(result.is_ok(), "expected Ok, got {:?}", result);
    }

    #[async_std::test]
    async fn error_messages_never_include_service_role_key() {
        let server = MockServer::start().await;
        let secret = "service-role-leaked-secret-12345";
        let client = make_client(&server.uri(), secret);

        Mock::given(method("GET"))
            .and(path("/auth/v1/admin/users"))
            .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
            .mount(&server)
            .await;

        let err = client.list_users(1, 50).await.expect_err("expected error");
        let msg = format!("{}", err);
        assert!(
            !msg.contains(secret),
            "error message leaked service role key: {}",
            msg
        );

        Mock::given(method("POST"))
            .and(path("/auth/v1/admin/users"))
            .respond_with(ResponseTemplate::new(409).set_body_string("duplicate"))
            .mount(&server)
            .await;

        let req = CreateUserRequest {
            email: "x@example.com".to_string(),
            password: "supersecret".to_string(),
            role: "my-headless-cms-writer".to_string(),
        };
        let err = client.create_user(&req).await.expect_err("expected error");
        let msg = format!("{}", err);
        assert!(
            !msg.contains(secret),
            "conflict message leaked service role key: {}",
            msg
        );
    }
}
