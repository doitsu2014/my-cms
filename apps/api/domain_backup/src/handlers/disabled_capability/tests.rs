//! WU-09 tests for IF-06..IF-08 (TS-09) and the adapter-selection guard (TS-10).
//!
//! These run without Docker: the router is the real `routes()` over a `Disconnected`
//! connection, so any store call would fail loudly. The ready-runtime variants (with real
//! tables and files) are in `infra::tests::capabilities_api`.

use axum::body::Body;
use axum::http::{header, StatusCode};

use super::test_support::*;
use crate::ports::audit::{AuditEventKind, AuditOutcome};

const CAPABILITY_ROUTES: [(&str, &str); 3] = [
    ("/backup/packages", "upload"),
    ("/backup/preflight", "preflight"),
    ("/backup/restore", "restore"),
];

/// IFC 2.8 and FD F-12: 403 `CAPABILITY_DISABLED` regardless of `BACKUP_ENABLED` and readiness.
#[tokio::test]
async fn ts09_disabled_capabilities_return_403_whatever_the_backup_state() {
    for (state_name, config) in [
        ("backup disabled", disabled_config()),
        (
            "enabled but not ready",
            enabled_config(&unique_root("cap-state")),
        ),
    ] {
        let app = offline_app(config, SpyAudit::recording());
        for (uri, _) in CAPABILITY_ROUTES {
            let response = send(&app, post_json(uri, r#"{"anything":"goes"}"#)).await;
            assert!(
                response.headers().get(header::LOCATION).is_none(),
                "{state_name} {uri}: a denial never redirects"
            );
            let result = json_of(response).await;
            assert_error(
                &result,
                StatusCode::FORBIDDEN,
                "Forbidden",
                "CAPABILITY_DISABLED",
                &format!("{state_name} {uri}"),
            );
            let message = result.1["errors"][0].as_str().unwrap();
            assert!(!message.is_empty() && message.len() < 120, "{message}");
        }
    }
}

/// F-12: the body is never read, even when `Content-Length` declares a huge or tiny body.
#[tokio::test]
async fn ts09_disabled_capabilities_never_poll_the_request_body() {
    for config_name in ["disabled", "enabled"] {
        let config = if config_name == "disabled" {
            disabled_config()
        } else {
            enabled_config(&unique_root("cap-body"))
        };
        let app = offline_app(config, SpyAudit::recording());
        for (uri, _) in CAPABILITY_ROUTES {
            for declared in ["1073741824", "0", "5"] {
                let spy = BodySpy::default();
                let request = builder("POST", uri)
                    .header(header::CONTENT_TYPE, "multipart/form-data; boundary=x")
                    .header(header::CONTENT_LENGTH, declared)
                    .body(spy.body())
                    .unwrap();
                let result = json_of(send(&app, request).await).await;
                assert_error(
                    &result,
                    StatusCode::FORBIDDEN,
                    "Forbidden",
                    "CAPABILITY_DISABLED",
                    &format!("{config_name} {uri} content-length {declared}"),
                );
                assert!(
                    !spy.polled(),
                    "{config_name} {uri} content-length {declared}: body stream was polled"
                );
            }
        }
    }
}

/// IFC 2.1 precedence: 403 role, then Host, then Origin come before `CAPABILITY_DISABLED`,
/// and a refused caller leaves no audit attempt.
#[tokio::test]
async fn ts09_role_host_and_origin_checks_precede_capability_disabled() {
    let audit = SpyAudit::recording();
    let app = offline_app(enabled_config(&unique_root("cap-prec")), audit.clone());
    for (uri, _) in CAPABILITY_ROUTES {
        let anonymous = axum::http::Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::HOST, "localhost:8989")
            .body(Body::empty())
            .unwrap();
        let writer_request = axum::http::Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::HOST, "localhost:8989")
            .extension(writer())
            .body(Body::empty())
            .unwrap();
        let bad_host = builder_with_host("POST", uri, "evil.example:8989")
            .body(Body::empty())
            .unwrap();
        let bad_origin = builder("POST", uri)
            .header("Origin", "http://evil.example")
            .body(Body::empty())
            .unwrap();
        for (request, reason) in [
            (anonymous, "ROLE_REQUIRED"),
            (writer_request, "ROLE_REQUIRED"),
            (bad_host, "HOST_NOT_ALLOWED"),
            (bad_origin, "ORIGIN_NOT_ALLOWED"),
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
    assert!(
        audit.events().is_empty(),
        "a refused caller must not reach the capability denial: {:?}",
        audit.events()
    );
}

/// F-12 and IFC 6.3: `capability.denied` is attempted with the closed field set, and an
/// audit failure still returns the denial.
#[tokio::test]
async fn ts09_capability_denial_attempts_audit_and_audit_failure_still_denies() {
    let root = unique_root("cap-audit");
    let audit = SpyAudit::recording();
    let app = offline_app(enabled_config(&root), audit.clone());
    let mut correlations = Vec::new();
    for (uri, _) in CAPABILITY_ROUTES {
        let result = json_of(send(&app, post_json(uri, "{}")).await).await;
        correlations.push(assert_error(
            &result,
            StatusCode::FORBIDDEN,
            "Forbidden",
            "CAPABILITY_DISABLED",
            uri,
        ));
    }
    let events = audit.events();
    assert_eq!(events.len(), 3, "{events:?}");
    for ((event, (_, operation)), correlation) in
        events.iter().zip(CAPABILITY_ROUTES).zip(&correlations)
    {
        assert_eq!(event.event, AuditEventKind::CapabilityDenied);
        assert_eq!(event.outcome, AuditOutcome::Denied);
        assert_eq!(event.operation, operation);
        assert_eq!(event.reason_code.as_deref(), Some("CAPABILITY_DISABLED"));
        assert_eq!(event.actor_sub.as_deref(), Some("test-admin"));
        assert_eq!(event.instance_id, INSTANCE_ID);
        assert_eq!(event.job_id, None);
        assert_eq!(event.allocation_id, None);
        assert!(event.digest.is_none());
        assert_eq!(&event.correlation_id, correlation);
    }

    let failing = SpyAudit::failing();
    let app = offline_app(enabled_config(&root), failing.clone());
    for (uri, _) in CAPABILITY_ROUTES {
        let result = json_of(send(&app, post_json(uri, "{}")).await).await;
        assert_error(
            &result,
            StatusCode::FORBIDDEN,
            "Forbidden",
            "CAPABILITY_DISABLED",
            uri,
        );
    }
    assert_eq!(failing.events().len(), 3, "the audit write is attempted");
}

/// F-12 postconditions: no state change anywhere (no storage root, no store call) and a
/// closed metric label for the denial.
#[tokio::test]
async fn ts09_denied_capabilities_create_no_files_and_record_one_denied_metric_each() {
    use crate::domain::metrics::{MetricErrorCode, MetricJobKind, MetricLabels, MetricOutcome};
    let root = unique_root("cap-files");
    let clock =
        crate::lifecycle::test_support::FakeClock::new(crate::lifecycle::test_support::t0());
    let log = crate::lifecycle::test_support::new_log();
    let store = crate::lifecycle::test_support::MemJobStore::new(clock, log.clone());
    let state = crate::handlers::http::BackupApiState::new(
        sea_orm::DatabaseConnection::Disconnected,
        enabled_config(&root),
    )
    .with_persistence(store.clone(), SpyAudit::recording());
    let metrics = state.metrics.clone();
    let app = app_from_state(state);
    for (uri, _) in CAPABILITY_ROUTES {
        let result = json_of(send(&app, post_json(uri, "{}")).await).await;
        assert_error(
            &result,
            StatusCode::FORBIDDEN,
            "Forbidden",
            "CAPABILITY_DISABLED",
            uri,
        );
    }
    assert!(!root.exists(), "no storage root may be created");
    assert!(store.calls().is_empty());
    assert!(
        crate::lifecycle::test_support::log_lines(&log).is_empty(),
        "no store call at all"
    );
    let denied = MetricLabels {
        job_kind: MetricJobKind::BackupDbSlice,
        outcome: MetricOutcome::Denied,
        error_code: MetricErrorCode::Forbidden,
    };
    assert_eq!(metrics.count(denied), 3);
}

fn rust_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

/// TS-10 (composition): production code has exactly one `CaptureAdapter`
/// (`FixtureCaptureAdapter`), one `FixtureStorage` (`FsFixtureStorage`), and nothing reads
/// an adapter-selecting environment value. Test doubles live in test-only files.
#[test]
fn ts10_runtime_composition_holds_only_the_fixture_adapter() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    let mut capture_impls = Vec::new();
    let mut storage_impls = Vec::new();
    for path in files {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let in_test_tree = path.components().any(|part| part.as_os_str() == "tests");
        if name == "tests.rs" || name == "test_support.rs" || in_test_tree {
            continue;
        }
        let text = std::fs::read_to_string(&path).unwrap();
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with("impl CaptureAdapter for ") {
                capture_impls.push(line.to_owned());
            }
            if line.starts_with("impl FixtureStorage for ") {
                storage_impls.push(line.to_owned());
            }
            assert!(
                !(line.contains("env::var") && line.to_ascii_uppercase().contains("ADAPTER")),
                "{name}: no environment value may select an adapter: {line}"
            );
        }
    }
    assert_eq!(
        capture_impls,
        vec!["impl CaptureAdapter for FixtureCaptureAdapter {"]
    );
    assert_eq!(
        storage_impls,
        vec!["impl FixtureStorage for FsFixtureStorage {"]
    );
}
