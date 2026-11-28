//! WU-07 tests: TS-13, TS-14b, TS-15, TS-16, TS-44a. DB-backed tests need Docker and
//! are opt-in (`-- --ignored`). Ground truth is always read with raw SQL, never through
//! the handler under test.

use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, Statement, Value,
};
use uuid::Uuid;

use super::{AcceptContext, Accepted, CreateJobError, CreateJobHandler, CreateRequest};
use crate::domain::clock::Clock;
use crate::domain::idempotency::{key_digest, request_fingerprint, OPERATION};
use crate::domain::status::BackupJobStatus;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::ports::audit::{AuditError, AuditEvent, AuditSink};
use crate::ports::job_store::{JobStore, ReasonCode, TransitionResult};
use crate::test_db::{backup_test_db, BackupTestDb};

const ACTOR: &str = "admin-sub-1";
const INSTANCE: &str = "local-1";
const KEY: &str = "key-12345678";
const RETENTION_DAYS: u32 = 30;
const EPOCH: i64 = 3;

fn t0() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 22, 12, 0, 0).unwrap()
}

#[derive(Debug)]
struct TestClock(Mutex<DateTime<Utc>>);

impl TestClock {
    fn new(at: DateTime<Utc>) -> Arc<Self> {
        Arc::new(Self(Mutex::new(at)))
    }
    fn set(&self, at: DateTime<Utc>) {
        *self.0.lock().unwrap() = at;
    }
}

impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

/// Rejects every write: a reachable database that refuses the audit statement.
#[derive(Debug)]
struct RejectingSink;

#[async_trait]
impl AuditSink for RejectingSink {
    async fn record(
        &self,
        _tx: Option<&DatabaseTransaction>,
        _ev: AuditEvent,
    ) -> Result<(), AuditError> {
        Err(AuditError::Rejected)
    }
}

/// Reports an outage (connection loss, pool exhaustion, statement timeout).
#[derive(Debug)]
struct OutageSink;

#[async_trait]
impl AuditSink for OutageSink {
    async fn record(
        &self,
        _tx: Option<&DatabaseTransaction>,
        _ev: AuditEvent,
    ) -> Result<(), AuditError> {
        Err(AuditError::Unavailable)
    }
}

/// What the spy saw on one `record` call.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Seen {
    saw_tx: bool,
    /// The job row as an independent connection sees it during the call.
    outside_has_job: bool,
}

/// Delegates to the real sink after recording whether it got a transaction and whether
/// the job was already visible to another connection. An audit written on its own
/// connection, or after the commit, is caught.
#[derive(Debug)]
struct SpySink {
    inner: DbAuditSink,
    conn: DatabaseConnection,
    seen: Arc<Mutex<Vec<Seen>>>,
}

#[async_trait]
impl AuditSink for SpySink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        let outside_has_job = match ev.job_id {
            Some(id) => {
                count(
                    &self.conn,
                    "SELECT count(*) AS n FROM backup_jobs WHERE id = $1",
                    vec![id.into()],
                )
                .await
                    > 0
            }
            None => false,
        };
        self.seen.lock().unwrap().push(Seen {
            saw_tx: tx.is_some(),
            outside_has_job,
        });
        self.inner.record(tx, ev).await
    }
}

fn pg(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Postgres, sql, values)
}

async fn count(c: &impl ConnectionTrait, sql: &str, values: Vec<Value>) -> i64 {
    c.query_one(pg(sql, values))
        .await
        .expect("count query")
        .expect("count row")
        .try_get::<i64>("", "n")
        .expect("count column")
}

async fn n(c: &impl ConnectionTrait, table: &str) -> i64 {
    count(c, &format!("SELECT count(*) AS n FROM {table}"), vec![]).await
}

struct Fx {
    db: BackupTestDb,
    clock: Arc<TestClock>,
    store: Arc<DbJobStore>,
    handler: Arc<CreateJobHandler>,
}

