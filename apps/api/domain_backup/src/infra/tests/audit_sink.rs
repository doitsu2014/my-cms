//! `DbAuditSink` red tests. TS-69 (audit sink records in the caller's transaction;
//! rollback leaves no audit row; AC-005 S2, T-08), plus append-only use of the
//! table (WU-02 known limitation: audit writes are plain INSERT only). FN-34.

use super::support::*;
use crate::ports::audit::{
    AuditError, AuditEvent, AuditEventKind, AuditOutcome, AuditSink, Digest,
};
use sea_orm::TransactionTrait;
use uuid::Uuid;

/// The closed event set, spelled independently of `as_str` (IFC section 7).
const EVENT_NAMES: [&str; 16] = [
    "job.accepted",
    "job.running",
    "job.incomplete",
    "job.failed",
    "dispatch.failed",
    "download.denied",
    "capability.denied",
    "allocation.registered",
    "allocation.delete_claimed",
    "allocation.deleted",
    "orphan.discovered",
    "cleanup.exhausted",
    "ownership.ambiguous",
    "reconcile.fenced",
    "reconcile.missing_file",
    "retention.purged",
];

fn event(instance: &str, kind: AuditEventKind) -> AuditEvent {
    AuditEvent {
        occurred_at: t0(),
        actor_sub: None,
        instance_id: instance.to_owned(),
        job_id: None,
        allocation_id: None,
        operation: "backup_db_slice".to_owned(),
        event: kind,
        outcome: AuditOutcome::Success,
        reason_code: None,
        digest: None,
        correlation_id: "corr-1".to_owned(),
    }
}

#[test]
fn event_kinds_map_to_the_closed_event_set() {
    // IFC section 7: one closed set of sixteen stored spellings. No Docker needed.
    let names: Vec<&str> = AuditEventKind::ALL.iter().map(|k| k.as_str()).collect();
    assert_eq!(names, EVENT_NAMES);
    let outcomes: Vec<&str> = [
        AuditOutcome::Success,
        AuditOutcome::Failure,
        AuditOutcome::Denied,
    ]
    .iter()
    .map(|o| o.as_str())
    .collect();
    assert_eq!(outcomes, ["success", "failure", "denied"]);
}

