//! Shared fixtures and raw-SQL readers. Ground truth is always read with raw
//! SQL, never through the store under test.

use crate::domain::clock::Clock;
use crate::domain::status::BackupJobStatus;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::ports::audit::{AuditError, AuditEvent, AuditSink};
use crate::ports::job_store::{IdemRecord, NewJob};
use crate::test_db::{backup_test_db, BackupTestDb};
use async_trait::async_trait;
use chrono::{DateTime, Duration, TimeZone, Utc};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbBackend, Statement, Value,
};
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;
use uuid::Uuid;

pub(super) const SEEDABLE: [BackupJobStatus; 4] = [
    BackupJobStatus::Queued,
    BackupJobStatus::Running,
    BackupJobStatus::Failed,
    BackupJobStatus::Incomplete,
];

pub(super) fn t0() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 22, 12, 0, 0).unwrap()
}

#[derive(Debug)]
pub(super) struct FixedClock(pub DateTime<Utc>);

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.0
    }
}

/// An audit sink whose write is rejected on a healthy connection (FD 6.6
/// `AuditError::Rejected`): proves an audit failure refuses the operation.
#[derive(Debug)]
pub(super) struct FailingAuditSink;

#[async_trait]
impl AuditSink for FailingAuditSink {
    async fn record(
        &self,
        _tx: Option<&DatabaseTransaction>,
        _ev: AuditEvent,
    ) -> Result<(), AuditError> {
        Err(AuditError::Rejected)
    }
}

/// An audit sink that reports an outage (connection loss, pool exhaustion or
/// statement timeout; FD 6.6 `AuditError::Unavailable`).
#[derive(Debug)]
pub(super) struct OutageAuditSink;

#[async_trait]
impl AuditSink for OutageAuditSink {
    async fn record(
        &self,
        _tx: Option<&DatabaseTransaction>,
        _ev: AuditEvent,
    ) -> Result<(), AuditError> {
        Err(AuditError::Unavailable)
    }
}

/// What a `TxProbeSink` saw on one `record` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Probe {
    /// The store passed its own transaction (not `None`).
    pub saw_tx: bool,
    /// The job row as another connection sees it during the call (status, version).
    pub outside_view: Option<(String, i64)>,
}

/// Spy around the real sink: records whether the store handed it a transaction and
/// what an independent connection sees at that moment, then delegates. A store that
/// audits on its own connection (`None`) or after committing is caught.
#[derive(Debug)]
pub(super) struct TxProbeSink {
    inner: DbAuditSink,
    conn: DatabaseConnection,
    pub seen: Arc<Mutex<Vec<Probe>>>,
}

impl TxProbeSink {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self {
            inner: DbAuditSink::new(conn.clone()),
            conn: conn.clone(),
            seen: Arc::default(),
        }
    }
}

#[async_trait]
impl AuditSink for TxProbeSink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        let outside_view = match ev.job_id {
            Some(id) => self
                .conn
                .query_one(pg(
                    "SELECT status, state_version FROM backup_jobs WHERE id = $1",
                    vec![id.into()],
                ))
                .await
                .expect("probe query")
                .map(|r| {
                    (
                        r.try_get::<String>("", "status").unwrap(),
                        r.try_get::<i64>("", "state_version").unwrap(),
                    )
                }),
            None => None,
        };
        self.seen.lock().unwrap().push(Probe {
            saw_tx: tx.is_some(),
            outside_view,
        });
        self.inner.record(tx, ev).await
    }
}

/// Writes the audit row on the given transaction (or its own connection when given
/// none), then reports failure. Proves the job update and the audit row are one unit:
/// no row may survive the refusal.
#[derive(Debug)]
pub(super) struct WriteThenFailSink {
    inner: DbAuditSink,
}

impl WriteThenFailSink {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self {
            inner: DbAuditSink::new(conn.clone()),
        }
    }
}

#[async_trait]
impl AuditSink for WriteThenFailSink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        self.inner.record(tx, ev).await?;
        Err(AuditError::Rejected)
    }
}

