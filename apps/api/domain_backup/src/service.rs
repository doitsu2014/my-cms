//! `DomainBackupService`: domain-owned config, migrations and administrator REST routes (FN-37).

use std::sync::{Arc, OnceLock};
use std::time::Duration;

use async_trait::async_trait;
use domain_interface::{
    DomainConfigError, DomainContext, DomainService, HealthDescriptor, MigrationDescriptor, Mount,
    RouteRegistration,
};
use sea_orm::DatabaseConnection;

use crate::domain::clock::{Clock, SystemClock};
use crate::domain::config::{BackupConfig, BackupConfigState};
use crate::domain::instance::InstanceRegistry;
use crate::domain::policy::BackupPolicy;
use crate::handlers::http::{routes, BackupApiState};
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::infra::db_retention_store::DbRetentionStore;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::infra::fs_namespace_lock::FsNamespaceLock;
use crate::lifecycle::cleaner::Cleaner;
use crate::lifecycle::fixture_capture::FixtureCaptureAdapter;
use crate::lifecycle::reconciler::{Reconciler, ReconcilerConfig};
use crate::lifecycle::runtime::{Runtime, RuntimeConfig, RuntimeState};
use crate::lifecycle::sweeper::{Sweeper, SweeperConfig};
use crate::lifecycle::worker::{RunOutcome, Worker, WorkerConfig};
use crate::ports::allocation_store::{AllocationLimits, AllocationStore};
use crate::ports::audit::AuditSink;
use crate::ports::fixture_storage::{FixtureStorage, StorageError};
use crate::ports::job_store::JobStore;
use crate::ports::namespace_lock::NamespaceLock;
use crate::ports::retention_store::RetentionStore;

/// Build the production runtime (real stores, real fixture storage, real namespace lock)
/// from a validated, enabled configuration. Nothing is started: `Runtime::start` runs after
/// the migrations (F-37).
pub fn build_runtime(
    conn: &DatabaseConnection,
    config: &BackupConfig,
) -> Result<Arc<Runtime>, StorageError> {
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    let bounds = config.bounds();
    let audit: Arc<dyn AuditSink> = Arc::new(DbAuditSink::new(conn.clone()));
    let jobs: Arc<dyn JobStore> =
        Arc::new(DbJobStore::new(conn.clone(), audit.clone(), clock.clone()));
    let allocs: Arc<dyn AllocationStore> = Arc::new(DbAllocationStore::new(
        conn.clone(),
        audit.clone(),
        clock.clone(),
        config.instance_id(),
        AllocationLimits {
            alloc_max_bytes: bounds.alloc_max_bytes(),
            total_max_bytes: bounds.total_max_bytes(),
            cleanup_max_attempts: i32::try_from(bounds.cleanup_max_attempts()).unwrap_or(i32::MAX),
            cleanup_backoff: Duration::from_secs(bounds.cleanup_backoff_secs()),
            orphan_grace: Duration::from_secs(bounds.orphan_grace_secs()),
        },
    ));
    let storage: Arc<dyn FixtureStorage> = Arc::new(FsFixtureStorage::open(
        config.storage_root(),
        allocs.clone(),
    )?);
    let retention: Arc<dyn RetentionStore> = Arc::new(DbRetentionStore::new(
        conn.clone(),
        audit.clone(),
        config.instance_id(),
    ));
    let reconciler = Arc::new(Reconciler::new(
        jobs.clone(),
        allocs.clone(),
        storage.clone(),
        audit,
        ReconcilerConfig::new(config.instance_id()),
    ));
    let cleaner = Arc::new(Cleaner::new(allocs.clone(), storage));
    let sweeper = Arc::new(Sweeper::new(
        jobs,
        allocs,
        retention,
        reconciler.clone(),
        SweeperConfig {
            queued_age: Duration::from_secs(bounds.job_deadline_secs()),
        },
    ));
    let mut runtime_config = RuntimeConfig::new(config.storage_root());
    runtime_config.sweep_every = Duration::from_secs(bounds.sweep_interval_secs());
    runtime_config.clean_every = Duration::from_secs(bounds.cleanup_backoff_secs());
    Ok(Arc::new(Runtime::new(
        reconciler,
        cleaner,
        sweeper,
        Arc::new(|root: &std::path::Path| FsNamespaceLock::try_acquire(root)),
        clock,
        runtime_config,
    )))
}

