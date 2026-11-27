//! Closed, low-cardinality metric labels (FN-36, TS-57).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricJobKind {
    BackupDbSlice,
}

impl MetricJobKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackupDbSlice => "backup_db_slice",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricOutcome {
    Accepted,
    Incomplete,
    Failed,
    Denied,
    Refused,
}

impl MetricOutcome {
    pub const ALL: [Self; 5] = [
        Self::Accepted,
        Self::Incomplete,
        Self::Failed,
        Self::Denied,
        Self::Refused,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Incomplete => "incomplete",
            Self::Failed => "failed",
            Self::Denied => "denied",
            Self::Refused => "refused",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricErrorCode {
    BackupDisabled,
    BackupNotReady,
    StoreUnavailable,
    AuditUnavailable,
    ValidationError,
    Forbidden,
    NotFound,
    Conflict,
    PayloadTooLarge,
    UnsupportedMediaType,
}

impl MetricErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackupDisabled => "BACKUP_DISABLED",
            Self::BackupNotReady => "BACKUP_NOT_READY",
            Self::StoreUnavailable => "STORE_UNAVAILABLE",
            Self::AuditUnavailable => "AUDIT_UNAVAILABLE",
            Self::ValidationError => "MALFORMED_REQUEST",
            Self::Forbidden => "ROLE_REQUIRED",
            Self::NotFound => "JOB_NOT_FOUND",
            Self::Conflict => "CONCURRENCY_LIMIT",
            Self::PayloadTooLarge => "PAYLOAD_TOO_LARGE",
            Self::UnsupportedMediaType => "UNSUPPORTED_MEDIA_TYPE",
        }
    }
}

/// Closed fields only; identifiers, paths, actors and arbitrary strings cannot be labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MetricLabels {
    pub job_kind: MetricJobKind,
    pub outcome: MetricOutcome,
    pub error_code: MetricErrorCode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ts57_metric_label_vocabulary_is_closed_and_identifier_free() {
        let labels = MetricLabels {
            job_kind: MetricJobKind::BackupDbSlice,
            outcome: MetricOutcome::Accepted,
            error_code: MetricErrorCode::ValidationError,
        };
        assert_eq!(labels.job_kind.as_str(), "backup_db_slice");
        assert_eq!(labels.outcome.as_str(), "accepted");
        assert_eq!(labels.error_code.as_str(), "MALFORMED_REQUEST");
        assert_eq!(MetricOutcome::ALL.len(), 5);
    }
}
