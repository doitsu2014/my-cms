//! Safe projection of persisted jobs to the U1 REST representation (FN-10, TS-12b, TS-49).

use serde::Serialize;

use crate::domain::status::{project_status, BackupJobStatus, JobKind};
use crate::ports::job_store::{ComponentName, ComponentOutcome, Components, JobRow};

/// Public, stable representation of one backup job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct JobResponse {
    pub id: String,
    pub instance_id: String,
    pub instance_label: String,
    pub operation: String,
    pub status: BackupJobStatus,
    pub reason_code: Option<String>,
    pub synthetic: bool,
    pub download_eligible: bool,
    pub components: Components,
    pub progress: Option<MeasuredProgress>,
    pub correlation_id: String,
    pub created_at: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
}

/// Progress is exposed only when the persisted components include an actual measurement.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MeasuredProgress {
    pub completed: u64,
    pub total: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    UnreachableStatus(BackupJobStatus),
}

impl JobResponse {
    /// Convert the store row without exposing actor identity, storage paths or database detail.
    pub fn from_row(row: &JobRow, instance_label: &str) -> Result<Self, ProjectionError> {
        let status = project_status(JobKind::BackupDbSlice, row.status)
            .map_err(|refused| ProjectionError::UnreachableStatus(refused.0))?;
        Ok(Self {
            id: row.id.to_string(),
            instance_id: row.instance_id.clone(),
            instance_label: instance_label.to_owned(),
            operation: row.operation.clone(),
            status,
            reason_code: row.reason_code.clone(),
            synthetic: true,
            download_eligible: false,
            components: row.components.clone(),
            progress: measured_progress(&row.components),
            correlation_id: row.correlation_id.clone(),
            created_at: row.created_at.to_rfc3339(),
            started_at: row.started_at.map(|at| at.to_rfc3339()),
            finished_at: row.finished_at.map(|at| at.to_rfc3339()),
        })
    }
}

fn measured_progress(components: &Components) -> Option<MeasuredProgress> {
    let completed = components
        .0
        .iter()
        .any(|component| {
            component.name == ComponentName::Database
                && component.state == ComponentOutcome::SyntheticCaptured
                && component.reason_code.as_deref() == Some("DB_SYNTHETIC_ONLY")
        })
        .then_some(1)?;
    Some(MeasuredProgress {
        completed,
        total: 1,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    fn row(status: BackupJobStatus) -> JobRow {
        JobRow {
            id: Uuid::from_u128(1),
            actor_sub: "must-not-be-projected".to_owned(),
            instance_id: "local-1".to_owned(),
            operation: "backup_db_slice".to_owned(),
            synthetic: true,
            status,
            state_version: 4,
            reason_code: None,
            components: Components::default(),
            epoch: 1,
            correlation_id: "corr-1".to_owned(),
            created_at: Utc.timestamp_opt(1_800_000_000, 0).unwrap(),
            updated_at: Utc.timestamp_opt(1_800_000_001, 0).unwrap(),
            started_at: None,
            finished_at: None,
            deadline_at: None,
            retain_until: Utc.timestamp_opt(1_900_000_000, 0).unwrap(),
        }
    }

    #[test]
    fn ts49_projection_omits_actor_and_unmeasured_progress_and_denies_download() {
        let projected = JobResponse::from_row(&row(BackupJobStatus::Queued), "Local").unwrap();
        let json = serde_json::to_value(projected).unwrap();
        assert_eq!(json["progress"], serde_json::Value::Null);
        assert_eq!(json["download_eligible"], false);
        assert_eq!(json["status"], "queued");
        assert!(json.get("actor_sub").is_none());
        assert!(json.get("state_version").is_none());
        assert!(json.get("deadline_at").is_none());
    }

    #[test]
    fn ts12b_projection_refuses_success_and_recovery_required() {
        for status in [
            BackupJobStatus::Succeeded,
            BackupJobStatus::RecoveryRequired,
        ] {
            assert_eq!(
                JobResponse::from_row(&row(status), "Local"),
                Err(ProjectionError::UnreachableStatus(status))
            );
        }
    }

    #[test]
    fn ts49_projection_reports_progress_only_for_a_measured_database_component() {
        let mut source = row(BackupJobStatus::Incomplete);
        source
            .components
            .0
            .push(crate::ports::job_store::ComponentState {
                name: ComponentName::Database,
                state: ComponentOutcome::SyntheticCaptured,
                reason_code: Some("DB_SYNTHETIC_ONLY".to_owned()),
            });
        let projected = JobResponse::from_row(&source, "Local").unwrap();
        assert_eq!(
            projected.progress,
            Some(MeasuredProgress {
                completed: 1,
                total: 1
            })
        );
        assert!(!projected.download_eligible);
    }
}