/// Wait until some backend is blocked on a lock, or the task has finished. A
/// condition poll with a generous bound, not a timing assumption.
pub(super) async fn wait_until_blocked_or_done<T>(conn: &DatabaseConnection, task: &JoinHandle<T>) {
    for _ in 0..500 {
        if task.is_finished() {
            return;
        }
        let blocked = count(
            conn,
            "SELECT COUNT(*)::bigint AS n FROM pg_stat_activity              WHERE wait_event_type = 'Lock' AND datname = current_database()",
            vec![],
        )
        .await;
        if blocked > 0 {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}

#[derive(Debug)]
pub(super) struct Fx {
    _db: BackupTestDb,
    pub conn: DatabaseConnection,
    pub store: Arc<DbJobStore>,
    pub sink: Arc<DbAuditSink>,
}

/// Fresh PostgreSQL 15 with the WU-02 migrations, a store wired to the real sink
/// and a clock fixed at `t0()`.
pub(super) async fn fx() -> Fx {
    let db = backup_test_db().await;
    let conn = db.conn.clone();
    let sink = Arc::new(DbAuditSink::new(conn.clone()));
    let store = store_with(&conn, sink.clone());
    Fx {
        _db: db,
        conn,
        store,
        sink,
    }
}

pub(super) fn store_with(conn: &DatabaseConnection, audit: Arc<dyn AuditSink>) -> Arc<DbJobStore> {
    Arc::new(DbJobStore::new(
        conn.clone(),
        audit,
        Arc::new(FixedClock(t0())),
    ))
}

pub(super) fn wire(s: BackupJobStatus) -> &'static str {
    match s {
        BackupJobStatus::Queued => "queued",
        BackupJobStatus::Running => "running",
        BackupJobStatus::Succeeded => "succeeded",
        BackupJobStatus::Failed => "failed",
        BackupJobStatus::RecoveryRequired => "recovery-required",
        BackupJobStatus::Incomplete => "incomplete",
    }
}

pub(super) fn is_terminal(s: BackupJobStatus) -> bool {
    matches!(s, BackupJobStatus::Failed | BackupJobStatus::Incomplete)
}

pub(super) fn hex64(a: u128, b: u128) -> String {
    format!("{a:032x}{b:032x}")
}

pub(super) fn new_job(instance: &str) -> NewJob {
    let id = Uuid::new_v4();
    NewJob {
        id,
        actor_sub: "admin-sub-1".to_owned(),
        instance_id: instance.to_owned(),
        correlation_id: format!("corr-{id}"),
        epoch: 3,
        created_at: t0(),
        retain_until: t0() + Duration::days(30),
    }
}

pub(super) fn idem_for(job: &NewJob) -> IdemRecord {
    IdemRecord {
        id: Uuid::new_v4(),
        actor_sub: job.actor_sub.clone(),
        operation: "backup_db_slice".to_owned(),
        key_digest: hex64(job.id.as_u128(), 1),
        request_fingerprint: hex64(job.id.as_u128(), 2),
        expires_at: t0() + Duration::hours(24),
    }
}

pub(super) fn pg(sql: &str, values: Vec<Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Postgres, sql, values)
}

pub(super) async fn exec(c: &impl ConnectionTrait, sql: &str, values: Vec<Value>) {
    if let Err(e) = c.execute(pg(sql, values)).await {
        panic!("expected success for `{sql}`: {e}");
    }
}

/// `sql` must select a single bigint column named `n`.
pub(super) async fn count(c: &impl ConnectionTrait, sql: &str, values: Vec<Value>) -> i64 {
    let row = c
        .query_one(pg(sql, values))
        .await
        .unwrap_or_else(|e| panic!("count query `{sql}`: {e}"))
        .expect("count row");
    row.try_get::<i64>("", "n").expect("column n")
}

pub(super) async fn jobs_for_instance(c: &impl ConnectionTrait, instance: &str) -> i64 {
    count(
        c,
        "SELECT COUNT(*)::bigint AS n FROM backup_jobs WHERE instance_id = $1",
        vec![instance.into()],
    )
    .await
}

pub(super) async fn active_jobs_for_instance(c: &impl ConnectionTrait, instance: &str) -> i64 {
    count(
        c,
        "SELECT COUNT(*)::bigint AS n FROM backup_jobs \
         WHERE instance_id = $1 AND status IN ('queued', 'running')",
        vec![instance.into()],
    )
    .await
}

pub(super) async fn idempotency_rows(c: &impl ConnectionTrait) -> i64 {
    count(
        c,
        "SELECT COUNT(*)::bigint AS n FROM backup_idempotency",
        vec![],
    )
    .await
}

pub(super) async fn audit_count_for_job(c: &impl ConnectionTrait, job: Uuid) -> i64 {
    count(
        c,
        "SELECT COUNT(*)::bigint AS n FROM backup_audit WHERE job_id = $1",
        vec![job.into()],
    )
    .await
}

pub(super) async fn audit_count_for_instance(c: &impl ConnectionTrait, instance: &str) -> i64 {
    count(
        c,
        "SELECT COUNT(*)::bigint AS n FROM backup_audit WHERE instance_id = $1",
        vec![instance.into()],
    )
    .await
}

/// Fields of a `backup_jobs` row as raw SQL sees them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RawJob {
    pub status: String,
    pub state_version: i64,
    pub reason_code: Option<String>,
    pub epoch: i64,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub components: String,
}

pub(super) async fn raw_job(c: &impl ConnectionTrait, id: Uuid) -> Option<RawJob> {
    let row = c
        .query_one(pg(
            "SELECT status, state_version, reason_code, epoch, started_at, finished_at, \
             deadline_at, updated_at, components::text AS components \
             FROM backup_jobs WHERE id = $1",
            vec![id.into()],
        ))
        .await
        .expect("job query")?;
    Some(RawJob {
        status: row.try_get("", "status").unwrap(),
        state_version: row.try_get("", "state_version").unwrap(),
        reason_code: row.try_get("", "reason_code").unwrap(),
        epoch: row.try_get("", "epoch").unwrap(),
        started_at: row.try_get("", "started_at").unwrap(),
        finished_at: row.try_get("", "finished_at").unwrap(),
        deadline_at: row.try_get("", "deadline_at").unwrap(),
        updated_at: row.try_get("", "updated_at").unwrap(),
        components: row.try_get("", "components").unwrap(),
    })
}

