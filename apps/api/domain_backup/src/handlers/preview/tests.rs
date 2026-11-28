//! WU-09 tests for IF-09 and IF-10: TS-43, TS-53, TS-64, plus the preview halves of the
//! precedence matrix (TS-09) and the body rules (TS-60, TS-62, TS-63) for the two routes.
//!
//! Docker-free: the runtime is Ready over in-memory stores, and the API state's store is
//! `Disconnected`, so any database or file-system touch by a preview would fail loudly.

use axum::body::Body;
use axum::http::{header, StatusCode};
use serde_json::json;

use crate::domain::metrics::{MetricErrorCode, MetricJobKind, MetricLabels, MetricOutcome};
use crate::handlers::disabled_capability::test_support::*;

const PREFLIGHT: &str = "/backup/previews/preflight";
const CONFIRM: &str = "/backup/previews/restore-confirmation";

fn fixture(id: &str, label: &str, result: &str) -> (String, String, String) {
    (id.to_owned(), label.to_owned(), result.to_owned())
}

fn trusted() -> [(String, String, String); 3] {
    [
        fixture("fixture-valid", "Synthetic valid fixture", "compatible"),
        fixture(
            "fixture-incompatible",
            "Synthetic incompatible fixture",
            "incompatible",
        ),
        fixture("fixture-corrupt", "Synthetic corrupt fixture", "corrupt"),
    ]
}

/// TS-53 and TS-64 (IF-09): a trusted fixture id gives its canned result plus the IFC 2.9
/// identification fields, read from the configured instance, labelled synthetic.
#[tokio::test]
async fn ts53_preflight_preview_returns_the_canned_result_for_each_trusted_fixture() {
    let ready = ready_offline("pv-preflight", SpyAudit::recording()).await;
    for (id, label, result) in trusted() {
        let body = json!({ "fixture_id": id }).to_string();
        let response = send(&ready.app, post_json(PREFLIGHT, &body)).await;
        let (status, json) = json_of(response).await;
        assert_eq!(status, StatusCode::OK, "{id}: {json}");
        assert_eq!(
            json,
            json!({
                "synthetic": true,
                "result": result,
                "fixture_label": label,
                "target_instance_id": INSTANCE_ID,
                "target_instance_label": INSTANCE_LABEL,
                "target_environment": "local",
                "data_scope": ["database (synthetic rows only)"],
                "impact_summary": "Synthetic preview only. No file was uploaded or read.",
            }),
            "{id}"
        );
        assert!(
            json.get("fixture_id").is_none(),
            "{id}: fixture_id is request-only"
        );
        assert!(json.get("scope_summary").is_none(), "{id}");
    }
}

/// TS-64 and AC-004 S2 (IF-10): confirm and cancel return the same labelled shape with
/// `executed: false` and the exact message text; nothing is described as performed.
#[tokio::test]
async fn ts64_restore_confirmation_preview_carries_the_identification_fields() {
    let ready = ready_offline("pv-confirm-shape", SpyAudit::recording()).await;
    for decision in ["confirm", "cancel"] {
        let body = json!({ "fixture_id": "fixture-valid", "decision": decision }).to_string();
        let (status, json) = json_of(send(&ready.app, post_json(CONFIRM, &body)).await).await;
        assert_eq!(status, StatusCode::OK, "{decision}: {json}");
        assert_eq!(
            json,
            json!({
                "synthetic": true,
                "executed": false,
                "decision": decision,
                "fixture_label": "Synthetic valid fixture",
                "target_instance_id": INSTANCE_ID,
                "target_instance_label": INSTANCE_LABEL,
                "target_environment": "local",
                "data_scope": ["database (synthetic rows only)"],
                "message": "Synthetic preview only. Nothing was recorded and no restore was performed.",
            }),
            "{decision}"
        );
        assert!(json.get("fixture_id").is_none());
        assert!(json.get("scope_summary").is_none());
    }
}

/// TS-64: the target fields come from the configured instance; a request that tries to
/// supply them is rejected (400) and the value never appears in any response.
#[tokio::test]
async fn ts64_request_supplied_target_values_are_rejected_not_echoed() {
    let ready = ready_offline("pv-target", SpyAudit::recording()).await;
    for (uri, base) in [
        (PREFLIGHT, json!({ "fixture_id": "fixture-valid" })),
        (
            CONFIRM,
            json!({ "fixture_id": "fixture-valid", "decision": "confirm" }),
        ),
    ] {
        for field in [
            "target_instance_id",
            "target_instance_label",
            "target_environment",
            "data_scope",
            "fixture_label",
        ] {
            let mut body = base.clone();
            body[field] = json!("production-CANARY");
            let response = send(&ready.app, post_json(uri, &body.to_string())).await;
            let result = json_of(response).await;
            assert!(!result.1.to_string().contains("CANARY"), "{uri} {field}");
            assert_error(
                &result,
                StatusCode::BAD_REQUEST,
                "ValidationError",
                "MALFORMED_REQUEST",
                &format!("{uri} {field}"),
            );
        }
    }
}

