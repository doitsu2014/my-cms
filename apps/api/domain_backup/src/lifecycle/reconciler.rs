//! WU-17 reconciler (FN-25..FN-28): epoch advance, job fencing, allocation and
//! namespace reconciliation, orphan discovery.
//!
//! The reconciler reads three complete inventories (jobs, allocations joined to their
//! owners, the bounded namespace listing) and acts on them. It never unlinks a file:
//! removal is the cleaner's audited claim, unlink, audited delete (WU-18). A truncated
//! namespace listing aborts the run before the first write, so a prefix of the namespace
//! can never be mistaken for all of it (TS-56).
use crate::domain::status::BackupJobStatus;
use crate::ports::allocation_store::{
    AllocationError, AllocationRecoveryRow, AllocationState, AllocationStore,
};
use crate::ports::audit::{AuditError, AuditEvent, AuditEventKind, AuditOutcome, AuditSink};
use crate::ports::fixture_storage::{
    classify_entry, EntryKind, FixtureStorage, InventoryPage, Ownership, StorageError, StorageKey,
    MAX_INVENTORY_ENTRIES,
};
use crate::ports::job_store::{JobRow, JobStore, ReasonCode, StoreError, TransitionResult};
use chrono::{DateTime, Utc};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

const OPERATION: &str = "reconciliation";

/// What one complete reconciliation did. Counts live in memory only (no persistent
/// counter, IFC 6.1a).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReconcileReport {
    /// Old-epoch jobs moved to `failed` by an applied fence CAS.
    pub fenced_jobs: usize,
    /// Unowned valid-name files persisted as `orphan_discovered`.
    pub discovered_orphans: usize,
    /// Unexpired `written` allocations of an `incomplete` job whose file is absent.
    pub missing_files: usize,
    /// Applied allocation CAS moves to `release_pending`.
    pub released_allocations: usize,
    /// Stale old-epoch `deleting` claims returned to `release_pending`.
    pub reset_claims: usize,
    /// Namespace entries that are never deleted and raise an ownership incident.
    pub ambiguous_entries: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconcileError {
    StoreUnavailable,
    AuditUnavailable,
    InventoryTruncated,
    StorageUnavailable,
}

/// Immutable reconciler configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReconcilerConfig {
    /// The configured instance id written on audit rows the reconciler appends.
    pub instance_id: String,
    /// Bound passed to `FixtureStorage::inventory` (IFC section 9, T-13).
    pub inventory_cap: usize,
}

impl ReconcilerConfig {
    pub fn new(instance_id: impl Into<String>) -> Self {
        Self {
            instance_id: instance_id.into(),
            inventory_cap: MAX_INVENTORY_ENTRIES,
        }
    }

    pub fn with_inventory_cap(mut self, cap: usize) -> Self {
        self.inventory_cap = cap;
        self
    }
}

fn from_store(error: StoreError) -> ReconcileError {
    match error {
        StoreError::AuditUnavailable => ReconcileError::AuditUnavailable,
        _ => ReconcileError::StoreUnavailable,
    }
}

fn from_allocation(error: AllocationError) -> ReconcileError {
    match error {
        AllocationError::AuditUnavailable => ReconcileError::AuditUnavailable,
        _ => ReconcileError::StoreUnavailable,
    }
}

fn from_audit(error: AuditError) -> ReconcileError {
    match error {
        AuditError::Rejected => ReconcileError::AuditUnavailable,
        AuditError::Unavailable => ReconcileError::StoreUnavailable,
    }
}

fn from_storage(error: StorageError) -> ReconcileError {
    match error {
        StorageError::LedgerUnavailable => ReconcileError::StoreUnavailable,
        _ => ReconcileError::StorageUnavailable,
    }
}

pub struct Reconciler {
    jobs: Arc<dyn JobStore>,
    allocs: Arc<dyn AllocationStore>,
    storage: Arc<dyn FixtureStorage>,
    audit: Arc<dyn AuditSink>,
    config: ReconcilerConfig,
}

impl std::fmt::Debug for Reconciler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reconciler").finish_non_exhaustive()
    }
}

impl Reconciler {
    pub fn new(
        jobs: Arc<dyn JobStore>,
        allocs: Arc<dyn AllocationStore>,
        storage: Arc<dyn FixtureStorage>,
        audit: Arc<dyn AuditSink>,
        config: ReconcilerConfig,
    ) -> Self {
        Self {
            jobs,
            allocs,
            storage,
            audit,
            config,
        }
    }

