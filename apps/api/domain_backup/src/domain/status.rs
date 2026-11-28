//! Job status enum and U1 write/projection guards.

use serde::{Deserialize, Serialize};

/// Job status. Keeps every documented value and adds `incomplete` (ADR-002, D-10).
/// Documented wire spelling: queued, running, succeeded, failed, recovery-required, incomplete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackupJobStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    #[serde(rename = "recovery-required")]
    RecoveryRequired,
    Incomplete,
}

impl BackupJobStatus {
    /// Every value of the enum.
    pub const ALL: [BackupJobStatus; 6] = [
        BackupJobStatus::Queued,
        BackupJobStatus::Running,
        BackupJobStatus::Succeeded,
        BackupJobStatus::Failed,
        BackupJobStatus::RecoveryRequired,
        BackupJobStatus::Incomplete,
    ];
}

/// Job kind. U1 has only the synthetic database slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JobKind {
    BackupDbSlice,
}

/// A status was refused for a U1 job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusRefused(pub BackupJobStatus);

/// U1 jobs never reach `succeeded` or `recovery-required`.
fn guard(kind: JobKind, status: BackupJobStatus) -> Result<BackupJobStatus, StatusRefused> {
    match (kind, status) {
        (
            JobKind::BackupDbSlice,
            BackupJobStatus::Succeeded | BackupJobStatus::RecoveryRequired,
        ) => Err(StatusRefused(status)),
        _ => Ok(status),
    }
}

/// Write guard: the status a persistence write may store for this job kind.
pub fn checked_status_for_write(
    kind: JobKind,
    status: BackupJobStatus,
) -> Result<BackupJobStatus, StatusRefused> {
    guard(kind, status)
}

/// Projection guard: the status the API may emit for this job kind.
pub fn project_status(
    kind: JobKind,
    stored: BackupJobStatus,
) -> Result<BackupJobStatus, StatusRefused> {
    guard(kind, stored)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Compile-time guard: adding a variant breaks this match until it is also added to
    /// `BackupJobStatus::ALL`, which every TS-11 test iterates.
    fn _exhaustive(s: BackupJobStatus) {
        use BackupJobStatus::*;
        match s {
            Queued | Running | Succeeded | Failed | RecoveryRequired | Incomplete => {}
        }
    }

    #[test]
    fn all_lists_every_variant_once() {
        // Structural (passes against the shell by design). Length equals `_exhaustive` arms.
        let set: std::collections::HashSet<_> = BackupJobStatus::ALL.iter().collect();
        assert_eq!(set.len(), 6);
        assert_eq!(BackupJobStatus::ALL.len(), 6);
    }

    const U1_KINDS: [JobKind; 1] = [JobKind::BackupDbSlice];

    fn unreachable_for_u1(s: BackupJobStatus) -> bool {
        matches!(
            s,
            BackupJobStatus::Succeeded | BackupJobStatus::RecoveryRequired
        )
    }

    /// TS-11 (pure part), AC-002 S3, T-06.
    #[test]
    fn no_write_or_projection_yields_succeeded_or_recovery_required() {
        for kind in U1_KINDS {
            for s in BackupJobStatus::ALL {
                let written = checked_status_for_write(kind, s);
                let projected = project_status(kind, s);
                if unreachable_for_u1(s) {
                    assert_eq!(
                        written,
                        Err(StatusRefused(s)),
                        "write of {s:?} must be refused"
                    );
                    assert_eq!(
                        projected,
                        Err(StatusRefused(s)),
                        "projection of {s:?} must be refused"
                    );
                } else {
                    assert_eq!(written, Ok(s), "write of {s:?} must be accepted unchanged");
                    assert_eq!(
                        projected,
                        Ok(s),
                        "projection of {s:?} must be emitted unchanged"
                    );
                }
            }
        }
    }

    /// TS-11 enum contract (IFC 3.1): documented values kept, `incomplete` added,
    /// wire spelling as documented (`recovery-required` via explicit rename).
    #[test]
    fn wire_spelling_keeps_documented_values_and_adds_incomplete() {
        let expected = [
            (BackupJobStatus::Queued, "\"queued\""),
            (BackupJobStatus::Running, "\"running\""),
            (BackupJobStatus::Succeeded, "\"succeeded\""),
            (BackupJobStatus::Failed, "\"failed\""),
            (BackupJobStatus::RecoveryRequired, "\"recovery-required\""),
            (BackupJobStatus::Incomplete, "\"incomplete\""),
        ];
        assert_eq!(expected.len(), BackupJobStatus::ALL.len());
        for (status, wire) in expected {
            assert_eq!(serde_json::to_string(&status).unwrap(), wire);
            assert_eq!(
                serde_json::from_str::<BackupJobStatus>(wire).unwrap(),
                status
            );
        }
        assert!(serde_json::from_str::<BackupJobStatus>("\"unknown\"").is_err());
    }
}