/// TS-43 (F-14, AC-004 S2): confirm and cancel enqueue nothing and call no mutation port: no
/// store or allocation call, no audit record, no file, no accepted metric.
#[tokio::test]
async fn ts43_confirm_and_cancel_previews_enqueue_nothing() {
    let ready = ready_offline("pv-nothing", SpyAudit::recording()).await;
    let calls_before = ready.calls();
    for decision in ["confirm", "cancel", "confirm", "cancel"] {
        let body = json!({ "fixture_id": "fixture-valid", "decision": decision }).to_string();
        let (status, json) = json_of(send(&ready.app, post_json(CONFIRM, &body)).await).await;
        assert_eq!(status, StatusCode::OK, "{json}");
        assert_eq!(json["executed"], false);
        assert_eq!(
            json["message"],
            "Synthetic preview only. Nothing was recorded and no restore was performed."
        );
    }
    let body = json!({ "fixture_id": "fixture-corrupt" }).to_string();
    assert_eq!(
        send(&ready.app, post_json(PREFLIGHT, &body)).await.status(),
        StatusCode::OK
    );
    assert_eq!(
        ready.calls(),
        calls_before,
        "no store, allocation or storage call"
    );
    assert!(ready.store.calls().is_empty(), "no job transition");
    assert!(
        ready.api_audit.events().is_empty(),
        "previews record nothing"
    );
    assert_eq!(
        std::fs::read_dir(&ready.root).unwrap().count(),
        0,
        "no file is created"
    );
    let accepted = MetricLabels {
        job_kind: MetricJobKind::BackupDbSlice,
        outcome: MetricOutcome::Accepted,
        error_code: MetricErrorCode::None,
    };
    assert_eq!(ready.state.metrics.count(accepted), 0);
}

/// TS-53: unknown ids, paths, uploads and extra fields are never accepted, and a rejected
/// preview touches nothing.
#[tokio::test]
async fn ts53_unknown_fixture_ids_paths_and_uploads_are_rejected() {
    let ready = ready_offline("pv-reject", SpyAudit::recording()).await;
    let calls_before = ready.calls();
    let bad_ids = [
        "unknown",
        "",
        "FIXTURE-VALID",
        " fixture-valid",
        "fixture-valid ",
        "fixture-valid/",
        "../fixture-valid",
        "../../etc/passwd",
        "/tmp/backup.tar.gz",
        "C:\\backup.zip",
        "backup.tar.gz",
        "fixture-valid\u{0}",
    ];
    for id in bad_ids {
        for (uri, extra) in [
            (PREFLIGHT, json!({})),
            (CONFIRM, json!({"decision": "confirm"})),
        ] {
            let mut body = extra.clone();
            body["fixture_id"] = json!(id);
            let result = json_of(send(&ready.app, post_json(uri, &body.to_string())).await).await;
            assert_error(
                &result,
                StatusCode::BAD_REQUEST,
                "ValidationError",
                "MALFORMED_REQUEST",
                &format!("{uri} {id:?}"),
            );
        }
    }
    for body in [
        json!({"fixture_id": "fixture-valid", "path": "/tmp/x.tar"}),
        json!({"fixture_id": "fixture-valid", "file": "AAAA"}),
        json!({"fixture_id": "fixture-valid", "package": {"name": "x"}}),
        json!({"fixture_id": "fixture-valid", "adapter": "real"}),
        json!({"fixture_id": "fixture-valid", "real": true}),
        json!({"fixture_id": ["fixture-valid"]}),
        json!({"fixture_id": 7}),
        json!({}),
    ] {
        let result = json_of(send(&ready.app, post_json(PREFLIGHT, &body.to_string())).await).await;
        assert_error(
            &result,
            StatusCode::BAD_REQUEST,
            "ValidationError",
            "MALFORMED_REQUEST",
            &body.to_string(),
        );
    }
    for body in [
        json!({"fixture_id": "fixture-valid"}),
        json!({"fixture_id": "fixture-valid", "decision": "restore"}),
        json!({"fixture_id": "fixture-valid", "decision": "Confirm"}),
        json!({"fixture_id": "fixture-valid", "decision": ""}),
        json!({"fixture_id": "fixture-valid", "decision": true}),
        json!({"decision": "confirm"}),
        json!({"fixture_id": "fixture-valid", "decision": "confirm", "execute": true}),
    ] {
        let result = json_of(send(&ready.app, post_json(CONFIRM, &body.to_string())).await).await;
        assert_error(
            &result,
            StatusCode::BAD_REQUEST,
            "ValidationError",
            "MALFORMED_REQUEST",
            &body.to_string(),
        );
    }
    for raw in ["", "not json", "{\"fixture_id\":", "[]", "null"] {
        let result = json_of(send(&ready.app, post_json(PREFLIGHT, raw)).await).await;
        assert_error(
            &result,
            StatusCode::BAD_REQUEST,
            "ValidationError",
            "MALFORMED_REQUEST",
            raw,
        );
    }
    assert_eq!(ready.calls(), calls_before);
    assert_eq!(std::fs::read_dir(&ready.root).unwrap().count(), 0);
}