    /// Atomically advance the durable lifecycle epoch and return it (FN-25).
    pub async fn advance_epoch(&self, now: DateTime<Utc>) -> Result<i64, ReconcileError> {
        self.jobs.advance_epoch(now).await.map_err(from_store)
    }

    /// Full reconciliation at `epoch` (startup and DB recovery): fence older-epoch jobs,
    /// reconcile allocations, persist orphans. Never unlinks a file (FN-26, FN-27).
    pub async fn reconcile(
        &self,
        epoch: i64,
        now: DateTime<Utc>,
    ) -> Result<ReconcileReport, ReconcileError> {
        // Namespace first: a truncated listing aborts before any read or write that
        // could act on a partial view.
        let page = self.complete_inventory().await?;
        let jobs = self
            .jobs
            .list_for_reconciliation()
            .await
            .map_err(from_store)?;
        let mut report = ReconcileReport::default();

        for job in &jobs {
            if job.epoch >= epoch {
                continue;
            }
            let (target, reason) = match job.status {
                BackupJobStatus::Running => {
                    (BackupJobStatus::Failed, ReasonCode::InterruptedRestart)
                }
                BackupJobStatus::Queued => (BackupJobStatus::Failed, ReasonCode::AbandonedQueued),
                _ => continue,
            };
            let fenced = self
                .jobs
                .fence_job(
                    job.id,
                    job.epoch,
                    job.status,
                    job.state_version,
                    target,
                    reason,
                    now,
                )
                .await
                .map_err(from_store)?;
            if fenced == TransitionResult::Applied {
                report.fenced_jobs += 1;
            }
        }

        // A fresh view, so owners fenced above read as `failed`.
        let rows = self
            .allocs
            .list_for_reconciliation()
            .await
            .map_err(from_allocation)?;
        let present: HashSet<String> = page
            .entries
            .iter()
            .filter(|e| e.kind == EntryKind::RegularFile)
            .map(|e| e.name.clone())
            .collect();
        let owners: HashMap<Uuid, &JobRow> = jobs.iter().map(|j| (j.id, j)).collect();
        for row in &rows {
            self.reconcile_allocation(row, epoch, now, &present, &owners, &mut report)
                .await?;
        }

        let ledger = ledger_states(&rows);
        for entry in &page.entries {
            match classify_entry(entry, ledger.get(&entry.name).copied()) {
                Ownership::Owned | Ownership::OwnedInCleanup => {}
                Ownership::Orphan => {
                    self.persist_orphan(&entry.name, entry.size_bytes, now)
                        .await?;
                    report.discovered_orphans += 1;
                }
                Ownership::Ambiguous => {
                    self.raise_incident(&entry.name, now).await?;
                    report.ambiguous_entries += 1;
                }
            }
        }
        Ok(report)
    }

    /// Discover-only scan shared with the periodic sweep (FN-28, FN-32): persists unowned
    /// valid-name files as `orphan_discovered`, never fences or deletes. Returns the number
    /// of orphans persisted.
    pub async fn discover_orphans(&self, now: DateTime<Utc>) -> Result<usize, ReconcileError> {
        let page = self.complete_inventory().await?;
        let rows = self
            .allocs
            .list_for_reconciliation()
            .await
            .map_err(from_allocation)?;
        let ledger = ledger_states(&rows);
        let mut found = 0;
        for entry in &page.entries {
            if classify_entry(entry, ledger.get(&entry.name).copied()) == Ownership::Orphan {
                self.persist_orphan(&entry.name, entry.size_bytes, now)
                    .await?;
                found += 1;
            }
        }
        Ok(found)
    }

    /// The bounded listing, or `InventoryTruncated` when it was cut (fail closed).
    async fn complete_inventory(&self) -> Result<InventoryPage, ReconcileError> {
        let page = self
            .storage
            .inventory(self.config.inventory_cap)
            .await
            .map_err(from_storage)?;
        if page.truncated {
            return Err(ReconcileError::InventoryTruncated);
        }
        Ok(page)
    }