async fn fx() -> Fx {
    fx_with(|conn| Arc::new(DbAuditSink::new(conn.clone()))).await
}

async fn fx_with(make_audit: impl FnOnce(&DatabaseConnection) -> Arc<dyn AuditSink>) -> Fx {
    let db = backup_test_db().await;
    let clock = TestClock::new(t0());
    let store_audit: Arc<dyn AuditSink> = Arc::new(DbAuditSink::new(db.conn.clone()));
    let store = Arc::new(DbJobStore::new(db.conn.clone(), store_audit, clock.clone()));
    let handler = Arc::new(CreateJobHandler::new(
        db.conn.clone(),
        store.clone(),
        make_audit(&db.conn),
        clock.clone(),
        RETENTION_DAYS,
        EPOCH,
    ));
    Fx {
        db,
        clock,
        store,
        handler,
    }
}

fn ctx() -> AcceptContext {
    AcceptContext {
        actor_sub: ACTOR.to_owned(),
        correlation_id: "corr-create-1".to_owned(),
    }
}

fn req(instance: &str, key: &str) -> CreateRequest {
    CreateRequest {
        instance_id: instance.to_owned(),
        idempotency_key: key.to_owned(),
    }
}

fn created(a: Result<Accepted, CreateJobError>) -> crate::ports::job_store::JobRow {
    match a {
        Ok(Accepted::Created(j)) => j,
        other => panic!("expected Created, got {other:?}"),
    }
}

fn replayed(a: Result<Accepted, CreateJobError>) -> crate::ports::job_store::JobRow {
    match a {
        Ok(Accepted::Replayed(j)) => j,
        other => panic!("expected Replayed, got {other:?}"),
    }
}

async fn no_trace_of_an_accept(conn: &DatabaseConnection) {
    assert_eq!(n(conn, "backup_jobs").await, 0, "no job persisted");
    assert_eq!(n(conn, "backup_idempotency").await, 0, "no idempotency row");
    assert_eq!(n(conn, "backup_allocations").await, 0, "no allocation");
    assert_eq!(
        count(
            conn,
            "SELECT count(*) AS n FROM backup_audit WHERE event = 'job.accepted'",
            vec![]
        )
        .await,
        0,
        "no accepted audit row"
    );
}

