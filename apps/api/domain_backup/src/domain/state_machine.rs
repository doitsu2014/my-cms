//! Pure job state-machine guard. F-15, functional-design 7.1.

use crate::domain::status::BackupJobStatus;

/// True when a U1 job may move from `from` to `to`.
///
/// Exactly four edges exist; `succeeded` and `recovery-required` are unreachable in U1.
pub fn is_allowed(from: BackupJobStatus, to: BackupJobStatus) -> bool {
    use BackupJobStatus::*;
    matches!(
        (from, to),
        (Queued, Running) | (Queued, Failed) | (Running, Incomplete) | (Running, Failed)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use BackupJobStatus::*;

    /// TS-11 (pure part), AC-002 S3, T-06.
    #[test]
    fn no_transition_reaches_succeeded_or_recovery_required() {
        for from in BackupJobStatus::ALL {
            for to in [Succeeded, RecoveryRequired] {
                assert!(
                    !is_allowed(from, to),
                    "{from:?} -> {to:?} must not be allowed for a U1 job"
                );
            }
        }
    }

    /// TS-11: terminal states (incomplete, failed) and the unreachable values have no way out.
    #[test]
    fn terminal_states_have_no_outgoing_transitions() {
        for from in [Incomplete, Failed, Succeeded, RecoveryRequired] {
            for to in BackupJobStatus::ALL {
                assert!(!is_allowed(from, to), "{from:?} -> {to:?} must be rejected");
            }
        }
    }

    /// F-15 / functional-design 7.1: exactly four edges are allowed.
    #[test]
    fn only_the_four_u1_edges_are_allowed() {
        let allowed = [
            (Queued, Running),
            (Queued, Failed),
            (Running, Incomplete),
            (Running, Failed),
        ];
        for (from, to) in allowed {
            assert!(is_allowed(from, to), "{from:?} -> {to:?} must be allowed");
        }
        for from in BackupJobStatus::ALL {
            for to in BackupJobStatus::ALL {
                if !allowed.contains(&(from, to)) {
                    assert!(!is_allowed(from, to), "{from:?} -> {to:?} must be rejected");
                }
            }
        }
    }
}
