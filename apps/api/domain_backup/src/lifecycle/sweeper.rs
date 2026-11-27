//! WU-19 sweeper (FN-20, FN-32, FN-33): the periodic backstop for the worker.
//!
//! One sweep runs, in this order and failing at the first error: (1) deadline enforcement
//! for current-epoch `running` jobs and the queued-age bound for `queued` jobs, (2) release
//! of `written` allocations at expiry, (3) retention purge through the injected
//! `RetentionStore`, (4) the discover-only namespace scan. Effects of earlier steps stay
//! durable when a later one fails. Orphan-grace promotion (FD 6.7 step 3) needs no step:
//! an `orphan_discovered` row past its grace is already claimable by the cleaner.
//!
//! Expiry alone never releases the allocation of a job that is still `running`.
use crate::domain::status::BackupJobStatus;
use crate::lifecycle::reconciler::{ReconcileError, Reconciler};
use crate::ports::allocation_store::{AllocationError, AllocationState, AllocationStore};
use crate::ports::job_store::{JobStore, ReasonCode, StoreError, TransitionResult};
use crate::ports::retention_store::RetentionStore;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::time::Duration;

/// What one sweep did (in-memory counts only).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SweepReport {
    /// Current-epoch `running` jobs failed `DEADLINE_EXCEEDED`.
    pub deadlines: usize,
    /// `queued` jobs older than the queued-age bound failed `DISPATCH_FAILED`.
    pub queued_expired: usize,
    /// `written` allocations released at expiry.
    pub expired: usize,
    /// `written` allocations released after their owner became terminal before expiry.
    pub terminal_released: usize,
    /// Jobs removed by the retention purge.
    pub purged: usize,
    /// Unowned files persisted as `orphan_discovered`.
    pub discovered_orphans: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SweepError {
    StoreUnavailable,
    AuditUnavailable,
    InventoryTruncated,
    StorageUnavailable,
}

fn from_store(error: StoreError) -> SweepError {
    match error {
        StoreError::AuditUnavailable => SweepError::AuditUnavailable,
        _ => SweepError::StoreUnavailable,
    }
}

fn from_allocation(error: AllocationError) -> SweepError {
    match error {
        AllocationError::AuditUnavailable => SweepError::AuditUnavailable,
        _ => SweepError::StoreUnavailable,
    }
}

fn from_reconcile(error: ReconcileError) -> SweepError {
    match error {
        ReconcileError::StoreUnavailable => SweepError::StoreUnavailable,
        ReconcileError::AuditUnavailable => SweepError::AuditUnavailable,
        ReconcileError::InventoryTruncated => SweepError::InventoryTruncated,
        ReconcileError::StorageUnavailable => SweepError::StorageUnavailable,
    }
}

/// Immutable sweeper configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SweeperConfig {
    /// A `queued` job not claimed within this age fails `DISPATCH_FAILED` (FD 7.1: CFG-10).
    pub queued_age: Duration,
}

impl Default for SweeperConfig {
    fn default() -> Self {
        Self {
            queued_age: Duration::from_secs(600),
        }
    }
}

pub struct Sweeper {
    jobs: Arc<dyn JobStore>,
    allocs: Arc<dyn AllocationStore>,
    retention: Arc<dyn RetentionStore>,
    reconciler: Arc<Reconciler>,
    config: SweeperConfig,
}

impl std::fmt::Debug for Sweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Sweeper").finish_non_exhaustive()
    }
}

impl Sweeper {
    pub fn new(
        jobs: Arc<dyn JobStore>,
        allocs: Arc<dyn AllocationStore>,
        retention: Arc<dyn RetentionStore>,
        reconciler: Arc<Reconciler>,
        config: SweeperConfig,
    ) -> Self {
        Self {
            jobs,
            allocs,
            retention,
            reconciler,
            config,
        }
    }

    /// One sweep at `epoch` and `now`: deadline enforcement, expiry release, retention
    /// purge, discover-only orphan scan, in that order.
    pub async fn sweep(&self, epoch: i64, now: DateTime<Utc>) -> Result<SweepReport, SweepError> {
        let mut report = SweepReport::default();
        self.enforce_deadlines(epoch, now, &mut report).await?;
        self.release_expired(now, &mut report).await?;
        report.purged = self
            .retention
            .purge_retained(now)
            .await
            .map_err(from_allocation)?
            .jobs_purged as usize;
        report.discovered_orphans = self
            .reconciler
            .discover_orphans(now)
            .await
            .map_err(from_reconcile)?;
        Ok(report)
    }

    async fn enforce_deadlines(
        &self,
        epoch: i64,
        now: DateTime<Utc>,
        report: &mut SweepReport,
    ) -> Result<(), SweepError> {
        let expired = self
            .jobs
            .list_deadline_expired(now, epoch)
            .await
            .map_err(from_store)?;
        for job in expired {
            let moved = self
                .jobs
                .transition(
                    job.id,
                    (BackupJobStatus::Running, job.state_version),
                    BackupJobStatus::Failed,
                    Some(ReasonCode::DeadlineExceeded),
                    None,
                )
                .await
                .map_err(from_store)?;
            if moved == TransitionResult::Applied {
                report.deadlines += 1;
            }
        }

        // A queued job nobody claimed would hold the concurrency-1 slot until restart.
        let queued_age = chrono::Duration::from_std(self.config.queued_age)
            .map_err(|_| SweepError::StoreUnavailable)?;
        let all = self
            .jobs
            .list_for_reconciliation()
            .await
            .map_err(from_store)?;
        for job in all
            .into_iter()
            .filter(|j| j.status == BackupJobStatus::Queued && j.created_at + queued_age <= now)
        {
            let moved = self
                .jobs
                .transition(
                    job.id,
                    (BackupJobStatus::Queued, job.state_version),
                    BackupJobStatus::Failed,
                    Some(ReasonCode::DispatchFailed),
                    None,
                )
                .await
                .map_err(from_store)?;
            if moved == TransitionResult::Applied {
                report.queued_expired += 1;
            }
        }
        Ok(())
    }

    async fn release_expired(
        &self,
        now: DateTime<Utc>,
        report: &mut SweepReport,
    ) -> Result<(), SweepError> {
        let rows = self
            .allocs
            .list_for_reconciliation()
            .await
            .map_err(from_allocation)?;
        for row in rows {
            let a = &row.allocation;
            // Expiry alone never releases active work.
            if a.state != AllocationState::Written
                || row.owner_status == Some(BackupJobStatus::Running)
                || (a.expires_at > now
                    && !matches!(row.owner_status, None | Some(BackupJobStatus::Failed)))
            {
                continue;
            }
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
                if a.expires_at <= now {
                    report.expired += 1;
                } else {
                    report.terminal_released += 1;
                }
            }
        }
        Ok(())
    }
}