// ---------------------------------------------------------------- TS-13

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts13_same_key_same_body_replays_the_existing_job() {
    let f = fx().await;
    let first = created(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    let second = replayed(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    assert_eq!(second.id, first.id, "replay returns the accepted job");
    assert_eq!(n(&f.db.conn, "backup_jobs").await, 1);
    assert_eq!(n(&f.db.conn, "backup_idempotency").await, 1);
    assert_eq!(
        count(
            &f.db.conn,
            "SELECT count(*) AS n FROM backup_audit WHERE event = 'job.accepted'",
            vec![]
        )
        .await,
        1,
        "a replay writes no second accepted event"
    );
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts13_same_key_different_body_is_key_reused() {
    let f = fx().await;
    created(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    let err = f.handler.create(&ctx(), req("local-2", KEY)).await;
    assert_eq!(err, Err(CreateJobError::IdempotencyKeyReused));
    assert_eq!(n(&f.db.conn, "backup_jobs").await, 1, "no second job");
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts13_the_key_is_scoped_by_actor() {
    let f = fx().await;
    created(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    let other = AcceptContext {
        actor_sub: "admin-sub-2".to_owned(),
        correlation_id: "corr-2".to_owned(),
    };
    // Another actor reusing the same key is a new request, not a replay; the single
    // active job of the instance then refuses it.
    assert_eq!(
        f.handler.create(&other, req(INSTANCE, KEY)).await,
        Err(CreateJobError::ConcurrencyLimit)
    );
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts13_raw_key_is_absent_from_every_table_and_only_digests_are_stored() {
    let needle = "RAW-KEY-needle-0123456789";
    let f = fx().await;
    created(f.handler.create(&ctx(), req(INSTANCE, needle)).await);
    for table in [
        "backup_jobs",
        "backup_idempotency",
        "backup_audit",
        "backup_allocations",
        "backup_epoch",
    ] {
        let hits = count(
            &f.db.conn,
            &format!("SELECT count(*) AS n FROM {table} t WHERE t::text LIKE '%needle%'"),
            vec![],
        )
        .await;
        assert_eq!(hits, 0, "raw key leaked into {table}");
    }
    let row = f
        .db
        .conn
        .query_one(pg(
            "SELECT key_digest, request_fingerprint, actor_sub, operation FROM backup_idempotency",
            vec![],
        ))
        .await
        .unwrap()
        .expect("one idempotency row");
    assert_eq!(
        row.try_get::<String>("", "key_digest").unwrap(),
        key_digest(ACTOR, OPERATION, needle)
    );
    assert_eq!(
        row.try_get::<String>("", "request_fingerprint").unwrap(),
        request_fingerprint(OPERATION, INSTANCE)
    );
    assert_eq!(row.try_get::<String>("", "actor_sub").unwrap(), ACTOR);
    assert_eq!(row.try_get::<String>("", "operation").unwrap(), OPERATION);
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts13_concurrent_same_key_same_body_yields_one_job_and_replays() {
    let f = fx().await;
    let mut tasks = Vec::new();
    for _ in 0..8 {
        let h = f.handler.clone();
        tasks.push(tokio::spawn(async move {
            h.create(&ctx(), req(INSTANCE, KEY)).await
        }));
    }
    let mut created_ids = Vec::new();
    let mut replay_ids = Vec::new();
    for t in tasks {
        match t.await.unwrap() {
            Ok(Accepted::Created(j)) => created_ids.push(j.id),
            Ok(Accepted::Replayed(j)) => replay_ids.push(j.id),
            Err(e) => panic!("a duplicate must replay, never fail: {e:?}"),
        }
    }
    assert_eq!(created_ids.len(), 1, "exactly one 202");
    assert_eq!(replay_ids.len(), 7, "the rest are 200 replays");
    assert!(replay_ids.iter().all(|id| *id == created_ids[0]));
    assert_eq!(n(&f.db.conn, "backup_jobs").await, 1);
    assert_eq!(n(&f.db.conn, "backup_idempotency").await, 1);
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts13_concurrent_same_key_different_body_yields_accept_and_key_reused() {
    let f = fx().await;
    for round in 0..4 {
        let key = format!("race-key-{round:04}");
        let (a, b) = (format!("inst-a-{round}"), format!("inst-b-{round}"));
        let (h1, h2) = (f.handler.clone(), f.handler.clone());
        let (k1, k2) = (key.clone(), key.clone());
        let t1 = tokio::spawn(async move { h1.create(&ctx(), req(&a, &k1)).await });
        let t2 = tokio::spawn(async move { h2.create(&ctx(), req(&b, &k2)).await });
        let (r1, r2) = (t1.await.unwrap(), t2.await.unwrap());
        let accepted = [&r1, &r2]
            .iter()
            .filter(|r| matches!(r, Ok(Accepted::Created(_))))
            .count();
        let reused = [&r1, &r2]
            .iter()
            .filter(|r| matches!(r, Err(CreateJobError::IdempotencyKeyReused)))
            .count();
        assert_eq!((accepted, reused), (1, 1), "round {round}: {r1:?} / {r2:?}");
    }
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts13_an_expired_but_unpurged_key_is_a_new_request() {
    let f = fx().await;
    let first = created(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    // Finish the first job so the instance is free; its idempotency row stays.
    let applied = f
        .store
        .transition(
            first.id,
            (BackupJobStatus::Queued, first.state_version),
            BackupJobStatus::Failed,
            Some(ReasonCode::DispatchFailed),
            None,
        )
        .await
        .unwrap();
    assert_eq!(applied, TransitionResult::Applied);
    // Just before expiry the key still replays.
    f.clock
        .set(t0() + Duration::days(i64::from(RETENTION_DAYS)) - Duration::seconds(1));
    replayed(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    // At expiry the row is treated as absent and is replaced inside the transaction.
    f.clock
        .set(t0() + Duration::days(i64::from(RETENTION_DAYS)));
    let second = created(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    assert_ne!(second.id, first.id, "a new job, not the old one");
    assert_eq!(
        n(&f.db.conn, "backup_idempotency").await,
        1,
        "old row replaced"
    );
    assert_eq!(
        n(&f.db.conn, "backup_jobs").await,
        2,
        "the old job row stays until its own retention purge"
    );
}

// --------------------------------------------------------------- TS-14b

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts14b_a_second_key_is_concurrency_limit_and_a_replay_never_is() {
    let f = fx().await;
    let first = created(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    assert_eq!(
        f.handler
            .create(&ctx(), req(INSTANCE, "second-key-1234"))
            .await,
        Err(CreateJobError::ConcurrencyLimit)
    );
    // Replay is decided first, so the accepted job is never answered CONCURRENCY_LIMIT.
    let again = replayed(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    assert_eq!(again.id, first.id);
    assert_eq!(
        count(
            &f.db.conn,
            "SELECT count(*) AS n FROM backup_jobs WHERE status IN ('queued','running')",
            vec![]
        )
        .await,
        1
    );
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts14b_concurrent_different_keys_leave_exactly_one_active_job() {
    let f = fx().await;
    let mut tasks = Vec::new();
    for i in 0..6 {
        let h = f.handler.clone();
        tasks.push(tokio::spawn(async move {
            h.create(&ctx(), req(INSTANCE, &format!("distinct-key-{i:04}")))
                .await
        }));
    }
    let (mut ok, mut limited) = (0, 0);
    for t in tasks {
        match t.await.unwrap() {
            Ok(Accepted::Created(_)) => ok += 1,
            Err(CreateJobError::ConcurrencyLimit) => limited += 1,
            other => panic!("unexpected outcome {other:?}"),
        }
    }
    assert_eq!((ok, limited), (1, 5));
    assert_eq!(n(&f.db.conn, "backup_jobs").await, 1);
    assert_eq!(n(&f.db.conn, "backup_idempotency").await, 1);
}

// ---------------------------------------------------------------- TS-15

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts15_a_rejected_audit_write_persists_nothing() {
    let f = fx_with(|_| Arc::new(RejectingSink)).await;
    let r = f.handler.create(&ctx(), req(INSTANCE, KEY)).await;
    assert_eq!(r, Err(CreateJobError::AuditUnavailable));
    no_trace_of_an_accept(&f.db.conn).await;
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts15_an_audit_outage_is_store_unavailable_and_persists_nothing() {
    let f = fx_with(|_| Arc::new(OutageSink)).await;
    let r = f.handler.create(&ctx(), req(INSTANCE, KEY)).await;
    assert_eq!(r, Err(CreateJobError::StoreUnavailable));
    no_trace_of_an_accept(&f.db.conn).await;
}

// ---------------------------------------------------------------- TS-16

const ARM_COMMIT_FAULT: &str = "
CREATE TABLE commit_fault (armed boolean NOT NULL);
INSERT INTO commit_fault VALUES (true);
CREATE FUNCTION fail_commit() RETURNS trigger AS $$
BEGIN
  IF (SELECT armed FROM commit_fault LIMIT 1) THEN
    RAISE EXCEPTION 'injected commit failure';
  END IF;
  RETURN NULL;
END $$ LANGUAGE plpgsql;
CREATE CONSTRAINT TRIGGER fail_commit AFTER INSERT ON backup_idempotency
  DEFERRABLE INITIALLY DEFERRED FOR EACH ROW EXECUTE FUNCTION fail_commit();
";

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts16_a_failed_commit_is_not_accepted_and_a_retry_resolves() {
    let f = fx().await;
    f.db.conn
        .execute_unprepared(ARM_COMMIT_FAULT)
        .await
        .expect("arm the deferred commit-time fault");

    // The fault fires at COMMIT, after every statement succeeded: no accepted answer.
    let r = f.handler.create(&ctx(), req(INSTANCE, KEY)).await;
    assert_eq!(r, Err(CreateJobError::StoreUnavailable));
    no_trace_of_an_accept(&f.db.conn).await;

    // Disarm; the same key now resolves to exactly one job, then replays.
    f.db.conn
        .execute_unprepared("UPDATE commit_fault SET armed = false")
        .await
        .unwrap();
    let job = created(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    let again = replayed(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    assert_eq!(again.id, job.id);
    assert_eq!(n(&f.db.conn, "backup_jobs").await, 1);
}

// --------------------------------------------------------------- TS-44a

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts44a_job_accepted_event_has_every_field_and_no_excluded_content() {
    let key = "audit-key-needle-9876";
    let f = fx().await;
    let job = created(f.handler.create(&ctx(), req(INSTANCE, key)).await);
    let rows =
        f.db.conn
            .query_all(pg(
                "SELECT occurred_at, actor_sub, instance_id, job_id, allocation_id, operation, \
             event, outcome, reason_code, digest, correlation_id, row_to_json(a)::text AS raw \
             FROM backup_audit a WHERE job_id = $1",
                vec![job.id.into()],
            ))
            .await
            .unwrap();
    assert_eq!(rows.len(), 1, "exactly one audit row for the accepted job");
    let r = &rows[0];
    assert_eq!(r.try_get::<String>("", "event").unwrap(), "job.accepted");
    assert_eq!(r.try_get::<String>("", "outcome").unwrap(), "success");
    assert_eq!(r.try_get::<String>("", "actor_sub").unwrap(), ACTOR);
    assert_eq!(r.try_get::<String>("", "instance_id").unwrap(), INSTANCE);
    assert_eq!(r.try_get::<Uuid>("", "job_id").unwrap(), job.id);
    assert_eq!(
        r.try_get::<Option<Uuid>>("", "allocation_id").unwrap(),
        None
    );
    assert_eq!(r.try_get::<String>("", "operation").unwrap(), OPERATION);
    assert_eq!(
        r.try_get::<Option<String>>("", "reason_code").unwrap(),
        None
    );
    assert_eq!(
        r.try_get::<String>("", "correlation_id").unwrap(),
        "corr-create-1"
    );
    assert_eq!(r.try_get::<DateTime<Utc>>("", "occurred_at").unwrap(), t0());
    assert_eq!(
        r.try_get::<Option<String>>("", "digest").unwrap(),
        Some(key_digest(ACTOR, OPERATION, key)),
        "the digest column carries the key digest, never the key"
    );
    let raw = r.try_get::<String>("", "raw").unwrap();
    assert!(!raw.contains("needle"), "raw key must not appear: {raw}");
}

#[tokio::test]
#[ignore = "needs Docker (PostgreSQL 15 testcontainer)"]
async fn ts44a_the_accepted_event_is_written_on_the_acceptance_transaction() {
    let seen: Arc<Mutex<Vec<Seen>>> = Arc::default();
    let seen_for_sink = seen.clone();
    let f = fx_with(move |conn| {
        Arc::new(SpySink {
            inner: DbAuditSink::new(conn.clone()),
            conn: conn.clone(),
            seen: seen_for_sink,
        })
    })
    .await;
    created(f.handler.create(&ctx(), req(INSTANCE, KEY)).await);
    let seen = seen.lock().unwrap().clone();
    assert_eq!(seen.len(), 1, "one audit call for the acceptance");
    assert!(seen[0].saw_tx, "audit must use the acceptance transaction");
    assert!(
        !seen[0].outside_has_job,
        "the job is uncommitted while the audit row is written (same transaction)"
    );
}