/// Upper bound on jobs one `worker_step` runs, so a step always returns.
const MAX_JOBS_PER_WORKER_STEP: usize = 16;

/// Build the production worker (F-16) for a started runtime's epoch: real job and allocation
/// stores, the synthetic fixture adapter (the only capture adapter, TS-10) and the configured
/// instance as claim policy. Deadline is CFG-10, allocation TTL is CFG-05.
pub fn build_worker(
    conn: &DatabaseConnection,
    config: &BackupConfig,
    epoch: i64,
) -> Result<Worker, StorageError> {
    let clock: Arc<dyn Clock> = Arc::new(SystemClock);
    let bounds = config.bounds();
    let audit: Arc<dyn AuditSink> = Arc::new(DbAuditSink::new(conn.clone()));
    let jobs: Arc<dyn JobStore> =
        Arc::new(DbJobStore::new(conn.clone(), audit.clone(), clock.clone()));
    let allocs: Arc<dyn AllocationStore> = Arc::new(DbAllocationStore::new(
        conn.clone(),
        audit,
        clock.clone(),
        config.instance_id(),
        AllocationLimits {
            alloc_max_bytes: bounds.alloc_max_bytes(),
            total_max_bytes: bounds.total_max_bytes(),
            cleanup_max_attempts: i32::try_from(bounds.cleanup_max_attempts()).unwrap_or(i32::MAX),
            cleanup_backoff: Duration::from_secs(bounds.cleanup_backoff_secs()),
            orphan_grace: Duration::from_secs(bounds.orphan_grace_secs()),
        },
    ));
    let storage: Arc<dyn FixtureStorage> = Arc::new(FsFixtureStorage::open(
        config.storage_root(),
        allocs.clone(),
    )?);
    let mut worker_config = WorkerConfig::new(epoch);
    worker_config.deadline = Duration::from_secs(bounds.job_deadline_secs());
    worker_config.allocation_ttl = Duration::from_secs(bounds.staging_expiry_secs());
    Ok(Worker::new(
        jobs,
        allocs,
        Arc::new(FixtureCaptureAdapter::new(storage)),
        Arc::new(BackupPolicy::new(InstanceRegistry::from_config(config))),
        clock,
        worker_config,
    ))
}

/// The service is cheap to clone; clones share the runtime slot, so the gateway can keep a
/// handle for the scheduler while the manifest owns another.
#[derive(Debug, Clone)]
pub struct DomainBackupService {
    config: BackupConfigState,
    runtime: Arc<OnceLock<Arc<Runtime>>>,
    worker: Arc<OnceLock<Arc<Worker>>>,
}

impl DomainBackupService {
    pub fn from_env() -> Self {
        Self::new(BackupConfig::from_env())
    }

    pub fn new(config: BackupConfigState) -> Self {
        Self {
            config,
            runtime: Arc::new(OnceLock::new()),
            worker: Arc::new(OnceLock::new()),
        }
    }

    pub fn config(&self) -> &BackupConfigState {
        &self.config
    }

    /// The runtime built by `startup_health` (started, or left not-ready when its start
    /// failed); `None` for a disabled backup or before `startup_health`.
    pub fn runtime(&self) -> Option<Arc<Runtime>> {
        self.runtime.get().cloned()
    }

    /// Run queued jobs through the worker (F-16) one at a time while admissions are open;
    /// returns how many jobs it ran. One driver task calls this, so at most one job runs at a
    /// time (concurrency 1). A refused claim (store or audit failure) ends the step; the job
    /// stays queued and the runtime's outage rules apply.
    pub async fn worker_step(&self) -> usize {
        let (Some(runtime), Some(worker)) = (self.runtime.get(), self.worker.get()) else {
            return 0;
        };
        let mut ran = 0;
        while ran < MAX_JOBS_PER_WORKER_STEP && runtime.admissions_open() {
            match worker.run_next().await {
                Ok(RunOutcome::Idle) => break,
                Ok(_) => ran += 1,
                Err(error) => {
                    tracing::warn!(?error, "backup worker claim refused; job stays queued");
                    break;
                }
            }
        }
        ran
    }