/// TS-60, TS-62 for the preview routes: 415 for a non-JSON type, 413 over 16 KiB by
/// `Content-Length` (body never polled) and by streamed length.
#[tokio::test]
async fn ts60_preview_routes_enforce_media_type_and_the_16_kib_body_cap() {
    let ready = ready_offline("pv-limits", SpyAudit::recording()).await;
    for uri in [PREFLIGHT, CONFIRM] {
        for content_type in [
            Some("multipart/form-data; boundary=x"),
            Some("text/plain"),
            Some("application/octet-stream"),
            None,
        ] {
            let spy = BodySpy::default();
            let mut request = builder("POST", uri);
            if let Some(content_type) = content_type {
                request = request.header(header::CONTENT_TYPE, content_type);
            }
            let result = json_of(send(&ready.app, request.body(spy.body()).unwrap()).await).await;
            assert_error(
                &result,
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "UnsupportedMediaType",
                "UNSUPPORTED_MEDIA_TYPE",
                &format!("{uri} {content_type:?}"),
            );
            assert!(!spy.polled(), "{uri}: body polled for a wrong media type");
        }

        let spy = BodySpy::default();
        let request = builder("POST", uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::CONTENT_LENGTH, (16 * 1024 + 1).to_string())
            .body(spy.body())
            .unwrap();
        let result = json_of(send(&ready.app, request).await).await;
        assert_error(
            &result,
            StatusCode::PAYLOAD_TOO_LARGE,
            "PayloadTooLarge",
            "PAYLOAD_TOO_LARGE",
            &format!("{uri} content-length"),
        );
        assert!(
            !spy.polled(),
            "{uri}: body polled despite a declared oversize"
        );

        let streamed = builder("POST", uri)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(vec![b' '; 16 * 1024 + 1]))
            .unwrap();
        let result = json_of(send(&ready.app, streamed).await).await;
        assert_error(
            &result,
            StatusCode::PAYLOAD_TOO_LARGE,
            "PayloadTooLarge",
            "PAYLOAD_TOO_LARGE",
            &format!("{uri} streamed"),
        );
    }
}

/// IFC 2.1 precedence for IF-09 and IF-10: role, Host and Origin first, then 503
/// `BACKUP_DISABLED` / `BACKUP_NOT_READY` before any body validation, then validation.
#[tokio::test]
async fn ts09_preview_precedence_matrix() {
    let garbage = || {
        builder("POST", PREFLIGHT)
            .header(header::CONTENT_TYPE, "text/plain")
            .header(header::CONTENT_LENGTH, "1073741824")
            .body(Body::empty())
            .unwrap()
    };
    for (name, config, reason) in [
        ("disabled", disabled_config(), "BACKUP_DISABLED"),
        (
            "not ready",
            enabled_config(&unique_root("pv-prec")),
            "BACKUP_NOT_READY",
        ),
    ] {
        let app = offline_app(config, SpyAudit::recording());
        for uri in [PREFLIGHT, CONFIRM] {
            let request = builder("POST", uri)
                .header(header::CONTENT_TYPE, "text/plain")
                .header(header::CONTENT_LENGTH, "1073741824")
                .body(Body::empty())
                .unwrap();
            let result = json_of(send(&app, request).await).await;
            assert_error(
                &result,
                StatusCode::SERVICE_UNAVAILABLE,
                "Unavailable",
                reason,
                &format!("{name} {uri}"),
            );
        }
        let result = json_of(send(&app, garbage()).await).await;
        assert_eq!(result.1["reason_code"], reason, "{name}");
    }

    let ready = ready_offline("pv-prec-ready", SpyAudit::recording()).await;
    for uri in [PREFLIGHT, CONFIRM] {
        let anonymous = axum::http::Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::HOST, "localhost:8989")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let writer_request = axum::http::Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::HOST, "localhost:8989")
            .header(header::CONTENT_TYPE, "application/json")
            .extension(writer())
            .body(Body::from("{}"))
            .unwrap();
        let bad_host = builder_with_host("POST", uri, "evil.example:8989")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{}"))
            .unwrap();
        let bad_origin = builder("POST", uri)
            .header("Origin", "http://evil.example")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{}"))
            .unwrap();
        for (request, reason) in [
            (anonymous, "ROLE_REQUIRED"),
            (writer_request, "ROLE_REQUIRED"),
            (bad_host, "HOST_NOT_ALLOWED"),
            (bad_origin, "ORIGIN_NOT_ALLOWED"),
        ] {
            let result = json_of(send(&ready.app, request).await).await;
            assert_error(
                &result,
                StatusCode::FORBIDDEN,
                "Forbidden",
                reason,
                &format!("{uri} expecting {reason}"),
            );
        }
        let result = json_of(send(&ready.app, post_json(uri, "{}")).await).await;
        assert_eq!(
            result.0,
            StatusCode::BAD_REQUEST,
            "{uri}: validation is last"
        );
    }
}
