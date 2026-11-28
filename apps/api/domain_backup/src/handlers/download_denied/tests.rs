//! WU-09 tests for IF-05 (TS-09) that need no database. The enabled-and-ready paths, where
//! `DOWNLOAD_DENIED` is reachable, are in `infra::tests::capabilities_api`.

use axum::body::Body;
use axum::http::{header, StatusCode};
use uuid::Uuid;

use crate::handlers::disabled_capability::test_support::*;

/// IFC 2.1 precedence: when backup is disabled, or enabled but not ready, IF-05 answers 503
/// (never `DOWNLOAD_DENIED`), for a random job id and for a malformed one.
#[tokio::test]
async fn ts09_download_is_503_not_denied_while_backup_is_disabled_or_not_ready() {
    let cases = [
        ("disabled", disabled_config(), "BACKUP_DISABLED"),
        (
            "enabled but not ready",
            enabled_config(&unique_root("dl-state")),
            "BACKUP_NOT_READY",
        ),
    ];
    for (name, config, reason) in cases {
        let audit = SpyAudit::recording();
        let app = offline_app(config, audit.clone());
        for id in [Uuid::new_v4().to_string(), "not-a-uuid".to_owned()] {
            let result =
                json_of(send(&app, get(&format!("/backup/jobs/{id}/download"))).await).await;
            assert_error(
                &result,
                StatusCode::SERVICE_UNAVAILABLE,
                "Unavailable",
                reason,
                &format!("{name} {id}"),
            );
        }
        assert!(audit.events().is_empty(), "{name}: no denial is attempted");
    }
}

/// IFC 2.1 precedence: role, Host and Origin checks come first, so a refused caller never
/// learns whether the download path is enabled.
#[tokio::test]
async fn ts09_download_role_and_host_checks_precede_everything() {
    let app = offline_app(disabled_config(), SpyAudit::recording());
    let uri = format!("/backup/jobs/{}/download", Uuid::new_v4());
    let anonymous = axum::http::Request::builder()
        .method("GET")
        .uri(&uri)
        .header(header::HOST, "localhost:8989")
        .body(Body::empty())
        .unwrap();
    let writer_request = axum::http::Request::builder()
        .method("GET")
        .uri(&uri)
        .header(header::HOST, "localhost:8989")
        .extension(writer())
        .body(Body::empty())
        .unwrap();
    let bad_host = builder_with_host("GET", &uri, "evil.example:8989")
        .body(Body::empty())
        .unwrap();
    for (request, reason) in [
        (anonymous, "ROLE_REQUIRED"),
        (writer_request, "ROLE_REQUIRED"),
        (bad_host, "HOST_NOT_ALLOWED"),
    ] {
        let result = json_of(send(&app, request).await).await;
        assert_error(
            &result,
            StatusCode::FORBIDDEN,
            "Forbidden",
            reason,
            &format!("{uri} expecting {reason}"),
        );
    }
}
