//! WU-18 cleaner (FN-29..FN-30): audited delete claim, unlink, audited `deleted` commit,
//! bounded retry and exhaustion.
//!
//! Every unlink, including a retry and a recovery after a crash, is preceded by its own
//! claim, and the claim's audit row commits in the claim's transaction. If that audit is
//! refused there is no `deleting` row and no unlink. The allocation store writes
//! `allocation.delete_claimed`, `allocation.deleted` and `cleanup.exhausted` in the same
//! transaction as the state change each one records, so the cleaner holds no audit sink.
use crate::ports::allocation_store::{
    AllocationError, AllocationState, AllocationStore, CleanupFailureOutcome,
};
use crate::ports::fixture_storage::FixtureStorage;
use crate::ports::job_store::ReasonCode;
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanOutcome {
    /// The file is gone (a missing file counts) and `deleted` is committed.
    Deleted,
    /// The unlink failed; the next attempt is scheduled after the configured backoff.
    RetryScheduled,
    /// The unlink failed for the last allowed attempt; the row is `cleanup_failed`.
    Exhausted,
    /// The row exists but its backoff or grace has not ended: nothing was claimed.
    NotDue,
    /// Unknown id, a state the cleaner does not claim, or a lost compare-and-set.
    Lost,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CleanError {
    StoreUnavailable,
    AuditUnavailable,
    StorageUnavailable,
}

fn from_allocation(error: AllocationError) -> CleanError {
    match error {
        AllocationError::AuditUnavailable => CleanError::AuditUnavailable,
        _ => CleanError::StoreUnavailable,
    }
}

pub struct Cleaner {
    allocs: Arc<dyn AllocationStore>,
    storage: Arc<dyn FixtureStorage>,
}

impl std::fmt::Debug for Cleaner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cleaner").finish_non_exhaustive()
    }
}

impl Cleaner {
    pub fn new(allocs: Arc<dyn AllocationStore>, storage: Arc<dyn FixtureStorage>) -> Self {
        Self { allocs, storage }
    }

    /// Clean every allocation due at `now`, oldest first, one at a time. The first error
    /// stops the batch and is returned; outcomes already reached are durable.
    pub async fn clean_due(&self, now: DateTime<Utc>) -> Result<Vec<CleanOutcome>, CleanError> {
        let due = self
            .allocs
            .due_for_cleanup(now)
            .await
            .map_err(from_allocation)?;
        let mut outcomes = Vec::with_capacity(due.len());
        for allocation in due {
            outcomes.push(self.clean_one(allocation.id, now).await?);
        }
        Ok(outcomes)
    }

    /// Claim, unlink and commit one allocation (FN-29, FN-30).
    pub async fn clean_one(
        &self,
        id: uuid::Uuid,
        now: DateTime<Utc>,
    ) -> Result<CleanOutcome, CleanError> {
        let Some(row) = self.allocs.get(id).await.map_err(from_allocation)? else {
            return Ok(CleanOutcome::Lost);
        };
        match row.state {
            AllocationState::ReleasePending => {
                if row.next_attempt_at.is_some_and(|at| at > now) {
                    return Ok(CleanOutcome::NotDue);
                }
            }
            AllocationState::OrphanDiscovered => {
                if !row.orphan_grace_until.is_some_and(|until| until <= now) {
                    return Ok(CleanOutcome::NotDue);
                }
            }
            _ => return Ok(CleanOutcome::Lost),
        }

        // The audited claim commits before anything is unlinked.
        let Some(claimed) = self
            .allocs
            .claim_for_delete(id, row.state_version)
            .await
            .map_err(from_allocation)?
        else {
            return Ok(CleanOutcome::Lost);
        };

        match self.storage.delete(&claimed.storage_key).await {
            Ok(_) => match self.allocs.mark_deleted(id, claimed.state_version).await {
                Ok(()) => Ok(CleanOutcome::Deleted),
                Err(AllocationError::Lost) => Ok(CleanOutcome::Lost),
                // The unlink may have happened; the row stays `deleting` and recovery
                // retries idempotently under a fresh audited claim.
                Err(error) => Err(from_allocation(error)),
            },
            Err(_) => {
                match self
                    .allocs
                    .record_cleanup_failure(
                        id,
                        claimed.state_version,
                        ReasonCode::AdapterError,
                        now,
                    )
                    .await
                {
                    Ok(CleanupFailureOutcome::Retry) => Ok(CleanOutcome::RetryScheduled),
                    Ok(CleanupFailureOutcome::Exhausted) => Ok(CleanOutcome::Exhausted),
                    Err(AllocationError::Lost) => Ok(CleanOutcome::Lost),
                    Err(error) => Err(from_allocation(error)),
                }
            }
        }
    }
}
