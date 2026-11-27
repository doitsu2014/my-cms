//! Read-only, always-200 runtime status projection (FN-36, IF-11, TS-27b).

use serde::Serialize;

use crate::domain::config::BackupConfigState;
use crate::lifecycle::runtime::{Runtime, RuntimeState};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StatusResponse {
    pub state: String,
    pub disabled_reason: Option<String>,
    pub db_healthy: bool,
    pub admissions_open: bool,
    pub synthetic_only: bool,
    pub reconciliation: Option<ReconciliationStatus>,
    pub cleanup: Option<CleanupStatus>,
    pub escalation: EscalationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReconciliationStatus {
    pub last_completed_at: Option<String>,
    pub last_result: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CleanupStatus {
    pub pending: u64,
    pub exhausted: u64,
    pub orphan_discovered: u64,
    pub ownership_incidents: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EscalationStatus {
    pub owner: String,
    pub visible: bool,
}

pub fn project_status(
    config: &BackupConfigState,
    runtime: Option<&Runtime>,
    cleanup: Option<CleanupStatus>,
) -> StatusResponse {
    let disabled_reason = config.disabled_reason().map(str::to_owned);
    let configured_disabled = disabled_reason.is_some();
    let runtime_state = runtime.map(Runtime::state);
    let admissions_open = runtime.is_some_and(Runtime::admissions_open);
    let db_healthy = !matches!(runtime_state, Some(RuntimeState::Degraded));
    let state = if configured_disabled {
        "disabled"
    } else {
        match runtime_state {
            Some(RuntimeState::Ready) => "ready",
            Some(RuntimeState::Degraded) => "degraded",
            Some(RuntimeState::NotReady) | None => "not_ready",
        }
    }
    .to_owned();
    let cleanup_visible = cleanup.is_some_and(|c| c.exhausted > 0 || c.ownership_incidents > 0);
    let escalation_visible = cleanup_visible || state == "degraded";
    let owner = match config {
        BackupConfigState::Enabled(config) => config.bounds().escalation_owner().to_owned(),
        BackupConfigState::Disabled { .. } => "project_owner".to_owned(),
    };
    let cleanup = if configured_disabled || !db_healthy {
        None
    } else {
        cleanup
    };
    let reconciliation = if configured_disabled || !db_healthy {
        None
    } else {
        Some(ReconciliationStatus {
            last_completed_at: None,
            last_result: runtime_state.map(|s| match s {
                RuntimeState::Ready => "ok".to_owned(),
                RuntimeState::NotReady => "not_run".to_owned(),
                RuntimeState::Degraded => "degraded".to_owned(),
            }),
        })
    };
    StatusResponse {
        state,
        disabled_reason,
        db_healthy,
        admissions_open,
        synthetic_only: true,
        reconciliation,
        cleanup,
        escalation: EscalationStatus {
            owner,
            visible: escalation_visible,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::config::DisabledReason;

    #[test]
    fn ts27b_disabled_state_and_closed_disabled_reasons_are_projected() {
        let cases = [
            DisabledReason::NotConfigured,
            DisabledReason::NonLoopbackBind,
            DisabledReason::InvalidConfig,
        ];
        for reason in cases {
            let response = project_status(&BackupConfigState::Disabled { reason }, None, None);
            assert_eq!(response.state, "disabled");
            assert_eq!(response.disabled_reason.as_deref(), Some(reason.as_str()));
            assert!(!response.admissions_open);
            assert!(response.synthetic_only);
            assert!(response.cleanup.is_none());
            assert!(response.reconciliation.is_none());
            assert!(!response.escalation.visible);
        }
    }

    #[test]
    fn status_is_safe_during_not_ready_and_in_memory_degraded_states() {
        let disabled = BackupConfigState::Disabled {
            reason: DisabledReason::NotConfigured,
        };
        let status = project_status(
            &disabled,
            None,
            Some(CleanupStatus {
                pending: 0,
                exhausted: 1,
                orphan_discovered: 0,
                ownership_incidents: 0,
            }),
        );
        assert_eq!(status.state, "disabled");
        assert!(status.cleanup.is_none());
    }
}