    /// One scheduler iteration, driven by the gateway: a closed runtime (an outage) runs the
    /// recovery order, an open one runs its due tick. A runtime that never started does
    /// nothing. Failures are logged and leave the runtime's own outage rules in charge.
    pub async fn scheduler_step(&self) {
        let Some(runtime) = self.runtime.get() else {
            return;
        };
        if runtime.state() == RuntimeState::NotReady {
            return;
        }
        if !runtime.admissions_open() {
            if let Err(error) = runtime.database_recovered().await {
                tracing::warn!(
                    ?error,
                    "backup recovery attempt failed; admissions stay closed"
                );
            }
        } else if let Err(error) = runtime.run_tick().await {
            tracing::warn!(?error, "backup scheduler tick failed");
        }
    }
}

#[async_trait]
impl DomainService for DomainBackupService {
    fn health(&self) -> HealthDescriptor {
        HealthDescriptor {
            name: "domain-backup",
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    fn required_env(&self) -> &'static [&'static str] {
        &[]
    }

    fn validate_config(&self) -> Result<(), DomainConfigError> {
        Ok(())
    }

    fn migrations(&self) -> Vec<MigrationDescriptor> {
        crate::migrations::migration_descriptors()
    }

    fn register_routes(&self, ctx: &DomainContext) -> Vec<RouteRegistration> {
        let mut state = BackupApiState::new(ctx.conn.as_ref().clone(), self.config.clone());
        if let Some(runtime) = self.runtime.get() {
            state = state.with_runtime(runtime.clone());
        }
        vec![RouteRegistration {
            mount: Mount::AdministratorIngress,
            router: routes(state),
            prefix: "/backup",
        }]
    }

    /// Runs after the gateway applied the migrations. An enabled backup builds and starts its
    /// runtime here (lock, epoch, reconcile, start-up cleanup, then admissions). A failure
    /// never aborts the gateway: the runtime stays not-ready and the API answers
    /// `BACKUP_NOT_READY`.
    async fn startup_health(&self, ctx: &DomainContext) -> Result<(), DomainConfigError> {
        let BackupConfigState::Enabled(config) = &self.config else {
            return Ok(());
        };
        if self.runtime.get().is_some() {
            return Ok(());
        }
        let runtime = match build_runtime(ctx.conn.as_ref(), config) {
            Ok(runtime) => runtime,
            Err(error) => {
                tracing::warn!(?error, "backup runtime not built; backup stays not ready");
                return Ok(());
            }
        };
        match runtime.start().await {
            Ok(()) => {
                tracing::info!("backup runtime started");
                match runtime
                    .epoch()
                    .map(|epoch| build_worker(ctx.conn.as_ref(), config, epoch))
                {
                    Some(Ok(worker)) => {
                        let _ = self.worker.set(Arc::new(worker));
                    }
                    Some(Err(error)) => {
                        tracing::warn!(
                            ?error,
                            "backup worker not built; queued jobs fail DISPATCH_FAILED"
                        )
                    }
                    None => tracing::warn!("started backup runtime has no epoch; worker not built"),
                }
            }
            Err(error) => {
                tracing::warn!(
                    ?error,
                    "backup runtime did not start; backup stays not ready"
                )
            }
        }
        let _ = self.runtime.set(runtime);
        Ok(())
    }

    async fn run_migrations(
        &self,
        conn: &DatabaseConnection,
        descriptors: &[MigrationDescriptor],
    ) -> Result<(), DomainConfigError> {
        if descriptors.is_empty() {
            return Ok(());
        }
        crate::migrations_cli::run(conn)
            .await
            .map_err(|error| DomainConfigError::MigrationExecution(error.to_string()))
    }
}