    async fn reconcile_allocation(
        &self,
        row: &AllocationRecoveryRow,
        epoch: i64,
        now: DateTime<Utc>,
        present: &HashSet<String>,
        owners: &HashMap<Uuid, &JobRow>,
        report: &mut ReconcileReport,
    ) -> Result<(), ReconcileError> {
        let a = &row.allocation;
        // Any `running` owner (including one whose fence was lost to a concurrent writer)
        // keeps its allocation: expiry alone never releases active work.
        let owner_running = row.owner_status == Some(BackupJobStatus::Running);
        match a.state {
            AllocationState::Registered if !owner_running => self.release(row, now, report).await,
            AllocationState::Written if !owner_running => {
                let owner_failed_or_absent =
                    matches!(row.owner_status, None | Some(BackupJobStatus::Failed));
                if a.expires_at <= now || owner_failed_or_absent {
                    return self.release(row, now, report).await;
                }
                let file_absent = !present.contains(&format!("{}.bin", a.id.hyphenated()));
                if file_absent && row.owner_status == Some(BackupJobStatus::Incomplete) {
                    self.note_missing_file(row, owners, now).await?;
                    report.missing_files += 1;
                }
                Ok(())
            }
            AllocationState::Deleting if a.epoch < epoch => {
                // A crashed claim is not an unlink failure: attempts stay as they are.
                let moved = self
                    .allocs
                    .reconcile_state(
                        a.id,
                        (a.state, a.state_version, a.epoch),
                        AllocationState::ReleasePending,
                        now,
                    )
                    .await
                    .map_err(from_allocation)?;
                if moved == TransitionResult::Applied {
                    report.reset_claims += 1;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    async fn release(
        &self,
        row: &AllocationRecoveryRow,
        now: DateTime<Utc>,
        report: &mut ReconcileReport,
    ) -> Result<(), ReconcileError> {
        let a = &row.allocation;
        let moved = self
            .allocs
            .reconcile_state(
                a.id,
                (a.state, a.state_version, a.epoch),
                AllocationState::ReleasePending,
                now,
            )
            .await
            .map_err(from_allocation)?;
        if moved == TransitionResult::Applied {
            report.released_allocations += 1;
        }
        Ok(())
    }

    /// Audit-only observation (IFC 6.1a): no state change, appended on every full scan.
    async fn note_missing_file(
        &self,
        row: &AllocationRecoveryRow,
        owners: &HashMap<Uuid, &JobRow>,
        now: DateTime<Utc>,
    ) -> Result<(), ReconcileError> {
        let a = &row.allocation;
        let correlation_id = a
            .job_id
            .and_then(|id| owners.get(&id))
            .map(|job| job.correlation_id.clone())
            .unwrap_or_else(|| a.id.to_string());
        self.audit
            .record(
                None,
                AuditEvent {
                    occurred_at: now,
                    actor_sub: None,
                    instance_id: self.config.instance_id.clone(),
                    job_id: a.job_id,
                    allocation_id: Some(a.id),
                    operation: OPERATION.to_owned(),
                    event: AuditEventKind::ReconcileMissingFile,
                    outcome: AuditOutcome::Success,
                    reason_code: Some("MISSING_FILE".to_owned()),
                    digest: None,
                    correlation_id,
                },
            )
            .await
            .map_err(from_audit)
    }

    async fn persist_orphan(
        &self,
        name: &str,
        size_bytes: u64,
        now: DateTime<Utc>,
    ) -> Result<(), ReconcileError> {
        let key = StorageKey::parse(&format!("alloc/{name}"))
            .map_err(|_| ReconcileError::StorageUnavailable)?;
        // `bytes_reserved` must be positive; an empty file still reserves one byte.
        self.allocs
            .insert_orphan(key.allocation_id(), &key, size_bytes.max(1), now)
            .await
            .map_err(from_allocation)
    }

    /// An entry that is never deleted. The audit row carries the allocation id when the
    /// name is a canonical key and no path, name or file data.
    async fn raise_incident(&self, name: &str, now: DateTime<Utc>) -> Result<(), ReconcileError> {
        let allocation_id = StorageKey::parse(&format!("alloc/{name}"))
            .ok()
            .map(|key| key.allocation_id());
        self.audit
            .record(
                None,
                AuditEvent {
                    occurred_at: now,
                    actor_sub: None,
                    instance_id: self.config.instance_id.clone(),
                    job_id: None,
                    allocation_id,
                    operation: OPERATION.to_owned(),
                    event: AuditEventKind::OwnershipAmbiguous,
                    outcome: AuditOutcome::Failure,
                    reason_code: None,
                    digest: None,
                    correlation_id: allocation_id.unwrap_or_else(Uuid::new_v4).to_string(),
                },
            )
            .await
            .map_err(from_audit)
    }
}

/// Ledger state keyed by the file name each allocation id maps to.
fn ledger_states(rows: &[AllocationRecoveryRow]) -> HashMap<String, AllocationState> {
    rows.iter()
        .map(|r| {
            (
                format!("{}.bin", r.allocation.id.hyphenated()),
                r.allocation.state,
            )
        })
        .collect()
}