#[test]
fn a_digest_is_only_a_sha256_hex_string() {
    // IFC section 7: `digest` carries only a SHA-256 of a non-secret identifier, never a raw
    // value (T-09). Unit-own, no Docker.
    let ok = hex64(1, 2);
    assert_eq!(Digest::new(&ok).expect("64 hex").as_str(), ok);
    for bad in [
        "",
        "short",
        "raw-idempotency-key-0123456789",
        &ok[..63],
        &format!("{ok}0"),
        &ok.replace('0', "g"),
    ] {
        assert!(Digest::new(bad).is_err(), "{bad:?} must be refused");
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_row_recorded_on_the_callers_transaction_is_visible_only_after_commit() {
    // TS-69: the row is written on the caller's transaction, not on its own connection.
    let f = fx().await;
    let tx = f.conn.begin().await.unwrap();
    f.sink
        .record(Some(&tx), event("audit-a", AuditEventKind::JobRunning))
        .await
        .expect("record");
    assert_eq!(
        audit_count_for_instance(&tx, "audit-a").await,
        1,
        "visible inside the caller's transaction"
    );
    assert_eq!(
        audit_count_for_instance(&f.conn, "audit-a").await,
        0,
        "invisible to other connections before commit"
    );
    tx.commit().await.unwrap();
    assert_eq!(audit_count_for_instance(&f.conn, "audit-a").await, 1);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn rolling_back_the_callers_transaction_leaves_no_audit_row() {
    // TS-69: rollback leaves no audit row. The in-transaction count first proves a write happened.
    let f = fx().await;
    let tx = f.conn.begin().await.unwrap();
    f.sink
        .record(Some(&tx), event("audit-a", AuditEventKind::JobAccepted))
        .await
        .expect("record");
    assert_eq!(
        audit_count_for_instance(&tx, "audit-a").await,
        1,
        "the row must exist on the transaction before the rollback"
    );
    tx.rollback().await.unwrap();
    assert_eq!(audit_count_for_instance(&f.conn, "audit-a").await, 0);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn recording_without_a_transaction_commits_immediately() {
    // `tx = None`: attempt-only events (download.denied, capability.denied) commit on their own.
    let f = fx().await;
    f.sink
        .record(None, event("audit-a", AuditEventKind::DownloadDenied))
        .await
        .expect("record");
    assert_eq!(audit_count_for_instance(&f.conn, "audit-a").await, 1);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn every_column_of_the_event_is_persisted() {
    let f = fx().await;
    let job = Uuid::new_v4();
    let allocation = Uuid::new_v4();
    let digest = hex64(7, 9);
    let ev = AuditEvent {
        occurred_at: t0(),
        actor_sub: Some("admin-sub-9".to_owned()),
        instance_id: "audit-a".to_owned(),
        job_id: Some(job),
        allocation_id: Some(allocation),
        operation: "backup_db_slice".to_owned(),
        event: AuditEventKind::CapabilityDenied,
        outcome: AuditOutcome::Denied,
        reason_code: Some("ROLE_REQUIRED".to_owned()),
        digest: Some(Digest::new(&digest).expect("valid digest")),
        correlation_id: "corr-9".to_owned(),
    };
    f.sink.record(None, ev).await.expect("record");

    let rows = audit_rows_for_instance(&f.conn, "audit-a").await;
    assert_eq!(rows.len(), 1);
    let row = &rows[0];
    assert_eq!(row.occurred_at, t0());
    assert_eq!(row.actor_sub.as_deref(), Some("admin-sub-9"));
    assert_eq!(row.job_id, Some(job));
    assert_eq!(row.allocation_id, Some(allocation));
    assert_eq!(row.operation, "backup_db_slice");
    assert_eq!(row.event, "capability.denied");
    assert_eq!(row.outcome, "denied");
    assert_eq!(row.reason_code.as_deref(), Some("ROLE_REQUIRED"));
    assert_eq!(row.digest.as_deref(), Some(digest.as_str()));
    assert_eq!(row.correlation_id, "corr-9");
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn every_event_kind_is_stored_with_its_closed_set_name() {
    let f = fx().await;
    for kind in AuditEventKind::ALL {
        f.sink
            .record(None, event("audit-a", kind))
            .await
            .unwrap_or_else(|e| panic!("{kind:?}: {e:?}"));
    }
    let mut stored: Vec<String> = audit_rows_for_instance(&f.conn, "audit-a")
        .await
        .into_iter()
        .map(|r| r.event)
        .collect();
    stored.sort();
    let mut expected: Vec<String> = EVENT_NAMES.iter().map(|s| (*s).to_owned()).collect();
    expected.sort();
    assert_eq!(stored, expected);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn recording_is_plain_insert_and_never_touches_existing_rows() {
    // WU-02 limitation: the append-only trigger rejects any UPDATE or DELETE, so the sink
    // must only INSERT. Recording the same event twice appends two rows.
    // Partly not red (structural): `INSERT .. ON CONFLICT DO NOTHING` cannot be told from
    // a plain INSERT, because every row gets a fresh generated id and nothing conflicts.
    // `ON CONFLICT DO UPDATE` and UPDATE-instead-of-INSERT are caught by the trigger.
    let f = fx().await;
    let ev = event("audit-a", AuditEventKind::JobRunning);
    f.sink.record(None, ev.clone()).await.expect("first");
    let first = audit_rows_for_instance(&f.conn, "audit-a").await;
    assert_eq!(first.len(), 1);
    f.sink
        .record(None, ev.clone())
        .await
        .expect("second, same event");
    let tx = f.conn.begin().await.unwrap();
    f.sink.record(Some(&tx), ev).await.expect("third, in a tx");
    tx.commit().await.unwrap();

    let all = audit_rows_for_instance(&f.conn, "audit-a").await;
    assert_eq!(all.len(), 3, "each record appends a row");
    assert_eq!(all[0], first[0], "earlier rows are untouched");
    let distinct_ids = count(
        &f.conn,
        "SELECT COUNT(DISTINCT id)::bigint AS n FROM backup_audit WHERE instance_id = $1",
        vec!["audit-a".into()],
    )
    .await;
    assert_eq!(distinct_ids, 3, "each row has its own generated id");
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn an_audit_write_failure_is_reported_not_swallowed() {
    // IFC 6.3: callers refuse the operation when the sink reports failure. The table is
    // gone but the connection is healthy, so this is a schema rejection (FD 6.6
    // `AuditError::Rejected`), not an outage.
    let f = fx().await;
    exec(
        &f.conn,
        "ALTER TABLE backup_audit RENAME TO backup_audit_moved",
        vec![],
    )
    .await;
    let outcome = f
        .sink
        .record(None, event("audit-a", AuditEventKind::JobFailed))
        .await;
    assert_eq!(outcome, Err(AuditError::Rejected));

    let tx = f.conn.begin().await.unwrap();
    let in_tx = f
        .sink
        .record(Some(&tx), event("audit-a", AuditEventKind::JobFailed))
        .await;
    assert_eq!(in_tx, Err(AuditError::Rejected));
    tx.rollback().await.unwrap();
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn sqlstates_split_into_statement_rejections_and_fail_closed_outages() {
    // Test-after (build-and-verify WU-03), FD 6.6: real server errors raised on a healthy
    // connection. Statement rejections (data 22, integrity 23, schema/access 42, PL/pgSQL
    // P0) are `Rejected`; every other SQLSTATE fails closed as an outage.
    use crate::infra::db_audit_sink::classify_audit_error;
    use sea_orm::{ConnectionTrait, DbBackend, Statement};
    let f = fx().await;
    let raise = |code: &str| {
        Statement::from_string(
            DbBackend::Postgres,
            format!("DO $$ BEGIN RAISE EXCEPTION 'probe' USING ERRCODE = '{code}'; END $$"),
        )
    };
    let cases = [
        ("22001", AuditError::Rejected),
        ("23505", AuditError::Rejected),
        ("42P01", AuditError::Rejected),
        ("P0001", AuditError::Rejected),
        ("53100", AuditError::Unavailable),
        ("25006", AuditError::Unavailable),
        ("57014", AuditError::Unavailable),
        ("40001", AuditError::Unavailable),
        ("XX000", AuditError::Unavailable),
    ];
    for (code, expected) in cases {
        let err = f
            .conn
            .execute(raise(code))
            .await
            .expect_err("the probe raises");
        assert_eq!(classify_audit_error(&err), expected, "SQLSTATE {code}");
    }
}

#[test]
fn connection_level_and_unclassifiable_failures_are_an_outage() {
    // FD 6.6: connection loss, pool exhaustion and statement timeout are the outage
    // predicate; a failure that cannot be told apart is treated as the outage (fail
    // closed). A rejection on a healthy connection is covered by the DB test above.
    use crate::infra::db_audit_sink::classify_audit_error;
    use sea_orm::{ConnAcquireErr, DbErr, RuntimeErr};
    let outages = [
        DbErr::ConnectionAcquire(ConnAcquireErr::Timeout),
        DbErr::ConnectionAcquire(ConnAcquireErr::ConnectionClosed),
        DbErr::Conn(RuntimeErr::Internal("connection reset".to_owned())),
        DbErr::Custom("unclassified".to_owned()),
    ];
    for err in &outages {
        assert_eq!(
            classify_audit_error(err),
            AuditError::Unavailable,
            "{err:?} must be treated as an outage"
        );
    }
}
