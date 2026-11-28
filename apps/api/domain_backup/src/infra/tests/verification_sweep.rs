//! WU-15 verification sweep: TS-39 (job-creation latency, measured with no target), TS-40
//! (concurrency and temporary-byte bounds under load), the API part of TS-41 (admin create,
//! `incomplete`, download denied) and TS-58 (open items listed as open, synthetic-only labels).
//!
//! All Docker tests drive the started `DomainBackupService` through its own composed routes,
//! the same path the gateway mounts under `AdministratorIngress`. They characterize behaviour
//! earlier work units built; nothing here changes application code. Latency numbers are
//! printed (`--nocapture`) and recorded as evidence only: P-01..P-05 have no approved targets.

use super::http_api::{admin, body, config, request, test_domain_context};
use crate::infra::tests::fs_support::TempDir;
use crate::service::DomainBackupService;
use crate::test_db::{backup_test_db, BackupTestDb};
use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use axum::Router;
use domain_interface::DomainService;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use serde_json::Value;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tower::ServiceExt;

fn create_request(key: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/backup/jobs")
        .header(header::HOST, "localhost:8989")
        .header("Origin", "http://localhost:3002")
        .header(header::CONTENT_TYPE, "application/json")
        .header("Idempotency-Key", key)
        .extension(admin())
        .body(Body::from(br#"{"instance_id":"local-1"}"#.to_vec()))
        .unwrap()
}

/// Started service plus its composed routes. The returned `BackupTestDb` owns the Postgres
/// container and must stay alive for the whole test (dropping it removes the container).
async fn started_service(root: &Path) -> (DomainBackupService, Router, BackupTestDb) {
    let db = backup_test_db().await;
    let service = DomainBackupService::new(config(root));
    let ctx = test_domain_context(db.conn.clone());
    service.startup_health(&ctx).await.unwrap();
    let app = service
        .register_routes(&ctx)
        .remove(0)
        .router
        .with_state(ctx.clone());
    (service, app, db)
}

async fn scalar_i64(conn: &DatabaseConnection, sql: &str) -> i64 {
    conn.query_one(Statement::from_string(DbBackend::Postgres, sql.to_owned()))
        .await
        .unwrap()
        .expect("scalar row")
        .try_get::<i64>("", "v")
        .unwrap()
}

fn tree_bytes(path: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(path) else {
        return 0;
    };
    entries
        .flatten()
        .map(|entry| match entry.metadata() {
            Ok(meta) if meta.is_dir() => tree_bytes(&entry.path()),
            Ok(meta) if meta.is_file() => meta.len(),
            _ => 0,
        })
        .sum()
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    let rank = ((p / 100.0) * sorted.len() as f64).ceil() as usize;
    sorted[rank.clamp(1, sorted.len()) - 1]
}

/// TS-39: N sequential creates with a fresh key each, the previous job finished through
/// `worker_step` first (one active job per instance). Reports min/median/p95/max in
/// milliseconds; asserts only that every create was accepted (no latency target exists).
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts39_job_creation_latency_is_measured_without_a_target() {
    const SAMPLES: usize = 30;
    let temp = TempDir::new("ts39-latency");
    let (service, app, _db) = started_service(&temp.root()).await;
    let mut millis = Vec::with_capacity(SAMPLES);
    for i in 0..SAMPLES {
        let key = format!("ts39-latency-key-{i:04}-abcdef");
        let started = Instant::now();
        let response = app.clone().oneshot(create_request(&key)).await.unwrap();
        let elapsed = started.elapsed();
        let (status, created) = body(response).await;
        assert_eq!(status, StatusCode::ACCEPTED, "create {i}: {created}");
        assert_eq!(created["job"]["status"], "queued");
        millis.push(elapsed.as_secs_f64() * 1000.0);
        assert_eq!(
            service.worker_step().await,
            1,
            "job {i} runs to a terminal state"
        );
    }
    let mut sorted = millis.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!(
        "TS39_LATENCY_MS samples={} min={:.2} median={:.2} p95={:.2} max={:.2} (no target; P-01..P-05 open)",
        sorted.len(),
        sorted[0],
        percentile(&sorted, 50.0),
        percentile(&sorted, 95.0),
        sorted[sorted.len() - 1]
    );
    assert_eq!(millis.len(), SAMPLES);
    assert!(millis.iter().all(|m| m.is_finite() && *m > 0.0));
}

/// TS-40: bursts of concurrent creates and concurrent worker runs. At most one active job
/// exists at any sampled instant; every create is either accepted or `CONCURRENCY_LIMIT`;
/// the worker runs each accepted job exactly once; live reserved bytes and bytes on disk
/// stay within CFG-11/CFG-12 (256 MiB per allocation, 1 GiB total).
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts40_concurrency_and_temporary_bytes_stay_bounded_under_load() {
    const ROUNDS: usize = 5;
    const CREATORS: usize = 12;
    const WORKERS: usize = 4;
    const ALLOC_MAX: i64 = 256 * 1024 * 1024;
    const TOTAL_MAX: i64 = 1024 * 1024 * 1024;

    let temp = TempDir::new("ts40-load");
    let root = temp.root();
    let (service, app, db) = started_service(&root).await;
    let conn = db.conn.clone();

    let stop = Arc::new(AtomicBool::new(false));
    let max_active = Arc::new(AtomicU64::new(0));
    let max_live_bytes = Arc::new(AtomicU64::new(0));
    let max_alloc_bytes = Arc::new(AtomicU64::new(0));
    let max_disk_bytes = Arc::new(AtomicU64::new(0));
    let sampler = {
        let (conn, stop) = (conn.clone(), stop.clone());
        let (active, live, single, disk) = (
            max_active.clone(),
            max_live_bytes.clone(),
            max_alloc_bytes.clone(),
            max_disk_bytes.clone(),
        );
        let root = root.clone();
        tokio::spawn(async move {
            while !stop.load(Ordering::SeqCst) {
                let a = scalar_i64(
                    &conn,
                    "SELECT count(*)::bigint AS v FROM backup_jobs WHERE status IN ('queued','running')",
                )
                .await;
                let l = scalar_i64(
                    &conn,
                    "SELECT COALESCE(sum(bytes_reserved),0)::bigint AS v FROM backup_allocations WHERE state <> 'deleted'",
                )
                .await;
                let s = scalar_i64(
                    &conn,
                    "SELECT COALESCE(max(bytes_reserved),0)::bigint AS v FROM backup_allocations",
                )
                .await;
                active.fetch_max(a as u64, Ordering::SeqCst);
                live.fetch_max(l as u64, Ordering::SeqCst);
                single.fetch_max(s as u64, Ordering::SeqCst);
                disk.fetch_max(tree_bytes(&root), Ordering::SeqCst);
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
    };

    let accepted = Arc::new(AtomicU64::new(0));
    let limited = Arc::new(AtomicU64::new(0));
    let ran = Arc::new(AtomicU64::new(0));
    for round in 0..ROUNDS {
        let mut tasks = Vec::new();
        for c in 0..CREATORS {
            let app = app.clone();
            let (accepted, limited) = (accepted.clone(), limited.clone());
            tasks.push(tokio::spawn(async move {
                let key = format!("ts40-load-key-{round:02}-{c:03}-abcdef");
                let (status, json) = body(app.oneshot(create_request(&key)).await.unwrap()).await;
                match status {
                    StatusCode::ACCEPTED => {
                        accepted.fetch_add(1, Ordering::SeqCst);
                    }
                    StatusCode::CONFLICT => {
                        assert_eq!(json["reason_code"], "CONCURRENCY_LIMIT", "{json}");
                        limited.fetch_add(1, Ordering::SeqCst);
                    }
                    other => panic!("unexpected create status {other}: {json}"),
                }
            }));
        }
        for _ in 0..WORKERS {
            let service = service.clone();
            let ran = ran.clone();
            tasks.push(tokio::spawn(async move {
                let n = service.worker_step().await;
                ran.fetch_add(n as u64, Ordering::SeqCst);
            }));
        }
        for task in tasks {
            task.await.unwrap();
        }
        // Drain whatever a worker step missed because it raced ahead of the accepted create.
        let n = service.worker_step().await;
        ran.fetch_add(n as u64, Ordering::SeqCst);
    }
    stop.store(true, Ordering::SeqCst);
    sampler.await.unwrap();

    let accepted = accepted.load(Ordering::SeqCst);
    let limited = limited.load(Ordering::SeqCst);
    let ran = ran.load(Ordering::SeqCst);
    let jobs_total = scalar_i64(&conn, "SELECT count(*)::bigint AS v FROM backup_jobs").await;
    let jobs_terminal = scalar_i64(
        &conn,
        "SELECT count(*)::bigint AS v FROM backup_jobs WHERE status IN ('incomplete','failed')",
    )
    .await;
    println!(
        "TS40_LOAD rounds={ROUNDS} creators_per_round={CREATORS} workers_per_round={WORKERS} \
         accepted={accepted} concurrency_limited={limited} worker_runs={ran} jobs={jobs_total} \
         terminal={jobs_terminal} max_active_jobs={} max_live_reserved_bytes={} \
         max_single_allocation_bytes={} max_disk_bytes={} (budget per alloc={ALLOC_MAX}, total={TOTAL_MAX})",
        max_active.load(Ordering::SeqCst),
        max_live_bytes.load(Ordering::SeqCst),
        max_alloc_bytes.load(Ordering::SeqCst),
        max_disk_bytes.load(Ordering::SeqCst),
    );

    assert!(
        accepted >= ROUNDS as u64,
        "at least one create per round is accepted"
    );
    assert_eq!(
        accepted + limited,
        (ROUNDS * CREATORS) as u64,
        "every create answered"
    );
    assert_eq!(ran, accepted, "each accepted job is run exactly once");
    assert_eq!(jobs_total as u64, accepted);
    assert_eq!(
        jobs_terminal, jobs_total,
        "all jobs reach incomplete or failed"
    );
    assert!(
        max_active.load(Ordering::SeqCst) <= 1,
        "at most one active job per instance"
    );
    assert!(max_alloc_bytes.load(Ordering::SeqCst) as i64 <= ALLOC_MAX);
    assert!(max_live_bytes.load(Ordering::SeqCst) as i64 <= TOTAL_MAX);
    assert!(max_disk_bytes.load(Ordering::SeqCst) as i64 <= TOTAL_MAX);
}

/// TS-41 API part (also TS-09, TS-11, TS-58 API labels through the composed routes): the
/// administrator creates a job, the worker runs it to `incomplete`, and a direct download
/// request answers 403 `DOWNLOAD_DENIED` with no file. Every response is labelled synthetic.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts41_admin_creates_job_ends_incomplete_and_download_is_denied() {
    let temp = TempDir::new("ts41-e2e");
    let (service, app, _db) = started_service(&temp.root()).await;

    let (status, backup_status) = body(
        app.clone()
            .oneshot(request("GET", "/backup/status"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(backup_status["synthetic_only"], true, "{backup_status}");

    let (status, instances) = body(
        app.clone()
            .oneshot(request("GET", "/backup/instances"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(instances["synthetic"], true, "{instances}");

    let (status, created) = body(
        app.clone()
            .oneshot(create_request("ts41-e2e-key-0001-abcdef"))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(created["job"]["synthetic"], true);
    let id = created["job"]["id"].as_str().unwrap().to_owned();

    assert_eq!(service.worker_step().await, 1);
    let (status, job) = body(
        app.clone()
            .oneshot(request("GET", &format!("/backup/jobs/{id}")))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(job["job"]["status"], "incomplete", "{job}");
    assert_eq!(job["job"]["synthetic"], true);
    assert_eq!(job["job"]["download_eligible"], false);
    assert_ne!(job["job"]["status"], "succeeded");

    let response = app
        .clone()
        .oneshot(request("GET", &format!("/backup/jobs/{id}/download")))
        .await
        .unwrap();
    for forbidden in [
        header::LOCATION,
        header::CONTENT_DISPOSITION,
        header::CONTENT_RANGE,
    ] {
        assert!(
            response.headers().get(&forbidden).is_none(),
            "no {forbidden} on denial"
        );
    }
    let (status, denied): (StatusCode, Value) = body(response).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(denied["reason_code"], "DOWNLOAD_DENIED", "{denied}");
    assert!(
        !denied.to_string().contains(&id),
        "denial does not echo the job id"
    );
}

// ---- TS-58: open items stay open; synthetic-only labels present ----

const OPEN_ITEMS: [&str; 6] = ["Q-04", "Q-06", "Q-07", "Q-10", "Q-11", "Q-12"];

fn workspace_file(relative: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("..")
        .join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// The feature doc must list every still-open owner question as open, plus audit retention
/// and the non-owner technical guards, and say that U1 is synthetic-only.
#[test]
fn ts58_feature_doc_lists_open_items_as_open_and_synthetic_only() {
    let doc = workspace_file("docs/features/backup-restore.md");
    let lower = doc.to_lowercase();
    for id in OPEN_ITEMS {
        assert!(
            doc.contains(id),
            "docs/features/backup-restore.md must list {id} as open"
        );
    }
    assert!(
        lower.contains("audit retention") || lower.contains("retention của audit"),
        "audit retention must be listed as open"
    );
    assert!(
        lower.contains("non-owner") && doc.contains("16 KiB"),
        "non-owner technical guards (16 KiB body cap, list size, fixture size, probe interval) must be listed"
    );
    assert!(
        lower.contains("synthetic-only"),
        "synthetic-only label must be stated"
    );
    assert!(
        doc.contains("BACKUP_ENABLED") && doc.contains("README"),
        "how to enable must name the BACKUP_* keys and point to the README"
    );
}
