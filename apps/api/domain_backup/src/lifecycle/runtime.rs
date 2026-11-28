//! WU-19 runtime (FN-31, FN-32): start-up barrier, outage guard, recovery order and the
//! scheduler tick.
//!
//! Start-up order is fixed: namespace lock, durable epoch advance, complete reconcile and
//! fence, start-up cleanup, and only then admissions. Recovery after an outage is the same
//! without the epoch bump (the process did not restart) plus a sweep: reconcile, cleanup,
//! sweep, reopen. Any failure on either path leaves admissions closed. A store or storage
//! failure on a tick closes admissions at once; an audit refusal (healthy connection) is
//! counted and the third consecutive refused sweep closes them (FD 6.7).
//!
//! The runtime owns no timer. `run_tick` is one scheduler iteration driven by the injected
//! clock; the periodic driver that calls it belongs to the gateway composition (WU-10).
use crate::domain::clock::Clock;
use crate::lifecycle::cleaner::{CleanError, CleanOutcome, Cleaner};
use crate::lifecycle::reconciler::{ReconcileError, Reconciler};
use crate::lifecycle::sweeper::{SweepError, SweepReport, Sweeper};
use crate::ports::namespace_lock::{LockError, LockGuard};
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    /// Never started successfully; admissions closed.
    NotReady,
    /// Started, admissions open, no degraded signal.
    Ready,
    /// Started but admissions are closed (outage, failed recovery, audit-blocked bound) or
    /// a degraded signal is raised (truncated sweep scan).
    Degraded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeError {
    LockUnavailable,
    StoreUnavailable,
    AuditUnavailable,
    InventoryTruncated,
    StorageUnavailable,
    /// Recovery or a tick was requested before a successful start.
    NotStarted,
}

impl From<ReconcileError> for RuntimeError {
    fn from(error: ReconcileError) -> Self {
        match error {
            ReconcileError::StoreUnavailable => Self::StoreUnavailable,
            ReconcileError::AuditUnavailable => Self::AuditUnavailable,
            ReconcileError::InventoryTruncated => Self::InventoryTruncated,
            ReconcileError::StorageUnavailable => Self::StorageUnavailable,
        }
    }
}

impl From<CleanError> for RuntimeError {
    fn from(error: CleanError) -> Self {
        match error {
            CleanError::StoreUnavailable => Self::StoreUnavailable,
            CleanError::AuditUnavailable => Self::AuditUnavailable,
            CleanError::StorageUnavailable => Self::StorageUnavailable,
        }
    }
}

impl From<SweepError> for RuntimeError {
    fn from(error: SweepError) -> Self {
        match error {
            SweepError::StoreUnavailable => Self::StoreUnavailable,
            SweepError::AuditUnavailable => Self::AuditUnavailable,
            SweepError::InventoryTruncated => Self::InventoryTruncated,
            SweepError::StorageUnavailable => Self::StorageUnavailable,
        }
    }
}

/// Acquires the namespace lock for a root. Production passes
/// `FsNamespaceLock::try_acquire`; the port trait has an associated function, so it is
/// injected as a function.
pub type LockAcquirer = Arc<dyn Fn(&Path) -> Result<LockGuard, LockError> + Send + Sync>;

/// Immutable runtime configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub namespace_root: PathBuf,
    /// Sweeper period (CFG-07, default 900 s).
    pub sweep_every: Duration,
    /// Cleaner tick period (CFG-09, default 60 s).
    pub clean_every: Duration,
    /// Consecutive refused-audit sweeps before the runtime closes admissions (FD 6.7).
    pub audit_refusal_limit: u32,
}

impl RuntimeConfig {
    pub fn new(namespace_root: impl Into<PathBuf>) -> Self {
        Self {
            namespace_root: namespace_root.into(),
            sweep_every: Duration::from_secs(900),
            clean_every: Duration::from_secs(60),
            audit_refusal_limit: 3,
        }
    }
}

/// What one scheduler tick did.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TickReport {
    /// `Some` when the cleaner tick was due and ran.
    pub cleaned: Option<Vec<CleanOutcome>>,
    /// `Some` when the sweep was due and ran.
    pub swept: Option<SweepReport>,
}

#[derive(Debug, Default)]
struct Inner {
    started: bool,
    degraded: bool,
    epoch: Option<i64>,
    /// Held for the life of the runtime; dropping it releases the namespace lock.
    guard: Option<LockGuard>,
    last_sweep: Option<DateTime<Utc>>,
    last_clean: Option<DateTime<Utc>>,
    audit_refusals: u32,
}

pub struct Runtime {
    reconciler: Arc<Reconciler>,
    cleaner: Arc<Cleaner>,
    sweeper: Arc<Sweeper>,
    acquire: LockAcquirer,
    clock: Arc<dyn Clock>,
    config: RuntimeConfig,
    admissions: AtomicBool,
    inner: Mutex<Inner>,
}

impl std::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Runtime").finish_non_exhaustive()
    }
}

fn due(last: Option<DateTime<Utc>>, now: DateTime<Utc>, every: Duration) -> bool {
    match (last, chrono::Duration::from_std(every)) {
        (None, _) => true,
        (Some(last), Ok(every)) => now - last >= every,
        // An unrepresentable period never fires; it cannot be configured from the owner values.
        (Some(_), Err(_)) => false,
    }
}