pub(super) async fn require_job(c: &impl ConnectionTrait, id: Uuid) -> RawJob {
    raw_job(c, id).await.expect("job row exists")
}

/// A `backup_jobs` row to seed with raw SQL (bypasses the store).
pub(super) struct Seed {
    pub instance: String,
    pub status: BackupJobStatus,
    pub version: i64,
    pub created_at: DateTime<Utc>,
    pub reason: Option<String>,
    pub components: String,
}

impl Seed {
    pub fn new(instance: &str, status: BackupJobStatus) -> Self {
        Self {
            instance: instance.to_owned(),
            status,
            version: 0,
            created_at: t0() - Duration::hours(1),
            reason: None,
            components: "[]".to_owned(),
        }
    }
    pub fn version(mut self, v: i64) -> Self {
        self.version = v;
        self
    }
    pub fn created_at(mut self, at: DateTime<Utc>) -> Self {
        self.created_at = at;
        self
    }
    pub fn reason(mut self, r: &str) -> Self {
        self.reason = Some(r.to_owned());
        self
    }
    pub fn components(mut self, json: &str) -> Self {
        self.components = json.to_owned();
        self
    }
}

pub(super) async fn seed(c: &impl ConnectionTrait, s: Seed) -> Uuid {
    let id = Uuid::new_v4();
    let started = (s.status != BackupJobStatus::Queued).then_some(s.created_at);
    let finished = is_terminal(s.status).then_some(s.created_at);
    exec(
        c,
        "INSERT INTO backup_jobs (id, actor_sub, instance_id, operation, status, state_version, \
         reason_code, components, epoch, correlation_id, created_at, updated_at, started_at, \
         finished_at, retain_until) \
         VALUES ($1, 'seed-actor', $2, 'backup_db_slice', $3, $4, $5, $6::jsonb, 1, $7, $8, $8, \
         $9, $10, $11)",
        vec![
            id.into(),
            s.instance.into(),
            wire(s.status).into(),
            s.version.into(),
            s.reason.into(),
            s.components.into(),
            format!("seed-corr-{id}").into(),
            s.created_at.into(),
            started.into(),
            finished.into(),
            (s.created_at + Duration::days(30)).into(),
        ],
    )
    .await;
    id
}

/// Fields of a `backup_audit` row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AuditRow {
    pub occurred_at: DateTime<Utc>,
    pub actor_sub: Option<String>,
    pub instance_id: String,
    pub job_id: Option<Uuid>,
    pub allocation_id: Option<Uuid>,
    pub operation: String,
    pub event: String,
    pub outcome: String,
    pub reason_code: Option<String>,
    pub digest: Option<String>,
    pub correlation_id: String,
}

async fn audit_rows_where(c: &impl ConnectionTrait, filter: &str, value: Value) -> Vec<AuditRow> {
    let sql = format!(
        "SELECT occurred_at, actor_sub, instance_id, job_id, allocation_id, operation, event, \
         outcome, reason_code, digest::text AS digest, correlation_id \
         FROM backup_audit WHERE {filter} ORDER BY occurred_at, event"
    );
    c.query_all(pg(&sql, vec![value]))
        .await
        .expect("audit query")
        .iter()
        .map(|r| AuditRow {
            occurred_at: r.try_get("", "occurred_at").unwrap(),
            actor_sub: r.try_get("", "actor_sub").unwrap(),
            instance_id: r.try_get("", "instance_id").unwrap(),
            job_id: r.try_get("", "job_id").unwrap(),
            allocation_id: r.try_get("", "allocation_id").unwrap(),
            operation: r.try_get("", "operation").unwrap(),
            event: r.try_get("", "event").unwrap(),
            outcome: r.try_get("", "outcome").unwrap(),
            reason_code: r.try_get("", "reason_code").unwrap(),
            digest: r.try_get("", "digest").unwrap(),
            correlation_id: r.try_get("", "correlation_id").unwrap(),
        })
        .collect()
}

pub(super) async fn audit_rows_for_job(c: &impl ConnectionTrait, job: Uuid) -> Vec<AuditRow> {
    audit_rows_where(c, "job_id = $1", job.into()).await
}

pub(super) async fn audit_rows_for_instance(
    c: &impl ConnectionTrait,
    instance: &str,
) -> Vec<AuditRow> {
    audit_rows_where(c, "instance_id = $1", instance.into()).await
}

/// Whole audit row as JSON text, for excluded-field scans.
pub(super) async fn audit_json_for_job(c: &impl ConnectionTrait, job: Uuid) -> Vec<String> {
    c.query_all(pg(
        "SELECT row_to_json(a)::text AS j FROM backup_audit a WHERE job_id = $1",
        vec![job.into()],
    ))
    .await
    .expect("audit json query")
    .iter()
    .map(|r| r.try_get::<String>("", "j").unwrap())
    .collect()
}