impl Runtime {
    pub fn new(
        reconciler: Arc<Reconciler>,
        cleaner: Arc<Cleaner>,
        sweeper: Arc<Sweeper>,
        acquire: LockAcquirer,
        clock: Arc<dyn Clock>,
        config: RuntimeConfig,
    ) -> Self {
        Self {
            reconciler,
            cleaner,
            sweeper,
            acquire,
            clock,
            config,
            admissions: AtomicBool::new(false),
            inner: Mutex::new(Inner::default()),
        }
    }

    fn inner(&self) -> std::sync::MutexGuard<'_, Inner> {
        // A poisoned lock only means another thread panicked; the flags stay usable.
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    pub fn admissions_open(&self) -> bool {
        self.admissions.load(Ordering::SeqCst)
    }

    pub fn state(&self) -> RuntimeState {
        let inner = self.inner();
        if !inner.started {
            RuntimeState::NotReady
        } else if self.admissions_open() && !inner.degraded {
            RuntimeState::Ready
        } else {
            RuntimeState::Degraded
        }
    }

    /// The durable epoch of this run, once started.
    pub fn epoch(&self) -> Option<i64> {
        self.inner().epoch
    }

    /// Start-up: lock, epoch advance, complete reconcile, start-up cleanup, then admissions.
    /// A failure at any step leaves admissions closed, the runtime `NotReady` and the lock
    /// released.
    pub async fn start(&self) -> Result<(), RuntimeError> {
        if self.inner().started {
            return Ok(());
        }
        let guard = (self.acquire)(&self.config.namespace_root)
            .map_err(|_| RuntimeError::LockUnavailable)?;
        let now = self.clock.now();
        let epoch = self.reconciler.advance_epoch(now).await?;
        self.reconciler.reconcile(epoch, now).await?;
        self.cleaner.clean_due(now).await?;

        let mut inner = self.inner();
        inner.guard = Some(guard);
        inner.epoch = Some(epoch);
        inner.started = true;
        inner.degraded = false;
        inner.audit_refusals = 0;
        inner.last_sweep = None;
        inner.last_clean = None;
        drop(inner);
        self.admissions.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// A database or audit outage: close admissions and stop store work.
    pub async fn database_outage(&self) {
        self.admissions.store(false, Ordering::SeqCst);
    }

    /// First successful probe after an outage: reconcile (no epoch bump), cleanup, sweep,
    /// and only then reopen admissions. Any failure leaves admissions closed.
    pub async fn database_recovered(&self) -> Result<(), RuntimeError> {
        let epoch = self.inner().epoch.ok_or(RuntimeError::NotStarted)?;
        self.admissions.store(false, Ordering::SeqCst);
        let now = self.clock.now();
        self.reconciler.reconcile(epoch, now).await?;
        self.cleaner.clean_due(now).await?;
        self.sweeper.sweep(epoch, now).await?;

        let mut inner = self.inner();
        inner.degraded = false;
        inner.audit_refusals = 0;
        inner.last_sweep = Some(now);
        inner.last_clean = Some(now);
        drop(inner);
        self.admissions.store(true, Ordering::SeqCst);
        Ok(())
    }

    /// One scheduler iteration: the cleaner tick and the sweep, each when due. A closed
    /// runtime makes no store call.
    pub async fn run_tick(&self) -> Result<TickReport, RuntimeError> {
        let epoch = self.inner().epoch.ok_or(RuntimeError::NotStarted)?;
        if !self.admissions_open() {
            return Ok(TickReport::default());
        }
        let now = self.clock.now();
        let (clean_due, sweep_due) = {
            let inner = self.inner();
            (
                due(inner.last_clean, now, self.config.clean_every),
                due(inner.last_sweep, now, self.config.sweep_every),
            )
        };
        let mut report = TickReport::default();
        if clean_due {
            match self.cleaner.clean_due(now).await {
                Ok(outcomes) => {
                    self.inner().last_clean = Some(now);
                    report.cleaned = Some(outcomes);
                }
                Err(error) => return Err(self.failed(error.into(), false)),
            }
        }
        if sweep_due {
            match self.sweeper.sweep(epoch, now).await {
                Ok(swept) => {
                    let mut inner = self.inner();
                    inner.last_sweep = Some(now);
                    inner.audit_refusals = 0;
                    inner.degraded = false;
                    report.swept = Some(swept);
                }
                Err(error) => return Err(self.failed(error.into(), true)),
            }
        }
        Ok(report)
    }

    /// Apply the outage rules to a failed tick step and hand the error back.
    fn failed(&self, error: RuntimeError, is_sweep: bool) -> RuntimeError {
        match error {
            RuntimeError::StoreUnavailable | RuntimeError::StorageUnavailable => {
                self.admissions.store(false, Ordering::SeqCst);
            }
            RuntimeError::AuditUnavailable if is_sweep => {
                let mut inner = self.inner();
                inner.audit_refusals += 1;
                if inner.audit_refusals >= self.config.audit_refusal_limit {
                    drop(inner);
                    self.admissions.store(false, Ordering::SeqCst);
                }
            }
            RuntimeError::InventoryTruncated => {
                self.inner().degraded = true;
            }
            _ => {}
        }
        error
    }
}
